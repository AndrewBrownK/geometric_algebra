implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<AntiCircleRotor> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<AntiCircleRotor>(this);
    }
}
public extension WeightExpansionInfixPartial<AntiCircleRotor> {
    // Fancy infix trick (second half)
    public CircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension AntiCircleRotor {
    public CircleRotor weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
public extension AntiCircleRotor {
    public DipoleInversion weight_expansion(AntiDipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e1234) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41) + (this.scalar * right_anti_dual.e4125), -(this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.scalar * right_anti_dual.e3215) - (right_anti_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), this.e31 * right_anti_dual.e25)
        );
    }
}
public extension AntiCircleRotor {
    public Motor weight_expansion(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, right_anti_dual.e12345) * float4(this.group0.xyz, this.scalar), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
public extension AntiCircleRotor {
    public Flector weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension AntiCircleRotor {
    public Flector weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4125), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) + float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy), this.scalar * right_anti_dual.e3215) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension AntiCircleRotor {
    public Motor weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiCircleRotor {
    public Motor weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar * right_anti_dual.e415, this.scalar * right_anti_dual.e425, this.scalar * right_anti_dual.e435, -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)) + float4((this.group0.xyz * right_anti_dual.group1.www), this.scalar * right_anti_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e5) * this.group1.xyz)), this.scalar * right_anti_dual.e5)
        );
    }
}
public extension AntiCircleRotor {
    public Plane weight_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual.scalar) * this.group2
        );
    }
}
public extension AntiCircleRotor {
    public DipoleInversion weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.xyz.yzx), this.e23 * right_anti_dual.e15) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), this.e31 * right_anti_dual.e25)
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual.e15 * this.scalar) + (right_anti_dual.scalar * this.e15), (right_anti_dual.e25 * this.scalar) + (right_anti_dual.scalar * this.e25), (right_anti_dual.e35 * this.scalar) + (right_anti_dual.scalar * this.e35), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), right_anti_dual.e31 * this.e25)
        );
    }
}
public extension AntiCircleRotor {
    public CircleRotor weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
public extension AntiCircleRotor {
    public VersorEven weight_expansion(DipoleInversion other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.e42 * right_anti_dual.e3) + (this.e23 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e423), (this.e43 * right_anti_dual.e1) + (this.e31 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e431), (this.e41 * right_anti_dual.e2) + (this.e12 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e412), -(this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) - float4((this.group0.xyz.zxy * right_anti_dual.group3.yzx), this.e41 * right_anti_dual.e235), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e415), (this.e25 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e425), (this.e35 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e435), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + float4((this.group0.xyz * right_anti_dual.group3.www), this.scalar * right_anti_dual.e321) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group3.yzx) - (this.group2.yzx * right_anti_dual.group3.zxy)), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.scalar) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group2.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.scalar * right_anti_dual.e3215)
        );
    }
}
public extension AntiCircleRotor {
    public CircleRotor weight_expansion(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.scalar * right_anti_dual.e321), 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * right_anti_dual.group0.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e45 * right_anti_dual.e321))
        );
    }
}
public extension AntiCircleRotor {
    public VersorEven weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e42 * right_anti_dual.e3, this.e43 * right_anti_dual.e1, this.e41 * right_anti_dual.e2, -(this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e45 * right_anti_dual.e321)) - float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx), this.e41 * right_anti_dual.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + float4((this.group0.xyz * right_anti_dual.group1.www), this.scalar * right_anti_dual.e321) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group1.yzx) - (this.group2.yzx * right_anti_dual.group1.zxy)), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group2.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiCircleRotor {
    public DipoleInversion weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group1.xyz), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e45 * right_anti_dual.e23), (this.e43 * right_anti_dual.e15) + (this.e45 * right_anti_dual.e31), (this.e41 * right_anti_dual.e25) + (this.e45 * right_anti_dual.e12), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group1.xyz.yzx), this.e15 * right_anti_dual.e23)
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.scalar) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(this.scalar) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.scalar) * this.group1.xyz)), this.e45 * right_anti_dual.scalar), 
            /* e15, e25, e35, e1234 */
            float4((this.e15 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e15), (this.e25 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e25), (this.e35 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e35), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e45 * right_anti_dual.e23, this.e45 * right_anti_dual.e31, this.e45 * right_anti_dual.e12, -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + (right_anti_dual.group1.zxyw * float4(this.group0.xyz.yzx, this.scalar)) - (right_anti_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension AntiCircleRotor {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension AntiCircleRotor {
    public AntiDipoleInversion weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * right_anti_dual.e5, this.e42 * right_anti_dual.e5, this.e43 * right_anti_dual.e5, -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * right_anti_dual.group0.yzx), 0.0) - float4((this.group2.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group0
        );
    }
}
public extension AntiCircleRotor {
    public Sphere weight_expansion(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            this.scalar * right_anti_dual.e1234
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
public extension AntiCircleRotor {
    public AntiDipoleInversion weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group0.yzx) - (this.group2.yzx * right_anti_dual.group0.zxy)), this.scalar * right_anti_dual.e4), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(right_anti_dual.group0.xyz, right_anti_dual.e5)
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd weight_expansion(VersorEven other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(this.scalar) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz)), this.scalar * right_anti_dual.scalar), 
            /* e23, e31, e12, e45 */
            (float4(this.scalar) * right_anti_dual.group1) + (float4(right_anti_dual.scalar) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar * right_anti_dual.e15, this.scalar * right_anti_dual.e25, this.scalar * right_anti_dual.e35, -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)) + (this.group2 * float4(right_anti_dual.group0.www, right_anti_dual.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41) + (this.scalar * right_anti_dual.e4125), -(this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.scalar * right_anti_dual.e3215) - (right_anti_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((this.group2.yzx * right_anti_dual.group0.zxy), this.e31 * right_anti_dual.e25)
        );
    }
}
public extension AntiCircleRotor {
    public VersorEven weight_expansion(VersorOdd other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.e23 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e423), (this.e31 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e431), (this.e12 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e412), -(this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) + float4((this.group0.xyz.yzx * right_anti_dual.group3.zxy), this.scalar * right_anti_dual.e12345) - float4((this.group0.xyz.zxy * right_anti_dual.group3.yzx), this.e41 * right_anti_dual.e235), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e415), (this.e25 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e425), (this.e35 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e435), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + float4((this.group0.xyz * right_anti_dual.group2.www), this.scalar * right_anti_dual.e321) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group3.yzx) - (this.group2.yzx * right_anti_dual.group3.zxy)), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<AntiDipoleInversion> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<AntiDipoleInversion>(this);
    }
}
public extension WeightExpansionInfixPartial<AntiDipoleInversion> {
    // Fancy infix trick (second half)
    public Sphere operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension AntiDipoleInversion {
    public Sphere weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group3.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group3.www), this.e1 * right_anti_dual.e235), 
            /* e1234 */
            (this.e4 * right_anti_dual.e321) + (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
    }
}
public extension AntiDipoleInversion {
    public CircleRotor weight_expansion(AntiDipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group3.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group3.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e4235)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) + float4((this.group3.yzx * right_anti_dual.group2.zxy), this.e4 * right_anti_dual.e3215) - (right_anti_dual.group2.yzxx * float4(this.group3.zxy, this.e423))
        );
    }
}
public extension AntiDipoleInversion {
    public Flector weight_expansion(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension AntiDipoleInversion {
    public Motor weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e4 * right_anti_dual.e15, this.e4 * right_anti_dual.e25, this.e4 * right_anti_dual.e35, -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * right_anti_dual.group0.zxy) - (this.group3.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public Motor weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e4) * float4(right_anti_dual.group0.xyz, right_anti_dual.e3215)) + float4(float3(0.0), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * right_anti_dual.group0.zxy) - (this.group3.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public Plane weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group3.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
public extension AntiDipoleInversion {
    public Flector weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) - (this.group3.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar weight_expansion(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            (this.e4 * right_anti_dual.e3215) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public CircleRotor weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group3.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((right_anti_dual.group2.xyz.yzx * this.group3.zxy), this.e423 * right_anti_dual.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((right_anti_dual.e42 * this.e3) + (right_anti_dual.e23 * this.e4) + (right_anti_dual.scalar * this.e423), (right_anti_dual.e43 * this.e1) + (right_anti_dual.e31 * this.e4) + (right_anti_dual.scalar * this.e431), (right_anti_dual.e41 * this.e2) + (right_anti_dual.e12 * this.e4) + (right_anti_dual.scalar * this.e412), -(right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) - float4((right_anti_dual.group0.xyz.zxy * this.group3.yzx), right_anti_dual.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * this.e4) + (right_anti_dual.scalar * this.e415), (right_anti_dual.e25 * this.e4) + (right_anti_dual.scalar * this.e425), (right_anti_dual.e35 * this.e4) + (right_anti_dual.scalar * this.e435), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + float4((right_anti_dual.group0.xyz * this.group3.www), right_anti_dual.scalar * this.e321) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group3.yzx) - (right_anti_dual.group2.yzx * this.group3.zxy)), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public Sphere weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group3.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e235)) - float4((this.group3.yzx * right_anti_dual.group1.zxy), this.e2 * right_anti_dual.e315), 
            /* e1234 */
            (this.e4 * right_anti_dual.e321) + (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion weight_expansion(DipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group3.zxyw * float4(this.group3.yzx, this.e4)) - (this.group3.zxyw * float4(right_anti_dual.group3.yzx, right_anti_dual.e4)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e4 * this.e321) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - float4((this.group3.www * right_anti_dual.group3.xyz), right_anti_dual.e1 * this.e423), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group3.yzxw * this.group1.zxyw) + (this.group2.wwwz * float4(right_anti_dual.group2.xyz, right_anti_dual.e3)) + float4((this.group0.xyz * right_anti_dual.group3.www), right_anti_dual.e1 * this.e235) + float4((right_anti_dual.group1.yzx * this.group3.zxy), right_anti_dual.e2 * this.e315) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((right_anti_dual.group3.zxy * this.group1.yzx), right_anti_dual.e125 * this.e3)
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, right_anti_dual.e3215) * float4(this.group0.xyz, this.e4), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public Sphere weight_expansion(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group2.www * right_anti_dual.group0.xyz), -(this.e1 * right_anti_dual.e235) - (this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)), 
            /* e1234 */
            this.e4 * right_anti_dual.e321
        );
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * right_anti_dual.group1.zxy) - (this.group3.zxy * right_anti_dual.group1.yzx)), this.e4 * right_anti_dual.e5), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3)) + float4((right_anti_dual.group1.www * this.group3.xyz), this.e4 * right_anti_dual.e321) - (right_anti_dual.group1.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e2)) + (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * right_anti_dual.e3) - (this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - float4((this.group1.yzx * right_anti_dual.group1.zxy), this.e1 * right_anti_dual.e235)
        );
    }
}
public extension AntiDipoleInversion {
    public CircleRotor weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(this.e1 * right_anti_dual.e23) - (this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group3.zxy), this.e423 * right_anti_dual.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e4 * right_anti_dual.e23, this.e4 * right_anti_dual.e31, this.e4 * right_anti_dual.e12, -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12)) + float4((this.group0.xyz * right_anti_dual.group0.www), this.e4 * right_anti_dual.e3215), 
            /* e415, e425, e435, e321 */
            float4(this.e4 * right_anti_dual.e15, this.e4 * right_anti_dual.e25, this.e4 * right_anti_dual.e35, -(this.e1 * right_anti_dual.e23) - (this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_anti_dual.group1.zxy) - (this.group3.zxy * right_anti_dual.group1.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * right_anti_dual.group0.zxy) - (this.group3.zxy * right_anti_dual.group0.yzx)), this.e4 * right_anti_dual.e5), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * right_anti_dual.e5, this.e2 * right_anti_dual.e5, this.e3 * right_anti_dual.e5, -(this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * right_anti_dual.e3 * -1.0, this.e435 * right_anti_dual.e1 * -1.0, this.e415 * right_anti_dual.e2 * -1.0, (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar weight_expansion(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiScalar(
            /* e12345 */
            (this.e4 * right_anti_dual.e3215) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)
        );
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * right_anti_dual.group0.zxy), this.e4 * right_anti_dual.e5) - (this.group3.zxyw * right_anti_dual.group0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * right_anti_dual.e5, this.e2 * right_anti_dual.e5, this.e3 * right_anti_dual.e5, -(this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven weight_expansion(VersorEven other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group3.zxyy * float4(right_anti_dual.group0.yzx, right_anti_dual.e4315)) + float4(float3(0.0), (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) + float4((this.group0.xyz * right_anti_dual.group0.www), this.e4 * right_anti_dual.e3215) + float4((this.group2.www * right_anti_dual.group1.xyz), this.e1 * right_anti_dual.e4235) - float4((this.group3.yzx * right_anti_dual.group0.zxy), this.e423 * right_anti_dual.e15), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group1) - (this.group3.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_anti_dual.group2.zxy) - (this.group3.zxy * right_anti_dual.group2.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion weight_expansion(VersorOdd other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * right_anti_dual.group3.zxy), this.e4 * right_anti_dual.e5) - (this.group3.zxyw * right_anti_dual.group3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)) + float4((right_anti_dual.group2.www * this.group3.xyz), this.e4 * right_anti_dual.e321) - (right_anti_dual.group3.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(right_anti_dual.group2.xyz, right_anti_dual.e2)) + (right_anti_dual.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4((this.group3.zxy * right_anti_dual.group1.yzx), this.e125 * right_anti_dual.e3) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group3.yzxz * float4(right_anti_dual.group1.zxy, right_anti_dual.e125)) - float4((this.group1.yzx * right_anti_dual.group3.zxy), this.e1 * right_anti_dual.e235) - float4((right_anti_dual.group3.www * this.group2.xyz), this.e2 * right_anti_dual.e315)
        );
    }
}
public extension AntiDualNum {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<AntiDualNum> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<AntiDualNum>(this);
    }
}
public extension WeightExpansionInfixPartial<AntiDualNum> {
    // Fancy infix trick (second half)
    public CircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension AntiDualNum {
    public CircleRotor weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * right_anti_dual.group2
        );
    }
}
public extension AntiDualNum {
    public DipoleInversion weight_expansion(AntiDipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}
public extension AntiDualNum {
    public DualNum weight_expansion(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
public extension AntiDualNum {
    public FlatPoint weight_expansion(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
public extension AntiDualNum {
    public Flector weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
public extension AntiDualNum {
    public Line weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group1.xyz
        );
    }
}
public extension AntiDualNum {
    public Motor weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
public extension AntiDualNum {
    public Plane weight_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum weight_expansion(AntiScalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
public extension AntiDualNum {
    public Dipole weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_anti_dual.group2.xyz
        );
    }
}
public extension AntiDualNum {
    public VersorOdd weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(right_anti_dual.group0.xyz, right_anti_dual.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xy.yy, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group2.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual.scalar * this.e3215)
        );
    }
}
public extension AntiDualNum {
    public Circle weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group2.xyz
        );
    }
}
public extension AntiDualNum {
    public VersorEven weight_expansion(DipoleInversion other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xy.yy, this.scalar, this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e4), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * float4(right_anti_dual.group2.xyz, right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2((right_anti_dual.e3215 * this.scalar) + (right_anti_dual.scalar * this.e3215), right_anti_dual.scalar * this.scalar)
        );
    }
}
public extension AntiDualNum {
    public AntiFlatPoint weight_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
public extension AntiDualNum {
    public AntiFlector weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
public extension AntiDualNum {
    public AntiLine weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_anti_dual.group1.xyz
        );
    }
}
public extension AntiDualNum {
    public AntiMotor weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e3215)) * float4(right_anti_dual.group1.xyz, 1.0)
        );
    }
}
public extension AntiDualNum {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension AntiDualNum {
    public AntiPlane weight_expansion(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension AntiDualNum {
    public Sphere weight_expansion(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            this.scalar * right_anti_dual.e1234
        );
    }
}
public extension AntiDualNum {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
public extension AntiDualNum {
    public VersorEven weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0
        );
    }
}
public extension AntiDualNum {
    public VersorOdd weight_expansion(VersorEven other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e3215)) * float4(right_anti_dual.group3.xyz, 1.0)
        );
    }
}
public extension AntiDualNum {
    public VersorEven weight_expansion(VersorOdd other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e12345)) * float4(right_anti_dual.group0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}
public extension AntiFlatPoint {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<AntiFlatPoint> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<AntiFlatPoint>(this);
    }
}
public extension WeightExpansionInfixPartial<AntiFlatPoint> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension AntiFlatPoint {
    public AntiScalar weight_expansion(AntiDipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            -(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)
        );
    }
}
public extension AntiFlatPoint {
    public AntiDualNum weight_expansion(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e3215 * this.e321, 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar weight_expansion(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar weight_expansion(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiDualNum weight_expansion(AntiMotor other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e321 * other.e3215, 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint weight_expansion(AntiScalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)
        );
    }
}
public extension AntiFlatPoint {
    public CircleRotor weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.scalar * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.group2.www * this.group0.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e45 * this.e321))
        );
    }
}
public extension AntiFlatPoint {
    public Sphere weight_expansion(DipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.group2.www * this.group0.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e235) + (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)), 
            /* e1234 */
            right_anti_dual.e4 * this.e321 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint weight_expansion(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public AntiDualNum weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint weight_expansion(Motor other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension AntiFlatPoint {
    public AntiDualNum weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public Sphere weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5)), 
            /* e1234 */
            this.e321 * right_anti_dual.e4 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public CircleRotor weight_expansion(VersorEven other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_anti_dual.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45))
        );
    }
}
public extension AntiFlatPoint {
    public Sphere weight_expansion(VersorOdd other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5)), 
            /* e1234 */
            this.e321 * right_anti_dual.e4 * -1.0
        );
    }
}
public extension AntiFlector {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<AntiFlector> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<AntiFlector>(this);
    }
}
public extension WeightExpansionInfixPartial<AntiFlector> {
    // Fancy infix trick (second half)
    public Sphere operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension AntiFlector {
    public Sphere weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group1.www), this.e1 * right_anti_dual.e235), 
            /* e1234 */
            (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
    }
}
public extension AntiFlector {
    public CircleRotor weight_expansion(AntiDipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group1.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group1.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235)) + (this.group1.wwwy * float4(right_anti_dual.group1.xyz, right_anti_dual.e4315)) + float4(float3(0.0), (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - float4((this.group1.zxy * right_anti_dual.group2.yzx), this.e235 * right_anti_dual.e41)
        );
    }
}
public extension AntiFlector {
    public Flector weight_expansion(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e321 * right_anti_dual.e5)
        );
    }
}
public extension AntiFlector {
    public Motor weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e45) * float4(this.group1.xyz, this.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public Motor weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e45) * this.group1.xyz * float3(-1.0)), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) - (this.e321 * right_anti_dual.e45)), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public Plane weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group1.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
public extension AntiFlector {
    public Flector weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) + float4((this.group1.zxy * right_anti_dual.group0.yzx), this.e321 * right_anti_dual.e5) - (this.group1.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
public extension AntiFlector {
    public AntiScalar weight_expansion(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)
        );
    }
}
public extension AntiFlector {
    public AntiFlector weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
public extension AntiFlector {
    public CircleRotor weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group1.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - float4((right_anti_dual.group2.xyz.yzx * this.group1.zxy), this.e235 * right_anti_dual.e41)
        );
    }
}
public extension AntiFlector {
    public VersorEven weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e42 * this.e3, right_anti_dual.e43 * this.e1, right_anti_dual.e41 * this.e2, -(right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e45 * this.e321)) - float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx), right_anti_dual.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + float4((right_anti_dual.group0.xyz * this.group1.www), right_anti_dual.scalar * this.e321) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group1.yzx) - (right_anti_dual.group2.yzx * this.group1.zxy)), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiFlector {
    public Sphere weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group1.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e235)) - float4((this.group1.yzx * right_anti_dual.group1.zxy), this.e2 * right_anti_dual.e315), 
            /* e1234 */
            (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
    }
}
public extension AntiFlector {
    public DipoleInversion weight_expansion(DipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group3.zxy * this.group1.yzx) - (right_anti_dual.group3.yzx * this.group1.zxy)), right_anti_dual.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3)) - float4((this.group1.www * right_anti_dual.group3.xyz), right_anti_dual.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321) - (right_anti_dual.e125 * this.e3)) + float4((right_anti_dual.group1.yzx * this.group1.zxy), right_anti_dual.e1 * this.e235) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (right_anti_dual.group2.wwwy * float4(this.group0.xyz, this.e2)) - (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235))
        );
    }
}
public extension AntiFlector {
    public AntiFlector weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
public extension AntiFlector {
    public AntiDualNum weight_expansion(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiMotor weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual.group1.zxy * this.group1.yzx) - (right_anti_dual.group1.yzx * this.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (right_anti_dual.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)) - (this.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e235))
        );
    }
}
public extension AntiFlector {
    public AntiFlatPoint weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group1.zxyx * float4(right_anti_dual.group1.xyz.yzx, right_anti_dual.e23))
        );
    }
}
public extension AntiFlector {
    public AntiFlector weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group0) - (this.group1.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e23)), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
public extension AntiFlector {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension AntiFlector {
    public AntiMotor weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e5 * right_anti_dual.e1 * -1.0, this.e5 * right_anti_dual.e2 * -1.0, this.e5 * right_anti_dual.e3 * -1.0, (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5)) + (right_anti_dual.group0.wwwx * float4(this.group1.xyz, this.e235))
        );
    }
}
public extension AntiFlector {
    public AntiScalar weight_expansion(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiScalar(
            /* e12345 */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)
        );
    }
}
public extension AntiFlector {
    public DipoleInversion weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)), this.e321 * right_anti_dual.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5))
        );
    }
}
public extension AntiFlector {
    public VersorEven weight_expansion(VersorEven other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group1.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e4235)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e235)), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group0 * float4(this.group1.www, this.e321)) + float4(float3(0.0), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group1.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz) + (this.group1.yzx * right_anti_dual.group2.zxy) - (this.group1.zxy * right_anti_dual.group2.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiFlector {
    public DipoleInversion weight_expansion(VersorOdd other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * right_anti_dual.group3.zxy) - (this.group1.zxy * right_anti_dual.group3.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)) + float4(float3(0.0), (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)) - (right_anti_dual.group3 * float4(this.group1.www, this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5) - (this.e5 * right_anti_dual.e321)) + float4((this.group1.zxy * right_anti_dual.group1.yzx), this.e235 * right_anti_dual.e1) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - (this.group1.wwwz * float4(right_anti_dual.group0.xyz, right_anti_dual.e125)) - float4((right_anti_dual.group3.www * this.group0.xyz), this.e1 * right_anti_dual.e235)
        );
    }
}
public extension AntiLine {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<AntiLine> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<AntiLine>(this);
    }
}
public extension WeightExpansionInfixPartial<AntiLine> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension AntiLine {
    public AntiScalar weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)
        );
    }
}
public extension AntiLine {
    public Sphere weight_expansion(AntiDipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group0.xyz.zxy), this.e23 * right_anti_dual.e15), 
            /* e1234 */
            -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
    }
}
public extension AntiLine {
    public AntiFlatPoint weight_expansion(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public Plane weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35))
        );
    }
}
public extension AntiLine {
    public Plane weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35))
        );
    }
}
public extension AntiLine {
    public AntiScalar weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)
        );
    }
}
public extension AntiLine {
    public Motor weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public AntiLine weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_anti_dual.scalar) * this.group1.xyz
        );
    }
}
public extension AntiLine {
    public Sphere weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group0.xyz.zxy), this.e23 * right_anti_dual.e15), 
            /* e1234 */
            -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
    }
}
public extension AntiLine {
    public DipoleInversion weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group1.xyz), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e45 * this.e23), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e45 * this.e31), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e45 * this.e12), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group1.xyz.yzx), right_anti_dual.e15 * this.e23)
        );
    }
}
public extension AntiLine {
    public AntiScalar weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)
        );
    }
}
public extension AntiLine {
    public CircleRotor weight_expansion(DipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(right_anti_dual.e1 * this.e23) - (right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group3.zxy), right_anti_dual.e423 * this.e15)
        );
    }
}
public extension AntiLine {
    public AntiLine weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_anti_dual.scalar) * this.group1.xyz
        );
    }
}
public extension AntiLine {
    public AntiFlatPoint weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
public extension AntiLine {
    public AntiDualNum weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12), 0.0)
        );
    }
}
public extension AntiLine {
    public AntiMotor weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(right_anti_dual.scalar) * this.group1.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12))
        );
    }
}
public extension AntiLine {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension AntiLine {
    public AntiFlatPoint weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (right_anti_dual.group0.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
public extension AntiLine {
    public Circle weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group0.xyz) + (this.group1.xyz.zxy * right_anti_dual.group0.yzx) - (this.group1.xyz.yzx * right_anti_dual.group0.zxy)
        );
    }
}
public extension AntiLine {
    public DipoleInversion weight_expansion(VersorEven other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group1.xyz), -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group0.zxy), this.e23 * right_anti_dual.e15)
        );
    }
}
public extension AntiLine {
    public CircleRotor weight_expansion(VersorOdd other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) - float4((this.group1.xyz.yzx * right_anti_dual.group3.zxy), this.e23 * right_anti_dual.e415)
        );
    }
}
public extension AntiMotor {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<AntiMotor> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<AntiMotor>(this);
    }
}
public extension WeightExpansionInfixPartial<AntiMotor> {
    // Fancy infix trick (second half)
    public CircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension AntiMotor {
    public CircleRotor weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group0.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
public extension AntiMotor {
    public DipoleInversion weight_expansion(AntiDipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group0.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e1234) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4125), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group1.zxy), this.scalar * right_anti_dual.e3215) - float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx), this.e23 * right_anti_dual.e15)
        );
    }
}
public extension AntiMotor {
    public Motor weight_expansion(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), this.scalar * right_anti_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e5) * this.group0
        );
    }
}
public extension AntiMotor {
    public Flector weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35))
        );
    }
}
public extension AntiMotor {
    public Flector weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * right_anti_dual.e4235, this.scalar * right_anti_dual.e4315, this.scalar * right_anti_dual.e4125, -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) + (this.group0 * float4(right_anti_dual.group0.www, right_anti_dual.e3215))
        );
    }
}
public extension AntiMotor {
    public Motor weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public Motor weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.www * right_anti_dual.group0.xyz), (this.scalar * right_anti_dual.e12345) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e5) * this.group0.xyz)), this.scalar * right_anti_dual.e5)
        );
    }
}
public extension AntiMotor {
    public Plane weight_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
public extension AntiMotor {
    public DipoleInversion weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx), this.e23 * right_anti_dual.e15)
        );
    }
}
public extension AntiMotor {
    public VersorOdd weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(right_anti_dual.group0.xyz, right_anti_dual.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group1.xyz)), right_anti_dual.e45 * this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual.e15 * this.scalar) + (right_anti_dual.scalar * this.e15), (right_anti_dual.e25 * this.scalar) + (right_anti_dual.scalar * this.e25), (right_anti_dual.e35 * this.scalar) + (right_anti_dual.scalar * this.e35), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e45 * this.e23, right_anti_dual.e45 * this.e31, right_anti_dual.e45 * this.e12, -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (this.group1.zxyw * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.scalar)) - (this.group1.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
public extension AntiMotor {
    public CircleRotor weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
public extension AntiMotor {
    public VersorEven weight_expansion(DipoleInversion other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e4 * this.e23, right_anti_dual.e4 * this.e31, right_anti_dual.e4 * this.e12, -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12)) + float4((right_anti_dual.group0.xyz * this.group0.www), right_anti_dual.e4 * this.e3215), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e4 * this.e15, right_anti_dual.e4 * this.e25, right_anti_dual.e4 * this.e35, -(right_anti_dual.e1 * this.e23) - (right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group2.xyz) + (right_anti_dual.group3.yzx * this.group1.zxy) - (right_anti_dual.group3.zxy * this.group1.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)
        );
    }
}
public extension AntiMotor {
    public AntiMotor weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, (right_anti_dual.e3215 * this.scalar) + (right_anti_dual.scalar * this.e3215)) * float4(this.group1.xyz, 1.0)
        );
    }
}
public extension AntiMotor {
    public AntiFlatPoint weight_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
public extension AntiMotor {
    public AntiFlector weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group0) - (right_anti_dual.group1.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
public extension AntiMotor {
    public AntiMotor weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.scalar) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12))
        );
    }
}
public extension AntiMotor {
    public AntiMotor weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.scalar), 
            /* e15, e25, e35, e3215 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12))
        );
    }
}
public extension AntiMotor {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension AntiMotor {
    public AntiFlector weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group0
        );
    }
}
public extension AntiMotor {
    public Sphere weight_expansion(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            this.scalar * right_anti_dual.e1234
        );
    }
}
public extension AntiMotor {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
public extension AntiMotor {
    public VersorEven weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e4) * float4(this.group0.xyz, this.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group0.xyz) + (this.group1.zxy * right_anti_dual.group0.yzx) - (this.group1.yzx * right_anti_dual.group0.zxy)), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0
        );
    }
}
public extension AntiMotor {
    public VersorOdd weight_expansion(VersorEven other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(this.scalar) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz)), this.scalar * right_anti_dual.e45), 
            /* e15, e25, e35, e1234 */
            float4(this.e15 * right_anti_dual.scalar, this.e25 * right_anti_dual.scalar, this.e35 * right_anti_dual.scalar, -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)) + (float4(this.scalar) * right_anti_dual.group2), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e35 * right_anti_dual.e42, this.e15 * right_anti_dual.e43, this.e25 * right_anti_dual.e41, -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + (this.group0 * float4(right_anti_dual.group1.www, right_anti_dual.e3215)) + float4((this.group0.www * right_anti_dual.group3.xyz), this.e3215 * right_anti_dual.scalar) - float4((this.group1.yzx * right_anti_dual.group0.zxy), this.e23 * right_anti_dual.e15)
        );
    }
}
public extension AntiMotor {
    public VersorEven weight_expansion(VersorOdd other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group0 * float4(right_anti_dual.group3.www, right_anti_dual.e12345)) + float4(float3(0.0), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) + float4((this.group0.www * right_anti_dual.group0.xyz), this.e3215 * right_anti_dual.e4), 
            /* e415, e425, e435, e321 */
            float4(this.e15 * right_anti_dual.e4, this.e25 * right_anti_dual.e4, this.e35 * right_anti_dual.e4, -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e5) * this.group0.xyz) + (this.group1.zxy * right_anti_dual.group3.yzx) - (this.group1.yzx * right_anti_dual.group3.zxy)), this.scalar * right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}
public extension AntiPlane {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<AntiPlane> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<AntiPlane>(this);
    }
}
public extension WeightExpansionInfixPartial<AntiPlane> {
    // Fancy infix trick (second half)
    public Sphere operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension AntiPlane {
    public Sphere weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group0.www), this.e1 * right_anti_dual.e235), 
            /* e1234 */
            (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
    }
}
public extension AntiPlane {
    public CircleRotor weight_expansion(AntiDipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(this.e3 * right_anti_dual.e25 * -1.0, this.e1 * right_anti_dual.e35 * -1.0, this.e2 * right_anti_dual.e15 * -1.0, (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)) + (this.group0.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235)) + (this.group0.wwwy * float4(right_anti_dual.group1.xyz, right_anti_dual.e4315))
        );
    }
}
public extension AntiPlane {
    public FlatPoint weight_expansion(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiPlane {
    public Line weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual.e45) * this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)
        );
    }
}
public extension AntiPlane {
    public Motor weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz * float3(-1.0)), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
public extension AntiPlane {
    public Plane weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
public extension AntiPlane {
    public Flector weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
public extension AntiPlane {
    public AntiScalar weight_expansion(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)
        );
    }
}
public extension AntiPlane {
    public AntiPlane weight_expansion(AntiScalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
public extension AntiPlane {
    public Circle weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Circle(
            /* e423, e431, e412 */
            (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.zxy * this.group0.yzx) - (right_anti_dual.group2.xyz.yzx * this.group0.zxy)
        );
    }
}
public extension AntiPlane {
    public AntiDipoleInversion weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e41 * this.e5, right_anti_dual.e42 * this.e5, right_anti_dual.e43 * this.e5, -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e5) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group2.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual.group2.yzx * this.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public Sphere weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e235)) - float4((this.group0.yzx * right_anti_dual.group1.zxy), this.e2 * right_anti_dual.e315), 
            /* e1234 */
            (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)
        );
    }
}
public extension AntiPlane {
    public DipoleInversion weight_expansion(DipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group3.zxy * this.group0.yzx) - (right_anti_dual.group3.yzx * this.group0.zxy)), right_anti_dual.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e1 * this.e5 * -1.0, right_anti_dual.e2 * this.e5 * -1.0, right_anti_dual.e3 * this.e5 * -1.0, (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e425 * this.e3, right_anti_dual.e435 * this.e1, right_anti_dual.e415 * this.e2, -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235))
        );
    }
}
public extension AntiPlane {
    public AntiPlane weight_expansion(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
public extension AntiPlane {
    public AntiDualNum weight_expansion(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5), 0.0)
        );
    }
}
public extension AntiPlane {
    public AntiMotor weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e5 * this.e1, right_anti_dual.e5 * this.e2, right_anti_dual.e5 * this.e3, -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)) - (this.group0.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e235))
        );
    }
}
public extension AntiPlane {
    public AntiFlatPoint weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (this.group0.zxyx * float4(right_anti_dual.group1.xyz.yzx, right_anti_dual.e23))
        );
    }
}
public extension AntiPlane {
    public AntiFlector weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e23)), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension AntiPlane {
    public AntiLine weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            (right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)
        );
    }
}
public extension AntiPlane {
    public AntiScalar weight_expansion(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiScalar(
            /* e12345 */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)
        );
    }
}
public extension AntiPlane {
    public Dipole weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)
        );
    }
}
public extension AntiPlane {
    public VersorEven weight_expansion(VersorEven other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e2 * right_anti_dual.e43 * -1.0, this.e3 * right_anti_dual.e41 * -1.0, this.e1 * right_anti_dual.e42 * -1.0, (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)) + (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e4235)), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (this.group0.yzx * right_anti_dual.group2.zxy) - (this.group0.zxy * right_anti_dual.group2.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiPlane {
    public DipoleInversion weight_expansion(VersorOdd other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * right_anti_dual.e1 * -1.0, this.e5 * right_anti_dual.e2 * -1.0, this.e5 * right_anti_dual.e3 * -1.0, (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)) + (this.group0.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - (this.group0.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e315))
        );
    }
}
public extension AntiScalar {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<AntiScalar> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<AntiScalar>(this);
    }
}
public extension WeightExpansionInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension AntiScalar {
    public AntiScalar weight_expansion(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public AntiScalar weight_expansion(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            float4(other.group2.xyz, other.e12345 * -1.0)[3] * this.e12345
        );
    }
}
public extension AntiScalar {
    public AntiScalar weight_expansion(DualNum other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public AntiScalar weight_expansion(Motor other) {
        return AntiScalar(
            /* e12345 */
            float4(other.group0.xyz, other.e12345 * -1.0)[3] * this.e12345
        );
    }
}
public extension AntiScalar {
    public AntiScalar weight_expansion(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public AntiScalar weight_expansion(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            float4(other.group0.xyz, other.e12345 * -1.0)[3] * this.e12345
        );
    }
}
public extension Circle {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<Circle> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<Circle>(this);
    }
}
public extension WeightExpansionInfixPartial<Circle> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension Circle {
    public AntiScalar weight_expansion(AntiDipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)
        );
    }
}
public extension Circle {
    public Plane weight_expansion(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension Circle {
    public AntiScalar weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)
        );
    }
}
public extension Circle {
    public AntiScalar weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)
        );
    }
}
public extension Circle {
    public Plane weight_expansion(AntiMotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension Circle {
    public Circle weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group2.xyz
        );
    }
}
public extension Circle {
    public AntiScalar weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)
        );
    }
}
public extension Circle {
    public CircleRotor weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
public extension Circle {
    public Sphere weight_expansion(DipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group3.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_anti_dual.group3.yzx * this.group1.zxy), right_anti_dual.e2 * this.e315), 
            /* e1234 */
            -(right_anti_dual.e4 * this.e321) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
    }
}
public extension Circle {
    public Circle weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group2.xyz
        );
    }
}
public extension Circle {
    public Sphere weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group1.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_anti_dual.group1.yzx * this.group1.zxy), right_anti_dual.e2 * this.e315), 
            /* e1234 */
            -(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
    }
}
public extension Circle {
    public AntiScalar weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)
        );
    }
}
public extension Circle {
    public CircleRotor weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
public extension Circle {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension Circle {
    public Sphere weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_anti_dual.group0.yzx * this.group1.zxy), right_anti_dual.e2 * this.e315), 
            /* e1234 */
            -(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
    }
}
public extension Circle {
    public Sphere weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e321 * right_anti_dual.e5) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(right_anti_dual.e5) * this.group0.xyz), this.e235 * right_anti_dual.e1), 
            /* e1234 */
            -(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)
        );
    }
}
public extension Circle {
    public CircleRotor weight_expansion(VersorEven other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43))
        );
    }
}
public extension Circle {
    public Sphere weight_expansion(VersorOdd other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e321 * right_anti_dual.e5) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group2.www), this.e235 * right_anti_dual.e1), 
            /* e1234 */
            -(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)
        );
    }
}
public extension CircleRotor {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<CircleRotor> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<CircleRotor>(this);
    }
}
public extension WeightExpansionInfixPartial<CircleRotor> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension CircleRotor {
    public AntiScalar weight_expansion(AntiDipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)
        );
    }
}
public extension CircleRotor {
    public Plane weight_expansion(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension CircleRotor {
    public AntiScalar weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)
        );
    }
}
public extension CircleRotor {
    public AntiScalar weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)
        );
    }
}
public extension CircleRotor {
    public Plane weight_expansion(AntiMotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension CircleRotor {
    public CircleRotor weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.scalar) * this.group2
        );
    }
}
public extension CircleRotor {
    public AntiScalar weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)
        );
    }
}
public extension CircleRotor {
    public CircleRotor weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.group2.www * this.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
public extension CircleRotor {
    public Sphere weight_expansion(DipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group3.www), right_anti_dual.e1 * this.e235), 
            /* e1234 */
            -(right_anti_dual.e4 * this.e321) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
    }
}
public extension CircleRotor {
    public CircleRotor weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.scalar) * this.group2
        );
    }
}
public extension CircleRotor {
    public Sphere weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group1.www), right_anti_dual.e1 * this.e235), 
            /* e1234 */
            -(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
    }
}
public extension CircleRotor {
    public AntiScalar weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)
        );
    }
}
public extension CircleRotor {
    public CircleRotor weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.group0.www * this.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
public extension CircleRotor {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension CircleRotor {
    public Sphere weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group0.www), right_anti_dual.e1 * this.e235), 
            /* e1234 */
            -(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)
        );
    }
}
public extension CircleRotor {
    public Sphere weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)
        );
    }
}
public extension CircleRotor {
    public CircleRotor weight_expansion(VersorEven other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), (this.e12345 * right_anti_dual.scalar) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43))
        );
    }
}
public extension CircleRotor {
    public Sphere weight_expansion(VersorOdd other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group3.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)
        );
    }
}
public extension Dipole {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<Dipole> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<Dipole>(this);
    }
}
public extension WeightExpansionInfixPartial<Dipole> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension Dipole {
    public AntiScalar weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)
        );
    }
}
public extension Dipole {
    public Sphere weight_expansion(AntiDipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), this.e15 * right_anti_dual.e23) - float4((this.group2.xyz.yzx * right_anti_dual.group0.xyz.zxy), this.e25 * right_anti_dual.e31), 
            /* e1234 */
            -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)
        );
    }
}
public extension Dipole {
    public Line weight_expansion(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual.e5) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.e5) * this.group1.xyz
        );
    }
}
public extension Dipole {
    public Plane weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Dipole {
    public Plane weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension Dipole {
    public AntiScalar weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)
        );
    }
}
public extension Dipole {
    public Motor weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Dipole {
    public Dipole weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(right_anti_dual.scalar) * this.group2.xyz
        );
    }
}
public extension Dipole {
    public Sphere weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.xyz.yzx), right_anti_dual.e15 * this.e23) - float4((right_anti_dual.group2.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e25 * this.e31), 
            /* e1234 */
            -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
    }
}
public extension Dipole {
    public DipoleInversion weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.xyz.yzx), right_anti_dual.e23 * this.e15) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), right_anti_dual.e31 * this.e25)
        );
    }
}
public extension Dipole {
    public AntiScalar weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)
        );
    }
}
public extension Dipole {
    public CircleRotor weight_expansion(DipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((this.group2.xyz.yzx * right_anti_dual.group3.zxy), right_anti_dual.e423 * this.e15)
        );
    }
}
public extension Dipole {
    public Dipole weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(right_anti_dual.scalar) * this.group2.xyz
        );
    }
}
public extension Dipole {
    public AntiScalar weight_expansion(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)
        );
    }
}
public extension Dipole {
    public CircleRotor weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - float4((this.group2.xyz.yzx * right_anti_dual.group1.zxy), right_anti_dual.e235 * this.e41)
        );
    }
}
public extension Dipole {
    public Sphere weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e23 * this.e15), 
            /* e1234 */
            -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
    }
}
public extension Dipole {
    public DipoleInversion weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx), right_anti_dual.e23 * this.e15)
        );
    }
}
public extension Dipole {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension Dipole {
    public Circle weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Circle(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_anti_dual.group0.yzx) - (this.group2.xyz.yzx * right_anti_dual.group0.zxy)
        );
    }
}
public extension Dipole {
    public Circle weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Circle(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_anti_dual.group0.yzx) - (this.group2.xyz.yzx * right_anti_dual.group0.zxy)
        );
    }
}
public extension Dipole {
    public DipoleInversion weight_expansion(VersorEven other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), this.e15 * right_anti_dual.e23) - float4((this.group2.xyz.yzx * right_anti_dual.group0.zxy), this.e25 * right_anti_dual.e31)
        );
    }
}
public extension Dipole {
    public CircleRotor weight_expansion(VersorOdd other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) - float4((this.group2.xyz.yzx * right_anti_dual.group3.zxy), this.e41 * right_anti_dual.e235)
        );
    }
}
public extension DipoleInversion {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<DipoleInversion> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<DipoleInversion>(this);
    }
}
public extension WeightExpansionInfixPartial<DipoleInversion> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension DipoleInversion {
    public AntiScalar weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public Sphere weight_expansion(AntiDipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), right_anti_dual.e31 * this.e25), 
            /* e1234 */
            -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public Motor weight_expansion(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension DipoleInversion {
    public Plane weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension DipoleInversion {
    public Plane weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
public extension DipoleInversion {
    public AntiScalar weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)
        );
    }
}
public extension DipoleInversion {
    public Motor weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz), (this.e1234 * right_anti_dual.e5) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
public extension DipoleInversion {
    public Sphere weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), right_anti_dual.e15 * this.e23) - float4((right_anti_dual.group2.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e25 * this.e31), 
            /* e1234 */
            -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual.group2.www * this.group2.xyz), (right_anti_dual.scalar * this.e1234) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.scalar * this.e3215) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), right_anti_dual.e31 * this.e25)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public CircleRotor weight_expansion(DipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (right_anti_dual.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) + float4((right_anti_dual.group3.yzx * this.group2.zxy), right_anti_dual.e4 * this.e3215) - (this.group2.yzxx * float4(right_anti_dual.group3.zxy, right_anti_dual.e423))
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
public extension DipoleInversion {
    public AntiScalar weight_expansion(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)
        );
    }
}
public extension DipoleInversion {
    public CircleRotor weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (right_anti_dual.group1.yzxx * float4(this.group2.zxy, this.e4235)) + (right_anti_dual.group1.wwwy * float4(this.group1.xyz, this.e4315)) + float4(float3(0.0), (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - float4((right_anti_dual.group1.zxy * this.group2.yzx), right_anti_dual.e235 * this.e41)
        );
    }
}
public extension DipoleInversion {
    public Sphere weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e23 * this.e15), 
            /* e1234 */
            -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual.group0.www * this.group2.xyz), (right_anti_dual.scalar * this.e1234) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group1.zxy), right_anti_dual.scalar * this.e3215) - float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx), right_anti_dual.e23 * this.e15)
        );
    }
}
public extension DipoleInversion {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension DipoleInversion {
    public CircleRotor weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.e3 * this.e25 * -1.0, right_anti_dual.e1 * this.e35 * -1.0, right_anti_dual.e2 * this.e15 * -1.0, (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)) + (right_anti_dual.group0.yzxx * float4(this.group2.zxy, this.e4235)) + (right_anti_dual.group0.wwwy * float4(this.group1.xyz, this.e4315))
        );
    }
}
public extension DipoleInversion {
    public CircleRotor weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(this.e25 * right_anti_dual.e3 * -1.0, this.e35 * right_anti_dual.e1 * -1.0, this.e15 * right_anti_dual.e2 * -1.0, (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)) + (float4(right_anti_dual.e5) * float4(this.group1.xyz, this.e1234)) + (right_anti_dual.group0.yzxx * float4(this.group2.zxy, this.e4235))
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion weight_expansion(VersorEven other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), (this.e1234 * right_anti_dual.scalar) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42) + (this.e4235 * right_anti_dual.scalar), (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43) + (this.e4315 * right_anti_dual.scalar), (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41) + (this.e4125 * right_anti_dual.scalar), -(this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.e3215 * right_anti_dual.scalar) - (right_anti_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((this.group2.yzx * right_anti_dual.group0.zxy), this.e31 * right_anti_dual.e25)
        );
    }
}
public extension DipoleInversion {
    public CircleRotor weight_expansion(VersorOdd other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(right_anti_dual.e5) * float4(this.group1.xyz, this.e1234)) + (right_anti_dual.group3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) - float4((this.group2.yzx * right_anti_dual.group3.zxy), this.e41 * right_anti_dual.e235)
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<DualNum> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<DualNum>(this);
    }
}
public extension WeightExpansionInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public Plane operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension DualNum {
    public Plane weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
public extension DualNum {
    public Motor weight_expansion(AntiDipoleInversion other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e1234), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension DualNum {
    public DualNum weight_expansion(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
public extension DualNum {
    public Line weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.e5) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e5) * right_anti_dual.group1.xyz
        );
    }
}
public extension DualNum {
    public Motor weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, this.e12345) * float4(right_anti_dual.group0.xyz, right_anti_dual.scalar), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.scalar)
        );
    }
}
public extension DualNum {
    public Plane weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
public extension DualNum {
    public Flector weight_expansion(DipoleInversion other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
public extension DualNum {
    public DualNum weight_expansion(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
public extension DualNum {
    public AntiDualNum weight_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e5 * other.e45, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension DualNum {
    public Flector weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual.e321 * this.e5 * -1.0)
        );
    }
}
public extension DualNum {
    public AntiFlatPoint weight_expansion(Line other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension DualNum {
    public Motor weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), right_anti_dual.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * right_anti_dual.group0
        );
    }
}
public extension DualNum {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension DualNum {
    public FlatPoint weight_expansion(Plane other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215).xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension DualNum {
    public AntiScalar weight_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4 * -1.0
        );
    }
}
public extension DualNum {
    public FlatPoint weight_expansion(Sphere other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e1234) * float4(-1.0)
        );
    }
}
public extension DualNum {
    public Motor weight_expansion(VersorEven other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, (this.e5 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.scalar)) * float4(right_anti_dual.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.scalar)
        );
    }
}
public extension DualNum {
    public Flector weight_expansion(VersorOdd other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_anti_dual.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
public extension FlatPoint {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<FlatPoint> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<FlatPoint>(this);
    }
}
public extension WeightExpansionInfixPartial<FlatPoint> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension FlatPoint {
    public AntiScalar weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)
        );
    }
}
public extension FlatPoint {
    public Plane weight_expansion(AntiDipoleInversion other) {
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
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
public extension FlatPoint {
    public FlatPoint weight_expansion(AntiScalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
public extension FlatPoint {
    public Plane weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
public extension FlatPoint {
    public Flector weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
public extension FlatPoint {
    public AntiScalar weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)
        );
    }
}
public extension FlatPoint {
    public Motor weight_expansion(DipoleInversion other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e4 * this.e15, right_anti_dual.e4 * this.e25, right_anti_dual.e4 * this.e35, -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group3.yzx * this.group0.zxy) - (right_anti_dual.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension FlatPoint {
    public FlatPoint weight_expansion(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
public extension FlatPoint {
    public AntiScalar weight_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45 * -1.0
        );
    }
}
public extension FlatPoint {
    public Motor weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45) * float4(right_anti_dual.group1.xyz, right_anti_dual.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension FlatPoint {
    public Plane weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35))
        );
    }
}
public extension FlatPoint {
    public Flector weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35))
        );
    }
}
public extension FlatPoint {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension FlatPoint {
    public Line weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.e45) * right_anti_dual.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)
        );
    }
}
public extension FlatPoint {
    public Line weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Line(
            /* e415, e425, e435 */
            (float3(right_anti_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_anti_dual.group0.xyz), 
            /* e235, e315, e125 */
            (this.group0.zxy * right_anti_dual.group0.yzx) - (this.group0.yzx * right_anti_dual.group0.zxy)
        );
    }
}
public extension FlatPoint {
    public Flector weight_expansion(VersorEven other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_anti_dual.e42) + (this.e45 * right_anti_dual.e23), (this.e15 * right_anti_dual.e43) + (this.e45 * right_anti_dual.e31), (this.e25 * right_anti_dual.e41) + (this.e45 * right_anti_dual.e12), -(this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e23))
        );
    }
}
public extension FlatPoint {
    public Motor weight_expansion(VersorOdd other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e15 * right_anti_dual.e4, this.e25 * right_anti_dual.e4, this.e35 * right_anti_dual.e4, -(this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_anti_dual.group3.yzx) - (this.group0.yzx * right_anti_dual.group3.zxy)), 0.0)
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<Flector> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<Flector>(this);
    }
}
public extension WeightExpansionInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension Flector {
    public AntiScalar weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)
        );
    }
}
public extension Flector {
    public Plane weight_expansion(AntiDipoleInversion other) {
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
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
public extension Flector {
    public Flector weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
public extension Flector {
    public Plane weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
public extension Flector {
    public Flector weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) + float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), right_anti_dual.scalar * this.e3215) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
public extension Flector {
    public AntiScalar weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)
        );
    }
}
public extension Flector {
    public Motor weight_expansion(DipoleInversion other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(right_anti_dual.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group3.yzx * this.group0.zxy) - (right_anti_dual.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public Flector weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
public extension Flector {
    public AntiScalar weight_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45 * -1.0
        );
    }
}
public extension Flector {
    public Motor weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * right_anti_dual.group1.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) - (right_anti_dual.e321 * this.e45)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public Plane weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35))
        );
    }
}
public extension Flector {
    public Flector weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar * this.e4235, right_anti_dual.scalar * this.e4315, right_anti_dual.scalar * this.e4125, -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) + (right_anti_dual.group0 * float4(this.group0.www, this.e3215))
        );
    }
}
public extension Flector {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension Flector {
    public Motor weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public Motor weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45 * right_anti_dual.e1 * -1.0, this.e45 * right_anti_dual.e2 * -1.0, this.e45 * right_anti_dual.e3 * -1.0, (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)) + (right_anti_dual.group0.wwwx * float4(this.group0.xyz, this.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_anti_dual.group0.yzx) - (this.group0.yzx * right_anti_dual.group0.zxy)), 0.0)
        );
    }
}
public extension Flector {
    public Flector weight_expansion(VersorEven other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e45 * right_anti_dual.e23) + (this.e4235 * right_anti_dual.scalar), (this.e45 * right_anti_dual.e31) + (this.e4315 * right_anti_dual.scalar), (this.e45 * right_anti_dual.e12) + (this.e4125 * right_anti_dual.scalar), -(this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + (right_anti_dual.group0.yzxw * float4(this.group0.zxy, this.e3215)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e23))
        );
    }
}
public extension Flector {
    public Motor weight_expansion(VersorOdd other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            (right_anti_dual.group3.wwwx * float4(this.group0.xyz, this.e4235)) + float4(float3(0.0), (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_anti_dual.group3.yzx) - (this.group0.yzx * right_anti_dual.group3.zxy)), 0.0)
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<Line> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<Line>(this);
    }
}
public extension WeightExpansionInfixPartial<Line> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension Line {
    public AntiScalar weight_expansion(AntiDipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)
        );
    }
}
public extension Line {
    public Line weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group1.xyz
        );
    }
}
public extension Line {
    public AntiScalar weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)
        );
    }
}
public extension Line {
    public Motor weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public Plane weight_expansion(DipoleInversion other) {
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
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
public extension Line {
    public Line weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.scalar) * this.group1.xyz
        );
    }
}
public extension Line {
    public Plane weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
public extension Line {
    public AntiScalar weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)
        );
    }
}
public extension Line {
    public Motor weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension Line {
    public Plane weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
public extension Line {
    public Plane weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
public extension Line {
    public Motor weight_expansion(VersorEven other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public Plane weight_expansion(VersorOdd other) {
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
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<Motor> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<Motor>(this);
    }
}
public extension WeightExpansionInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public Plane operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension Motor {
    public Plane weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
public extension Motor {
    public Motor weight_expansion(AntiDipoleInversion other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * right_anti_dual.group0.xyz), (right_anti_dual.e1234 * this.e5) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    public Motor weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
public extension Motor {
    public Motor weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * right_anti_dual.group0.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    public Motor weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.scalar * this.e415, right_anti_dual.scalar * this.e425, right_anti_dual.scalar * this.e435, -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)) + float4((right_anti_dual.group0.xyz * this.group1.www), right_anti_dual.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group1.xyz) + (float3(this.e5) * right_anti_dual.group1.xyz)), right_anti_dual.scalar * this.e5)
        );
    }
}
public extension Motor {
    public Plane weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
public extension Motor {
    public Flector weight_expansion(DipoleInversion other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group3.yzxx * float4(this.group0.zxy, this.e235)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321))
        );
    }
}
public extension Motor {
    public Motor weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
public extension Motor {
    public AntiDualNum weight_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e45 * this.e5, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension Motor {
    public Flector weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) - float4((right_anti_dual.group1.zxy * this.group0.yzx), right_anti_dual.e321 * this.e5)
        );
    }
}
public extension Motor {
    public Motor weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    public Motor weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual.group0.www * this.group0.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group1.xyz) + (float3(this.e5) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.e5)
        );
    }
}
public extension Motor {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension Motor {
    public Flector weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
public extension Motor {
    public AntiScalar weight_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4 * -1.0
        );
    }
}
public extension Motor {
    public Flector weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_anti_dual.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
public extension Motor {
    public Motor weight_expansion(VersorEven other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e1234)) + (float4(right_anti_dual.scalar) * this.group0) + float4(float3(0.0), -(this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group1.xyz)), this.e5 * right_anti_dual.scalar)
        );
    }
}
public extension Motor {
    public Flector weight_expansion(VersorOdd other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_anti_dual.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e235 * right_anti_dual.e4) - (this.e5 * right_anti_dual.e423), -(this.e315 * right_anti_dual.e4) - (this.e5 * right_anti_dual.e431), -(this.e125 * right_anti_dual.e4) - (this.e5 * right_anti_dual.e412), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group3.yzxx * float4(this.group0.zxy, this.e235)) - float4((this.group0.yzx * right_anti_dual.group3.zxy), this.e5 * right_anti_dual.e321)
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<MultiVector> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<MultiVector>(this);
    }
}
public extension WeightExpansionInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiDipoleInversion other) {
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
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(DipoleInversion other) {
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
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(VersorEven other) {
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
        return MultiVector(
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
    }
}
public extension MultiVector {
    public MultiVector weight_expansion(VersorOdd other) {
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
        return MultiVector(
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
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<Plane> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<Plane>(this);
    }
}
public extension WeightExpansionInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Plane operator /(AntiScalar rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension Plane {
    public Plane weight_expansion(AntiScalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
public extension Plane {
    public Plane weight_expansion(CircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group2.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
public extension Plane {
    public AntiScalar weight_expansion(DipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)
        );
    }
}
public extension Plane {
    public Plane weight_expansion(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
public extension Plane {
    public AntiScalar weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)
        );
    }
}
public extension Plane {
    public Plane weight_expansion(Motor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
public extension Plane {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension Plane {
    public AntiScalar weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)
        );
    }
}
public extension Plane {
    public AntiScalar weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return AntiScalar(
            /* e12345 */
            (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)
        );
    }
}
public extension Plane {
    public Plane weight_expansion(VersorEven other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
public extension Plane {
    public AntiScalar weight_expansion(VersorOdd other) {
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
        return AntiScalar(
            /* e12345 */
            (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)
        );
    }
}
public extension RoundPoint {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<RoundPoint> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<RoundPoint>(this);
    }
}
public extension WeightExpansionInfixPartial<RoundPoint> {
    // Fancy infix trick (second half)
    public Sphere operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension RoundPoint {
    public Sphere weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)), 
            /* e1234 */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
    }
}
public extension RoundPoint {
    public CircleRotor weight_expansion(AntiDipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.e25 * this.e3 * -1.0, right_anti_dual.e35 * this.e1 * -1.0, right_anti_dual.e15 * this.e2 * -1.0, (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)) + (float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.e1234)) + (this.group0.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235))
        );
    }
}
public extension RoundPoint {
    public FlatPoint weight_expansion(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group0
        );
    }
}
public extension RoundPoint {
    public Line weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line(
            /* e415, e425, e435 */
            (float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e45) * this.group0.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy)
        );
    }
}
public extension RoundPoint {
    public Motor weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e45 * this.e1 * -1.0, right_anti_dual.e45 * this.e2 * -1.0, right_anti_dual.e45 * this.e3 * -1.0, (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)) + (this.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
public extension RoundPoint {
    public Plane weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
public extension RoundPoint {
    public Flector weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
public extension RoundPoint {
    public AntiScalar weight_expansion(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)
        );
    }
}
public extension RoundPoint {
    public RoundPoint weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e5 */
            this.e5 * right_anti_dual.scalar
        );
    }
}
public extension RoundPoint {
    public Circle weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Circle(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.zxy * this.group0.yzx) - (right_anti_dual.group2.xyz.yzx * this.group0.zxy)
        );
    }
}
public extension RoundPoint {
    public AntiDipoleInversion weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group0.yzx) - (right_anti_dual.group2.yzx * this.group0.zxy)), right_anti_dual.scalar * this.e4), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * float4(this.group0.xyz, this.e5)
        );
    }
}
public extension RoundPoint {
    public Sphere weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e321 * this.e5) - (right_anti_dual.e125 * this.e3)) - (this.group0.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((float3(this.e5) * right_anti_dual.group0.xyz), right_anti_dual.e235 * this.e1), 
            /* e1234 */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
    }
}
public extension RoundPoint {
    public DipoleInversion weight_expansion(DipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group3.zxyw * this.group0.yzxw) - float4((right_anti_dual.group3.yzx * this.group0.zxy), right_anti_dual.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e1 * this.e5 * -1.0, right_anti_dual.e2 * this.e5 * -1.0, right_anti_dual.e3 * this.e5 * -1.0, (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235))
        );
    }
}
public extension RoundPoint {
    public VersorEven weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group0
        );
    }
}
public extension RoundPoint {
    public Sphere weight_expansion(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)), 
            /* e1234 */
            right_anti_dual.e321 * this.e4
        );
    }
}
public extension RoundPoint {
    public DipoleInversion weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group1.xyz)), right_anti_dual.e321 * this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5))
        );
    }
}
public extension RoundPoint {
    public Circle weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Circle(
            /* e423, e431, e412 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.xyz.zxy * this.group0.yzx) - (right_anti_dual.group1.xyz.yzx * this.group0.zxy)
        );
    }
}
public extension RoundPoint {
    public VersorEven weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e4) * float4(right_anti_dual.group0.xyz, right_anti_dual.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy)), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group0
        );
    }
}
public extension RoundPoint {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension RoundPoint {
    public Dipole weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)
        );
    }
}
public extension RoundPoint {
    public AntiScalar weight_expansion(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiScalar(
            /* e12345 */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234)
        );
    }
}
public extension RoundPoint {
    public Dipole weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.group0.zxy * this.group0.yzx), right_anti_dual.e5 * this.e4) - (right_anti_dual.group0.yzxw * float4(this.group0.zxy, this.e5)), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)
        );
    }
}
public extension RoundPoint {
    public VersorEven weight_expansion(VersorEven other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e2 * right_anti_dual.e43 * -1.0, this.e3 * right_anti_dual.e41 * -1.0, this.e1 * right_anti_dual.e42 * -1.0, (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234)) + (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e4235)) + (this.group0.wwwy * float4(right_anti_dual.group1.xyz, right_anti_dual.e4315)), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (this.group0.yzx * right_anti_dual.group2.zxy) - (this.group0.zxy * right_anti_dual.group2.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group0
        );
    }
}
public extension RoundPoint {
    public DipoleInversion weight_expansion(VersorOdd other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e5)) - float4((this.group0.zxy * right_anti_dual.group3.yzx), this.e5 * right_anti_dual.e4), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * right_anti_dual.e1 * -1.0, this.e5 * right_anti_dual.e2 * -1.0, this.e5 * right_anti_dual.e3 * -1.0, (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321)) + (this.group0.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e3 * right_anti_dual.e425) + (this.e4 * right_anti_dual.e235), (this.e1 * right_anti_dual.e435) + (this.e4 * right_anti_dual.e315), (this.e2 * right_anti_dual.e415) + (this.e4 * right_anti_dual.e125), -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((float3(this.e5) * right_anti_dual.group0.xyz), this.e2 * right_anti_dual.e315)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<Scalar> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<Scalar>(this);
    }
}
public extension WeightExpansionInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public CircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension Scalar {
    public CircleRotor weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * right_anti_dual.group2
        );
    }
}
public extension Scalar {
    public DipoleInversion weight_expansion(AntiDipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}
public extension Scalar {
    public DualNum weight_expansion(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
public extension Scalar {
    public FlatPoint weight_expansion(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
public extension Scalar {
    public Flector weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
public extension Scalar {
    public Line weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group1.xyz
        );
    }
}
public extension Scalar {
    public Motor weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
public extension Scalar {
    public Plane weight_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
public extension Scalar {
    public Scalar weight_expansion(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar * -1.0
        );
    }
}
public extension Scalar {
    public Dipole weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_anti_dual.group2.xyz
        );
    }
}
public extension Scalar {
    public AntiCircleRotor weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.scalar) * right_anti_dual.group2
        );
    }
}
public extension Scalar {
    public Circle weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_anti_dual.group2.xyz
        );
    }
}
public extension Scalar {
    public AntiDipoleInversion weight_expansion(DipoleInversion other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}
public extension Scalar {
    public AntiDualNum weight_expansion(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
public extension Scalar {
    public AntiFlatPoint weight_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
public extension Scalar {
    public AntiFlector weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
public extension Scalar {
    public AntiLine weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_anti_dual.group1.xyz
        );
    }
}
public extension Scalar {
    public AntiMotor weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
public extension Scalar {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension Scalar {
    public AntiPlane weight_expansion(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension Scalar {
    public Sphere weight_expansion(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            this.scalar * right_anti_dual.e1234
        );
    }
}
public extension Scalar {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public RoundPoint weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e5 */
            right_anti_dual.e5 * this.scalar
        );
    }
}
public extension Scalar {
    public VersorOdd weight_expansion(VersorEven other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}
public extension Scalar {
    public VersorEven weight_expansion(VersorOdd other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}
public extension Sphere {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<Sphere> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<Sphere>(this);
    }
}
public extension WeightExpansionInfixPartial<Sphere> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDualNum rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension Sphere {
    public AntiScalar weight_expansion(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234
        );
    }
}
public extension Sphere {
    public AntiScalar weight_expansion(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234
        );
    }
}
public extension Sphere {
    public Sphere weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            right_anti_dual.scalar * this.e1234
        );
    }
}
public extension Sphere {
    public Sphere weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            right_anti_dual.scalar * this.e1234
        );
    }
}
public extension Sphere {
    public AntiScalar weight_expansion(DipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public Sphere weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            right_anti_dual.scalar * this.e1234
        );
    }
}
public extension Sphere {
    public AntiScalar weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public Sphere weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            right_anti_dual.scalar * this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension Sphere {
    public AntiScalar weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public AntiScalar weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return AntiScalar(
            /* e12345 */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public Sphere weight_expansion(VersorEven other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234 */
            this.e1234 * right_anti_dual.scalar
        );
    }
}
public extension Sphere {
    public AntiScalar weight_expansion(VersorOdd other) {
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
        return AntiScalar(
            /* e12345 */
            (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5)
        );
    }
}
public extension VersorEven {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<VersorEven> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<VersorEven>(this);
    }
}
public extension WeightExpansionInfixPartial<VersorEven> {
    // Fancy infix trick (second half)
    public Sphere operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension VersorEven {
    public Sphere weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)), 
            /* e1234 */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
    }
}
public extension VersorEven {
    public CircleRotor weight_expansion(AntiDipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.e1234)) + (this.group3.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235)) + float4(float3(0.0), (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) - float4((right_anti_dual.group2.yzx * this.group3.zxy), right_anti_dual.e41 * this.e235)
        );
    }
}
public extension VersorEven {
    public Flector weight_expansion(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
public extension VersorEven {
    public Motor weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e15 * this.e4, right_anti_dual.e25 * this.e4, right_anti_dual.e35 * this.e4, -(right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.zxy * this.group3.yzx) - (right_anti_dual.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
public extension VersorEven {
    public Motor weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            (this.group3.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e4235)) + float4(float3(0.0), (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.zxy * this.group3.yzx) - (right_anti_dual.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
public extension VersorEven {
    public Plane weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (this.group3.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
public extension VersorEven {
    public Flector weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e235 * this.e4) + (right_anti_dual.e5 * this.e423), (right_anti_dual.e315 * this.e4) + (right_anti_dual.e5 * this.e431), (right_anti_dual.e125 * this.e4) + (right_anti_dual.e5 * this.e412), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) + float4((right_anti_dual.group0.yzx * this.group3.zxy), right_anti_dual.e5 * this.e321) - (this.group3.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
public extension VersorEven {
    public AntiScalar weight_expansion(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)
        );
    }
}
public extension VersorEven {
    public VersorEven weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
public extension VersorEven {
    public CircleRotor weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) - float4((right_anti_dual.group2.xyz.yzx * this.group3.zxy), right_anti_dual.e41 * this.e235)
        );
    }
}
public extension VersorEven {
    public VersorEven weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((right_anti_dual.e23 * this.e4) + (right_anti_dual.scalar * this.e423), (right_anti_dual.e31 * this.e4) + (right_anti_dual.scalar * this.e431), (right_anti_dual.e12 * this.e4) + (right_anti_dual.scalar * this.e412), -(right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) + float4((right_anti_dual.group0.xyz.yzx * this.group3.zxy), right_anti_dual.scalar * this.e12345) - float4((right_anti_dual.group0.xyz.zxy * this.group3.yzx), right_anti_dual.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * this.e4) + (right_anti_dual.scalar * this.e415), (right_anti_dual.e25 * this.e4) + (right_anti_dual.scalar * this.e425), (right_anti_dual.e35 * this.e4) + (right_anti_dual.scalar * this.e435), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + float4((right_anti_dual.group0.xyz * this.group2.www), right_anti_dual.scalar * this.e321) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group3.yzx) - (right_anti_dual.group2.yzx * this.group3.zxy)), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
public extension VersorEven {
    public Sphere weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e321 * this.e5) - (right_anti_dual.e125 * this.e3)) - (this.group3.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group2.www), right_anti_dual.e235 * this.e1), 
            /* e1234 */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
    }
}
public extension VersorEven {
    public DipoleInversion weight_expansion(DipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group3.zxyw * this.group3.yzxw) - float4((right_anti_dual.group3.yzx * this.group3.zxy), right_anti_dual.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - float4((this.group2.www * right_anti_dual.group3.xyz), right_anti_dual.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_anti_dual.group1.yzx * this.group3.zxy), right_anti_dual.e1 * this.e235) + float4((this.group3.www * right_anti_dual.group2.xyz), right_anti_dual.e2 * this.e315) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((right_anti_dual.group3.zxy * this.group1.yzx), right_anti_dual.e125 * this.e3)
        );
    }
}
public extension VersorEven {
    public VersorEven weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, (right_anti_dual.e3215 * this.e4) + (right_anti_dual.scalar * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
public extension VersorEven {
    public Sphere weight_expansion(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)), 
            /* e1234 */
            right_anti_dual.e321 * this.e4
        );
    }
}
public extension VersorEven {
    public DipoleInversion weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group1.zxy * this.group3.yzx) - (right_anti_dual.group1.yzx * this.group3.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            (this.group3 * float4(right_anti_dual.group1.www, right_anti_dual.e321)) + float4(float3(0.0), -(right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - (right_anti_dual.group1.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + (right_anti_dual.group1.wwwz * float4(this.group0.xyz, this.e125)) + float4(float3(0.0), (right_anti_dual.e5 * this.e321) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)) + float4((this.group3.www * right_anti_dual.group0.xyz), right_anti_dual.e1 * this.e235) - float4((right_anti_dual.group1.zxy * this.group1.yzx), right_anti_dual.e235 * this.e1)
        );
    }
}
public extension VersorEven {
    public CircleRotor weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) - float4((right_anti_dual.group1.xyz.yzx * this.group3.zxy), right_anti_dual.e23 * this.e415)
        );
    }
}
public extension VersorEven {
    public VersorEven weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual.group0 * float4(this.group3.www, this.e12345)) + float4(float3(0.0), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) + float4((right_anti_dual.group0.www * this.group0.xyz), right_anti_dual.e3215 * this.e4), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e15 * this.e4, right_anti_dual.e25 * this.e4, right_anti_dual.e35 * this.e4, -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + (float4(right_anti_dual.scalar) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.zxy * this.group3.yzx) - (right_anti_dual.group1.yzx * this.group3.zxy)), right_anti_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
public extension VersorEven {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension VersorEven {
    public DipoleInversion weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group0.zxy * this.group3.yzx) - (right_anti_dual.group0.yzx * this.group3.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e5 * this.e1, right_anti_dual.e5 * this.e2, right_anti_dual.e5 * this.e3, -(right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - (right_anti_dual.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235)) + (right_anti_dual.group0.wwwy * float4(this.group0.xyz, this.e315))
        );
    }
}
public extension VersorEven {
    public AntiScalar weight_expansion(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiScalar(
            /* e12345 */
            (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5)
        );
    }
}
public extension VersorEven {
    public DipoleInversion weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.group0.zxy * this.group3.yzx), right_anti_dual.e5 * this.e4) - (right_anti_dual.group0.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e5 * this.e1, right_anti_dual.e5 * this.e2, right_anti_dual.e5 * this.e3, -(right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)) - (right_anti_dual.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e3 * this.e425) - (right_anti_dual.e4 * this.e235), -(right_anti_dual.e1 * this.e435) - (right_anti_dual.e4 * this.e315), -(right_anti_dual.e2 * this.e415) - (right_anti_dual.e4 * this.e125), (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4((float3(right_anti_dual.e5) * this.group0.xyz), right_anti_dual.e2 * this.e315)
        );
    }
}
public extension VersorEven {
    public VersorEven weight_expansion(VersorEven other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual.scalar) * this.group0) + (this.group3.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e4235)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) + float4((this.group3.zxy * right_anti_dual.group0.yzx), this.e5 * right_anti_dual.e1234) - float4((this.group3.yzx * right_anti_dual.group0.zxy), this.e423 * right_anti_dual.e15), 
            /* e415, e425, e435, e321 */
            float4((this.e5 * right_anti_dual.e41) + (this.e4 * right_anti_dual.e15), (this.e5 * right_anti_dual.e42) + (this.e4 * right_anti_dual.e25), (this.e5 * right_anti_dual.e43) + (this.e4 * right_anti_dual.e35), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group1) - (this.group3.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_anti_dual.group2.zxy) - (this.group3.zxy * right_anti_dual.group2.yzx)), this.e5 * right_anti_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
public extension VersorEven {
    public DipoleInversion weight_expansion(VersorOdd other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (this.group3.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e5)) - (right_anti_dual.group3.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)) - (right_anti_dual.group3.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_anti_dual.group1.yzx * this.group3.zxy), right_anti_dual.e5 * this.e321) + float4((right_anti_dual.group2.www * this.group0.xyz), right_anti_dual.e2 * this.e315) + float4((this.group3.www * right_anti_dual.group2.xyz), right_anti_dual.e1 * this.e235) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((right_anti_dual.group3.zxy * this.group1.yzx), right_anti_dual.e315 * this.e2) - float4((right_anti_dual.group3.www * this.group2.xyz), right_anti_dual.e125 * this.e3)
        );
    }
}
public extension VersorOdd {
    // Fancy infix trick (first half)
    public WeightExpansionInfixPartial<VersorOdd> operator /(WeightExpansionInfix rhs) {
        return WeightExpansionInfixPartial<VersorOdd>(this);
    }
}
public extension WeightExpansionInfixPartial<VersorOdd> {
    // Fancy infix trick (second half)
    public CircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_expansion(rhs);
    }
}
public extension VersorOdd {
    public CircleRotor weight_expansion(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), (right_anti_dual.e12345 * this.scalar) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43))
        );
    }
}
public extension VersorOdd {
    public DipoleInversion weight_expansion(AntiDipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), (right_anti_dual.e1234 * this.scalar) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.e4235 * this.scalar), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.e4315 * this.scalar), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.e4125 * this.scalar), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((right_anti_dual.group2.yzx * this.group0.zxy), right_anti_dual.e31 * this.e25)
        );
    }
}
public extension VersorOdd {
    public Motor weight_expansion(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, (right_anti_dual.e5 * this.e1234) + (right_anti_dual.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
public extension VersorOdd {
    public Flector weight_expansion(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e35 * this.e42) + (right_anti_dual.e45 * this.e23), (right_anti_dual.e15 * this.e43) + (right_anti_dual.e45 * this.e31), (right_anti_dual.e25 * this.e41) + (right_anti_dual.e45 * this.e12), -(right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension VersorOdd {
    public Flector weight_expansion(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e45 * this.e23) + (right_anti_dual.e4235 * this.scalar), (right_anti_dual.e45 * this.e31) + (right_anti_dual.e4315 * this.scalar), (right_anti_dual.e45 * this.e12) + (right_anti_dual.e4125 * this.scalar), -(right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (this.group0.yzxw * float4(right_anti_dual.group0.zxy, right_anti_dual.e3215)) - (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
public extension VersorOdd {
    public Motor weight_expansion(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension VersorOdd {
    public Motor weight_expansion(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e1234)) + (float4(this.scalar) * right_anti_dual.group0) + float4(float3(0.0), -(right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group1.xyz)), right_anti_dual.e5 * this.scalar)
        );
    }
}
public extension VersorOdd {
    public Plane weight_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
public extension VersorOdd {
    public VersorOdd weight_expansion(AntiScalar other) {
        Scalar right_anti_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
public extension VersorOdd {
    public DipoleInversion weight_expansion(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), right_anti_dual.e15 * this.e23) - float4((right_anti_dual.group2.xyz.yzx * this.group0.zxy), right_anti_dual.e25 * this.e31)
        );
    }
}
public extension VersorOdd {
    public VersorOdd weight_expansion(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar * this.e15, right_anti_dual.scalar * this.e25, right_anti_dual.scalar * this.e35, -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)) + (right_anti_dual.group2 * float4(this.group0.www, this.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.scalar * this.e3215) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((right_anti_dual.group2.yzx * this.group0.zxy), right_anti_dual.e31 * this.e25)
        );
    }
}
public extension VersorOdd {
    public CircleRotor weight_expansion(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43))
        );
    }
}
public extension VersorOdd {
    public VersorEven weight_expansion(DipoleInversion other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual.group3.zxyy * float4(this.group0.yzx, this.e4315)) + float4(float3(0.0), (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) + float4((right_anti_dual.group0.xyz * this.group0.www), right_anti_dual.e4 * this.e3215) + float4((right_anti_dual.group2.www * this.group1.xyz), right_anti_dual.e1 * this.e4235) - float4((right_anti_dual.group3.yzx * this.group0.zxy), right_anti_dual.e423 * this.e15), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group1) - (right_anti_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group2.xyz) + (right_anti_dual.group3.yzx * this.group2.zxy) - (right_anti_dual.group3.zxy * this.group2.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)
        );
    }
}
public extension VersorOdd {
    public VersorOdd weight_expansion(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, (right_anti_dual.e3215 * this.scalar) + (right_anti_dual.scalar * this.e3215)) * float4(this.group3.xyz, 1.0)
        );
    }
}
public extension VersorOdd {
    public CircleRotor weight_expansion(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e321 * this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45))
        );
    }
}
public extension VersorOdd {
    public VersorEven weight_expansion(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual.group1.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e235)), 
            /* e415, e425, e435, e321 */
            (this.group0 * float4(right_anti_dual.group1.www, right_anti_dual.e321)) + float4(float3(0.0), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.yzx * this.group2.zxy) - (right_anti_dual.group1.zxy * this.group2.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension VersorOdd {
    public DipoleInversion weight_expansion(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group0.zxy), right_anti_dual.e23 * this.e15)
        );
    }
}
public extension VersorOdd {
    public VersorOdd weight_expansion(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(right_anti_dual.scalar) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.e45), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e15 * this.scalar, right_anti_dual.e25 * this.scalar, right_anti_dual.e35 * this.scalar, -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)) + (float4(right_anti_dual.scalar) * this.group2), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e35 * this.e42, right_anti_dual.e15 * this.e43, right_anti_dual.e25 * this.e41, -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (right_anti_dual.group0 * float4(this.group1.www, this.e3215)) + float4((right_anti_dual.group0.www * this.group3.xyz), right_anti_dual.e3215 * this.scalar) - float4((right_anti_dual.group1.yzx * this.group0.zxy), right_anti_dual.e23 * this.e15)
        );
    }
}
public extension VersorOdd {
    public MultiVector weight_expansion(MultiVector other) {
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
        return MultiVector(
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
    }
}
public extension VersorOdd {
    public VersorEven weight_expansion(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e2 * this.e43 * -1.0, right_anti_dual.e3 * this.e41 * -1.0, right_anti_dual.e1 * this.e42 * -1.0, (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)) + (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e4235)), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (right_anti_dual.group0.yzx * this.group2.zxy) - (right_anti_dual.group0.zxy * this.group2.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension VersorOdd {
    public Sphere weight_expansion(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234 */
            right_anti_dual.e1234 * this.scalar
        );
    }
}
public extension VersorOdd {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar
        );
    }
}
public extension VersorOdd {
    public VersorEven weight_expansion(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e2 * this.e43 * -1.0, right_anti_dual.e3 * this.e41 * -1.0, right_anti_dual.e1 * this.e42 * -1.0, (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234)) + (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e4235)) + (right_anti_dual.group0.wwwy * float4(this.group1.xyz, this.e4315)), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (right_anti_dual.group0.yzx * this.group2.zxy) - (right_anti_dual.group0.zxy * this.group2.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0
        );
    }
}
public extension VersorOdd {
    public VersorOdd weight_expansion(VersorEven other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e15, e25, e35, e1234 */
            (float4(right_anti_dual.scalar) * this.group2) + (float4(this.scalar) * right_anti_dual.group2) + float4(float3(0.0), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.e4235 * this.scalar), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.e4315 * this.scalar), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.e4125 * this.scalar), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (right_anti_dual.group0.yzxw * float4(this.group2.zxy, this.e3215)) + float4((right_anti_dual.group0.www * this.group3.xyz), right_anti_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e23)) - float4((right_anti_dual.group2.yzx * this.group0.zxy), right_anti_dual.e31 * this.e25)
        );
    }
}
public extension VersorOdd {
    public VersorEven weight_expansion(VersorOdd other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(this.scalar) * right_anti_dual.group0) + (right_anti_dual.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) + float4((right_anti_dual.group3.zxy * this.group0.yzx), right_anti_dual.e5 * this.e1234) - float4((right_anti_dual.group3.yzx * this.group0.zxy), right_anti_dual.e423 * this.e15), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e5 * this.e41) + (right_anti_dual.e4 * this.e15), (right_anti_dual.e5 * this.e42) + (right_anti_dual.e4 * this.e25), (right_anti_dual.e5 * this.e43) + (right_anti_dual.e4 * this.e35), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group1) - (right_anti_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group2.xyz) + (right_anti_dual.group3.yzx * this.group2.zxy) - (right_anti_dual.group3.zxy * this.group2.yzx)), right_anti_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}

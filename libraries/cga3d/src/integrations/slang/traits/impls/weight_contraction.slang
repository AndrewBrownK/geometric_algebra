implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<AntiCircleRotor> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<AntiCircleRotor>(this);
    }
}
public extension WeightContractionInfixPartial<AntiCircleRotor> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint weight_contraction(AntiDipoleInversion other) {
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((right_anti_dual.group2.www * this.group2.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual.e12345) * this.group2
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435))
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar weight_contraction(DipoleInversion other) {
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
        return Scalar(
            /* scalar */
            -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)
        );
    }
}
public extension AntiCircleRotor {
    public AntiPlane weight_contraction(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e45 * right_anti_dual.e321)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e45 * right_anti_dual.e321)
        );
    }
}
public extension AntiCircleRotor {
    public AntiPlane weight_contraction(Motor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float4(other.group1.xyz, other.e5 * -1.0)[3]) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension AntiCircleRotor {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group9.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_anti_dual.e1234) * this.group2.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e12345) * this.group1.xyz, 
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
    public RoundPoint weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_anti_dual.e1234) * this.group2.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual.e12345) * this.group2
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint weight_contraction(VersorEven other) {
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((right_anti_dual.group2.www * this.group2.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_contraction(VersorOdd other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<AntiDipoleInversion> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<AntiDipoleInversion>(this);
    }
}
public extension WeightContractionInfixPartial<AntiDipoleInversion> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.e4 * right_anti_dual.e12345) - (this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412) + (this.e1 * right_anti_dual.e12345), (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e12345), (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e12345), -(this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + (right_anti_dual.group2.yzxw * float4(this.group0.xyz.zxy, this.e5)) - (right_anti_dual.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), this.e425 * right_anti_dual.e315)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor weight_contraction(AntiDipoleInversion other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group3.yzx) - (this.group0.xyz.yzx * right_anti_dual.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_anti_dual.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((this.group2.zxy * right_anti_dual.group3.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar weight_contraction(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * right_anti_dual.group1.yzx) - (this.group0.xyz.yzx * right_anti_dual.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_anti_dual.e3215, this.e431 * right_anti_dual.e3215, this.e412 * right_anti_dual.e3215, -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_anti_dual.group1.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) - float4((this.group2.zxy * right_anti_dual.group1.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e315) + (this.e321 * right_anti_dual.e415), (this.e423 * right_anti_dual.e125) + (this.e321 * right_anti_dual.e425), (this.e431 * right_anti_dual.e235) + (this.e321 * right_anti_dual.e435), -(this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group1.xyz.zxy), this.e423 * right_anti_dual.e415), 
            /* e5 */
            -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.e4 * right_anti_dual.e12345) - (this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_anti_dual.e415) + (this.e1 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e425) + (this.e2 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e435) + (this.e3 * right_anti_dual.e12345), -(this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx), this.e5 * right_anti_dual.e12345) - (right_anti_dual.group1.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_anti_dual.e3215, this.e431 * right_anti_dual.e3215, this.e412 * right_anti_dual.e3215, -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * right_anti_dual.e4315 * -1.0, this.e235 * right_anti_dual.e4125 * -1.0, this.e315 * right_anti_dual.e4235 * -1.0, (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)) + (float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_anti_dual.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) - float4((this.group0.xyz.yzx * right_anti_dual.group2.xyz.zxy), this.e423 * right_anti_dual.e415) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), this.e431 * right_anti_dual.e425), 
            /* e5 */
            -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint weight_contraction(DipoleInversion other) {
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.e423 * this.e415) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), right_anti_dual.e431 * this.e425), 
            /* e5 */
            -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiMotor weight_contraction(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e4), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiPlane weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321), -(this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125)) - (right_anti_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiPlane weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321), -(this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125)) - (right_anti_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Scalar(
            /* scalar */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiMotor weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz), (this.e4 * right_anti_dual.e3215) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((this.e4 * right_anti_dual.e3215) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) + (float4(right_anti_dual.e12345) * float4(this.group3.xyz, this.e4)) - float4((this.group0.xyz.yzx * right_anti_dual.group8.xyz.zxy), this.e423 * right_anti_dual.e415) - float4((right_anti_dual.group7.xyz.yzx * this.group2.zxy), this.e431 * right_anti_dual.e425), 
            /* e5 */
            (this.e5 * right_anti_dual.e12345) - (this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group9.yzx) - (this.group0.xyz.yzx * right_anti_dual.group9.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e3215) * this.group0.xyz) + (float3(right_anti_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.e12345) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * right_anti_dual.e4315 * -1.0, this.e235 * right_anti_dual.e4125 * -1.0, this.e315 * right_anti_dual.e4235 * -1.0, (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)) + (float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_anti_dual.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor weight_contraction(VersorEven other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group3.yzx) - (this.group0.xyz.yzx * right_anti_dual.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_anti_dual.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((this.group2.zxy * right_anti_dual.group3.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_contraction(VersorOdd other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.e4 * right_anti_dual.e12345) - (this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412) + (this.e1 * right_anti_dual.e12345), (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e12345), (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e12345), -(this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), this.e5 * right_anti_dual.e12345) - (right_anti_dual.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((this.group2.zxy * right_anti_dual.group0.yzx), this.e425 * right_anti_dual.e315)
        );
    }
}
public extension AntiDualNum {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<AntiDualNum> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<AntiDualNum>(this);
    }
}
public extension WeightContractionInfixPartial<AntiDualNum> {
    // Fancy infix trick (second half)
    public AntiMotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension AntiDualNum {
    public AntiMotor weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.xx, this.e3215, this.scalar) * float4(right_anti_dual.group0.xyz, right_anti_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e12345)
        );
    }
}
public extension AntiDualNum {
    public AntiFlector weight_contraction(AntiDipoleInversion other) {
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
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group3.xyz, right_anti_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum weight_contraction(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension AntiDualNum {
    public DualNum weight_contraction(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e3215, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public AntiFlector weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e45 * -1.0)
        );
    }
}
public extension AntiDualNum {
    public FlatPoint weight_contraction(AntiLine other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public AntiMotor weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.scalar * right_anti_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * right_anti_dual.group0
        );
    }
}
public extension AntiDualNum {
    public AntiFlatPoint weight_contraction(AntiPlane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public AntiPlane weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiPlane weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiLine weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e3215) * right_anti_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e3215) * right_anti_dual.group1.xyz
        );
    }
}
public extension AntiDualNum {
    public AntiMotor weight_contraction(DipoleInversion other) {
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
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e4), 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((this.e3215 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group4.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e5 */
            this.e3215 * right_anti_dual.e45 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e3215) * right_anti_dual.group7.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e3215) * right_anti_dual.group9.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiDualNum {
    public AntiFlatPoint weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e1234) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum weight_contraction(Scalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension AntiDualNum {
    public Scalar weight_contraction(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
public extension AntiDualNum {
    public AntiFlector weight_contraction(VersorEven other) {
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
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group3.xyz, right_anti_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiMotor weight_contraction(VersorOdd other) {
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
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.xx, this.e3215, (this.e3215 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e12345)) * float4(right_anti_dual.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e12345)
        );
    }
}
public extension AntiFlatPoint {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<AntiFlatPoint> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<AntiFlatPoint>(this);
    }
}
public extension WeightContractionInfixPartial<AntiFlatPoint> {
    // Fancy infix trick (second half)
    public AntiFlector operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension AntiFlatPoint {
    public AntiFlector weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_anti_dual.e412) + (this.e321 * right_anti_dual.e415), (this.e125 * right_anti_dual.e423) + (this.e321 * right_anti_dual.e425), (this.e235 * right_anti_dual.e431) + (this.e321 * right_anti_dual.e435), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
public extension AntiFlatPoint {
    public AntiMotor weight_contraction(AntiDipoleInversion other) {
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
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e235 * right_anti_dual.e1234, this.e315 * right_anti_dual.e1234, this.e125 * right_anti_dual.e1234, -(this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - float4((this.group0.www * right_anti_dual.group3.xyz), this.e235 * right_anti_dual.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx)), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint weight_contraction(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public Scalar weight_contraction(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiMotor weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(right_anti_dual.group1.xyz, right_anti_dual.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group1.zxy) - (this.group0.zxy * right_anti_dual.group1.yzx)), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiPlane weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * right_anti_dual.group0.xyz), -(this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435))
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlector weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.group0.www * right_anti_dual.group0.xyz), -(this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435))
        );
    }
}
public extension AntiFlatPoint {
    public AntiLine weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e321) * right_anti_dual.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)
        );
    }
}
public extension AntiFlatPoint {
    public Scalar weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)
        );
    }
}
public extension AntiFlatPoint {
    public Scalar weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e45 * this.e321)
        );
    }
}
public extension AntiFlatPoint {
    public AntiPlane weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_anti_dual.e412) + (this.e321 * right_anti_dual.e415), (this.e125 * right_anti_dual.e423) + (this.e321 * right_anti_dual.e425), (this.e235 * right_anti_dual.e431) + (this.e321 * right_anti_dual.e435), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
public extension AntiFlatPoint {
    public AntiPlane weight_contraction(DipoleInversion other) {
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
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321), -(right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125)) - (this.group0.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e415))
        );
    }
}
public extension AntiFlatPoint {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2(-(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_anti_dual.group6.xyz), 0.0) + float4((right_anti_dual.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * right_anti_dual.group9.zxy) - (this.group0.zxy * right_anti_dual.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_anti_dual.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiLine weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiLine(
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_anti_dual.group0.xyz), 
            /* e15, e25, e35 */
            (this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint weight_contraction(Scalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public AntiMotor weight_contraction(VersorEven other) {
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
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e235 * right_anti_dual.e1234, this.e315 * right_anti_dual.e1234, this.e125 * right_anti_dual.e1234, -(this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx)), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlector weight_contraction(VersorOdd other) {
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
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_anti_dual.e412) + (this.e321 * right_anti_dual.e415), (this.e125 * right_anti_dual.e423) + (this.e321 * right_anti_dual.e425), (this.e235 * right_anti_dual.e431) + (this.e321 * right_anti_dual.e435), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e415))
        );
    }
}
public extension AntiFlector {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<AntiFlector> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<AntiFlector>(this);
    }
}
public extension WeightContractionInfixPartial<AntiFlector> {
    // Fancy infix trick (second half)
    public AntiFlector operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension AntiFlector {
    public AntiFlector weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_anti_dual.e415) + (this.e1 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e425) + (this.e2 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e435) + (this.e3 * right_anti_dual.e12345), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + float4((right_anti_dual.group0.xyz.zxy * this.group0.yzx), this.e5 * right_anti_dual.e12345) - float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
public extension AntiFlector {
    public AntiMotor weight_contraction(AntiDipoleInversion other) {
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
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) + float4((right_anti_dual.group2.www * this.group0.xyz), this.e1 * right_anti_dual.e4235) - float4((this.group0.www * right_anti_dual.group3.xyz), this.e235 * right_anti_dual.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
public extension AntiFlector {
    public Scalar weight_contraction(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0
        );
    }
}
public extension AntiFlector {
    public AntiMotor weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * right_anti_dual.group1.xyz * float3(-1.0)), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) - (this.e321 * right_anti_dual.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group1.zxy) - (this.group0.zxy * right_anti_dual.group1.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiPlane weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * right_anti_dual.group0.xyz), -(this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435))
        );
    }
}
public extension AntiFlector {
    public AntiFlector weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e1 * right_anti_dual.e12345, this.e2 * right_anti_dual.e12345, this.e3 * right_anti_dual.e12345, -(this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + (right_anti_dual.group0 * float4(this.group0.www, this.e5))
        );
    }
}
public extension AntiFlector {
    public AntiMotor weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * right_anti_dual.group0.xyz * float3(-1.0)), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public Scalar weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)
        );
    }
}
public extension AntiFlector {
    public Scalar weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e45 * this.e321)
        );
    }
}
public extension AntiFlector {
    public AntiPlane weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_anti_dual.e412) + (this.e321 * right_anti_dual.e415), (this.e125 * right_anti_dual.e423) + (this.e321 * right_anti_dual.e425), (this.e235 * right_anti_dual.e431) + (this.e321 * right_anti_dual.e435), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
public extension AntiFlector {
    public AntiPlane weight_contraction(DipoleInversion other) {
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
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321), -(right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125)) - (this.group0.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e415))
        );
    }
}
public extension AntiFlector {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_anti_dual.group6.xyz), 0.0) + float4((float3(right_anti_dual.e12345) * this.group1.xyz), 0.0) + float4((right_anti_dual.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (this.e5 * right_anti_dual.e12345) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * right_anti_dual.group9.zxy) - (this.group0.zxy * right_anti_dual.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_anti_dual.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiMotor weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e321 * right_anti_dual.e4235 * -1.0, this.e321 * right_anti_dual.e4315 * -1.0, this.e321 * right_anti_dual.e4125 * -1.0, (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)) + float4((float3(right_anti_dual.e1234) * this.group0.xyz), this.e1 * right_anti_dual.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
public extension AntiFlector {
    public AntiMotor weight_contraction(VersorEven other) {
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
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) + float4((right_anti_dual.group2.www * this.group0.xyz), this.e1 * right_anti_dual.e4235) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector weight_contraction(VersorOdd other) {
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
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_anti_dual.e415) + (this.e1 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e425) + (this.e2 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e435) + (this.e3 * right_anti_dual.e12345), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + (right_anti_dual.group0.zxyw * float4(this.group0.yzx, this.e5)) - (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e415))
        );
    }
}
public extension AntiLine {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<AntiLine> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<AntiLine>(this);
    }
}
public extension WeightContractionInfixPartial<AntiLine> {
    // Fancy infix trick (second half)
    public AntiMotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension AntiLine {
    public AntiMotor weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public AntiPlane weight_contraction(AntiDipoleInversion other) {
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
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public AntiLine weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_anti_dual.e12345) * this.group1.xyz
        );
    }
}
public extension AntiLine {
    public AntiPlane weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_anti_dual.e4315 * -1.0, this.e23 * right_anti_dual.e4125 * -1.0, this.e31 * right_anti_dual.e4235 * -1.0, (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group1.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public Scalar weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)
        );
    }
}
public extension AntiLine {
    public AntiMotor weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public AntiPlane weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_anti_dual.e4315 * -1.0, this.e23 * right_anti_dual.e4125 * -1.0, this.e31 * right_anti_dual.e4235 * -1.0, (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public Scalar weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)
        );
    }
}
public extension AntiLine {
    public Scalar weight_contraction(DipoleInversion other) {
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
        return Scalar(
            /* scalar */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12)
        );
    }
}
public extension AntiLine {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2(-(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.xyz.yzx * right_anti_dual.group9.zxy), 0.0) - float4((float3(right_anti_dual.e1234) * this.group1.xyz), 0.0) - float4((this.group0.xyz.zxy * right_anti_dual.group9.yzx), 0.0), 
            /* e5 */
            (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
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
    public AntiPlane weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public AntiLine weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_anti_dual.e12345) * this.group1.xyz
        );
    }
}
public extension AntiLine {
    public AntiPlane weight_contraction(VersorEven other) {
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
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public AntiMotor weight_contraction(VersorOdd other) {
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
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<AntiMotor> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<AntiMotor>(this);
    }
}
public extension WeightContractionInfixPartial<AntiMotor> {
    // Fancy infix trick (second half)
    public AntiMotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension AntiMotor {
    public AntiMotor weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23 * right_anti_dual.e12345, this.e31 * right_anti_dual.e12345, this.e12 * right_anti_dual.e12345, -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) + float4((right_anti_dual.group0.xyz * this.group1.www), this.scalar * right_anti_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz)), this.e3215 * right_anti_dual.e12345)
        );
    }
}
public extension AntiMotor {
    public AntiFlector weight_contraction(AntiDipoleInversion other) {
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
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group3.xyz, right_anti_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((right_anti_dual.group0.xyz * this.group1.www), this.e3215 * right_anti_dual.e45)
        );
    }
}
public extension AntiMotor {
    public AntiMotor weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
public extension AntiMotor {
    public DualNum weight_contraction(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e3215, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiFlector weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            (right_anti_dual.group1.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) - float4((this.group0.zxy * right_anti_dual.group1.yzx), this.e3215 * right_anti_dual.e45)
        );
    }
}
public extension AntiMotor {
    public AntiMotor weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz), (this.scalar * right_anti_dual.e12345) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz)), this.e3215 * right_anti_dual.e12345)
        );
    }
}
public extension AntiMotor {
    public AntiFlector weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_anti_dual.e4315 * -1.0, this.e23 * right_anti_dual.e4125 * -1.0, this.e31 * right_anti_dual.e4235 * -1.0, (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
public extension AntiMotor {
    public AntiPlane weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.group0.xyz * this.group1.www * float3(-1.0)), this.e3215 * right_anti_dual.e45 * -1.0)
        );
    }
}
public extension AntiMotor {
    public AntiPlane weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45) * float4(-1.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * right_anti_dual.group0.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor weight_contraction(DipoleInversion other) {
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
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * right_anti_dual.group0.xyz), (right_anti_dual.e4 * this.e3215) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((this.scalar * right_anti_dual.e12345) + (this.e3215 * right_anti_dual.e4) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.yzx * right_anti_dual.group9.zxy), 0.0) - float4((float3(this.e3215) * right_anti_dual.group4.xyz), 0.0) - float4((float3(right_anti_dual.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * right_anti_dual.group9.yzx), 0.0), 
            /* e5 */
            (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) - (this.e3215 * right_anti_dual.e45), 
            /* e15, e25, e35, e45 */
            float4(((float3(this.e3215) * right_anti_dual.group6.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_anti_dual.group7.xyz) + (float3(right_anti_dual.e12345) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e3215) * right_anti_dual.group9.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public AntiFlector weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
public extension AntiMotor {
    public AntiMotor weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
public extension AntiMotor {
    public Scalar weight_contraction(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
public extension AntiMotor {
    public AntiFlector weight_contraction(VersorEven other) {
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
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group3.xyz, right_anti_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e15 * right_anti_dual.e1234) - (this.e3215 * right_anti_dual.e41), -(this.e25 * right_anti_dual.e1234) - (this.e3215 * right_anti_dual.e42), -(this.e35 * right_anti_dual.e1234) - (this.e3215 * right_anti_dual.e43), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((this.group0.zxy * right_anti_dual.group3.yzx), this.e3215 * right_anti_dual.e45)
        );
    }
}
public extension AntiMotor {
    public AntiMotor weight_contraction(VersorOdd other) {
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
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e4)) + (float4(right_anti_dual.e12345) * this.group0) + float4(float3(0.0), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz)), this.e3215 * right_anti_dual.e12345)
        );
    }
}
public extension AntiPlane {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<AntiPlane> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<AntiPlane>(this);
    }
}
public extension WeightContractionInfixPartial<AntiPlane> {
    // Fancy infix trick (second half)
    public AntiPlane operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension AntiPlane {
    public AntiPlane weight_contraction(AntiCircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public Scalar weight_contraction(AntiDipoleInversion other) {
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
        return Scalar(
            /* scalar */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)
        );
    }
}
public extension AntiPlane {
    public AntiPlane weight_contraction(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public Scalar weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)
        );
    }
}
public extension AntiPlane {
    public AntiPlane weight_contraction(AntiMotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public Scalar weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)
        );
    }
}
public extension AntiPlane {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            this.e5 * right_anti_dual.e12345, 
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
    public Scalar weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Scalar(
            /* scalar */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)
        );
    }
}
public extension AntiPlane {
    public AntiPlane weight_contraction(Scalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public Scalar weight_contraction(VersorEven other) {
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
        return Scalar(
            /* scalar */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)
        );
    }
}
public extension AntiPlane {
    public AntiPlane weight_contraction(VersorOdd other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiScalar {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<AntiScalar> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<AntiScalar>(this);
    }
}
public extension WeightContractionInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public CircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension AntiScalar {
    public CircleRotor weight_contraction(AntiCircleRotor other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.e12345) * right_anti_dual.group2
        );
    }
}
public extension AntiScalar {
    public DipoleInversion weight_contraction(AntiDipoleInversion other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension AntiScalar {
    public DualNum weight_contraction(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345) * other.group0.xy
        );
    }
}
public extension AntiScalar {
    public FlatPoint weight_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
public extension AntiScalar {
    public Flector weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
public extension AntiScalar {
    public Line weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_anti_dual.group1.xyz
        );
    }
}
public extension AntiScalar {
    public Motor weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
public extension AntiScalar {
    public Plane weight_contraction(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
public extension AntiScalar {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public Dipole weight_contraction(Circle other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35 */
            float3(this.e12345) * right_anti_dual.group2.xyz
        );
    }
}
public extension AntiScalar {
    public AntiCircleRotor weight_contraction(CircleRotor other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * right_anti_dual.group2
        );
    }
}
public extension AntiScalar {
    public Circle weight_contraction(Dipole other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_anti_dual.group2.xyz
        );
    }
}
public extension AntiScalar {
    public AntiDipoleInversion weight_contraction(DipoleInversion other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * right_anti_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension AntiScalar {
    public AntiDualNum weight_contraction(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
public extension AntiScalar {
    public AntiFlatPoint weight_contraction(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
public extension AntiScalar {
    public AntiFlector weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
public extension AntiScalar {
    public AntiLine weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * right_anti_dual.group1.xyz
        );
    }
}
public extension AntiScalar {
    public AntiMotor weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
public extension AntiScalar {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2(this.e12345) * right_anti_dual.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e5 */
            this.e12345 * right_anti_dual.e5, 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_anti_dual.group3, 
            /* e41, e42, e43 */
            float3(this.e12345) * right_anti_dual.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * right_anti_dual.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group6, 
            /* e423, e431, e412 */
            float3(this.e12345) * right_anti_dual.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_anti_dual.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group9, 
            /* e1234 */
            this.e12345 * right_anti_dual.e1234
        );
    }
}
public extension AntiScalar {
    public AntiPlane weight_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension AntiScalar {
    public Sphere weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1234 */
            this.e12345 * right_anti_dual.e1234
        );
    }
}
public extension AntiScalar {
    public AntiScalar weight_contraction(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar
        );
    }
}
public extension AntiScalar {
    public RoundPoint weight_contraction(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5 */
            this.e12345 * right_anti_dual.e5
        );
    }
}
public extension AntiScalar {
    public VersorOdd weight_contraction(VersorEven other) {
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
            float4(this.e12345) * right_anti_dual.group0, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension AntiScalar {
    public VersorEven weight_contraction(VersorOdd other) {
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
            float4(this.e12345) * right_anti_dual.group0, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * right_anti_dual.group2, 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension Circle {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<Circle> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<Circle>(this);
    }
}
public extension WeightContractionInfixPartial<Circle> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension Circle {
    public AntiDipoleInversion weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e125 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.e235 * right_anti_dual.e415) - float4((this.group2.xyz.zxy * right_anti_dual.group0.xyz.yzx), this.e315 * right_anti_dual.e425)
        );
    }
}
public extension Circle {
    public AntiCircleRotor weight_contraction(AntiDipoleInversion other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group3.yzx) - (this.group0.xyz.yzx * right_anti_dual.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((this.group2.xyz.zxy * right_anti_dual.group3.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
public extension Circle {
    public Circle weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(right_anti_dual.e12345) * this.group2.xyz
        );
    }
}
public extension Circle {
    public Scalar weight_contraction(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)
        );
    }
}
public extension Circle {
    public AntiCircleRotor weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * right_anti_dual.group1.yzx) - (this.group0.xyz.yzx * right_anti_dual.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_anti_dual.e3215, this.e431 * right_anti_dual.e3215, this.e412 * right_anti_dual.e3215, -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) - float4((this.group2.xyz.zxy * right_anti_dual.group1.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
public extension Circle {
    public RoundPoint weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e315) + (this.e321 * right_anti_dual.e415), (this.e423 * right_anti_dual.e125) + (this.e321 * right_anti_dual.e425), (this.e431 * right_anti_dual.e235) + (this.e321 * right_anti_dual.e435), -(this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group1.xyz.zxy), this.e423 * right_anti_dual.e415), 
            /* e5 */
            -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)
        );
    }
}
public extension Circle {
    public AntiDipoleInversion weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e321 * right_anti_dual.e415), (this.e423 * right_anti_dual.e125) + (this.e321 * right_anti_dual.e425), (this.e431 * right_anti_dual.e235) + (this.e321 * right_anti_dual.e435), -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group1.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
public extension Circle {
    public Dipole weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_anti_dual.e3215, this.e431 * right_anti_dual.e3215, this.e412 * right_anti_dual.e3215, -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * right_anti_dual.group0.zxy) - (this.group2.xyz.zxy * right_anti_dual.group0.yzx)
        );
    }
}
public extension Circle {
    public Scalar weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)
        );
    }
}
public extension Circle {
    public Scalar weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)
        );
    }
}
public extension Circle {
    public RoundPoint weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.xyz.zxy), right_anti_dual.e423 * this.e415) - float4((right_anti_dual.group2.xyz.zxy * this.group0.xyz.yzx), right_anti_dual.e431 * this.e425), 
            /* e5 */
            -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)
        );
    }
}
public extension Circle {
    public RoundPoint weight_contraction(DipoleInversion other) {
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.xyz.zxy), right_anti_dual.e423 * this.e415) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), right_anti_dual.e431 * this.e425), 
            /* e5 */
            -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)
        );
    }
}
public extension Circle {
    public AntiLine weight_contraction(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual.e3215) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_anti_dual.e3215) * this.group1.xyz
        );
    }
}
public extension Circle {
    public AntiPlane weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e315 * this.e412) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e125 * this.e423) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e235 * this.e431) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
public extension Circle {
    public AntiPlane weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e315 * this.e412) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e125 * this.e423) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e235 * this.e431) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
public extension Circle {
    public Scalar weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)
        );
    }
}
public extension Circle {
    public AntiMotor weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Circle {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) - float4((this.group0.xyz.yzx * right_anti_dual.group8.xyz.zxy), this.e423 * right_anti_dual.e415) - float4((this.group2.xyz.zxy * right_anti_dual.group7.xyz.yzx), this.e431 * right_anti_dual.e425), 
            /* e5 */
            -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group9.yzxx * float4(this.group2.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group9.yzx) - (this.group0.xyz.yzx * right_anti_dual.group9.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e3215) * this.group0.xyz) + (float3(right_anti_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.e12345) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Circle {
    public Dipole weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * right_anti_dual.group0.zxy) - (this.group2.xyz.zxy * right_anti_dual.group0.yzx)
        );
    }
}
public extension Circle {
    public Circle weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(right_anti_dual.e12345) * this.group2.xyz
        );
    }
}
public extension Circle {
    public AntiCircleRotor weight_contraction(VersorEven other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group3.yzx) - (this.group0.xyz.yzx * right_anti_dual.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((this.group2.xyz.zxy * right_anti_dual.group3.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
public extension Circle {
    public AntiDipoleInversion weight_contraction(VersorOdd other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e125 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.e235 * right_anti_dual.e415) - float4((this.group2.xyz.zxy * right_anti_dual.group0.yzx), this.e315 * right_anti_dual.e425)
        );
    }
}
public extension CircleRotor {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<CircleRotor> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<CircleRotor>(this);
    }
}
public extension WeightContractionInfixPartial<CircleRotor> {
    // Fancy infix trick (second half)
    public VersorEven operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension CircleRotor {
    public VersorEven weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_anti_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz)), right_anti_dual.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(right_anti_dual.e12345) * this.group1) + (float4(this.e12345) * right_anti_dual.group1), 
            /* e235, e315, e125, e5 */
            float4((right_anti_dual.e235 * this.e12345) + (right_anti_dual.e12345 * this.e235), (right_anti_dual.e315 * this.e12345) + (right_anti_dual.e12345 * this.e315), (right_anti_dual.e125 * this.e12345) + (right_anti_dual.e12345 * this.e125), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.e423 * this.e415) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), right_anti_dual.e431 * this.e425)
        );
    }
}
public extension CircleRotor {
    public VersorOdd weight_contraction(AntiDipoleInversion other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((this.e412 * right_anti_dual.e4315) + (this.e415 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e41), (this.e423 * right_anti_dual.e4125) + (this.e425 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e42), (this.e431 * right_anti_dual.e4235) + (this.e435 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e43), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((this.group0.xyz.yzx * right_anti_dual.group3.zxy), this.e423 * right_anti_dual.e15), 
            /* e23, e31, e12, e45 */
            float4((this.e235 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e23), (this.e315 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e31), (this.e125 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e12), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) + float4((this.group0.xyz * right_anti_dual.group3.www), this.e12345 * right_anti_dual.e45) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e3215) * this.group1.xyz) + (this.group2.yzx * right_anti_dual.group3.zxy) - (this.group2.zxy * right_anti_dual.group3.yzx)), this.e12345 * right_anti_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension CircleRotor {
    public VersorEven weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e12345) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 * float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, right_anti_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension CircleRotor {
    public AntiCircleRotor weight_contraction(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e12345 * right_anti_dual.e45), 
            /* e15, e25, e35, scalar */
            float4((this.group2.www * right_anti_dual.group0.xyz), -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45))
        );
    }
}
public extension CircleRotor {
    public VersorOdd weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e412 * right_anti_dual.e4315, this.e423 * right_anti_dual.e4125, this.e431 * right_anti_dual.e4235, -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) - float4((this.group0.xyz.yzx * right_anti_dual.group1.zxy), this.e423 * right_anti_dual.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) + float4((this.group0.xyz * right_anti_dual.group1.www), this.e12345 * right_anti_dual.e45) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz), 0.0) + float4((float3(right_anti_dual.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * right_anti_dual.group1.zxy), 0.0) - float4((this.group2.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group1.xyz), -(this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e321 * right_anti_dual.e415), (this.e423 * right_anti_dual.e125) + (this.e321 * right_anti_dual.e425), (this.e431 * right_anti_dual.e235) + (this.e321 * right_anti_dual.e435), -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group1.xyz.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
public extension CircleRotor {
    public VersorEven weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e12345) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz)), this.e321 * right_anti_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e12345 * right_anti_dual.e235, this.e12345 * right_anti_dual.e315, this.e12345 * right_anti_dual.e125, -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + (this.group2 * float4(right_anti_dual.group0.www, right_anti_dual.e5)), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e315) + (this.e321 * right_anti_dual.e415), (this.e423 * right_anti_dual.e125) + (this.e321 * right_anti_dual.e425), (this.e431 * right_anti_dual.e235) + (this.e321 * right_anti_dual.e435), -(this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group1.zxy), this.e423 * right_anti_dual.e415)
        );
    }
}
public extension CircleRotor {
    public DipoleInversion weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_anti_dual.e3215, this.e431 * right_anti_dual.e3215, this.e412 * right_anti_dual.e3215, -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * right_anti_dual.group0.zxy), 0.0) - float4((this.group2.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0
        );
    }
}
public extension CircleRotor {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension CircleRotor {
    public AntiCircleRotor weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43))
        );
    }
}
public extension CircleRotor {
    public AntiCircleRotor weight_contraction(CircleRotor other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e125 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.e235 * this.e415) - float4((right_anti_dual.group2.xyz.zxy * this.group0.xyz.yzx), right_anti_dual.e315 * this.e425)
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion weight_contraction(DipoleInversion other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.e4 * this.e12345) - (right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412) + (right_anti_dual.e1 * this.e12345), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423) + (right_anti_dual.e2 * this.e12345), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431) + (right_anti_dual.e3 * this.e12345), -(right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + (this.group2.yzxw * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e5)) - (this.group2.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e415)) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), right_anti_dual.e425 * this.e315)
        );
    }
}
public extension CircleRotor {
    public AntiMotor weight_contraction(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, right_anti_dual.scalar) * float4(this.group0.xyz, this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
public extension CircleRotor {
    public AntiFlector weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e315 * this.e412) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e125 * this.e423) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e235 * this.e431) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
public extension CircleRotor {
    public AntiFlector weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e321 * this.e415) + (right_anti_dual.e1 * this.e12345), (right_anti_dual.e321 * this.e425) + (right_anti_dual.e2 * this.e12345), (right_anti_dual.e321 * this.e435) + (right_anti_dual.e3 * this.e12345), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + float4((this.group0.xyz.zxy * right_anti_dual.group0.yzx), right_anti_dual.e5 * this.e12345) - float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
public extension CircleRotor {
    public AntiMotor weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension CircleRotor {
    public AntiMotor weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e23 * this.e12345, right_anti_dual.e31 * this.e12345, right_anti_dual.e12 * this.e12345, -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) + float4((this.group0.xyz * right_anti_dual.group1.www), right_anti_dual.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual.e3215) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz)), right_anti_dual.e3215 * this.e12345)
        );
    }
}
public extension CircleRotor {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((this.e12345 * right_anti_dual.scalar) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), this.e12345 * right_anti_dual.e12345), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412) + (this.e12345 * right_anti_dual.e1), (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423) + (this.e12345 * right_anti_dual.e2), (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431) + (this.e12345 * right_anti_dual.e3), -(this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) + float4((this.group0.xyz.zxy * right_anti_dual.group8.xyz.yzx), this.e12345 * right_anti_dual.e4) - float4((this.group0.xyz.yzx * right_anti_dual.group8.xyz.zxy), this.e423 * right_anti_dual.e415) - float4((right_anti_dual.group7.xyz.yzx * this.group2.zxy), this.e431 * right_anti_dual.e425), 
            /* e5 */
            (this.e12345 * right_anti_dual.e5) - (this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e315 * right_anti_dual.e4125) + (this.e12345 * right_anti_dual.e15), (this.e125 * right_anti_dual.e4235) + (this.e12345 * right_anti_dual.e25), (this.e235 * right_anti_dual.e4315) + (this.e12345 * right_anti_dual.e35), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) + float4((right_anti_dual.group9.www * this.group1.xyz), this.e12345 * right_anti_dual.e45) - (right_anti_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * right_anti_dual.group4.xyz) + (float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group9.yzx) - (this.group0.xyz.yzx * right_anti_dual.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e12345) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.e3215) * this.group0.xyz) + (float3(right_anti_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * right_anti_dual.group6) + (float4(right_anti_dual.e12345) * this.group1), 
            /* e423, e431, e412 */
            (float3(this.e12345) * right_anti_dual.group7.xyz) + (float3(right_anti_dual.e12345) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(this.e12345) * right_anti_dual.group8.xyz) + (float3(right_anti_dual.e12345) * this.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group9, 
            /* e1234 */
            this.e12345 * right_anti_dual.e1234
        );
    }
}
public extension CircleRotor {
    public AntiPlane weight_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension CircleRotor {
    public DipoleInversion weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e3215) * this.group1.xyz) + (this.group2.yzx * right_anti_dual.group0.zxy) - (this.group2.zxy * right_anti_dual.group0.yzx)), this.e12345 * right_anti_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0
        );
    }
}
public extension CircleRotor {
    public CircleRotor weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.e12345) * this.group2
        );
    }
}
public extension CircleRotor {
    public RoundPoint weight_contraction(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5 */
            this.e12345 * right_anti_dual.e5
        );
    }
}
public extension CircleRotor {
    public VersorOdd weight_contraction(VersorEven other) {
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
            float4((this.e415 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e41), (this.e425 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e42), (this.e435 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e43), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) + float4((this.group0.xyz.zxy * right_anti_dual.group3.yzx), this.e12345 * right_anti_dual.scalar) - float4((this.group0.xyz.yzx * right_anti_dual.group3.zxy), this.e423 * right_anti_dual.e15), 
            /* e23, e31, e12, e45 */
            float4((this.e235 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e23), (this.e315 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e31), (this.e125 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.e12), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) + float4((this.group0.xyz * right_anti_dual.group3.www), this.e12345 * right_anti_dual.e45) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e3215) * this.group1.xyz) + (this.group2.yzx * right_anti_dual.group3.zxy) - (this.group2.zxy * right_anti_dual.group3.yzx)), this.e12345 * right_anti_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension CircleRotor {
    public VersorEven weight_contraction(VersorOdd other) {
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
            float4(((float3(this.e12345) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e12345) * this.group0.xyz)), this.e12345 * right_anti_dual.e12345), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * right_anti_dual.group1) + (float4(right_anti_dual.e12345) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(this.e12345 * right_anti_dual.e235, this.e12345 * right_anti_dual.e315, this.e12345 * right_anti_dual.e125, -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + (this.group2 * float4(right_anti_dual.group0.www, right_anti_dual.e5)), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412) + (this.e12345 * right_anti_dual.e1), (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423) + (this.e12345 * right_anti_dual.e2), (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431) + (this.e12345 * right_anti_dual.e3), -(this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) + float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), this.e12345 * right_anti_dual.e4) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.e423 * right_anti_dual.e415) - float4((this.group2.zxy * right_anti_dual.group0.yzx), this.e431 * right_anti_dual.e425)
        );
    }
}
public extension Dipole {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<Dipole> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<Dipole>(this);
    }
}
public extension WeightContractionInfixPartial<Dipole> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension Dipole {
    public AntiCircleRotor weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43))
        );
    }
}
public extension Dipole {
    public RoundPoint weight_contraction(AntiDipoleInversion other) {
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_anti_dual.group2.www), this.e41 * right_anti_dual.e4235), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)
        );
    }
}
public extension Dipole {
    public Dipole weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(right_anti_dual.e12345) * this.group2.xyz
        );
    }
}
public extension Dipole {
    public RoundPoint weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)
        );
    }
}
public extension Dipole {
    public Scalar weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)
        );
    }
}
public extension Dipole {
    public AntiCircleRotor weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435))
        );
    }
}
public extension Dipole {
    public RoundPoint weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)
        );
    }
}
public extension Dipole {
    public Scalar weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)
        );
    }
}
public extension Dipole {
    public Scalar weight_contraction(DipoleInversion other) {
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
        return Scalar(
            /* scalar */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)
        );
    }
}
public extension Dipole {
    public AntiPlane weight_contraction(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension Dipole {
    public Scalar weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)
        );
    }
}
public extension Dipole {
    public Scalar weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)
        );
    }
}
public extension Dipole {
    public AntiPlane weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.group0.xyz * right_anti_dual.group1.www), right_anti_dual.e3215 * this.e45)
        );
    }
}
public extension Dipole {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2(-(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group9.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_anti_dual.e1234) * this.group2.xyz), this.e41 * right_anti_dual.e4235), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e12345) * this.group1.xyz, 
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
    public RoundPoint weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group0.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_anti_dual.e1234) * this.group2.xyz), this.e41 * right_anti_dual.e4235), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)
        );
    }
}
public extension Dipole {
    public Dipole weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(right_anti_dual.e12345) * this.group2.xyz
        );
    }
}
public extension Dipole {
    public RoundPoint weight_contraction(VersorEven other) {
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_anti_dual.group2.www), this.e41 * right_anti_dual.e4235), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)
        );
    }
}
public extension Dipole {
    public AntiCircleRotor weight_contraction(VersorOdd other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
public extension DipoleInversion {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<DipoleInversion> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<DipoleInversion>(this);
    }
}
public extension WeightContractionInfixPartial<DipoleInversion> {
    // Fancy infix trick (second half)
    public VersorOdd operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension DipoleInversion {
    public VersorOdd weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_anti_dual.e412 * this.e4315) + (right_anti_dual.e415 * this.e1234) + (right_anti_dual.e12345 * this.e41), (right_anti_dual.e423 * this.e4125) + (right_anti_dual.e425 * this.e1234) + (right_anti_dual.e12345 * this.e42), (right_anti_dual.e431 * this.e4235) + (right_anti_dual.e435 * this.e1234) + (right_anti_dual.e12345 * this.e43), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((right_anti_dual.group0.xyz.yzx * this.group3.zxy), right_anti_dual.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e235 * this.e1234) + (right_anti_dual.e12345 * this.e23), (right_anti_dual.e315 * this.e1234) + (right_anti_dual.e12345 * this.e31), (right_anti_dual.e125 * this.e1234) + (right_anti_dual.e12345 * this.e12), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) + float4((right_anti_dual.group0.xyz * this.group3.www), right_anti_dual.e12345 * this.e45) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e12345) * this.group2.xyz) + (float3(this.e3215) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.yzx * this.group3.zxy) - (right_anti_dual.group2.zxy * this.group3.yzx)), right_anti_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiDipoleInversion other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)) + float4(float3(0.0), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234) - (right_anti_dual.e1234 * this.e45) - (right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) - float4((this.group3.www * right_anti_dual.group3.xyz), right_anti_dual.e4235 * this.e41), 
            /* e1, e2, e3, e5 */
            (right_anti_dual.group3.zxyw * this.group1.yzxw) + (this.group2.wwwz * float4(right_anti_dual.group2.xyz, right_anti_dual.e4125)) + float4((this.group0.xyz * right_anti_dual.group3.www), right_anti_dual.e4235 * this.e15) + float4((right_anti_dual.group1.zxy * this.group3.yzx), right_anti_dual.e4315 * this.e25) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15)) - float4((right_anti_dual.group3.yzx * this.group1.zxy), right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension DipoleInversion {
    public VersorOdd weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, right_anti_dual.e5) * float4(this.group0.xyz, this.e1234), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public RoundPoint weight_contraction(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e5 */
            -(this.e4235 * right_anti_dual.e15) - (this.e4315 * right_anti_dual.e25) - (this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45)
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_anti_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * right_anti_dual.group1.yzx) - (this.group3.yzx * right_anti_dual.group1.zxy)), this.e1234 * right_anti_dual.e3215), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) + float4((right_anti_dual.group1.www * this.group3.xyz), this.e1234 * right_anti_dual.e45) - (right_anti_dual.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e4315)) + (right_anti_dual.group1.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (this.e35 * right_anti_dual.e4125) - (this.e4315 * right_anti_dual.e25) - (this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45)) - float4((this.group1.zxy * right_anti_dual.group1.yzx), this.e4235 * right_anti_dual.e15)
        );
    }
}
public extension DipoleInversion {
    public AntiCircleRotor weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz), -(this.e4235 * right_anti_dual.e415) - (this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)), 
            /* e15, e25, e35, scalar */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)) - float4((right_anti_dual.group1.xyz.zxy * this.group3.yzx), this.e41 * right_anti_dual.e235)
        );
    }
}
public extension DipoleInversion {
    public VersorOdd weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e1234 * right_anti_dual.e415, this.e1234 * right_anti_dual.e425, this.e1234 * right_anti_dual.e435, -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)) + float4((this.group0.xyz * right_anti_dual.group0.www), this.e1234 * right_anti_dual.e5), 
            /* e23, e31, e12, e45 */
            float4(this.e1234 * right_anti_dual.e235, this.e1234 * right_anti_dual.e315, this.e1234 * right_anti_dual.e125, -(this.e4235 * right_anti_dual.e415) - (this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) + (float4(right_anti_dual.e12345) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e12345) * this.group2.xyz) + (this.group3.zxy * right_anti_dual.group1.yzx) - (this.group3.yzx * right_anti_dual.group1.zxy)), this.e1234 * right_anti_dual.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * right_anti_dual.group0.yzx) - (this.group3.yzx * right_anti_dual.group0.zxy)), this.e1234 * right_anti_dual.e3215), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * right_anti_dual.e3215, this.e4315 * right_anti_dual.e3215, this.e4125 * right_anti_dual.e3215, -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_anti_dual.e4315 * -1.0, this.e23 * right_anti_dual.e4125 * -1.0, this.e31 * right_anti_dual.e4235 * -1.0, (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
public extension DipoleInversion {
    public RoundPoint weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group3.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e42)) + float4((right_anti_dual.group2.xyz * this.group2.www), right_anti_dual.e41 * this.e4235), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension DipoleInversion {
    public RoundPoint weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)) + float4((this.group2.www * right_anti_dual.group2.xyz), right_anti_dual.e42 * this.e4315), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension DipoleInversion {
    public AntiCircleRotor weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group3.yzx) - (right_anti_dual.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((right_anti_dual.group2.xyz.zxy * this.group3.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
public extension DipoleInversion {
    public AntiCircleRotor weight_contraction(DipoleInversion other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group3.yzx) - (right_anti_dual.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4)) + (this.group3.zxyx * float4(right_anti_dual.group2.yzx, right_anti_dual.e1)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((right_anti_dual.group2.zxy * this.group3.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
public extension DipoleInversion {
    public AntiFlector weight_contraction(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension DipoleInversion {
    public AntiMotor weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e235 * this.e1234, right_anti_dual.e315 * this.e1234, right_anti_dual.e125 * this.e1234, -(right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - float4((right_anti_dual.group0.www * this.group3.xyz), right_anti_dual.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension DipoleInversion {
    public AntiMotor weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) + float4((this.group2.www * right_anti_dual.group0.xyz), right_anti_dual.e1 * this.e4235) - float4((right_anti_dual.group0.www * this.group3.xyz), right_anti_dual.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension DipoleInversion {
    public AntiPlane weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group3.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e15))
        );
    }
}
public extension DipoleInversion {
    public AntiFlector weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) + float4((this.group0.xyz * right_anti_dual.group1.www), right_anti_dual.e3215 * this.e45) - (this.group3.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e15))
        );
    }
}
public extension DipoleInversion {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((this.e1234 * right_anti_dual.e5) + (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * right_anti_dual.group3) + (this.group3.yzxy * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e42)) + float4((this.group0.xyz * right_anti_dual.group9.www), this.e4235 * right_anti_dual.e41) + float4((this.group1.yzx * right_anti_dual.group9.zxy), this.e4125 * right_anti_dual.e43) - (float4(right_anti_dual.e1234) * float4(this.group2.xyz, this.e45)) - (right_anti_dual.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((right_anti_dual.group4.xyz * this.group3.www), this.e41 * right_anti_dual.e4235) - float4((right_anti_dual.group5.xyz.yzx * this.group3.zxy), this.e42 * right_anti_dual.e4315), 
            /* e5 */
            (this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) - (this.e4235 * right_anti_dual.e15) - (this.e4315 * right_anti_dual.e25) - (this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) + (float4(right_anti_dual.e12345) * float4(this.group2.xyz, this.e45)) - float4((right_anti_dual.group8.xyz.zxy * this.group3.yzx), this.e4235 * right_anti_dual.e415), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group6.xyz) + (float3(right_anti_dual.e12345) * this.group0.xyz) + (right_anti_dual.group7.xyz.zxy * this.group3.yzx) - (right_anti_dual.group7.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_anti_dual.group8.xyz) + (float3(this.e3215) * right_anti_dual.group7.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz) - (float3(right_anti_dual.e321) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group9.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group9.zxy, right_anti_dual.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group9.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group3.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3, 
            /* e1234 */
            this.e1234 * right_anti_dual.e12345
        );
    }
}
public extension DipoleInversion {
    public Scalar weight_contraction(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group0.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group0.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * right_anti_dual.e3215, this.e4315 * right_anti_dual.e3215, this.e4125 * right_anti_dual.e3215, -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public Scalar weight_contraction(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Scalar(
            /* scalar */
            (this.e1234 * right_anti_dual.e5) + (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion weight_contraction(VersorEven other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (this.e4235 * right_anti_dual.e41) + (this.e4315 * right_anti_dual.e42) + (this.e4125 * right_anti_dual.e43) - (this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) + float4((right_anti_dual.group3.www * this.group3.xyz), this.e1234 * right_anti_dual.e45) - (right_anti_dual.group3.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(right_anti_dual.group2.xyz, right_anti_dual.e4315)) + (right_anti_dual.group3.zxyx * float4(this.group1.yzx, this.e15)) + float4((this.group3.yzx * right_anti_dual.group1.zxy), this.e35 * right_anti_dual.e4125) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (this.group3.zxyz * float4(right_anti_dual.group1.yzx, right_anti_dual.e35)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - float4((this.group1.zxy * right_anti_dual.group3.yzx), this.e4235 * right_anti_dual.e15)
        );
    }
}
public extension DipoleInversion {
    public VersorOdd weight_contraction(VersorOdd other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group3.yzxy * float4(right_anti_dual.group0.zxy, right_anti_dual.e2)) + float4(float3(0.0), (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) + float4((this.group0.xyz * right_anti_dual.group0.www), this.e1234 * right_anti_dual.e5) + float4((this.group2.www * right_anti_dual.group1.xyz), this.e4235 * right_anti_dual.e1) - float4((this.group3.zxy * right_anti_dual.group0.yzx), this.e41 * right_anti_dual.e235), 
            /* e23, e31, e12, e45 */
            float4((this.e1234 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e423), (this.e1234 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e431), (this.e1234 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e412), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) + (float4(right_anti_dual.e12345) * this.group1) - (this.group3.xyzx * right_anti_dual.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e12345) * this.group2.xyz) + (this.group3.zxy * right_anti_dual.group2.yzx) - (this.group3.yzx * right_anti_dual.group2.zxy)), this.e1234 * right_anti_dual.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<DualNum> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<DualNum>(this);
    }
}
public extension WeightContractionInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public VersorEven operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension DualNum {
    public VersorEven weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4(right_anti_dual.group0.xyz, right_anti_dual.e12345), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e5 */
            right_anti_dual.group2 * float4(this.group0.xy.yy, this.e12345, this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public VersorOdd weight_contraction(AntiDipoleInversion other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.group0.xy.yy, this.e12345, this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e1234), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension DualNum {
    public DualNum weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return DualNum(
            /* e5, e12345 */
            float2((right_anti_dual.e5 * this.e12345) + (right_anti_dual.e12345 * this.e5), right_anti_dual.e12345 * this.e12345)
        );
    }
}
public extension DualNum {
    public FlatPoint weight_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
public extension DualNum {
    public Flector weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
public extension DualNum {
    public Line weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_anti_dual.group1.xyz
        );
    }
}
public extension DualNum {
    public Motor weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.yy, this.e12345, (this.e5 * right_anti_dual.e12345) + (this.e12345 * right_anti_dual.e5)) * float4(right_anti_dual.group1.xyz, 1.0)
        );
    }
}
public extension DualNum {
    public Plane weight_contraction(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
public extension DualNum {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension DualNum {
    public Dipole weight_contraction(Circle other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35 */
            float3(this.e12345) * right_anti_dual.group2.xyz
        );
    }
}
public extension DualNum {
    public AntiCircleRotor weight_contraction(CircleRotor other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * right_anti_dual.group2
        );
    }
}
public extension DualNum {
    public Circle weight_contraction(Dipole other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_anti_dual.group2.xyz
        );
    }
}
public extension DualNum {
    public AntiDipoleInversion weight_contraction(DipoleInversion other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * right_anti_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension DualNum {
    public AntiDualNum weight_contraction(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
public extension DualNum {
    public AntiFlatPoint weight_contraction(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
public extension DualNum {
    public AntiFlector weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
public extension DualNum {
    public AntiLine weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * right_anti_dual.group1.xyz
        );
    }
}
public extension DualNum {
    public AntiMotor weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
public extension DualNum {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((this.e5 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.scalar), this.e12345 * right_anti_dual.e12345), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e5 */
            (this.e5 * right_anti_dual.e12345) + (this.e12345 * right_anti_dual.e5), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_anti_dual.group3, 
            /* e41, e42, e43 */
            float3(this.e12345) * right_anti_dual.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * right_anti_dual.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group6, 
            /* e423, e431, e412 */
            float3(this.e12345) * right_anti_dual.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_anti_dual.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group9, 
            /* e1234 */
            this.e12345 * right_anti_dual.e1234
        );
    }
}
public extension DualNum {
    public AntiPlane weight_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension DualNum {
    public VersorOdd weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.e5 * right_anti_dual.e1234), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e12345 * right_anti_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0
        );
    }
}
public extension DualNum {
    public DualNum weight_contraction(Scalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension DualNum {
    public RoundPoint weight_contraction(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5 */
            this.e12345 * right_anti_dual.e5
        );
    }
}
public extension DualNum {
    public VersorOdd weight_contraction(VersorEven other) {
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
            float4(this.group0.xy.yy, this.e12345, (this.e5 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.scalar)) * float4(right_anti_dual.group0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension DualNum {
    public VersorEven weight_contraction(VersorOdd other) {
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
            float4(this.e12345) * right_anti_dual.group0, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.yy, this.e12345, (this.e5 * right_anti_dual.e12345) + (this.e12345 * right_anti_dual.e5)) * float4(right_anti_dual.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension FlatPoint {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<FlatPoint> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<FlatPoint>(this);
    }
}
public extension WeightContractionInfixPartial<FlatPoint> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension FlatPoint {
    public AntiCircleRotor weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_anti_dual.e12345 * this.e45), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.group2.www * this.group0.xyz), -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45))
        );
    }
}
public extension FlatPoint {
    public RoundPoint weight_contraction(AntiDipoleInversion other) {
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (right_anti_dual.e4235 * this.e15) + (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45)
        );
    }
}
public extension FlatPoint {
    public FlatPoint weight_contraction(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension FlatPoint {
    public DualNum weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 0.0)
        );
    }
}
public extension FlatPoint {
    public FlatPoint weight_contraction(AntiMotor other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension FlatPoint {
    public DualNum weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 0.0)
        );
    }
}
public extension FlatPoint {
    public Scalar weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)
        );
    }
}
public extension FlatPoint {
    public Scalar weight_contraction(DipoleInversion other) {
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
        return Scalar(
            /* scalar */
            -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)
        );
    }
}
public extension FlatPoint {
    public DualNum weight_contraction(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e5 * this.e45 * -1.0, 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension FlatPoint {
    public Scalar weight_contraction(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * this.e45 * -1.0
        );
    }
}
public extension FlatPoint {
    public Scalar weight_contraction(Flector other) {
        return Scalar(
            /* scalar */
            this.e45 * other.e45 * -1.0
        );
    }
}
public extension FlatPoint {
    public DualNum weight_contraction(Motor other) {
        return DualNum(
            /* e5, e12345 */
            float2(float4(other.group1.xyz, other.e5 * -1.0)[3] * this.e45, 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension FlatPoint {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2(-(this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * this.group0, 
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
    public RoundPoint weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215)
        );
    }
}
public extension FlatPoint {
    public FlatPoint weight_contraction(Scalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension FlatPoint {
    public RoundPoint weight_contraction(VersorEven other) {
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215)
        );
    }
}
public extension FlatPoint {
    public AntiCircleRotor weight_contraction(VersorOdd other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45 * right_anti_dual.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz), -(this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321))
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<Flector> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<Flector>(this);
    }
}
public extension WeightContractionInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public VersorOdd operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension Flector {
    public VersorOdd weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.e412 * this.e4315, right_anti_dual.e423 * this.e4125, right_anti_dual.e431 * this.e4235, -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)) - float4((right_anti_dual.group0.xyz.yzx * this.group1.zxy), right_anti_dual.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) + float4((right_anti_dual.group0.xyz * this.group1.www), right_anti_dual.e12345 * this.e45) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz), 0.0) + float4((float3(this.e3215) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group2.yzx * this.group1.zxy), 0.0) - float4((right_anti_dual.group2.zxy * this.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
public extension Flector {
    public AntiDipoleInversion weight_contraction(AntiDipoleInversion other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e1234) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group3.yzx * this.group1.zxy) - (right_anti_dual.group3.zxy * this.group1.yzx)), right_anti_dual.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)) + float4(float3(0.0), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) - float4((this.group1.www * right_anti_dual.group3.xyz), right_anti_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e35 * this.e4125)) + float4((right_anti_dual.group1.zxy * this.group1.yzx), right_anti_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (right_anti_dual.group2.wwwy * float4(this.group0.xyz, this.e4315)) - (this.group1.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15))
        );
    }
}
public extension Flector {
    public Flector weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
public extension Flector {
    public DualNum weight_contraction(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 0.0)
        );
    }
}
public extension Flector {
    public Motor weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual.group1.yzx * this.group1.zxy) - (right_anti_dual.group1.zxy * this.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (right_anti_dual.group1.wwwx * float4(this.group1.xyz, this.e15)) + float4(float3(0.0), (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215)) - (this.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e15))
        );
    }
}
public extension Flector {
    public FlatPoint weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) - (this.group1.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e415))
        );
    }
}
public extension Flector {
    public Flector weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) + (float4(right_anti_dual.e12345) * this.group0) - (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
public extension Flector {
    public Motor weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((this.group1.zxy * right_anti_dual.group0.yzx) - (this.group1.yzx * right_anti_dual.group0.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(this.e3215 * right_anti_dual.e4235 * -1.0, this.e3215 * right_anti_dual.e4315 * -1.0, this.e3215 * right_anti_dual.e4125 * -1.0, (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215)) + (right_anti_dual.group0.wwwx * float4(this.group1.xyz, this.e15))
        );
    }
}
public extension Flector {
    public RoundPoint weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension Flector {
    public RoundPoint weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension Flector {
    public AntiCircleRotor weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_anti_dual.group0.xyz.zxy * this.group1.yzx) - (right_anti_dual.group0.xyz.yzx * this.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e423 * this.e3215, right_anti_dual.e431 * this.e3215, right_anti_dual.e412 * this.e3215, -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)) - float4((right_anti_dual.group2.xyz.zxy * this.group1.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
public extension Flector {
    public AntiCircleRotor weight_contraction(DipoleInversion other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_anti_dual.group0.xyz.zxy * this.group1.yzx) - (right_anti_dual.group0.xyz.yzx * this.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e423 * this.e3215, right_anti_dual.e431 * this.e3215, right_anti_dual.e412 * this.e3215, -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4)) + (this.group1.zxyx * float4(right_anti_dual.group2.yzx, right_anti_dual.e1)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)) - float4((right_anti_dual.group2.zxy * this.group1.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
public extension Flector {
    public AntiFlector weight_contraction(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e45)
        );
    }
}
public extension Flector {
    public AntiMotor weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e321) * float4(this.group1.xyz, this.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group1.zxy) - (right_anti_dual.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public AntiMotor weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual.group0.www * this.group1.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) - (right_anti_dual.e321 * this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group1.zxy) - (right_anti_dual.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public AntiPlane weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12 * this.e4315, right_anti_dual.e23 * this.e4125, right_anti_dual.e31 * this.e4235, -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group1.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e15))
        );
    }
}
public extension Flector {
    public AntiFlector weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) + float4((right_anti_dual.group0.zxy * this.group1.yzx), right_anti_dual.e3215 * this.e45) - (this.group1.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e15))
        );
    }
}
public extension Flector {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(this.e15 * right_anti_dual.e1234) - (this.e4125 * right_anti_dual.e31), -(this.e25 * right_anti_dual.e1234) - (this.e4235 * right_anti_dual.e12), -(this.e35 * right_anti_dual.e1234) - (this.e4315 * right_anti_dual.e23), (this.e4315 * right_anti_dual.e42) + (this.e4125 * right_anti_dual.e43)) + (this.group1.yzxx * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e41)) - float4((right_anti_dual.group4.xyz * this.group1.www), this.e45 * right_anti_dual.e1234), 
            /* e5 */
            (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215) - (this.e4235 * right_anti_dual.e15) - (this.e4315 * right_anti_dual.e25) - (this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) + (float4(right_anti_dual.e12345) * this.group0) - float4((right_anti_dual.group8.xyz.zxy * this.group1.yzx), this.e4235 * right_anti_dual.e415), 
            /* e41, e42, e43 */
            (right_anti_dual.group7.xyz.zxy * this.group1.yzx) - (right_anti_dual.group7.xyz.yzx * this.group1.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_anti_dual.group7.xyz) - (float3(right_anti_dual.e321) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_anti_dual.group9.yzx) - (this.group1.yzx * right_anti_dual.group9.zxy)), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(right_anti_dual.e1234) * this.group1.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group1.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension Flector {
    public Scalar weight_contraction(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)
        );
    }
}
public extension Flector {
    public AntiDipoleInversion weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e1234) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_anti_dual.group0.yzx) - (this.group1.yzx * right_anti_dual.group0.zxy)), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual.e3215) * this.group1.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz)), this.e45 * right_anti_dual.e1234 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(right_anti_dual.e1234) * this.group0.xyz * float3(-1.0)), (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215))
        );
    }
}
public extension Flector {
    public Flector weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
public extension Flector {
    public Scalar weight_contraction(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Scalar(
            /* scalar */
            (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)
        );
    }
}
public extension Flector {
    public AntiDipoleInversion weight_contraction(VersorEven other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e1234) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_anti_dual.group3.yzx) - (this.group1.yzx * right_anti_dual.group3.zxy)), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)) + float4(float3(0.0), (this.e4315 * right_anti_dual.e42) + (this.e4125 * right_anti_dual.e43)) - float4((this.group1.www * right_anti_dual.group3.xyz), this.e45 * right_anti_dual.e1234), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215) - (this.e3215 * right_anti_dual.e45)) + float4((this.group1.yzx * right_anti_dual.group1.zxy), this.e15 * right_anti_dual.e4235) - (this.group1.zxyy * float4(right_anti_dual.group1.yzx, right_anti_dual.e25)) - (this.group1.wwwz * float4(right_anti_dual.group0.xyz, right_anti_dual.e35)) - (right_anti_dual.group2.wwwx * float4(this.group0.xyz, this.e4235))
        );
    }
}
public extension Flector {
    public VersorOdd weight_contraction(VersorOdd other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group1.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e1)) + float4(float3(0.0), (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group0 * float4(this.group1.www, this.e45)) + float4(float3(0.0), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) - (this.group1.xyzx * right_anti_dual.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * right_anti_dual.group1.xyz), 0.0) + float4((float3(right_anti_dual.e12345) * this.group0.xyz), 0.0) + float4((this.group1.zxy * right_anti_dual.group2.yzx), 0.0) - float4((this.group1.yzx * right_anti_dual.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<Line> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<Line>(this);
    }
}
public extension WeightContractionInfixPartial<Line> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension Line {
    public AntiDipoleInversion weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group1.xyz), -(right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
public extension Line {
    public AntiCircleRotor weight_contraction(AntiDipoleInversion other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz), -(right_anti_dual.e4235 * this.e415) - (right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)) - float4((this.group1.xyz.zxy * right_anti_dual.group3.yzx), right_anti_dual.e41 * this.e235)
        );
    }
}
public extension Line {
    public Line weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.e12345) * this.group1.xyz
        );
    }
}
public extension Line {
    public FlatPoint weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) - (right_anti_dual.group1.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
public extension Line {
    public DualNum weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0)
        );
    }
}
public extension Line {
    public Motor weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(right_anti_dual.e12345) * this.group1.xyz), -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435))
        );
    }
}
public extension Line {
    public FlatPoint weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
public extension Line {
    public Scalar weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)
        );
    }
}
public extension Line {
    public Scalar weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)
        );
    }
}
public extension Line {
    public RoundPoint weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_anti_dual.e423 * this.e415), 
            /* e5 */
            -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)
        );
    }
}
public extension Line {
    public RoundPoint weight_contraction(DipoleInversion other) {
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_anti_dual.e423 * this.e415), 
            /* e5 */
            -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)
        );
    }
}
public extension Line {
    public FlatPoint weight_contraction(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public AntiPlane weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(right_anti_dual.e321) * this.group0.xyz), -(right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435))
        );
    }
}
public extension Line {
    public AntiPlane weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(right_anti_dual.e321) * this.group0.xyz), -(right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435))
        );
    }
}
public extension Line {
    public Scalar weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Scalar(
            /* scalar */
            -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)
        );
    }
}
public extension Line {
    public AntiMotor weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2(-(this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_anti_dual.e321) + (this.e315 * right_anti_dual.e412), (this.e425 * right_anti_dual.e321) + (this.e125 * right_anti_dual.e423), (this.e435 * right_anti_dual.e321) + (this.e235 * right_anti_dual.e431), -(this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) - float4((this.group1.xyz.zxy * right_anti_dual.group7.xyz.yzx), this.e415 * right_anti_dual.e423), 
            /* e5 */
            -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group9.yzxx * float4(this.group1.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual.e12345) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Line {
    public Dipole weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz), -(this.e415 * right_anti_dual.e4235) - (this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e3215) * this.group0.xyz) + (this.group1.xyz.yzx * right_anti_dual.group0.zxy) - (this.group1.xyz.zxy * right_anti_dual.group0.yzx)
        );
    }
}
public extension Line {
    public Line weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.e12345) * this.group1.xyz
        );
    }
}
public extension Line {
    public AntiCircleRotor weight_contraction(VersorEven other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz), -(this.e415 * right_anti_dual.e4235) - (this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((this.group1.xyz.zxy * right_anti_dual.group3.yzx), this.e415 * right_anti_dual.e23)
        );
    }
}
public extension Line {
    public AntiDipoleInversion weight_contraction(VersorOdd other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group1.xyz), -(this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_anti_dual.e321) + (this.e315 * right_anti_dual.e412), (this.e425 * right_anti_dual.e321) + (this.e125 * right_anti_dual.e423), (this.e435 * right_anti_dual.e321) + (this.e235 * right_anti_dual.e431), -(this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((this.group1.xyz.zxy * right_anti_dual.group0.yzx), this.e415 * right_anti_dual.e235)
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<Motor> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<Motor>(this);
    }
}
public extension WeightContractionInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public VersorEven operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension Motor {
    public VersorEven weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4(right_anti_dual.group0.xyz, right_anti_dual.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz)), right_anti_dual.e321 * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e12345 * this.e235, right_anti_dual.e12345 * this.e315, right_anti_dual.e12345 * this.e125, -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + (right_anti_dual.group2 * float4(this.group0.www, this.e5)), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group1.zxy), right_anti_dual.e423 * this.e415)
        );
    }
}
public extension Motor {
    public VersorOdd weight_contraction(AntiDipoleInversion other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.e1234 * this.e415, right_anti_dual.e1234 * this.e425, right_anti_dual.e1234 * this.e435, -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)) + float4((right_anti_dual.group0.xyz * this.group0.www), right_anti_dual.e1234 * this.e5), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e1234 * this.e235, right_anti_dual.e1234 * this.e315, right_anti_dual.e1234 * this.e125, -(right_anti_dual.e4235 * this.e415) - (right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(this.e12345) * right_anti_dual.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e3215) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual.group2.xyz) + (right_anti_dual.group3.zxy * this.group1.yzx) - (right_anti_dual.group3.yzx * this.group1.zxy)), right_anti_dual.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension Motor {
    public Motor weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, (right_anti_dual.e5 * this.e12345) + (right_anti_dual.e12345 * this.e5)) * float4(this.group1.xyz, 1.0)
        );
    }
}
public extension Motor {
    public FlatPoint weight_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
public extension Motor {
    public Flector weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(this.e12345) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
public extension Motor {
    public Motor weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e12345) * right_anti_dual.group1.xyz), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435))
        );
    }
}
public extension Motor {
    public Motor weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(right_anti_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz)), right_anti_dual.e12345 * this.e12345), 
            /* e235, e315, e125, e5 */
            (float4(right_anti_dual.e12345) * this.group1) + (float4(this.e12345) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435))
        );
    }
}
public extension Motor {
    public Flector weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0
        );
    }
}
public extension Motor {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension Motor {
    public AntiCircleRotor weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435))
        );
    }
}
public extension Motor {
    public AntiCircleRotor weight_contraction(CircleRotor other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435))
        );
    }
}
public extension Motor {
    public AntiDipoleInversion weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group1.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
public extension Motor {
    public AntiDipoleInversion weight_contraction(DipoleInversion other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.e4 * this.e12345) - (right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e321 * this.e415) + (right_anti_dual.e1 * this.e12345), (right_anti_dual.e321 * this.e425) + (right_anti_dual.e2 * this.e12345), (right_anti_dual.e321 * this.e435) + (right_anti_dual.e3 * this.e12345), -(right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx), right_anti_dual.e5 * this.e12345) - (this.group1.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e415))
        );
    }
}
public extension Motor {
    public AntiMotor weight_contraction(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_anti_dual.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e3215) * this.group0
        );
    }
}
public extension Motor {
    public AntiFlector weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.group0.www * this.group0.xyz), -(right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435))
        );
    }
}
public extension Motor {
    public AntiFlector weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e1 * this.e12345, right_anti_dual.e2 * this.e12345, right_anti_dual.e3 * this.e12345, -(right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + (this.group0 * float4(right_anti_dual.group0.www, right_anti_dual.e5))
        );
    }
}
public extension Motor {
    public AntiMotor weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    public AntiMotor weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual.e3215) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz)), right_anti_dual.e3215 * this.e12345)
        );
    }
}
public extension Motor {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((this.e12345 * right_anti_dual.scalar) + (this.e5 * right_anti_dual.e1234) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), this.e12345 * right_anti_dual.e12345), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_anti_dual.e321) + (this.e12345 * right_anti_dual.e1), (this.e425 * right_anti_dual.e321) + (this.e12345 * right_anti_dual.e2), (this.e435 * right_anti_dual.e321) + (this.e12345 * right_anti_dual.e3), -(this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) + float4((right_anti_dual.group7.xyz.zxy * this.group1.yzx), this.e12345 * right_anti_dual.e4) - float4((right_anti_dual.group7.xyz.yzx * this.group1.zxy), this.e415 * right_anti_dual.e423), 
            /* e5 */
            (this.e12345 * right_anti_dual.e5) + (this.e5 * right_anti_dual.e12345) - (this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e12345 * right_anti_dual.e15) + (this.e315 * right_anti_dual.e4125), (this.e12345 * right_anti_dual.e25) + (this.e125 * right_anti_dual.e4235), (this.e12345 * right_anti_dual.e35) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) + (this.group0 * float4(right_anti_dual.group9.www, right_anti_dual.e45)) - (right_anti_dual.group9.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * right_anti_dual.group4.xyz) + (float3(right_anti_dual.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e12345) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.e1234) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * right_anti_dual.group6.xyz) + (float3(right_anti_dual.e12345) * this.group0.xyz)), this.e12345 * right_anti_dual.e321), 
            /* e423, e431, e412 */
            float3(this.e12345) * right_anti_dual.group7.xyz, 
            /* e235, e315, e125 */
            (float3(this.e12345) * right_anti_dual.group8.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group9, 
            /* e1234 */
            this.e12345 * right_anti_dual.e1234
        );
    }
}
public extension Motor {
    public AntiPlane weight_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension Motor {
    public VersorOdd weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.e1234) * float4(this.group0.xyz, this.e5), 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz), -(this.e415 * right_anti_dual.e4235) - (this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e3215) * this.group0.xyz) + (this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), this.e12345 * right_anti_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0
        );
    }
}
public extension Motor {
    public Motor weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
public extension Motor {
    public RoundPoint weight_contraction(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5 */
            this.e12345 * right_anti_dual.e5
        );
    }
}
public extension Motor {
    public VersorOdd weight_contraction(VersorEven other) {
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
            (this.group0 * float4(right_anti_dual.group2.www, right_anti_dual.scalar)) + float4(float3(0.0), -(this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) + float4((this.group0.www * right_anti_dual.group0.xyz), this.e5 * right_anti_dual.e1234), 
            /* e23, e31, e12, e45 */
            float4(this.e235 * right_anti_dual.e1234, this.e315 * right_anti_dual.e1234, this.e125 * right_anti_dual.e1234, -(this.e415 * right_anti_dual.e4235) - (this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) + (float4(this.e12345) * right_anti_dual.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e3215) * this.group0.xyz) + (this.group1.yzx * right_anti_dual.group3.zxy) - (this.group1.zxy * right_anti_dual.group3.yzx)), this.e12345 * right_anti_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension Motor {
    public VersorEven weight_contraction(VersorOdd other) {
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
            float4(this.e12345) * right_anti_dual.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e12345) * this.group0.xyz)), this.e12345 * right_anti_dual.e321), 
            /* e235, e315, e125, e5 */
            (float4(this.e12345) * right_anti_dual.group2) + (float4(right_anti_dual.e12345) * this.group1) + float4(float3(0.0), -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)), 
            /* e1, e2, e3, e4 */
            float4((this.e12345 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e412), (this.e12345 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e423), (this.e12345 * right_anti_dual.e3) + (this.e235 * right_anti_dual.e431), -(this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) + (this.group0 * float4(right_anti_dual.group1.www, right_anti_dual.e4)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e415))
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<MultiVector> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<MultiVector>(this);
    }
}
public extension WeightContractionInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiCircleRotor other) {
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
            float2((right_anti_dual.e12345 * this.scalar) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), right_anti_dual.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412) + (right_anti_dual.e12345 * this.e1), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423) + (right_anti_dual.e12345 * this.e2), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431) + (right_anti_dual.e12345 * this.e3), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) + float4((right_anti_dual.group0.xyz.zxy * this.group8.xyz.yzx), right_anti_dual.e12345 * this.e4) - float4((right_anti_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_anti_dual.e423 * this.e415) - float4((this.group7.xyz.yzx * right_anti_dual.group2.zxy), right_anti_dual.e431 * this.e425), 
            /* e5 */
            (right_anti_dual.e12345 * this.e5) - (right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e315 * this.e4125) + (right_anti_dual.e12345 * this.e15), (right_anti_dual.e125 * this.e4235) + (right_anti_dual.e12345 * this.e25), (right_anti_dual.e235 * this.e4315) + (right_anti_dual.e12345 * this.e35), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) + float4((this.group9.www * right_anti_dual.group1.xyz), right_anti_dual.e12345 * this.e45) - (this.group9.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e12345) * this.group4.xyz) + (float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group9.yzx) - (right_anti_dual.group0.xyz.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e12345) * this.group5.xyz) + (float3(this.e3215) * right_anti_dual.group0.xyz) + (float3(this.e1234) * right_anti_dual.group2.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(right_anti_dual.e12345) * this.group6) + (float4(this.e12345) * right_anti_dual.group1), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.e12345) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e12345) * this.group8.xyz) + (float3(this.e12345) * right_anti_dual.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group9, 
            /* e1234 */
            right_anti_dual.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiDipoleInversion other) {
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
            float2((right_anti_dual.e1234 * this.e5) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)) + (this.group9.yzxz * float4(right_anti_dual.group1.zxy, right_anti_dual.e43)) + float4((this.group4.xyz * right_anti_dual.group3.www), right_anti_dual.e41 * this.e4235) + float4((this.group5.xyz.yzx * right_anti_dual.group3.zxy), right_anti_dual.e42 * this.e4315) - (float4(right_anti_dual.e1234) * this.group3) - (right_anti_dual.group3.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((right_anti_dual.group0.xyz * this.group9.www), right_anti_dual.e4235 * this.e41) - float4((right_anti_dual.group1.yzx * this.group9.zxy), right_anti_dual.e4125 * this.e43), 
            /* e5 */
            (right_anti_dual.e4235 * this.e15) + (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(this.e12345) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)) - float4((this.group8.xyz.zxy * right_anti_dual.group3.yzx), right_anti_dual.e4235 * this.e415), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group6.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz) + (this.group7.xyz.zxy * right_anti_dual.group3.yzx) - (this.group7.xyz.yzx * right_anti_dual.group3.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * this.group8.xyz) + (float3(right_anti_dual.e3215) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz) - (float3(this.e321) * right_anti_dual.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual.group3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3, 
            /* e1234 */
            right_anti_dual.e1234 * this.e12345
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual.e5 * this.e1234) + (right_anti_dual.e12345 * this.scalar), right_anti_dual.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e5 */
            (right_anti_dual.e5 * this.e12345) + (right_anti_dual.e12345 * this.e5), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group9, 
            /* e1234 */
            right_anti_dual.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e5 */
            -(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_anti_dual.group0, 
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
    public MultiVector weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e15 * this.e1234) + (right_anti_dual.e4125 * this.e31), (right_anti_dual.e25 * this.e1234) + (right_anti_dual.e4235 * this.e12), (right_anti_dual.e35 * this.e1234) + (right_anti_dual.e4315 * this.e23), -(right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) + float4((this.group4.xyz * right_anti_dual.group1.www), right_anti_dual.e45 * this.e1234) - (right_anti_dual.group1.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5 */
            (right_anti_dual.e4235 * this.e15) + (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(this.e12345) * right_anti_dual.group0) - float4((this.group8.xyz.zxy * right_anti_dual.group1.yzx), right_anti_dual.e4235 * this.e415), 
            /* e41, e42, e43 */
            (this.group7.xyz.zxy * right_anti_dual.group1.yzx) - (this.group7.xyz.yzx * right_anti_dual.group1.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_anti_dual.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group1.yzx * this.group9.zxy) - (right_anti_dual.group1.zxy * this.group9.yzx)), right_anti_dual.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(this.e1234) * right_anti_dual.group1.xyz, 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) - float4((right_anti_dual.group1.xyz.zxy * this.group7.xyz.yzx), right_anti_dual.e415 * this.e423), 
            /* e5 */
            -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_anti_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * right_anti_dual.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), right_anti_dual.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e12345 * this.e1), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e12345 * this.e2), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e12345 * this.e3), -(right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) + float4((this.group7.xyz.zxy * right_anti_dual.group1.yzx), right_anti_dual.e12345 * this.e4) - float4((this.group7.xyz.yzx * right_anti_dual.group1.zxy), right_anti_dual.e415 * this.e423), 
            /* e5 */
            (right_anti_dual.e12345 * this.e5) + (right_anti_dual.e5 * this.e12345) - (right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e12345 * this.e15) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e12345 * this.e25) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e12345 * this.e35) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) + (right_anti_dual.group0 * float4(this.group9.www, this.e45)) - (this.group9.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e12345) * this.group4.xyz) + (float3(this.e1234) * right_anti_dual.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e12345) * this.group5.xyz) + (float3(this.e1234) * right_anti_dual.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual.e12345) * this.group6.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz)), right_anti_dual.e12345 * this.e321), 
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e12345) * this.group8.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group9, 
            /* e1234 */
            right_anti_dual.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5 */
            (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (this.group7.xyz.zxy * right_anti_dual.group0.yzx) - (this.group7.xyz.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_anti_dual.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((this.group9.zxy * right_anti_dual.group0.yzx) - (this.group9.yzx * right_anti_dual.group0.zxy)), this.e1234 * right_anti_dual.e3215), 
            /* e423, e431, e412 */
            float3(this.e1234) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(Circle other) {
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
            float2(-(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group9.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e42)) + float4((float3(this.e1234) * right_anti_dual.group2.xyz), right_anti_dual.e41 * this.e4235), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45), 
            /* e41, e42, e43 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * right_anti_dual.group1.xyz, 
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
    public MultiVector weight_contraction(CircleRotor other) {
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
            float2((right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group9.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)) + float4((float3(this.e1234) * right_anti_dual.group2.xyz), right_anti_dual.e42 * this.e4315), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45), 
            /* e41, e42, e43 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * right_anti_dual.group1.xyz, 
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
    public MultiVector weight_contraction(Dipole other) {
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
            float2(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) - float4((right_anti_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_anti_dual.e423 * this.e415) - float4((right_anti_dual.group2.xyz.zxy * this.group7.xyz.yzx), right_anti_dual.e431 * this.e425), 
            /* e5 */
            -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_anti_dual.group2.xyz.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group9.yzx) - (right_anti_dual.group0.xyz.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_anti_dual.group0.xyz) + (float3(this.e1234) * right_anti_dual.group2.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e423, e431, e412 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_anti_dual.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(DipoleInversion other) {
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
            float2((right_anti_dual.e4 * this.e3215) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) + (float4(this.e12345) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)) - float4((right_anti_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_anti_dual.e423 * this.e415) - float4((this.group7.xyz.yzx * right_anti_dual.group2.zxy), right_anti_dual.e431 * this.e425), 
            /* e5 */
            (right_anti_dual.e5 * this.e12345) - (right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group9.yzx) - (right_anti_dual.group0.xyz.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_anti_dual.group0.xyz) + (float3(this.e1234) * right_anti_dual.group2.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e423, e431, e412 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_anti_dual.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual.e3215 * this.e4) + (right_anti_dual.scalar * this.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            right_anti_dual.e3215 * this.e45, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_anti_dual.e3215) * this.group7.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual.e3215) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual.e321) * this.group6.xyz), 0.0) + float4((this.group7.xyz.zxy * right_anti_dual.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual.group0.yzx * this.group9.zxy) - (right_anti_dual.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e321 * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual.e321) * this.group6.xyz), 0.0) + float4((float3(this.e12345) * right_anti_dual.group1.xyz), 0.0) + float4((this.group7.xyz.zxy * right_anti_dual.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e5 */
            (right_anti_dual.e5 * this.e12345) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual.group0.yzx * this.group9.zxy) - (right_anti_dual.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e321 * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.zxy * this.group9.yzx), 0.0) - float4((right_anti_dual.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
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
    public MultiVector weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual.scalar * this.e12345) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual.e3215) * this.group4.xyz), 0.0) + float4((float3(this.e1234) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.zxy * this.group9.yzx), 0.0) - float4((right_anti_dual.group0.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.e3215) * this.group6.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e3215) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual.e3215) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((right_anti_dual.scalar * this.e12345) + (right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), right_anti_dual.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e15 * this.e1234) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e4125 * this.e31), (right_anti_dual.e25 * this.e1234) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e4235 * this.e12), (right_anti_dual.e35 * this.e1234) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e4315 * this.e23), -(right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412) - (right_anti_dual.e1234 * this.e45)) + (float4(right_anti_dual.e12345) * this.group1) + (float4(this.e12345) * right_anti_dual.group1) + (this.group9.yzxx * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e41)) + float4((this.group4.xyz * right_anti_dual.group9.www), right_anti_dual.e45 * this.e1234) + float4((right_anti_dual.group7.xyz.zxy * this.group8.xyz.yzx), right_anti_dual.e42 * this.e4315) + float4((right_anti_dual.group8.xyz.yzx * this.group7.xyz.zxy), right_anti_dual.e43 * this.e4125) - (right_anti_dual.group9.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((float3(right_anti_dual.e1234) * this.group3.xyz), right_anti_dual.e4125 * this.e43) - float4((right_anti_dual.group4.xyz * this.group9.www), right_anti_dual.e423 * this.e415) - float4((right_anti_dual.group5.xyz.yzx * this.group9.zxy), right_anti_dual.e431 * this.e425) - float4((right_anti_dual.group7.xyz.yzx * this.group8.xyz.zxy), right_anti_dual.e412 * this.e435) - float4((right_anti_dual.group8.xyz.zxy * this.group7.xyz.yzx), right_anti_dual.e4235 * this.e41), 
            /* e5 */
            (right_anti_dual.e12345 * this.e5) + (right_anti_dual.e5 * this.e12345) + (right_anti_dual.e4235 * this.e15) + (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215) - (right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125) + (right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235) + (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315) + (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e435 * this.e4125) - (right_anti_dual.e4235 * this.e415) - (right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(right_anti_dual.e12345) * this.group3) + (float4(this.e12345) * right_anti_dual.group3) - (this.group9.yzxx * float4(right_anti_dual.group8.xyz.zxy, right_anti_dual.e415)) - float4((this.group8.xyz.zxy * right_anti_dual.group9.yzx), right_anti_dual.e425 * this.e4315), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e12345) * this.group4.xyz) + (float3(right_anti_dual.e1234) * this.group6.xyz) + (float3(this.e12345) * right_anti_dual.group4.xyz) + (float3(this.e1234) * right_anti_dual.group6.xyz) + (right_anti_dual.group7.xyz.zxy * this.group9.yzx) + (this.group7.xyz.zxy * right_anti_dual.group9.yzx) - (right_anti_dual.group7.xyz.yzx * this.group9.zxy) - (this.group7.xyz.yzx * right_anti_dual.group9.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e12345) * this.group5.xyz) + (float3(right_anti_dual.e3215) * this.group7.xyz) + (float3(right_anti_dual.e1234) * this.group8.xyz) + (float3(this.e12345) * right_anti_dual.group5.xyz) + (float3(this.e3215) * right_anti_dual.group7.xyz) + (float3(this.e1234) * right_anti_dual.group8.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(right_anti_dual.e12345) * this.group6) + (float4(this.e12345) * right_anti_dual.group6) + (right_anti_dual.group9.yzxw * float4(this.group9.zxy, this.e1234)) - float4((right_anti_dual.group9.zxy * this.group9.yzx), right_anti_dual.e1234 * this.e3215), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.e12345) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual.group7.xyz) + (float3(this.e1234) * right_anti_dual.group9.xyz) - (float3(right_anti_dual.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e12345) * this.group8.xyz) + (float3(right_anti_dual.e3215) * this.group9.xyz) + (float3(this.e12345) * right_anti_dual.group8.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e12345) * this.group9) + (float4(this.e12345) * right_anti_dual.group9), 
            /* e1234 */
            (right_anti_dual.e12345 * this.e1234) + (right_anti_dual.e1234 * this.e12345)
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            right_anti_dual.e5 * this.e12345, 
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
    public MultiVector weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e45 * right_anti_dual.e1234) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((float3(right_anti_dual.e1234) * this.group3.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5 */
            (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * right_anti_dual.group0.yzx) - (this.group7.xyz.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e3215) * this.group7.xyz) + (float3(right_anti_dual.e1234) * this.group8.xyz) - (float3(this.e321) * right_anti_dual.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * right_anti_dual.group0.yzx), this.e1234 * right_anti_dual.e3215) - (this.group9.yzxw * float4(right_anti_dual.group0.zxy, right_anti_dual.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1234 */
            this.e12345 * right_anti_dual.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual.e12345) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e5 */
            right_anti_dual.e12345 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(right_anti_dual.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(right_anti_dual.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group9, 
            /* e1234 */
            right_anti_dual.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5 */
            this.e12345 * right_anti_dual.e5, 
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
    public MultiVector weight_contraction(VersorEven other) {
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
            float2((this.e12345 * right_anti_dual.scalar) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)) + (this.group9.yzxz * float4(right_anti_dual.group1.zxy, right_anti_dual.e43)) + float4((this.group4.xyz * right_anti_dual.group3.www), this.e4235 * right_anti_dual.e41) + float4((this.group5.xyz.yzx * right_anti_dual.group3.zxy), this.e4315 * right_anti_dual.e42) - (right_anti_dual.group3.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((this.group9.zxy * right_anti_dual.group1.yzx), this.e43 * right_anti_dual.e4125) - float4((this.group9.www * right_anti_dual.group0.xyz), this.e45 * right_anti_dual.e1234) - float4((right_anti_dual.group2.www * this.group3.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5 */
            (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215) - (this.e4235 * right_anti_dual.e15) - (this.e4315 * right_anti_dual.e25) - (this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) + (float4(this.e12345) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)) - (right_anti_dual.group3.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * right_anti_dual.group3.yzx) - (this.group7.xyz.yzx * right_anti_dual.group3.zxy), 
            /* e23, e31, e12 */
            (float3(this.e12345) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e1234) * this.group8.xyz) + (float3(right_anti_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_anti_dual.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * right_anti_dual.group3.yzx), this.e1234 * right_anti_dual.e3215) - (this.group9.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual.group3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3, 
            /* e1234 */
            this.e12345 * right_anti_dual.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector weight_contraction(VersorOdd other) {
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
            float2((this.scalar * right_anti_dual.e12345) + (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435), this.e12345 * right_anti_dual.e12345), 
            /* e1, e2, e3, e4 */
            float4((this.e1 * right_anti_dual.e12345) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e2 * right_anti_dual.e12345) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e3 * right_anti_dual.e12345) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412) - (this.e412 * right_anti_dual.e435)) + (float4(this.e12345) * right_anti_dual.group3) + float4((this.group7.xyz.zxy * right_anti_dual.group2.yzx), this.e4 * right_anti_dual.e12345) - float4((this.group7.xyz.yzx * right_anti_dual.group2.zxy), this.e423 * right_anti_dual.e415) - float4((this.group8.xyz.zxy * right_anti_dual.group0.yzx), this.e431 * right_anti_dual.e425), 
            /* e5 */
            (this.e12345 * right_anti_dual.e5) + (this.e5 * right_anti_dual.e12345) - (this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) + (float4(right_anti_dual.e12345) * this.group3) - (this.group9.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e12345) * this.group4.xyz) + (this.group9.yzx * right_anti_dual.group0.zxy) - (this.group9.zxy * right_anti_dual.group0.yzx), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_anti_dual.group0.xyz) + (float3(this.e1234) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e12345) * this.group5.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * right_anti_dual.group1) + (float4(right_anti_dual.e12345) * this.group6), 
            /* e423, e431, e412 */
            (float3(this.e12345) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e12345) * this.group7.xyz), 
            /* e235, e315, e125 */
            (float3(this.e12345) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e12345) * this.group8.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group9, 
            /* e1234 */
            this.e1234 * right_anti_dual.e12345
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<Plane> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<Plane>(this);
    }
}
public extension WeightContractionInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public DipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension Plane {
    public DipoleInversion weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e423 * this.e3215, right_anti_dual.e431 * this.e3215, right_anti_dual.e412 * this.e3215, -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group2.yzx * this.group0.zxy), 0.0) - float4((right_anti_dual.group2.zxy * this.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0
        );
    }
}
public extension Plane {
    public AntiDipoleInversion weight_contraction(AntiDipoleInversion other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e1234) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group3.yzx * this.group0.zxy) - (right_anti_dual.group3.zxy * this.group0.yzx)), right_anti_dual.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e4235 * this.e3215 * -1.0, right_anti_dual.e4315 * this.e3215 * -1.0, right_anti_dual.e4125 * this.e3215 * -1.0, (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12 * this.e4315, right_anti_dual.e23 * this.e4125, right_anti_dual.e31 * this.e4235, -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15))
        );
    }
}
public extension Plane {
    public Plane weight_contraction(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane {
    public DualNum weight_contraction(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 0.0)
        );
    }
}
public extension Plane {
    public Motor weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e3215 * this.e4235, right_anti_dual.e3215 * this.e4315, right_anti_dual.e3215 * this.e4125, -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215)) - (this.group0.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e15))
        );
    }
}
public extension Plane {
    public FlatPoint weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e415))
        );
    }
}
public extension Plane {
    public Flector weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0
        );
    }
}
public extension Plane {
    public Line weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line(
            /* e415, e425, e435 */
            (right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz)
        );
    }
}
public extension Plane {
    public RoundPoint weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension Plane {
    public RoundPoint weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension Plane {
    public Dipole weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            (right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e423 * this.e3215, right_anti_dual.e431 * this.e3215, right_anti_dual.e412 * this.e3215, -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(this.e3215) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.yzx * this.group0.zxy) - (right_anti_dual.group2.xyz.zxy * this.group0.yzx)
        );
    }
}
public extension Plane {
    public AntiCircleRotor weight_contraction(DipoleInversion other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e423 * this.e3215, right_anti_dual.e431 * this.e3215, right_anti_dual.e412 * this.e3215, -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual.e125 * this.e4315 * -1.0, right_anti_dual.e235 * this.e4125 * -1.0, right_anti_dual.e315 * this.e4235 * -1.0, (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)) + (float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4)) + (this.group0.zxyx * float4(right_anti_dual.group2.yzx, right_anti_dual.e1))
        );
    }
}
public extension Plane {
    public AntiFlatPoint weight_contraction(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Plane {
    public AntiLine weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual.e321) * this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)
        );
    }
}
public extension Plane {
    public AntiMotor weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual.group0.www * this.group0.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Plane {
    public AntiPlane weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12 * this.e4315, right_anti_dual.e23 * this.e4125, right_anti_dual.e31 * this.e4235, -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e15))
        );
    }
}
public extension Plane {
    public AntiFlector weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12 * this.e4315, right_anti_dual.e23 * this.e4125, right_anti_dual.e31 * this.e4235, -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e15))
        );
    }
}
public extension Plane {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e41)), 
            /* e5 */
            -(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual.group8.xyz.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43 */
            (right_anti_dual.group7.xyz.zxy * this.group0.yzx) - (right_anti_dual.group7.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_anti_dual.group7.xyz) - (float3(right_anti_dual.e321) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group9.yzx * this.group0.zxy) - (right_anti_dual.group9.zxy * this.group0.yzx)), right_anti_dual.e1234 * this.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(right_anti_dual.e1234) * this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension Plane {
    public Scalar weight_contraction(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)
        );
    }
}
public extension Plane {
    public Circle weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual.e1234) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * right_anti_dual.group0.yzx) - (this.group0.yzx * right_anti_dual.group0.zxy)), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz)
        );
    }
}
public extension Plane {
    public Plane weight_contraction(Scalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane {
    public Scalar weight_contraction(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Scalar(
            /* scalar */
            (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)
        );
    }
}
public extension Plane {
    public AntiDipoleInversion weight_contraction(VersorEven other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual.e1234) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * right_anti_dual.group3.yzx) - (this.group0.yzx * right_anti_dual.group3.zxy)), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * right_anti_dual.e4235 * -1.0, this.e3215 * right_anti_dual.e4315 * -1.0, this.e3215 * right_anti_dual.e4125 * -1.0, (this.e4315 * right_anti_dual.e42) + (this.e4125 * right_anti_dual.e43)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e4315 * right_anti_dual.e12, this.e4125 * right_anti_dual.e23, this.e4235 * right_anti_dual.e31, -(this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15)) - (this.group0.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e25))
        );
    }
}
public extension Plane {
    public VersorOdd weight_contraction(VersorOdd other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e4125 * right_anti_dual.e431 * -1.0, this.e4235 * right_anti_dual.e412 * -1.0, this.e4315 * right_anti_dual.e423 * -1.0, (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)) + (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e1)), 
            /* e23, e31, e12, e45 */
            float4(this.e3215 * right_anti_dual.e423, this.e3215 * right_anti_dual.e431, this.e3215 * right_anti_dual.e412, -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * right_anti_dual.group1.xyz), 0.0) + float4((this.group0.zxy * right_anti_dual.group2.yzx), 0.0) - float4((this.group0.yzx * right_anti_dual.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0
        );
    }
}
public extension RoundPoint {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<RoundPoint> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<RoundPoint>(this);
    }
}
public extension WeightContractionInfixPartial<RoundPoint> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension RoundPoint {
    public RoundPoint weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5 */
            right_anti_dual.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public Scalar weight_contraction(AntiDipoleInversion other) {
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
        return Scalar(
            /* scalar */
            (right_anti_dual.e1234 * this.e5) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)
        );
    }
}
public extension RoundPoint {
    public RoundPoint weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5 */
            right_anti_dual.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public Scalar weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)
        );
    }
}
public extension RoundPoint {
    public RoundPoint weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5 */
            right_anti_dual.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public Scalar weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)
        );
    }
}
public extension RoundPoint {
    public Scalar weight_contraction(DualNum other) {
        return Scalar(
            /* scalar */
            other.e5 * this.e4 * -1.0
        );
    }
}
public extension RoundPoint {
    public Scalar weight_contraction(Motor other) {
        return Scalar(
            /* scalar */
            float4(other.group1.xyz, other.e5 * -1.0)[3] * this.e4
        );
    }
}
public extension RoundPoint {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5 */
            right_anti_dual.e12345 * this.e5, 
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
    public Scalar weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Scalar(
            /* scalar */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234)
        );
    }
}
public extension RoundPoint {
    public RoundPoint weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5 */
            right_anti_dual.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public Scalar weight_contraction(VersorEven other) {
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
        return Scalar(
            /* scalar */
            (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234)
        );
    }
}
public extension RoundPoint {
    public RoundPoint weight_contraction(VersorOdd other) {
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5 */
            this.e5 * right_anti_dual.e12345
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<Scalar> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<Scalar>(this);
    }
}
public extension WeightContractionInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension Scalar {
    public Scalar weight_contraction(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar weight_contraction(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar weight_contraction(AntiMotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar weight_contraction(MultiVector other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar weight_contraction(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar weight_contraction(VersorOdd other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension Sphere {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<Sphere> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<Sphere>(this);
    }
}
public extension WeightContractionInfixPartial<Sphere> {
    // Fancy infix trick (second half)
    public DipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension Sphere {
    public DipoleInversion weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.yzx * this.group0.zxy) - (right_anti_dual.group2.zxy * this.group0.yzx)), right_anti_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0
        );
    }
}
public extension Sphere {
    public AntiDipoleInversion weight_contraction(AntiDipoleInversion other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e4235 * this.e3215 * -1.0, right_anti_dual.e4315 * this.e3215 * -1.0, right_anti_dual.e4125 * this.e3215 * -1.0, (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15))
        );
    }
}
public extension Sphere {
    public VersorOdd weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), right_anti_dual.e5 * this.e1234), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), right_anti_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0
        );
    }
}
public extension Sphere {
    public RoundPoint weight_contraction(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e5 */
            -(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215)
        );
    }
}
public extension Sphere {
    public AntiDipoleInversion weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_anti_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), right_anti_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group1.xyz)), right_anti_dual.e45 * this.e1234), 
            /* e1, e2, e3, e5 */
            float4((float3(this.e1234) * right_anti_dual.group0.xyz), -(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215))
        );
    }
}
public extension Sphere {
    public Dipole weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz), -(right_anti_dual.e415 * this.e4235) - (right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)), 
            /* e15, e25, e35 */
            (float3(this.e3215) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.xyz.yzx * this.group0.zxy) - (right_anti_dual.group1.xyz.zxy * this.group0.yzx)
        );
    }
}
public extension Sphere {
    public VersorOdd weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e1234) * float4(right_anti_dual.group0.xyz, right_anti_dual.e5), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz), -(right_anti_dual.e415 * this.e4235) - (right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), right_anti_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0
        );
    }
}
public extension Sphere {
    public Circle weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(this.e1234) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)), right_anti_dual.e3215 * this.e1234), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz)
        );
    }
}
public extension Sphere {
    public RoundPoint weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group0.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e42)) + float4((float3(this.e1234) * right_anti_dual.group2.xyz), right_anti_dual.e41 * this.e4235), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension Sphere {
    public RoundPoint weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)) + float4((float3(this.e1234) * right_anti_dual.group2.xyz), right_anti_dual.e42 * this.e4315), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension Sphere {
    public Dipole weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(this.e3215) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.yzx * this.group0.zxy) - (right_anti_dual.group2.xyz.zxy * this.group0.yzx)
        );
    }
}
public extension Sphere {
    public AntiCircleRotor weight_contraction(DipoleInversion other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual.e125 * this.e4315 * -1.0, right_anti_dual.e235 * this.e4125 * -1.0, right_anti_dual.e315 * this.e4235 * -1.0, (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)) + (float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4)) + (this.group0.zxyx * float4(right_anti_dual.group2.yzx, right_anti_dual.e1))
        );
    }
}
public extension Sphere {
    public AntiFlatPoint weight_contraction(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e1234)
        );
    }
}
public extension Sphere {
    public AntiLine weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            (float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e321) * this.group0.xyz), 
            /* e15, e25, e35 */
            (right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)
        );
    }
}
public extension Sphere {
    public AntiMotor weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e321 * this.e4235 * -1.0, right_anti_dual.e321 * this.e4315 * -1.0, right_anti_dual.e321 * this.e4125 * -1.0, (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)) + float4((float3(this.e1234) * right_anti_dual.group0.xyz), right_anti_dual.e1 * this.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Sphere {
    public AntiPlane weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e15))
        );
    }
}
public extension Sphere {
    public AntiFlector weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e15))
        );
    }
}
public extension Sphere {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e45 * this.e1234) + (right_anti_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e41)) + float4((float3(this.e1234) * right_anti_dual.group3.xyz), right_anti_dual.e42 * this.e4315), 
            /* e5 */
            -(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual.group8.xyz.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group6.xyz) + (right_anti_dual.group7.xyz.zxy * this.group0.yzx) - (right_anti_dual.group7.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_anti_dual.group7.xyz) + (float3(this.e1234) * right_anti_dual.group8.xyz) - (float3(right_anti_dual.e321) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group9.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_anti_dual.group9.zxy * this.group0.yzx), right_anti_dual.e1234 * this.e3215), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group9.xyz) - (float3(right_anti_dual.e1234) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1234 */
            right_anti_dual.e12345 * this.e1234
        );
    }
}
public extension Sphere {
    public Scalar weight_contraction(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public Circle weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Circle(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group0.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_anti_dual.group0.zxy * this.group0.yzx), right_anti_dual.e1234 * this.e3215), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz)
        );
    }
}
public extension Sphere {
    public Sphere weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1234 */
            right_anti_dual.e12345 * this.e1234
        );
    }
}
public extension Sphere {
    public Scalar weight_contraction(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Scalar(
            /* scalar */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public AntiDipoleInversion weight_contraction(VersorEven other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.group0.zxy * right_anti_dual.group3.yzx), this.e1234 * right_anti_dual.e3215) - (this.group0.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * right_anti_dual.e4235 * -1.0, this.e3215 * right_anti_dual.e4315 * -1.0, this.e3215 * right_anti_dual.e4125 * -1.0, (this.e4315 * right_anti_dual.e42) + (this.e4125 * right_anti_dual.e43) + (this.e1234 * right_anti_dual.e45)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4((this.e4315 * right_anti_dual.e12) + (this.e1234 * right_anti_dual.e15), (this.e4125 * right_anti_dual.e23) + (this.e1234 * right_anti_dual.e25), (this.e4235 * right_anti_dual.e31) + (this.e1234 * right_anti_dual.e35), -(this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15)) - (this.group0.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e25))
        );
    }
}
public extension Sphere {
    public VersorOdd weight_contraction(VersorOdd other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e4125 * right_anti_dual.e431 * -1.0, this.e4235 * right_anti_dual.e412 * -1.0, this.e4315 * right_anti_dual.e423 * -1.0, (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5)) + (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e1)) + float4((float3(this.e1234) * right_anti_dual.group1.xyz), this.e4315 * right_anti_dual.e2), 
            /* e23, e31, e12, e45 */
            float4((this.e3215 * right_anti_dual.e423) + (this.e1234 * right_anti_dual.e235), (this.e3215 * right_anti_dual.e431) + (this.e1234 * right_anti_dual.e315), (this.e3215 * right_anti_dual.e412) + (this.e1234 * right_anti_dual.e125), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_anti_dual.group1.xyz) + (this.group0.zxy * right_anti_dual.group2.yzx) - (this.group0.yzx * right_anti_dual.group2.zxy)), this.e1234 * right_anti_dual.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0
        );
    }
}
public extension VersorEven {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<VersorEven> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<VersorEven>(this);
    }
}
public extension WeightContractionInfixPartial<VersorEven> {
    // Fancy infix trick (second half)
    public VersorEven operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension VersorEven {
    public VersorEven weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_anti_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz)), right_anti_dual.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(right_anti_dual.e12345) * this.group1) + (float4(this.e12345) * right_anti_dual.group1), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e12345 * this.e235, right_anti_dual.e12345 * this.e315, right_anti_dual.e12345 * this.e125, -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + (right_anti_dual.group2 * float4(this.group0.www, this.e5)), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412) + (right_anti_dual.e12345 * this.e1), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423) + (right_anti_dual.e12345 * this.e2), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431) + (right_anti_dual.e12345 * this.e3), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) + float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), right_anti_dual.e12345 * this.e4) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.e423 * this.e415) - float4((right_anti_dual.group2.zxy * this.group0.yzx), right_anti_dual.e431 * this.e425)
        );
    }
}
public extension VersorEven {
    public VersorOdd weight_contraction(AntiDipoleInversion other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual.group3.yzxy * float4(this.group0.zxy, this.e2)) + float4(float3(0.0), (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) + float4((right_anti_dual.group0.xyz * this.group0.www), right_anti_dual.e1234 * this.e5) + float4((right_anti_dual.group2.www * this.group1.xyz), right_anti_dual.e4235 * this.e1) - float4((right_anti_dual.group3.zxy * this.group0.yzx), right_anti_dual.e41 * this.e235), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e1234 * this.e235) + (right_anti_dual.e3215 * this.e423), (right_anti_dual.e1234 * this.e315) + (right_anti_dual.e3215 * this.e431), (right_anti_dual.e1234 * this.e125) + (right_anti_dual.e3215 * this.e412), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(this.e12345) * right_anti_dual.group1) - (right_anti_dual.group3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e3215) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual.group2.xyz) + (right_anti_dual.group3.zxy * this.group2.yzx) - (right_anti_dual.group3.yzx * this.group2.zxy)), right_anti_dual.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension VersorEven {
    public VersorEven weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, (right_anti_dual.e5 * this.e12345) + (right_anti_dual.e12345 * this.e5)) * float4(this.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension VersorEven {
    public AntiCircleRotor weight_contraction(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_anti_dual.e45 * this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz), -(right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321))
        );
    }
}
public extension VersorEven {
    public VersorOdd weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual.group1.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e15)), 
            /* e23, e31, e12, e45 */
            (this.group0 * float4(right_anti_dual.group1.www, right_anti_dual.e45)) + float4(float3(0.0), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) - (right_anti_dual.group1.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.e3215) * this.group1.xyz), 0.0) + float4((float3(this.e12345) * right_anti_dual.group0.xyz), 0.0) + float4((right_anti_dual.group1.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual.group1.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group1.xyz), -(right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((right_anti_dual.group1.xyz.zxy * this.group0.yzx), right_anti_dual.e415 * this.e235)
        );
    }
}
public extension VersorEven {
    public VersorEven weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual.e12345) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz)), right_anti_dual.e12345 * this.e321), 
            /* e235, e315, e125, e5 */
            (float4(right_anti_dual.e12345) * this.group2) + (float4(this.e12345) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e12345 * this.e1) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e12345 * this.e2) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e12345 * this.e3) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) + (right_anti_dual.group0 * float4(this.group1.www, this.e4)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e415))
        );
    }
}
public extension VersorEven {
    public VersorOdd weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.e4125 * this.e431 * -1.0, right_anti_dual.e4235 * this.e412 * -1.0, right_anti_dual.e4315 * this.e423 * -1.0, (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)) + (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e1)), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e3215 * this.e423, right_anti_dual.e3215 * this.e431, right_anti_dual.e3215 * this.e412, -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.e3215) * this.group1.xyz), 0.0) + float4((right_anti_dual.group0.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual.group0.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0
        );
    }
}
public extension VersorEven {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension VersorEven {
    public AntiCircleRotor weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
public extension VersorEven {
    public AntiCircleRotor weight_contraction(CircleRotor other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e125 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.e235 * this.e415) - float4((right_anti_dual.group2.xyz.zxy * this.group0.yzx), right_anti_dual.e315 * this.e425)
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion weight_contraction(DipoleInversion other) {
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
            float3(this.e12345) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.e4 * this.e12345) - (right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412) + (right_anti_dual.e1 * this.e12345), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423) + (right_anti_dual.e2 * this.e12345), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431) + (right_anti_dual.e3 * this.e12345), -(right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), right_anti_dual.e5 * this.e12345) - (this.group2.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e415)) - float4((right_anti_dual.group2.zxy * this.group0.yzx), right_anti_dual.e425 * this.e315)
        );
    }
}
public extension VersorEven {
    public AntiMotor weight_contraction(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, (right_anti_dual.e3215 * this.e4) + (right_anti_dual.scalar * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
public extension VersorEven {
    public AntiFlector weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e315 * this.e412) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e125 * this.e423) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e235 * this.e431) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
public extension VersorEven {
    public AntiFlector weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e321 * this.e415) + (right_anti_dual.e1 * this.e12345), (right_anti_dual.e321 * this.e425) + (right_anti_dual.e2 * this.e12345), (right_anti_dual.e321 * this.e435) + (right_anti_dual.e3 * this.e12345), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + (this.group0.zxyw * float4(right_anti_dual.group0.yzx, right_anti_dual.e5)) - (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
public extension VersorEven {
    public AntiMotor weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension VersorEven {
    public AntiMotor weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e4)) + (float4(this.e12345) * right_anti_dual.group0) + float4(float3(0.0), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual.e3215) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz)), right_anti_dual.e3215 * this.e12345)
        );
    }
}
public extension VersorEven {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((right_anti_dual.scalar * this.e12345) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435), right_anti_dual.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e1 * this.e12345) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e2 * this.e12345) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e3 * this.e12345) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412) - (right_anti_dual.e412 * this.e435)) + (float4(right_anti_dual.e12345) * this.group3) + float4((right_anti_dual.group7.xyz.zxy * this.group2.yzx), right_anti_dual.e4 * this.e12345) - float4((right_anti_dual.group7.xyz.yzx * this.group2.zxy), right_anti_dual.e423 * this.e415) - float4((right_anti_dual.group8.xyz.zxy * this.group0.yzx), right_anti_dual.e431 * this.e425), 
            /* e5 */
            (right_anti_dual.e12345 * this.e5) + (right_anti_dual.e5 * this.e12345) - (right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(this.e12345) * right_anti_dual.group3) - (right_anti_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual.group4.xyz) + (right_anti_dual.group9.yzx * this.group0.zxy) - (right_anti_dual.group9.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e3215) * this.group0.xyz) + (float3(right_anti_dual.e1234) * this.group2.xyz) + (float3(this.e12345) * right_anti_dual.group5.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(right_anti_dual.e12345) * this.group1) + (float4(this.e12345) * right_anti_dual.group6), 
            /* e423, e431, e412 */
            (float3(right_anti_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual.group7.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e12345) * this.group2.xyz) + (float3(this.e12345) * right_anti_dual.group8.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group9, 
            /* e1234 */
            right_anti_dual.e1234 * this.e12345
        );
    }
}
public extension VersorEven {
    public AntiPlane weight_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension VersorEven {
    public VersorOdd weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.e4125 * this.e431 * -1.0, right_anti_dual.e4235 * this.e412 * -1.0, right_anti_dual.e4315 * this.e423 * -1.0, (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5)) + (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e1)) + float4((float3(right_anti_dual.e1234) * this.group1.xyz), right_anti_dual.e4315 * this.e2), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e3215 * this.e423) + (right_anti_dual.e1234 * this.e235), (right_anti_dual.e3215 * this.e431) + (right_anti_dual.e1234 * this.e315), (right_anti_dual.e3215 * this.e412) + (right_anti_dual.e1234 * this.e125), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e3215) * this.group1.xyz) + (right_anti_dual.group0.zxy * this.group2.yzx) - (right_anti_dual.group0.yzx * this.group2.zxy)), right_anti_dual.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0
        );
    }
}
public extension VersorEven {
    public VersorEven weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e12345) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension VersorEven {
    public RoundPoint weight_contraction(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5 */
            right_anti_dual.e5 * this.e12345
        );
    }
}
public extension VersorEven {
    public VersorOdd weight_contraction(VersorEven other) {
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
            (this.group0.zxyw * float4(right_anti_dual.group3.yzx, right_anti_dual.scalar)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) + float4((this.group0.www * right_anti_dual.group0.xyz), this.e5 * right_anti_dual.e1234) + float4((right_anti_dual.group2.www * this.group1.xyz), this.e1 * right_anti_dual.e4235) - (this.group0.yzxx * float4(right_anti_dual.group3.zxy, right_anti_dual.e15)), 
            /* e23, e31, e12, e45 */
            float4((this.e12345 * right_anti_dual.e23) + (this.e235 * right_anti_dual.e1234), (this.e12345 * right_anti_dual.e31) + (this.e315 * right_anti_dual.e1234), (this.e12345 * right_anti_dual.e12) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) + (this.group0 * float4(right_anti_dual.group3.www, right_anti_dual.e45)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e3215) * this.group1.xyz) + (this.group2.yzx * right_anti_dual.group3.zxy) - (this.group2.zxy * right_anti_dual.group3.yzx)), this.e12345 * right_anti_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
public extension VersorEven {
    public VersorEven weight_contraction(VersorOdd other) {
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
            float4(((float3(right_anti_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz)), right_anti_dual.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(right_anti_dual.e12345) * this.group1) + (float4(this.e12345) * right_anti_dual.group1), 
            /* e235, e315, e125, e5 */
            (float4(right_anti_dual.e12345) * this.group2) + (float4(this.e12345) * right_anti_dual.group2) + float4(float3(0.0), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412) + (right_anti_dual.e1 * this.e12345), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423) + (right_anti_dual.e2 * this.e12345), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431) + (right_anti_dual.e3 * this.e12345), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) + (right_anti_dual.group0.zxyw * float4(this.group2.yzx, this.e4)) + float4((right_anti_dual.group0.www * this.group3.xyz), right_anti_dual.e4 * this.e12345) - (right_anti_dual.group0.yzxx * float4(this.group2.zxy, this.e415)) - float4((right_anti_dual.group2.zxy * this.group0.yzx), right_anti_dual.e431 * this.e425)
        );
    }
}
public extension VersorOdd {
    // Fancy infix trick (first half)
    public WeightContractionInfixPartial<VersorOdd> operator /(WeightContractionInfix rhs) {
        return WeightContractionInfixPartial<VersorOdd>(this);
    }
}
public extension WeightContractionInfixPartial<VersorOdd> {
    // Fancy infix trick (second half)
    public VersorOdd operator /(AntiCircleRotor rhs) {
        return this.a.weight_contraction(rhs);
    }
}
public extension VersorOdd {
    public VersorOdd weight_contraction(AntiCircleRotor other) {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_anti_dual.e415 * this.e1234) + (right_anti_dual.e12345 * this.e41), (right_anti_dual.e425 * this.e1234) + (right_anti_dual.e12345 * this.e42), (right_anti_dual.e435 * this.e1234) + (right_anti_dual.e12345 * this.e43), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) + float4((right_anti_dual.group0.xyz.zxy * this.group3.yzx), right_anti_dual.e12345 * this.scalar) - float4((right_anti_dual.group0.xyz.yzx * this.group3.zxy), right_anti_dual.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e235 * this.e1234) + (right_anti_dual.e12345 * this.e23), (right_anti_dual.e315 * this.e1234) + (right_anti_dual.e12345 * this.e31), (right_anti_dual.e125 * this.e1234) + (right_anti_dual.e12345 * this.e12), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) + float4((right_anti_dual.group0.xyz * this.group3.www), right_anti_dual.e12345 * this.e45) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e12345) * this.group2.xyz) + (float3(this.e3215) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.yzx * this.group3.zxy) - (right_anti_dual.group2.zxy * this.group3.yzx)), right_anti_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion weight_contraction(AntiDipoleInversion other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)) + float4(float3(0.0), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234) - (right_anti_dual.e4235 * this.e41) - (right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) - float4((this.group3.www * right_anti_dual.group3.xyz), right_anti_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_anti_dual.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_anti_dual.group2.xyz, right_anti_dual.e4315)) + float4((right_anti_dual.group1.zxy * this.group3.yzx), right_anti_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15)) - float4((right_anti_dual.group3.yzx * this.group1.zxy), right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public VersorOdd weight_contraction(AntiDualNum other) {
        DualNum right_anti_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, (right_anti_dual.e5 * this.e1234) + (right_anti_dual.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public RoundPoint weight_contraction(AntiFlatPoint other) {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e5 */
            -(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215)
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion weight_contraction(AntiFlector other) {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_anti_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group1.yzx * this.group3.zxy) - (right_anti_dual.group1.zxy * this.group3.yzx)), right_anti_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) + float4((right_anti_dual.group1.www * this.group3.xyz), right_anti_dual.e45 * this.e1234) - (right_anti_dual.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (right_anti_dual.group1.zxyy * float4(this.group1.yzx, this.e25)) + (right_anti_dual.group1.wwwz * float4(this.group0.xyz, this.e35)) + (this.group2.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e4235)) + float4(float3(0.0), (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215)) - float4((right_anti_dual.group1.yzx * this.group1.zxy), right_anti_dual.e15 * this.e4235)
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor weight_contraction(AntiLine other) {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz), -(right_anti_dual.e415 * this.e4235) - (right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((right_anti_dual.group1.xyz.zxy * this.group3.yzx), right_anti_dual.e415 * this.e23)
        );
    }
}
public extension VersorOdd {
    public VersorOdd weight_contraction(AntiMotor other) {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual.group0 * float4(this.group2.www, this.scalar)) + float4(float3(0.0), -(right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) + float4((right_anti_dual.group0.www * this.group0.xyz), right_anti_dual.e5 * this.e1234), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e235 * this.e1234, right_anti_dual.e315 * this.e1234, right_anti_dual.e125 * this.e1234, -(right_anti_dual.e415 * this.e4235) - (right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) + (float4(right_anti_dual.e12345) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e12345) * this.group2.xyz) + (float3(this.e3215) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.yzx * this.group3.zxy) - (right_anti_dual.group1.zxy * this.group3.yzx)), right_anti_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion weight_contraction(AntiPlane other) {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx)), right_anti_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e3215 * this.e4235, right_anti_dual.e3215 * this.e4315, right_anti_dual.e3215 * this.e4125, -(right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e4315 * this.e12 * -1.0, right_anti_dual.e4125 * this.e23 * -1.0, right_anti_dual.e4235 * this.e31 * -1.0, (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45)) + (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e15)) + (right_anti_dual.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
public extension VersorOdd {
    public RoundPoint weight_contraction(Circle other) {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group3.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e42)) + float4((right_anti_dual.group2.xyz * this.group2.www), right_anti_dual.e41 * this.e4235), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public RoundPoint weight_contraction(CircleRotor other) {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)) + float4((this.group2.www * right_anti_dual.group2.xyz), right_anti_dual.e42 * this.e4315), 
            /* e5 */
            -(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor weight_contraction(Dipole other) {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group3.yzx) - (right_anti_dual.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((right_anti_dual.group2.xyz.zxy * this.group3.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor weight_contraction(DipoleInversion other) {
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
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group3.yzx) - (right_anti_dual.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4)) + (this.group3.zxyx * float4(right_anti_dual.group2.yzx, right_anti_dual.e1)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((right_anti_dual.group2.zxy * this.group3.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
public extension VersorOdd {
    public AntiFlector weight_contraction(DualNum other) {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension VersorOdd {
    public AntiMotor weight_contraction(FlatPoint other) {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e235 * this.e1234, right_anti_dual.e315 * this.e1234, right_anti_dual.e125 * this.e1234, -(right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension VersorOdd {
    public AntiMotor weight_contraction(Flector other) {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) + float4((this.group2.www * right_anti_dual.group0.xyz), right_anti_dual.e1 * this.e4235) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension VersorOdd {
    public AntiPlane weight_contraction(Line other) {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group3.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e15))
        );
    }
}
public extension VersorOdd {
    public AntiFlector weight_contraction(Motor other) {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e15 * this.e1234) + (right_anti_dual.e3215 * this.e41), (right_anti_dual.e25 * this.e1234) + (right_anti_dual.e3215 * this.e42), (right_anti_dual.e35 * this.e1234) + (right_anti_dual.e3215 * this.e43), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) + float4((right_anti_dual.group0.zxy * this.group3.yzx), right_anti_dual.e3215 * this.e45) - (this.group3.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e15))
        );
    }
}
public extension VersorOdd {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            (this.group3.yzxx * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e41)) + float4((right_anti_dual.group9.zxy * this.group1.yzx), right_anti_dual.e43 * this.e4125) + float4((right_anti_dual.group9.www * this.group0.xyz), right_anti_dual.e45 * this.e1234) + float4((this.group2.www * right_anti_dual.group3.xyz), right_anti_dual.e42 * this.e4315) - (float4(right_anti_dual.e1234) * float4(this.group2.xyz, this.e45)) - (right_anti_dual.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((right_anti_dual.group4.xyz * this.group3.www), right_anti_dual.e4235 * this.e41) - float4((right_anti_dual.group5.xyz.yzx * this.group3.zxy), right_anti_dual.e4315 * this.e42), 
            /* e5 */
            (right_anti_dual.e4235 * this.e15) + (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) + (float4(right_anti_dual.e12345) * float4(this.group2.xyz, this.e45)) - (this.group3.yzxx * float4(right_anti_dual.group8.xyz.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e12345) * this.group0.xyz) + (float3(this.e1234) * right_anti_dual.group6.xyz) + (right_anti_dual.group7.xyz.zxy * this.group3.yzx) - (right_anti_dual.group7.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e12345) * this.group1.xyz) + (float3(this.e1234) * right_anti_dual.group8.xyz) + (float3(this.e3215) * right_anti_dual.group7.xyz) - (float3(right_anti_dual.e321) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group9.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_anti_dual.group9.zxy * this.group3.yzx), right_anti_dual.e1234 * this.e3215), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group9.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual.e3215) * this.group3.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3, 
            /* e1234 */
            right_anti_dual.e12345 * this.e1234
        );
    }
}
public extension VersorOdd {
    public Scalar weight_contraction(Plane other) {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion weight_contraction(RoundPoint other) {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group0.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_anti_dual.group0.zxy * this.group3.yzx), right_anti_dual.e1234 * this.e3215), 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e3215 * this.e4235, right_anti_dual.e3215 * this.e4315, right_anti_dual.e3215 * this.e4125, -(right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43) - (right_anti_dual.e1234 * this.e45)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(right_anti_dual.e4315 * this.e12) - (right_anti_dual.e1234 * this.e15), -(right_anti_dual.e4125 * this.e23) - (right_anti_dual.e1234 * this.e25), -(right_anti_dual.e4235 * this.e31) - (right_anti_dual.e1234 * this.e35), (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45)) + (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e15)) + (right_anti_dual.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
public extension VersorOdd {
    public VersorOdd weight_contraction(Scalar other) {
        AntiScalar right_anti_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public Scalar weight_contraction(Sphere other) {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Scalar(
            /* scalar */
            (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234)
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion weight_contraction(VersorEven other) {
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
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)) + float4(float3(0.0), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234) - (right_anti_dual.e4235 * this.e41) - (right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) - float4((this.group3.www * right_anti_dual.group3.xyz), right_anti_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_anti_dual.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_anti_dual.group2.xyz, right_anti_dual.e4315)) + float4((right_anti_dual.group1.zxy * this.group3.yzx), right_anti_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15)) - float4((right_anti_dual.group3.yzx * this.group1.zxy), right_anti_dual.e35 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public VersorOdd weight_contraction(VersorOdd other) {
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
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual.group0.zxyw * float4(this.group3.yzx, this.scalar)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) + float4((right_anti_dual.group0.www * this.group0.xyz), right_anti_dual.e5 * this.e1234) + float4((this.group2.www * right_anti_dual.group1.xyz), right_anti_dual.e1 * this.e4235) - (right_anti_dual.group0.yzxx * float4(this.group3.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e12345 * this.e23) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e12345 * this.e31) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e12345 * this.e12) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) + (right_anti_dual.group0 * float4(this.group3.www, this.e45)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e12345) * this.group2.xyz) + (float3(this.e3215) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.yzx * this.group3.zxy) - (right_anti_dual.group2.zxy * this.group3.yzx)), right_anti_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}

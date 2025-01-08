implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<AntiCircleRotor> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<AntiCircleRotor>(this);
    }
}
public extension BulkContractionInfixPartial<AntiCircleRotor> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((right_dual.group2.www * this.group2.xyz), this.e42 * right_dual.e4315), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_dual.e12345) * this.group2
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435))
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)
        );
    }
}
public extension AntiCircleRotor {
    public AntiPlane bulk_contraction(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e45 * right_dual.e321)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e45 * right_dual.e321)
        );
    }
}
public extension AntiCircleRotor {
    public AntiPlane bulk_contraction(Motor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float4(other.group1.xyz, other.e5 * -1.0)[3]) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension AntiCircleRotor {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group9.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_dual.e1234) * this.group2.xyz), this.e42 * right_dual.e4315), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e12345) * this.group1.xyz, 
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
    public RoundPoint bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_dual.e1234) * this.group2.xyz), this.e42 * right_dual.e4315), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_dual.e12345) * this.group2
        );
    }
}
public extension AntiCircleRotor {
    public RoundPoint bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((right_dual.group2.www * this.group2.xyz), this.e42 * right_dual.e4315), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)
        );
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<AntiDipoleInversion> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<AntiDipoleInversion>(this);
    }
}
public extension BulkContractionInfixPartial<AntiDipoleInversion> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.e4 * right_dual.e12345) - (this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412) + (this.e1 * right_dual.e12345), (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423) + (this.e2 * right_dual.e12345), (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431) + (this.e3 * right_dual.e12345), -(this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + (right_dual.group2.yzxw * float4(this.group0.xyz.zxy, this.e5)) - (right_dual.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), this.e425 * right_dual.e315)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            (float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group3.yzx) - (this.group0.xyz.yzx * right_dual.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_dual.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((this.group2.zxy * right_dual.group3.yzx), this.e423 * right_dual.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_dual.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar bulk_contraction(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * right_dual.group1.yzx) - (this.group0.xyz.yzx * right_dual.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_dual.e3215, this.e431 * right_dual.e3215, this.e412 * right_dual.e3215, -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_dual.group1.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)) - float4((this.group2.zxy * right_dual.group1.yzx), this.e423 * right_dual.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e315) + (this.e321 * right_dual.e415), (this.e423 * right_dual.e125) + (this.e321 * right_dual.e425), (this.e431 * right_dual.e235) + (this.e321 * right_dual.e435), -(this.e431 * right_dual.e425) - (this.e412 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group1.xyz.zxy), this.e423 * right_dual.e415), 
            /* e5 */
            -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.e4 * right_dual.e12345) - (this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_dual.e415) + (this.e1 * right_dual.e12345), (this.e321 * right_dual.e425) + (this.e2 * right_dual.e12345), (this.e321 * right_dual.e435) + (this.e3 * right_dual.e12345), -(this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + float4((this.group0.xyz.zxy * right_dual.group1.yzx), this.e5 * right_dual.e12345) - (right_dual.group1.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_dual.e3215, this.e431 * right_dual.e3215, this.e412 * right_dual.e3215, -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * right_dual.e4315 * -1.0, this.e235 * right_dual.e4125 * -1.0, this.e315 * right_dual.e4235 * -1.0, (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125)) + (float4(right_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_dual.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) - float4((this.group0.xyz.yzx * right_dual.group2.xyz.zxy), this.e423 * right_dual.e415) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), this.e431 * right_dual.e425), 
            /* e5 */
            -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)
        );
    }
}
public extension AntiDipoleInversion {
    public RoundPoint bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.e423 * this.e415) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), right_dual.e431 * this.e425), 
            /* e5 */
            -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiMotor bulk_contraction(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.e3215) * float4(this.group0.xyz, this.e4), 
            /* e15, e25, e35, e3215 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiPlane bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321), -(this.e425 * right_dual.e315) - (this.e435 * right_dual.e125)) - (right_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiPlane bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321), -(this.e425 * right_dual.e315) - (this.e435 * right_dual.e125)) - (right_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Scalar(
            /* scalar */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiMotor bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e3215) * this.group0.xyz), (this.e4 * right_dual.e3215) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((this.e4 * right_dual.e3215) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) + (float4(right_dual.e12345) * float4(this.group3.xyz, this.e4)) - float4((this.group0.xyz.yzx * right_dual.group8.xyz.zxy), this.e423 * right_dual.e415) - float4((right_dual.group7.xyz.yzx * this.group2.zxy), this.e431 * right_dual.e425), 
            /* e5 */
            (this.e5 * right_dual.e12345) - (this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group9.yzx) - (this.group0.xyz.yzx * right_dual.group9.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e3215) * this.group0.xyz) + (float3(right_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.e12345) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * right_dual.e4315 * -1.0, this.e235 * right_dual.e4125 * -1.0, this.e315 * right_dual.e4235 * -1.0, (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)) + (float4(right_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_dual.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_dual.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public AntiCircleRotor bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            (float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group3.yzx) - (this.group0.xyz.yzx * right_dual.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_dual.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((this.group2.zxy * right_dual.group3.yzx), this.e423 * right_dual.e15)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.e4 * right_dual.e12345) - (this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412) + (this.e1 * right_dual.e12345), (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423) + (this.e2 * right_dual.e12345), (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431) + (this.e3 * right_dual.e12345), -(this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + float4((this.group0.xyz.zxy * right_dual.group2.yzx), this.e5 * right_dual.e12345) - (right_dual.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((this.group2.zxy * right_dual.group0.yzx), this.e425 * right_dual.e315)
        );
    }
}
public extension AntiDualNum {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<AntiDualNum> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<AntiDualNum>(this);
    }
}
public extension BulkContractionInfixPartial<AntiDualNum> {
    // Fancy infix trick (second half)
    public AntiMotor operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension AntiDualNum {
    public AntiMotor bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.xx, this.e3215, this.scalar) * float4(right_dual.group0.xyz, right_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e12345)
        );
    }
}
public extension AntiDualNum {
    public AntiFlector bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float4(this.e3215) * float4(right_dual.group3.xyz, right_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum bulk_contraction(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension AntiDualNum {
    public DualNum bulk_contraction(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e3215, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public AntiFlector bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e3215 * right_dual.e45 * -1.0)
        );
    }
}
public extension AntiDualNum {
    public FlatPoint bulk_contraction(AntiLine other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public AntiMotor bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.scalar * right_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * right_dual.group0
        );
    }
}
public extension AntiDualNum {
    public AntiFlatPoint bulk_contraction(AntiPlane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public AntiPlane bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiPlane bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiLine bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e3215) * right_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e3215) * right_dual.group1.xyz
        );
    }
}
public extension AntiDualNum {
    public AntiMotor bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e4), 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiDualNum {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((this.e3215 * right_dual.e4) + (this.scalar * right_dual.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group4.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e5 */
            this.e3215 * right_dual.e45 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e3215) * right_dual.group7.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * right_dual.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e3215) * right_dual.group9.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * right_dual.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiDualNum {
    public AntiFlatPoint bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e1234) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum bulk_contraction(Scalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension AntiDualNum {
    public Scalar bulk_contraction(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
public extension AntiDualNum {
    public AntiFlector bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float4(this.e3215) * float4(right_dual.group3.xyz, right_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45) * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiMotor bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(this.group0.xy.xx, this.e3215, (this.e3215 * right_dual.e4) + (this.scalar * right_dual.e12345)) * float4(right_dual.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e12345)
        );
    }
}
public extension AntiFlatPoint {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<AntiFlatPoint> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<AntiFlatPoint>(this);
    }
}
public extension BulkContractionInfixPartial<AntiFlatPoint> {
    // Fancy infix trick (second half)
    public AntiFlector operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension AntiFlatPoint {
    public AntiFlector bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_dual.e412) + (this.e321 * right_dual.e415), (this.e125 * right_dual.e423) + (this.e321 * right_dual.e425), (this.e235 * right_dual.e431) + (this.e321 * right_dual.e435), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((right_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_dual.e415)
        );
    }
}
public extension AntiFlatPoint {
    public AntiMotor bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float4(this.e235 * right_dual.e1234, this.e315 * right_dual.e1234, this.e125 * right_dual.e1234, -(this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) - float4((this.group0.www * right_dual.group3.xyz), this.e235 * right_dual.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx)), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint bulk_contraction(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public Scalar bulk_contraction(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiMotor bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(right_dual.group1.xyz, right_dual.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group1.zxy) - (this.group0.zxy * right_dual.group1.yzx)), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiPlane bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * right_dual.group0.xyz), -(this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435))
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlector bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.group0.www * right_dual.group0.xyz), -(this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435))
        );
    }
}
public extension AntiFlatPoint {
    public AntiLine bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e321) * right_dual.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx)
        );
    }
}
public extension AntiFlatPoint {
    public Scalar bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)
        );
    }
}
public extension AntiFlatPoint {
    public Scalar bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e45 * this.e321)
        );
    }
}
public extension AntiFlatPoint {
    public AntiPlane bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_dual.e412) + (this.e321 * right_dual.e415), (this.e125 * right_dual.e423) + (this.e321 * right_dual.e425), (this.e235 * right_dual.e431) + (this.e321 * right_dual.e435), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((right_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_dual.e415)
        );
    }
}
public extension AntiFlatPoint {
    public AntiPlane bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321), -(right_dual.e425 * this.e315) - (right_dual.e435 * this.e125)) - (this.group0.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e415))
        );
    }
}
public extension AntiFlatPoint {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2(-(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_dual.group6.xyz), 0.0) + float4((right_dual.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((right_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * right_dual.group9.zxy) - (this.group0.zxy * right_dual.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_dual.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiLine bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiLine(
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_dual.group0.xyz), 
            /* e15, e25, e35 */
            (this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint bulk_contraction(Scalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiFlatPoint {
    public AntiMotor bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float4(this.e235 * right_dual.e1234, this.e315 * right_dual.e1234, this.e125 * right_dual.e1234, -(this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx)), 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlector bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_dual.e412) + (this.e321 * right_dual.e415), (this.e125 * right_dual.e423) + (this.e321 * right_dual.e425), (this.e235 * right_dual.e431) + (this.e321 * right_dual.e435), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - (this.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e415))
        );
    }
}
public extension AntiFlector {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<AntiFlector> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<AntiFlector>(this);
    }
}
public extension BulkContractionInfixPartial<AntiFlector> {
    // Fancy infix trick (second half)
    public AntiFlector operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension AntiFlector {
    public AntiFlector bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_dual.e415) + (this.e1 * right_dual.e12345), (this.e321 * right_dual.e425) + (this.e2 * right_dual.e12345), (this.e321 * right_dual.e435) + (this.e3 * right_dual.e12345), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + float4((right_dual.group0.xyz.zxy * this.group0.yzx), this.e5 * right_dual.e12345) - float4((right_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_dual.e415)
        );
    }
}
public extension AntiFlector {
    public AntiMotor bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) + float4((right_dual.group2.www * this.group0.xyz), this.e1 * right_dual.e4235) - float4((this.group0.www * right_dual.group3.xyz), this.e235 * right_dual.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
public extension AntiFlector {
    public Scalar bulk_contraction(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0
        );
    }
}
public extension AntiFlector {
    public AntiMotor bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * right_dual.group1.xyz * float3(-1.0)), (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) - (this.e321 * right_dual.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group1.zxy) - (this.group0.zxy * right_dual.group1.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiPlane bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * right_dual.group0.xyz), -(this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435))
        );
    }
}
public extension AntiFlector {
    public AntiFlector bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e1 * right_dual.e12345, this.e2 * right_dual.e12345, this.e3 * right_dual.e12345, -(this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + (right_dual.group0 * float4(this.group0.www, this.e5))
        );
    }
}
public extension AntiFlector {
    public AntiMotor bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * right_dual.group0.xyz * float3(-1.0)), (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public Scalar bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)
        );
    }
}
public extension AntiFlector {
    public Scalar bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e45 * this.e321)
        );
    }
}
public extension AntiFlector {
    public AntiPlane bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_dual.e412) + (this.e321 * right_dual.e415), (this.e125 * right_dual.e423) + (this.e321 * right_dual.e425), (this.e235 * right_dual.e431) + (this.e321 * right_dual.e435), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((right_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_dual.e415)
        );
    }
}
public extension AntiFlector {
    public AntiPlane bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321), -(right_dual.e425 * this.e315) - (right_dual.e435 * this.e125)) - (this.group0.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e415))
        );
    }
}
public extension AntiFlector {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_dual.group6.xyz), 0.0) + float4((float3(right_dual.e12345) * this.group1.xyz), 0.0) + float4((right_dual.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((right_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (this.e5 * right_dual.e12345) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * right_dual.group9.zxy) - (this.group0.zxy * right_dual.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_dual.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiMotor bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e321 * right_dual.e4235 * -1.0, this.e321 * right_dual.e4315 * -1.0, this.e321 * right_dual.e4125 * -1.0, (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)) + float4((float3(right_dual.e1234) * this.group0.xyz), this.e1 * right_dual.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
public extension AntiFlector {
    public AntiMotor bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) + float4((right_dual.group2.www * this.group0.xyz), this.e1 * right_dual.e4235) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx)), 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_dual.e415) + (this.e1 * right_dual.e12345), (this.e321 * right_dual.e425) + (this.e2 * right_dual.e12345), (this.e321 * right_dual.e435) + (this.e3 * right_dual.e12345), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + (right_dual.group0.zxyw * float4(this.group0.yzx, this.e5)) - (this.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e415))
        );
    }
}
public extension AntiLine {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<AntiLine> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<AntiLine>(this);
    }
}
public extension BulkContractionInfixPartial<AntiLine> {
    // Fancy infix trick (second half)
    public AntiMotor operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension AntiLine {
    public AntiMotor bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e12345) * this.group0.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public AntiPlane bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public AntiLine bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_dual.e12345) * this.group1.xyz
        );
    }
}
public extension AntiLine {
    public AntiPlane bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_dual.e4315 * -1.0, this.e23 * right_dual.e4125 * -1.0, this.e31 * right_dual.e4235 * -1.0, (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group1.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public Scalar bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)
        );
    }
}
public extension AntiLine {
    public AntiMotor bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e12345) * this.group0.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiLine {
    public AntiPlane bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_dual.e4315 * -1.0, this.e23 * right_dual.e4125 * -1.0, this.e31 * right_dual.e4235 * -1.0, (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public Scalar bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)
        );
    }
}
public extension AntiLine {
    public Scalar bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12)
        );
    }
}
public extension AntiLine {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2(-(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.xyz.yzx * right_dual.group9.zxy), 0.0) - float4((float3(right_dual.e1234) * this.group1.xyz), 0.0) - float4((this.group0.xyz.zxy * right_dual.group9.yzx), 0.0), 
            /* e5 */
            (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.yy, right_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_dual.e12345) * this.group0.xyz, 
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
    public AntiPlane bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public AntiLine bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_dual.e12345) * this.group1.xyz
        );
    }
}
public extension AntiLine {
    public AntiPlane bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
public extension AntiLine {
    public AntiMotor bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4((float3(right_dual.e12345) * this.group0.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<AntiMotor> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<AntiMotor>(this);
    }
}
public extension BulkContractionInfixPartial<AntiMotor> {
    // Fancy infix trick (second half)
    public AntiMotor operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23 * right_dual.e12345, this.e31 * right_dual.e12345, this.e12 * right_dual.e12345, -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) + float4((right_dual.group0.xyz * this.group1.www), this.scalar * right_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_dual.group1.xyz) + (float3(right_dual.e12345) * this.group1.xyz)), this.e3215 * right_dual.e12345)
        );
    }
}
public extension AntiMotor {
    public AntiFlector bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float4(this.e3215) * float4(right_dual.group3.xyz, right_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((right_dual.group0.xyz * this.group1.www), this.e3215 * right_dual.e45)
        );
    }
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
public extension AntiMotor {
    public DualNum bulk_contraction(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e3215, 1.0) * float2(-1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiFlector bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            (right_dual.group1.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) - float4((this.group0.zxy * right_dual.group1.yzx), this.e3215 * right_dual.e45)
        );
    }
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e12345) * this.group0.xyz), (this.scalar * right_dual.e12345) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_dual.group0.xyz) + (float3(right_dual.e12345) * this.group1.xyz)), this.e3215 * right_dual.e12345)
        );
    }
}
public extension AntiMotor {
    public AntiFlector bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_dual.e4315 * -1.0, this.e23 * right_dual.e4125 * -1.0, this.e31 * right_dual.e4235 * -1.0, (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
public extension AntiMotor {
    public AntiPlane bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual.group0.xyz * this.group1.www * float3(-1.0)), this.e3215 * right_dual.e45 * -1.0)
        );
    }
}
public extension AntiMotor {
    public AntiPlane bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45) * float4(-1.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * right_dual.group0.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float4((float3(this.e3215) * right_dual.group0.xyz), (right_dual.e4 * this.e3215) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension AntiMotor {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((this.scalar * right_dual.e12345) + (this.e3215 * right_dual.e4) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.yzx * right_dual.group9.zxy), 0.0) - float4((float3(this.e3215) * right_dual.group4.xyz), 0.0) - float4((float3(right_dual.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * right_dual.group9.yzx), 0.0), 
            /* e5 */
            (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) - (this.e3215 * right_dual.e45), 
            /* e15, e25, e35, e45 */
            float4(((float3(this.e3215) * right_dual.group6.xyz) + (float3(right_dual.e12345) * this.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_dual.group7.xyz) + (float3(right_dual.e12345) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * right_dual.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e3215) * right_dual.group9.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * right_dual.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public AntiFlector bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
public extension AntiMotor {
    public Scalar bulk_contraction(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
public extension AntiMotor {
    public AntiFlector bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float4(this.e3215) * float4(right_dual.group3.xyz, right_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e15 * right_dual.e1234) - (this.e3215 * right_dual.e41), -(this.e25 * right_dual.e1234) - (this.e3215 * right_dual.e42), -(this.e35 * right_dual.e1234) - (this.e3215 * right_dual.e43), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((this.group0.zxy * right_dual.group3.yzx), this.e3215 * right_dual.e45)
        );
    }
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e4)) + (float4(right_dual.e12345) * this.group0) + float4(float3(0.0), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_dual.group1.xyz) + (float3(right_dual.e12345) * this.group1.xyz)), this.e3215 * right_dual.e12345)
        );
    }
}
public extension AntiPlane {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<AntiPlane> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<AntiPlane>(this);
    }
}
public extension BulkContractionInfixPartial<AntiPlane> {
    // Fancy infix trick (second half)
    public AntiPlane operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension AntiPlane {
    public AntiPlane bulk_contraction(AntiCircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public Scalar bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)
        );
    }
}
public extension AntiPlane {
    public AntiPlane bulk_contraction(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public Scalar bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125)
        );
    }
}
public extension AntiPlane {
    public AntiPlane bulk_contraction(AntiMotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public Scalar bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125)
        );
    }
}
public extension AntiPlane {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.group0.xy.yy, right_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            this.e5 * right_dual.e12345, 
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
    public Scalar bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Scalar(
            /* scalar */
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)
        );
    }
}
public extension AntiPlane {
    public AntiPlane bulk_contraction(Scalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiPlane {
    public Scalar bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)
        );
    }
}
public extension AntiPlane {
    public AntiPlane bulk_contraction(VersorOdd other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension AntiScalar {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<AntiScalar> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<AntiScalar>(this);
    }
}
public extension BulkContractionInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public CircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension AntiScalar {
    public CircleRotor bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.e12345) * right_dual.group2
        );
    }
}
public extension AntiScalar {
    public DipoleInversion bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * right_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension AntiScalar {
    public DualNum bulk_contraction(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345) * other.group0.xy
        );
    }
}
public extension AntiScalar {
    public FlatPoint bulk_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
public extension AntiScalar {
    public Flector bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
public extension AntiScalar {
    public Line bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual.group1.xyz
        );
    }
}
public extension AntiScalar {
    public Motor bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * right_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
public extension AntiScalar {
    public Plane bulk_contraction(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
public extension AntiScalar {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public Dipole bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35 */
            float3(this.e12345) * right_dual.group2.xyz
        );
    }
}
public extension AntiScalar {
    public AntiCircleRotor bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * right_dual.group2
        );
    }
}
public extension AntiScalar {
    public Circle bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual.group2.xyz
        );
    }
}
public extension AntiScalar {
    public AntiDipoleInversion bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * right_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension AntiScalar {
    public AntiDualNum bulk_contraction(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
public extension AntiScalar {
    public AntiFlatPoint bulk_contraction(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
public extension AntiScalar {
    public AntiFlector bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
public extension AntiScalar {
    public AntiLine bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * right_dual.group1.xyz
        );
    }
}
public extension AntiScalar {
    public AntiMotor bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * right_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
public extension AntiScalar {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2(this.e12345) * right_dual.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group1, 
            /* e5 */
            this.e12345 * right_dual.e5, 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_dual.group3, 
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * right_dual.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group6, 
            /* e423, e431, e412 */
            float3(this.e12345) * right_dual.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group9, 
            /* e1234 */
            this.e12345 * right_dual.e1234
        );
    }
}
public extension AntiScalar {
    public AntiPlane bulk_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension AntiScalar {
    public Sphere bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0, 
            /* e1234 */
            this.e12345 * right_dual.e1234
        );
    }
}
public extension AntiScalar {
    public AntiScalar bulk_contraction(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar
        );
    }
}
public extension AntiScalar {
    public RoundPoint bulk_contraction(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5 */
            this.e12345 * right_dual.e5
        );
    }
}
public extension AntiScalar {
    public VersorOdd bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float4(this.e12345) * right_dual.group0, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * right_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension AntiScalar {
    public VersorEven bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(this.e12345) * right_dual.group0, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * right_dual.group2, 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension Circle {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<Circle> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<Circle>(this);
    }
}
public extension BulkContractionInfixPartial<Circle> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension Circle {
    public AntiDipoleInversion bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e125 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), this.e235 * right_dual.e415) - float4((this.group2.xyz.zxy * right_dual.group0.xyz.yzx), this.e315 * right_dual.e425)
        );
    }
}
public extension Circle {
    public AntiCircleRotor bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            (float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group3.yzx) - (this.group0.xyz.yzx * right_dual.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((this.group2.xyz.zxy * right_dual.group3.yzx), this.e423 * right_dual.e15)
        );
    }
}
public extension Circle {
    public Circle bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(right_dual.e12345) * this.group2.xyz
        );
    }
}
public extension Circle {
    public Scalar bulk_contraction(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)
        );
    }
}
public extension Circle {
    public AntiCircleRotor bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * right_dual.group1.yzx) - (this.group0.xyz.yzx * right_dual.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_dual.e3215, this.e431 * right_dual.e3215, this.e412 * right_dual.e3215, -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)) - float4((this.group2.xyz.zxy * right_dual.group1.yzx), this.e423 * right_dual.e15)
        );
    }
}
public extension Circle {
    public RoundPoint bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e315) + (this.e321 * right_dual.e415), (this.e423 * right_dual.e125) + (this.e321 * right_dual.e425), (this.e431 * right_dual.e235) + (this.e321 * right_dual.e435), -(this.e431 * right_dual.e425) - (this.e412 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group1.xyz.zxy), this.e423 * right_dual.e415), 
            /* e5 */
            -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)
        );
    }
}
public extension Circle {
    public AntiDipoleInversion bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e321 * right_dual.e415), (this.e423 * right_dual.e125) + (this.e321 * right_dual.e425), (this.e431 * right_dual.e235) + (this.e321 * right_dual.e435), -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group1.zxy), this.e235 * right_dual.e415)
        );
    }
}
public extension Circle {
    public Dipole bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_dual.e3215, this.e431 * right_dual.e3215, this.e412 * right_dual.e3215, -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(right_dual.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * right_dual.group0.zxy) - (this.group2.xyz.zxy * right_dual.group0.yzx)
        );
    }
}
public extension Circle {
    public Scalar bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)
        );
    }
}
public extension Circle {
    public Scalar bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)
        );
    }
}
public extension Circle {
    public RoundPoint bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) - float4((right_dual.group0.xyz.yzx * this.group2.xyz.zxy), right_dual.e423 * this.e415) - float4((right_dual.group2.xyz.zxy * this.group0.xyz.yzx), right_dual.e431 * this.e425), 
            /* e5 */
            -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)
        );
    }
}
public extension Circle {
    public RoundPoint bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) - float4((right_dual.group0.xyz.yzx * this.group2.xyz.zxy), right_dual.e423 * this.e415) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), right_dual.e431 * this.e425), 
            /* e5 */
            -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)
        );
    }
}
public extension Circle {
    public AntiLine bulk_contraction(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual.e3215) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_dual.e3215) * this.group1.xyz
        );
    }
}
public extension Circle {
    public AntiPlane bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual.e315 * this.e412) + (right_dual.e321 * this.e415), (right_dual.e125 * this.e423) + (right_dual.e321 * this.e425), (right_dual.e235 * this.e431) + (right_dual.e321 * this.e435), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_dual.group0.zxy), right_dual.e235 * this.e415)
        );
    }
}
public extension Circle {
    public AntiPlane bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual.e315 * this.e412) + (right_dual.e321 * this.e415), (right_dual.e125 * this.e423) + (right_dual.e321 * this.e425), (right_dual.e235 * this.e431) + (right_dual.e321 * this.e435), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_dual.group0.zxy), right_dual.e235 * this.e415)
        );
    }
}
public extension Circle {
    public Scalar bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Scalar(
            /* scalar */
            -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)
        );
    }
}
public extension Circle {
    public AntiMotor bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e3215) * this.group0.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Circle {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) - float4((this.group0.xyz.yzx * right_dual.group8.xyz.zxy), this.e423 * right_dual.e415) - float4((this.group2.xyz.zxy * right_dual.group7.xyz.yzx), this.e431 * right_dual.e425), 
            /* e5 */
            -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group9.yzxx * float4(this.group2.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group9.yzx) - (this.group0.xyz.yzx * right_dual.group9.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e3215) * this.group0.xyz) + (float3(right_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.e12345) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Circle {
    public Dipole bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(right_dual.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * right_dual.group0.zxy) - (this.group2.xyz.zxy * right_dual.group0.yzx)
        );
    }
}
public extension Circle {
    public Circle bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(right_dual.e12345) * this.group2.xyz
        );
    }
}
public extension Circle {
    public AntiCircleRotor bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            (float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group3.yzx) - (this.group0.xyz.yzx * right_dual.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((this.group2.xyz.zxy * right_dual.group3.yzx), this.e423 * right_dual.e15)
        );
    }
}
public extension Circle {
    public AntiDipoleInversion bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e125 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), this.e235 * right_dual.e415) - float4((this.group2.xyz.zxy * right_dual.group0.yzx), this.e315 * right_dual.e425)
        );
    }
}
public extension CircleRotor {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<CircleRotor> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<CircleRotor>(this);
    }
}
public extension BulkContractionInfixPartial<CircleRotor> {
    // Fancy infix trick (second half)
    public VersorEven operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension CircleRotor {
    public VersorEven bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_dual.group0.xyz)), right_dual.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(right_dual.e12345) * this.group1) + (float4(this.e12345) * right_dual.group1), 
            /* e235, e315, e125, e5 */
            float4((right_dual.e235 * this.e12345) + (right_dual.e12345 * this.e235), (right_dual.e315 * this.e12345) + (right_dual.e12345 * this.e315), (right_dual.e125 * this.e12345) + (right_dual.e12345 * this.e125), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.e423 * this.e415) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), right_dual.e431 * this.e425)
        );
    }
}
public extension CircleRotor {
    public VersorOdd bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float4((this.e412 * right_dual.e4315) + (this.e415 * right_dual.e1234) + (this.e12345 * right_dual.e41), (this.e423 * right_dual.e4125) + (this.e425 * right_dual.e1234) + (this.e12345 * right_dual.e42), (this.e431 * right_dual.e4235) + (this.e435 * right_dual.e1234) + (this.e12345 * right_dual.e43), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((this.group0.xyz.yzx * right_dual.group3.zxy), this.e423 * right_dual.e15), 
            /* e23, e31, e12, e45 */
            float4((this.e235 * right_dual.e1234) + (this.e12345 * right_dual.e23), (this.e315 * right_dual.e1234) + (this.e12345 * right_dual.e31), (this.e125 * right_dual.e1234) + (this.e12345 * right_dual.e12), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) + float4((this.group0.xyz * right_dual.group3.www), this.e12345 * right_dual.e45) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * right_dual.group2.xyz) + (float3(right_dual.e3215) * this.group1.xyz) + (this.group2.yzx * right_dual.group3.zxy) - (this.group2.zxy * right_dual.group3.yzx)), this.e12345 * right_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension CircleRotor {
    public VersorEven bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.e12345) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 * float4(right_dual.group0.xy.yy, right_dual.e12345, right_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension CircleRotor {
    public AntiCircleRotor bulk_contraction(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e12345 * right_dual.e45), 
            /* e15, e25, e35, scalar */
            float4((this.group2.www * right_dual.group0.xyz), -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45))
        );
    }
}
public extension CircleRotor {
    public VersorOdd bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e412 * right_dual.e4315, this.e423 * right_dual.e4125, this.e431 * right_dual.e4235, -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)) - float4((this.group0.xyz.yzx * right_dual.group1.zxy), this.e423 * right_dual.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) + float4((this.group0.xyz * right_dual.group1.www), this.e12345 * right_dual.e45) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e12345) * right_dual.group0.xyz), 0.0) + float4((float3(right_dual.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * right_dual.group1.zxy), 0.0) - float4((this.group2.zxy * right_dual.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group1.xyz), -(this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e321 * right_dual.e415), (this.e423 * right_dual.e125) + (this.e321 * right_dual.e425), (this.e431 * right_dual.e235) + (this.e321 * right_dual.e435), -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group1.xyz.zxy), this.e235 * right_dual.e415)
        );
    }
}
public extension CircleRotor {
    public VersorEven bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.e12345) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * right_dual.group0.xyz) + (float3(right_dual.e12345) * this.group1.xyz)), this.e321 * right_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e12345 * right_dual.e235, this.e12345 * right_dual.e315, this.e12345 * right_dual.e125, -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + (this.group2 * float4(right_dual.group0.www, right_dual.e5)), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e315) + (this.e321 * right_dual.e415), (this.e423 * right_dual.e125) + (this.e321 * right_dual.e425), (this.e431 * right_dual.e235) + (this.e321 * right_dual.e435), -(this.e431 * right_dual.e425) - (this.e412 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group1.zxy), this.e423 * right_dual.e415)
        );
    }
}
public extension CircleRotor {
    public DipoleInversion bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_dual.e3215, this.e431 * right_dual.e3215, this.e412 * right_dual.e3215, -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * right_dual.group0.zxy), 0.0) - float4((this.group2.zxy * right_dual.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0
        );
    }
}
public extension CircleRotor {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension CircleRotor {
    public AntiCircleRotor bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43))
        );
    }
}
public extension CircleRotor {
    public AntiCircleRotor bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e125 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.e235 * this.e415) - float4((right_dual.group2.xyz.zxy * this.group0.xyz.yzx), right_dual.e315 * this.e425)
        );
    }
}
public extension CircleRotor {
    public AntiDipoleInversion bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.e4 * this.e12345) - (right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412) + (right_dual.e1 * this.e12345), (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423) + (right_dual.e2 * this.e12345), (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431) + (right_dual.e3 * this.e12345), -(right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + (this.group2.yzxw * float4(right_dual.group0.xyz.zxy, right_dual.e5)) - (this.group2.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e415)) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), right_dual.e425 * this.e315)
        );
    }
}
public extension CircleRotor {
    public AntiMotor bulk_contraction(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.group0.xy.xx, right_dual.e3215, right_dual.scalar) * float4(this.group0.xyz, this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
public extension CircleRotor {
    public AntiFlector bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual.e315 * this.e412) + (right_dual.e321 * this.e415), (right_dual.e125 * this.e423) + (right_dual.e321 * this.e425), (right_dual.e235 * this.e431) + (right_dual.e321 * this.e435), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_dual.group0.zxy), right_dual.e235 * this.e415)
        );
    }
}
public extension CircleRotor {
    public AntiFlector bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual.e321 * this.e415) + (right_dual.e1 * this.e12345), (right_dual.e321 * this.e425) + (right_dual.e2 * this.e12345), (right_dual.e321 * this.e435) + (right_dual.e3 * this.e12345), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + float4((this.group0.xyz.zxy * right_dual.group0.yzx), right_dual.e5 * this.e12345) - float4((this.group0.xyz.yzx * right_dual.group0.zxy), right_dual.e235 * this.e415)
        );
    }
}
public extension CircleRotor {
    public AntiMotor bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_dual.group0.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension CircleRotor {
    public AntiMotor bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.e23 * this.e12345, right_dual.e31 * this.e12345, right_dual.e12 * this.e12345, -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) + float4((this.group0.xyz * right_dual.group1.www), right_dual.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual.e3215) * this.group1.xyz) + (float3(this.e12345) * right_dual.group1.xyz)), right_dual.e3215 * this.e12345)
        );
    }
}
public extension CircleRotor {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((this.e12345 * right_dual.scalar) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), this.e12345 * right_dual.e12345), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412) + (this.e12345 * right_dual.e1), (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423) + (this.e12345 * right_dual.e2), (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431) + (this.e12345 * right_dual.e3), -(this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) + float4((this.group0.xyz.zxy * right_dual.group8.xyz.yzx), this.e12345 * right_dual.e4) - float4((this.group0.xyz.yzx * right_dual.group8.xyz.zxy), this.e423 * right_dual.e415) - float4((right_dual.group7.xyz.yzx * this.group2.zxy), this.e431 * right_dual.e425), 
            /* e5 */
            (this.e12345 * right_dual.e5) - (this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e315 * right_dual.e4125) + (this.e12345 * right_dual.e15), (this.e125 * right_dual.e4235) + (this.e12345 * right_dual.e25), (this.e235 * right_dual.e4315) + (this.e12345 * right_dual.e35), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) + float4((right_dual.group9.www * this.group1.xyz), this.e12345 * right_dual.e45) - (right_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * right_dual.group4.xyz) + (float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group9.yzx) - (this.group0.xyz.yzx * right_dual.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e12345) * right_dual.group5.xyz) + (float3(right_dual.e3215) * this.group0.xyz) + (float3(right_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * right_dual.group6) + (float4(right_dual.e12345) * this.group1), 
            /* e423, e431, e412 */
            (float3(this.e12345) * right_dual.group7.xyz) + (float3(right_dual.e12345) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(this.e12345) * right_dual.group8.xyz) + (float3(right_dual.e12345) * this.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group9, 
            /* e1234 */
            this.e12345 * right_dual.e1234
        );
    }
}
public extension CircleRotor {
    public AntiPlane bulk_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension CircleRotor {
    public DipoleInversion bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e3215) * this.group1.xyz) + (this.group2.yzx * right_dual.group0.zxy) - (this.group2.zxy * right_dual.group0.yzx)), this.e12345 * right_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0
        );
    }
}
public extension CircleRotor {
    public CircleRotor bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_dual.e12345) * this.group2
        );
    }
}
public extension CircleRotor {
    public RoundPoint bulk_contraction(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5 */
            this.e12345 * right_dual.e5
        );
    }
}
public extension CircleRotor {
    public VersorOdd bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float4((this.e415 * right_dual.e1234) + (this.e12345 * right_dual.e41), (this.e425 * right_dual.e1234) + (this.e12345 * right_dual.e42), (this.e435 * right_dual.e1234) + (this.e12345 * right_dual.e43), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) + float4((this.group0.xyz.zxy * right_dual.group3.yzx), this.e12345 * right_dual.scalar) - float4((this.group0.xyz.yzx * right_dual.group3.zxy), this.e423 * right_dual.e15), 
            /* e23, e31, e12, e45 */
            float4((this.e235 * right_dual.e1234) + (this.e12345 * right_dual.e23), (this.e315 * right_dual.e1234) + (this.e12345 * right_dual.e31), (this.e125 * right_dual.e1234) + (this.e12345 * right_dual.e12), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) + float4((this.group0.xyz * right_dual.group3.www), this.e12345 * right_dual.e45) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * right_dual.group2.xyz) + (float3(right_dual.e3215) * this.group1.xyz) + (this.group2.yzx * right_dual.group3.zxy) - (this.group2.zxy * right_dual.group3.yzx)), this.e12345 * right_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension CircleRotor {
    public VersorEven bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(((float3(this.e12345) * right_dual.group0.xyz) + (float3(right_dual.e12345) * this.group0.xyz)), this.e12345 * right_dual.e12345), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * right_dual.group1) + (float4(right_dual.e12345) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(this.e12345 * right_dual.e235, this.e12345 * right_dual.e315, this.e12345 * right_dual.e125, -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + (this.group2 * float4(right_dual.group0.www, right_dual.e5)), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412) + (this.e12345 * right_dual.e1), (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423) + (this.e12345 * right_dual.e2), (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431) + (this.e12345 * right_dual.e3), -(this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) + float4((this.group0.xyz.zxy * right_dual.group2.yzx), this.e12345 * right_dual.e4) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), this.e423 * right_dual.e415) - float4((this.group2.zxy * right_dual.group0.yzx), this.e431 * right_dual.e425)
        );
    }
}
public extension Dipole {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<Dipole> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<Dipole>(this);
    }
}
public extension BulkContractionInfixPartial<Dipole> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension Dipole {
    public AntiCircleRotor bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43))
        );
    }
}
public extension Dipole {
    public RoundPoint bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_dual.group2.www), this.e41 * right_dual.e4235), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)
        );
    }
}
public extension Dipole {
    public Dipole bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(right_dual.e12345) * this.group2.xyz
        );
    }
}
public extension Dipole {
    public RoundPoint bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)
        );
    }
}
public extension Dipole {
    public Scalar bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)
        );
    }
}
public extension Dipole {
    public AntiCircleRotor bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435))
        );
    }
}
public extension Dipole {
    public RoundPoint bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)
        );
    }
}
public extension Dipole {
    public Scalar bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)
        );
    }
}
public extension Dipole {
    public Scalar bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)
        );
    }
}
public extension Dipole {
    public AntiPlane bulk_contraction(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension Dipole {
    public Scalar bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            -(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)
        );
    }
}
public extension Dipole {
    public Scalar bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            -(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)
        );
    }
}
public extension Dipole {
    public AntiPlane bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.group0.xyz * right_dual.group1.www), right_dual.e3215 * this.e45)
        );
    }
}
public extension Dipole {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2(-(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group9.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_dual.e1234) * this.group2.xyz), this.e41 * right_dual.e4235), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e12345) * this.group1.xyz, 
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
    public RoundPoint bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group0.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_dual.e1234) * this.group2.xyz), this.e41 * right_dual.e4235), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)
        );
    }
}
public extension Dipole {
    public Dipole bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(right_dual.e12345) * this.group2.xyz
        );
    }
}
public extension Dipole {
    public RoundPoint bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_dual.group2.www), this.e41 * right_dual.e4235), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)
        );
    }
}
public extension Dipole {
    public AntiCircleRotor bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
public extension DipoleInversion {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<DipoleInversion> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<DipoleInversion>(this);
    }
}
public extension BulkContractionInfixPartial<DipoleInversion> {
    // Fancy infix trick (second half)
    public VersorOdd operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension DipoleInversion {
    public VersorOdd bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual.e412 * this.e4315) + (right_dual.e415 * this.e1234) + (right_dual.e12345 * this.e41), (right_dual.e423 * this.e4125) + (right_dual.e425 * this.e1234) + (right_dual.e12345 * this.e42), (right_dual.e431 * this.e4235) + (right_dual.e435 * this.e1234) + (right_dual.e12345 * this.e43), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((right_dual.group0.xyz.yzx * this.group3.zxy), right_dual.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e235 * this.e1234) + (right_dual.e12345 * this.e23), (right_dual.e315 * this.e1234) + (right_dual.e12345 * this.e31), (right_dual.e125 * this.e1234) + (right_dual.e12345 * this.e12), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) + float4((right_dual.group0.xyz * this.group3.www), right_dual.e12345 * this.e45) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e12345) * this.group2.xyz) + (float3(this.e3215) * right_dual.group1.xyz) + (right_dual.group2.yzx * this.group3.zxy) - (right_dual.group2.zxy * this.group3.yzx)), right_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            (float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e41)) + float4(float3(0.0), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234) - (right_dual.e1234 * this.e45) - (right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) - float4((this.group3.www * right_dual.group3.xyz), right_dual.e4235 * this.e41), 
            /* e1, e2, e3, e5 */
            (right_dual.group3.zxyw * this.group1.yzxw) + (this.group2.wwwz * float4(right_dual.group2.xyz, right_dual.e4125)) + float4((this.group0.xyz * right_dual.group3.www), right_dual.e4235 * this.e15) + float4((right_dual.group1.zxy * this.group3.yzx), right_dual.e4315 * this.e25) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - float4((right_dual.group3.yzx * this.group1.zxy), right_dual.e35 * this.e4125)
        );
    }
}
public extension DipoleInversion {
    public VersorOdd bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.group0.xy.yy, right_dual.e12345, right_dual.e5) * float4(this.group0.xyz, this.e1234), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public RoundPoint bulk_contraction(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e5 */
            -(this.e4235 * right_dual.e15) - (this.e4315 * right_dual.e25) - (this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45)
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * right_dual.group1.yzx) - (this.group3.yzx * right_dual.group1.zxy)), this.e1234 * right_dual.e3215), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) + float4((right_dual.group1.www * this.group3.xyz), this.e1234 * right_dual.e45) - (right_dual.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(right_dual.group0.xyz, right_dual.e4315)) + (right_dual.group1.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (this.e35 * right_dual.e4125) - (this.e4315 * right_dual.e25) - (this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45)) - float4((this.group1.zxy * right_dual.group1.yzx), this.e4235 * right_dual.e15)
        );
    }
}
public extension DipoleInversion {
    public AntiCircleRotor bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_dual.group1.xyz), -(this.e4235 * right_dual.e415) - (this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)), 
            /* e15, e25, e35, scalar */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)) - float4((right_dual.group1.xyz.zxy * this.group3.yzx), this.e41 * right_dual.e235)
        );
    }
}
public extension DipoleInversion {
    public VersorOdd bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e1234 * right_dual.e415, this.e1234 * right_dual.e425, this.e1234 * right_dual.e435, -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)) + float4((this.group0.xyz * right_dual.group0.www), this.e1234 * right_dual.e5), 
            /* e23, e31, e12, e45 */
            float4(this.e1234 * right_dual.e235, this.e1234 * right_dual.e315, this.e1234 * right_dual.e125, -(this.e4235 * right_dual.e415) - (this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) + (float4(right_dual.e12345) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_dual.group0.xyz) + (float3(right_dual.e12345) * this.group2.xyz) + (this.group3.zxy * right_dual.group1.yzx) - (this.group3.yzx * right_dual.group1.zxy)), this.e1234 * right_dual.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * right_dual.group0.yzx) - (this.group3.yzx * right_dual.group0.zxy)), this.e1234 * right_dual.e3215), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * right_dual.e3215, this.e4315 * right_dual.e3215, this.e4125 * right_dual.e3215, -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_dual.e4315 * -1.0, this.e23 * right_dual.e4125 * -1.0, this.e31 * right_dual.e4235 * -1.0, (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_dual.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
public extension DipoleInversion {
    public RoundPoint bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group3.yzxy * float4(right_dual.group1.zxy, right_dual.e42)) + float4((right_dual.group2.xyz * this.group2.www), right_dual.e41 * this.e4235), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)
        );
    }
}
public extension DipoleInversion {
    public RoundPoint bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e41)) + float4((this.group2.www * right_dual.group2.xyz), right_dual.e42 * this.e4315), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)
        );
    }
}
public extension DipoleInversion {
    public AntiCircleRotor bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group3.yzx) - (right_dual.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((right_dual.group2.xyz.zxy * this.group3.yzx), right_dual.e423 * this.e15)
        );
    }
}
public extension DipoleInversion {
    public AntiCircleRotor bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            (float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group3.yzx) - (right_dual.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (this.group3.zxyx * float4(right_dual.group2.yzx, right_dual.e1)) + float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((right_dual.group2.zxy * this.group3.yzx), right_dual.e423 * this.e15)
        );
    }
}
public extension DipoleInversion {
    public AntiFlector bulk_contraction(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension DipoleInversion {
    public AntiMotor bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.e235 * this.e1234, right_dual.e315 * this.e1234, right_dual.e125 * this.e1234, -(right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) - float4((right_dual.group0.www * this.group3.xyz), right_dual.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension DipoleInversion {
    public AntiMotor bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) + float4((this.group2.www * right_dual.group0.xyz), right_dual.e1 * this.e4235) - float4((right_dual.group0.www * this.group3.xyz), right_dual.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension DipoleInversion {
    public AntiPlane bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group3.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
public extension DipoleInversion {
    public AntiFlector bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) + float4((this.group0.xyz * right_dual.group1.www), right_dual.e3215 * this.e45) - (this.group3.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
public extension DipoleInversion {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((this.e1234 * right_dual.e5) + (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * right_dual.group3) + (this.group3.yzxy * float4(right_dual.group5.xyz.zxy, right_dual.e42)) + float4((this.group0.xyz * right_dual.group9.www), this.e4235 * right_dual.e41) + float4((this.group1.yzx * right_dual.group9.zxy), this.e4125 * right_dual.e43) - (float4(right_dual.e1234) * float4(this.group2.xyz, this.e45)) - (right_dual.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((right_dual.group4.xyz * this.group3.www), this.e41 * right_dual.e4235) - float4((right_dual.group5.xyz.yzx * this.group3.zxy), this.e42 * right_dual.e4315), 
            /* e5 */
            (this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) - (this.e4235 * right_dual.e15) - (this.e4315 * right_dual.e25) - (this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) + (float4(right_dual.e12345) * float4(this.group2.xyz, this.e45)) - float4((right_dual.group8.xyz.zxy * this.group3.yzx), this.e4235 * right_dual.e415), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual.group6.xyz) + (float3(right_dual.e12345) * this.group0.xyz) + (right_dual.group7.xyz.zxy * this.group3.yzx) - (right_dual.group7.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_dual.group8.xyz) + (float3(this.e3215) * right_dual.group7.xyz) + (float3(right_dual.e12345) * this.group1.xyz) - (float3(right_dual.e321) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group9.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group9.zxy, right_dual.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual.group9.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group3.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3, 
            /* e1234 */
            this.e1234 * right_dual.e12345
        );
    }
}
public extension DipoleInversion {
    public Scalar bulk_contraction(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group0.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group0.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * right_dual.e3215, this.e4315 * right_dual.e3215, this.e4125 * right_dual.e3215, -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_dual.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension DipoleInversion {
    public Scalar bulk_contraction(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Scalar(
            /* scalar */
            (this.e1234 * right_dual.e5) + (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4)
        );
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            (float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (this.e4235 * right_dual.e41) + (this.e4315 * right_dual.e42) + (this.e4125 * right_dual.e43) - (this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) + float4((right_dual.group3.www * this.group3.xyz), this.e1234 * right_dual.e45) - (right_dual.group3.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(right_dual.group2.xyz, right_dual.e4315)) + (right_dual.group3.zxyx * float4(this.group1.yzx, this.e15)) + float4((this.group3.yzx * right_dual.group1.zxy), this.e35 * right_dual.e4125) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (this.group3.zxyz * float4(right_dual.group1.yzx, right_dual.e35)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - float4((this.group1.zxy * right_dual.group3.yzx), this.e4235 * right_dual.e15)
        );
    }
}
public extension DipoleInversion {
    public VersorOdd bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            (this.group3.yzxy * float4(right_dual.group0.zxy, right_dual.e2)) + float4(float3(0.0), (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) + float4((this.group0.xyz * right_dual.group0.www), this.e1234 * right_dual.e5) + float4((this.group2.www * right_dual.group1.xyz), this.e4235 * right_dual.e1) - float4((this.group3.zxy * right_dual.group0.yzx), this.e41 * right_dual.e235), 
            /* e23, e31, e12, e45 */
            float4((this.e1234 * right_dual.e235) + (this.e3215 * right_dual.e423), (this.e1234 * right_dual.e315) + (this.e3215 * right_dual.e431), (this.e1234 * right_dual.e125) + (this.e3215 * right_dual.e412), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) + (float4(right_dual.e12345) * this.group1) - (this.group3.xyzx * right_dual.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_dual.group1.xyz) + (float3(right_dual.e12345) * this.group2.xyz) + (this.group3.zxy * right_dual.group2.yzx) - (this.group3.yzx * right_dual.group2.zxy)), this.e1234 * right_dual.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<DualNum> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<DualNum>(this);
    }
}
public extension BulkContractionInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public VersorEven operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension DualNum {
    public VersorEven bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4(right_dual.group0.xyz, right_dual.e12345), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e5 */
            right_dual.group2 * float4(this.group0.xy.yy, this.e12345, this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public VersorOdd bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float4(this.group0.xy.yy, this.e12345, this.e5) * float4(right_dual.group0.xyz, right_dual.e1234), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * right_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension DualNum {
    public DualNum bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return DualNum(
            /* e5, e12345 */
            float2((right_dual.e5 * this.e12345) + (right_dual.e12345 * this.e5), right_dual.e12345 * this.e12345)
        );
    }
}
public extension DualNum {
    public FlatPoint bulk_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
public extension DualNum {
    public Flector bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
public extension DualNum {
    public Line bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual.group1.xyz
        );
    }
}
public extension DualNum {
    public Motor bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * right_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.yy, this.e12345, (this.e5 * right_dual.e12345) + (this.e12345 * right_dual.e5)) * float4(right_dual.group1.xyz, 1.0)
        );
    }
}
public extension DualNum {
    public Plane bulk_contraction(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
public extension DualNum {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension DualNum {
    public Dipole bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35 */
            float3(this.e12345) * right_dual.group2.xyz
        );
    }
}
public extension DualNum {
    public AntiCircleRotor bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * right_dual.group2
        );
    }
}
public extension DualNum {
    public Circle bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual.group2.xyz
        );
    }
}
public extension DualNum {
    public AntiDipoleInversion bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * right_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension DualNum {
    public AntiDualNum bulk_contraction(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
public extension DualNum {
    public AntiFlatPoint bulk_contraction(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
public extension DualNum {
    public AntiFlector bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
public extension DualNum {
    public AntiLine bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * right_dual.group1.xyz
        );
    }
}
public extension DualNum {
    public AntiMotor bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * right_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
public extension DualNum {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((this.e5 * right_dual.e1234) + (this.e12345 * right_dual.scalar), this.e12345 * right_dual.e12345), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group1, 
            /* e5 */
            (this.e5 * right_dual.e12345) + (this.e12345 * right_dual.e5), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_dual.group3, 
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * right_dual.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group6, 
            /* e423, e431, e412 */
            float3(this.e12345) * right_dual.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group9, 
            /* e1234 */
            this.e12345 * right_dual.e1234
        );
    }
}
public extension DualNum {
    public AntiPlane bulk_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension DualNum {
    public VersorOdd bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.e5 * right_dual.e1234), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e12345 * right_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0
        );
    }
}
public extension DualNum {
    public DualNum bulk_contraction(Scalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
public extension DualNum {
    public RoundPoint bulk_contraction(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5 */
            this.e12345 * right_dual.e5
        );
    }
}
public extension DualNum {
    public VersorOdd bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float4(this.group0.xy.yy, this.e12345, (this.e5 * right_dual.e1234) + (this.e12345 * right_dual.scalar)) * float4(right_dual.group0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * right_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension DualNum {
    public VersorEven bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(this.e12345) * right_dual.group0, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.yy, this.e12345, (this.e5 * right_dual.e12345) + (this.e12345 * right_dual.e5)) * float4(right_dual.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension FlatPoint {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<FlatPoint> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<FlatPoint>(this);
    }
}
public extension BulkContractionInfixPartial<FlatPoint> {
    // Fancy infix trick (second half)
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension FlatPoint {
    public AntiCircleRotor bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
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
            float4(float3(0.0), right_dual.e12345 * this.e45), 
            /* e15, e25, e35, scalar */
            float4((right_dual.group2.www * this.group0.xyz), -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45))
        );
    }
}
public extension FlatPoint {
    public RoundPoint bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float4(right_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (right_dual.e4235 * this.e15) + (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45)
        );
    }
}
public extension FlatPoint {
    public FlatPoint bulk_contraction(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension FlatPoint {
    public DualNum bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 0.0)
        );
    }
}
public extension FlatPoint {
    public FlatPoint bulk_contraction(AntiMotor other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension FlatPoint {
    public DualNum bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 0.0)
        );
    }
}
public extension FlatPoint {
    public Scalar bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Scalar(
            /* scalar */
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)
        );
    }
}
public extension FlatPoint {
    public Scalar bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)
        );
    }
}
public extension FlatPoint {
    public DualNum bulk_contraction(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e5 * this.e45 * -1.0, 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension FlatPoint {
    public Scalar bulk_contraction(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * this.e45 * -1.0
        );
    }
}
public extension FlatPoint {
    public Scalar bulk_contraction(Flector other) {
        return Scalar(
            /* scalar */
            this.e45 * other.e45 * -1.0
        );
    }
}
public extension FlatPoint {
    public DualNum bulk_contraction(Motor other) {
        return DualNum(
            /* e5, e12345 */
            float2(float4(other.group1.xyz, other.e5 * -1.0)[3] * this.e45, 1.0) * float2(1.0, 0.0)
        );
    }
}
public extension FlatPoint {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2(-(this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * this.group0, 
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
    public RoundPoint bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215)
        );
    }
}
public extension FlatPoint {
    public FlatPoint bulk_contraction(Scalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension FlatPoint {
    public RoundPoint bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float4(right_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5 */
            (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215)
        );
    }
}
public extension FlatPoint {
    public AntiCircleRotor bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(float3(0.0), this.e45 * right_dual.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group0.xyz), -(this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321))
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<Flector> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<Flector>(this);
    }
}
public extension BulkContractionInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public VersorOdd operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension Flector {
    public VersorOdd bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.e412 * this.e4315, right_dual.e423 * this.e4125, right_dual.e431 * this.e4235, -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)) - float4((right_dual.group0.xyz.yzx * this.group1.zxy), right_dual.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) + float4((right_dual.group0.xyz * this.group1.www), right_dual.e12345 * this.e45) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.e12345) * this.group0.xyz), 0.0) + float4((float3(this.e3215) * right_dual.group1.xyz), 0.0) + float4((right_dual.group2.yzx * this.group1.zxy), 0.0) - float4((right_dual.group2.zxy * this.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
public extension Flector {
    public AntiDipoleInversion bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float3(right_dual.e1234) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group3.yzx * this.group1.zxy) - (right_dual.group3.zxy * this.group1.yzx)), right_dual.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(right_dual.group3.www, right_dual.e41)) + float4(float3(0.0), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) - float4((this.group1.www * right_dual.group3.xyz), right_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e35 * this.e4125)) + float4((right_dual.group1.zxy * this.group1.yzx), right_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (right_dual.group2.wwwy * float4(this.group0.xyz, this.e4315)) - (this.group1.zxyx * float4(right_dual.group1.yzx, right_dual.e15))
        );
    }
}
public extension Flector {
    public Flector bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
public extension Flector {
    public DualNum bulk_contraction(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 0.0)
        );
    }
}
public extension Flector {
    public Motor bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_dual.group1.yzx * this.group1.zxy) - (right_dual.group1.zxy * this.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (right_dual.group1.wwwx * float4(this.group1.xyz, this.e15)) + float4(float3(0.0), (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215)) - (this.group1.wwwx * float4(right_dual.group1.xyz, right_dual.e15))
        );
    }
}
public extension Flector {
    public FlatPoint bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) - (this.group1.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e415))
        );
    }
}
public extension Flector {
    public Flector bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) + (float4(right_dual.e12345) * this.group0) - (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
public extension Flector {
    public Motor bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((this.group1.zxy * right_dual.group0.yzx) - (this.group1.yzx * right_dual.group0.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(this.e3215 * right_dual.e4235 * -1.0, this.e3215 * right_dual.e4315 * -1.0, this.e3215 * right_dual.e4125 * -1.0, (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215)) + (right_dual.group0.wwwx * float4(this.group1.xyz, this.e15))
        );
    }
}
public extension Flector {
    public RoundPoint bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)
        );
    }
}
public extension Flector {
    public RoundPoint bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)
        );
    }
}
public extension Flector {
    public AntiCircleRotor bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_dual.group0.xyz.zxy * this.group1.yzx) - (right_dual.group0.xyz.yzx * this.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * this.e3215, right_dual.e431 * this.e3215, right_dual.e412 * this.e3215, -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)) - float4((right_dual.group2.xyz.zxy * this.group1.yzx), right_dual.e423 * this.e15)
        );
    }
}
public extension Flector {
    public AntiCircleRotor bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            (right_dual.group0.xyz.zxy * this.group1.yzx) - (right_dual.group0.xyz.yzx * this.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * this.e3215, right_dual.e431 * this.e3215, right_dual.e412 * this.e3215, -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (this.group1.zxyx * float4(right_dual.group2.yzx, right_dual.e1)) + float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)) - float4((right_dual.group2.zxy * this.group1.yzx), right_dual.e423 * this.e15)
        );
    }
}
public extension Flector {
    public AntiFlector bulk_contraction(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), right_dual.e3215 * this.e45)
        );
    }
}
public extension Flector {
    public AntiMotor bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.e321) * float4(this.group1.xyz, this.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group1.zxy) - (right_dual.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public AntiMotor bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_dual.group0.www * this.group1.xyz * float3(-1.0)), (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) - (right_dual.e321 * this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group1.zxy) - (right_dual.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
public extension Flector {
    public AntiPlane bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * this.e4315, right_dual.e23 * this.e4125, right_dual.e31 * this.e4235, -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group1.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
public extension Flector {
    public AntiFlector bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) + float4((right_dual.group0.zxy * this.group1.yzx), right_dual.e3215 * this.e45) - (this.group1.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
public extension Flector {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(this.e15 * right_dual.e1234) - (this.e4125 * right_dual.e31), -(this.e25 * right_dual.e1234) - (this.e4235 * right_dual.e12), -(this.e35 * right_dual.e1234) - (this.e4315 * right_dual.e23), (this.e4315 * right_dual.e42) + (this.e4125 * right_dual.e43)) + (this.group1.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)) - float4((right_dual.group4.xyz * this.group1.www), this.e45 * right_dual.e1234), 
            /* e5 */
            (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215) - (this.e4235 * right_dual.e15) - (this.e4315 * right_dual.e25) - (this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) + (float4(right_dual.e12345) * this.group0) - float4((right_dual.group8.xyz.zxy * this.group1.yzx), this.e4235 * right_dual.e415), 
            /* e41, e42, e43 */
            (right_dual.group7.xyz.zxy * this.group1.yzx) - (right_dual.group7.xyz.yzx * this.group1.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_dual.group7.xyz) - (float3(right_dual.e321) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_dual.group9.yzx) - (this.group1.yzx * right_dual.group9.zxy)), this.e3215 * right_dual.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(right_dual.e1234) * this.group1.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group1.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension Flector {
    public Scalar bulk_contraction(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125)
        );
    }
}
public extension Flector {
    public AntiDipoleInversion bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual.e1234) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_dual.group0.yzx) - (this.group1.yzx * right_dual.group0.zxy)), this.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_dual.e3215) * this.group1.xyz) - (float3(this.e3215) * right_dual.group0.xyz)), this.e45 * right_dual.e1234 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(right_dual.e1234) * this.group0.xyz * float3(-1.0)), (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215))
        );
    }
}
public extension Flector {
    public Flector bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
public extension Flector {
    public Scalar bulk_contraction(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Scalar(
            /* scalar */
            (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4)
        );
    }
}
public extension Flector {
    public AntiDipoleInversion bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float3(right_dual.e1234) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_dual.group3.yzx) - (this.group1.yzx * right_dual.group3.zxy)), this.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(right_dual.group3.www, right_dual.e41)) + float4(float3(0.0), (this.e4315 * right_dual.e42) + (this.e4125 * right_dual.e43)) - float4((this.group1.www * right_dual.group3.xyz), this.e45 * right_dual.e1234), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215) - (this.e3215 * right_dual.e45)) + float4((this.group1.yzx * right_dual.group1.zxy), this.e15 * right_dual.e4235) - (this.group1.zxyy * float4(right_dual.group1.yzx, right_dual.e25)) - (this.group1.wwwz * float4(right_dual.group0.xyz, right_dual.e35)) - (right_dual.group2.wwwx * float4(this.group0.xyz, this.e4235))
        );
    }
}
public extension Flector {
    public VersorOdd bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            (this.group1.yzxx * float4(right_dual.group0.zxy, right_dual.e1)) + float4(float3(0.0), (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            (right_dual.group0 * float4(this.group1.www, this.e45)) + float4(float3(0.0), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) - (this.group1.xyzx * right_dual.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * right_dual.group1.xyz), 0.0) + float4((float3(right_dual.e12345) * this.group0.xyz), 0.0) + float4((this.group1.zxy * right_dual.group2.yzx), 0.0) - float4((this.group1.yzx * right_dual.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<Line> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<Line>(this);
    }
}
public extension BulkContractionInfixPartial<Line> {
    // Fancy infix trick (second half)
    public AntiDipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension Line {
    public AntiDipoleInversion bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
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
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group1.xyz), -(right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e412 * this.e315) + (right_dual.e321 * this.e415), (right_dual.e423 * this.e125) + (right_dual.e321 * this.e425), (right_dual.e431 * this.e235) + (right_dual.e321 * this.e435), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_dual.e235 * this.e415)
        );
    }
}
public extension Line {
    public AntiCircleRotor bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float3(right_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual.e1234) * this.group1.xyz), -(right_dual.e4235 * this.e415) - (right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)), 
            /* e15, e25, e35, scalar */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)) - float4((this.group1.xyz.zxy * right_dual.group3.yzx), right_dual.e41 * this.e235)
        );
    }
}
public extension Line {
    public Line bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.e12345) * this.group1.xyz
        );
    }
}
public extension Line {
    public FlatPoint bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) - (right_dual.group1.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
public extension Line {
    public DualNum bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0)
        );
    }
}
public extension Line {
    public Motor bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(right_dual.e12345) * this.group1.xyz), -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435))
        );
    }
}
public extension Line {
    public FlatPoint bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
public extension Line {
    public Scalar bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Scalar(
            /* scalar */
            -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)
        );
    }
}
public extension Line {
    public Scalar bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)
        );
    }
}
public extension Line {
    public RoundPoint bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e321 * this.e415), (right_dual.e423 * this.e125) + (right_dual.e321 * this.e425), (right_dual.e431 * this.e235) + (right_dual.e321 * this.e435), -(right_dual.e431 * this.e425) - (right_dual.e412 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_dual.e423 * this.e415), 
            /* e5 */
            -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)
        );
    }
}
public extension Line {
    public RoundPoint bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float4((right_dual.e412 * this.e315) + (right_dual.e321 * this.e415), (right_dual.e423 * this.e125) + (right_dual.e321 * this.e425), (right_dual.e431 * this.e235) + (right_dual.e321 * this.e435), -(right_dual.e431 * this.e425) - (right_dual.e412 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_dual.e423 * this.e415), 
            /* e5 */
            -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)
        );
    }
}
public extension Line {
    public FlatPoint bulk_contraction(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public AntiPlane bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(right_dual.e321) * this.group0.xyz), -(right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435))
        );
    }
}
public extension Line {
    public AntiPlane bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(right_dual.e321) * this.group0.xyz), -(right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435))
        );
    }
}
public extension Line {
    public Scalar bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Scalar(
            /* scalar */
            -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)
        );
    }
}
public extension Line {
    public AntiMotor bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Line {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2(-(this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_dual.e321) + (this.e315 * right_dual.e412), (this.e425 * right_dual.e321) + (this.e125 * right_dual.e423), (this.e435 * right_dual.e321) + (this.e235 * right_dual.e431), -(this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) - float4((this.group1.xyz.zxy * right_dual.group7.xyz.yzx), this.e415 * right_dual.e423), 
            /* e5 */
            -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group9.yzxx * float4(this.group1.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e1234) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.group0.xy.yy, right_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual.e12345) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Line {
    public Dipole bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual.e1234) * this.group1.xyz), -(this.e415 * right_dual.e4235) - (this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)), 
            /* e15, e25, e35 */
            (float3(right_dual.e3215) * this.group0.xyz) + (this.group1.xyz.yzx * right_dual.group0.zxy) - (this.group1.xyz.zxy * right_dual.group0.yzx)
        );
    }
}
public extension Line {
    public Line bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_dual.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.e12345) * this.group1.xyz
        );
    }
}
public extension Line {
    public AntiCircleRotor bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float3(right_dual.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual.e1234) * this.group1.xyz), -(this.e415 * right_dual.e4235) - (this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((this.group1.xyz.zxy * right_dual.group3.yzx), this.e415 * right_dual.e23)
        );
    }
}
public extension Line {
    public AntiDipoleInversion bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group1.xyz), -(this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_dual.e321) + (this.e315 * right_dual.e412), (this.e425 * right_dual.e321) + (this.e125 * right_dual.e423), (this.e435 * right_dual.e321) + (this.e235 * right_dual.e431), -(this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((this.group1.xyz.zxy * right_dual.group0.yzx), this.e415 * right_dual.e235)
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<Motor> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<Motor>(this);
    }
}
public extension BulkContractionInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public VersorEven operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension Motor {
    public VersorEven bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4(right_dual.group0.xyz, right_dual.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_dual.group1.xyz)), right_dual.e321 * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e12345 * this.e235, right_dual.e12345 * this.e315, right_dual.e12345 * this.e125, -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + (right_dual.group2 * float4(this.group0.www, this.e5)), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e321 * this.e415), (right_dual.e423 * this.e125) + (right_dual.e321 * this.e425), (right_dual.e431 * this.e235) + (right_dual.e321 * this.e435), -(right_dual.e431 * this.e425) - (right_dual.e412 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group1.zxy), right_dual.e423 * this.e415)
        );
    }
}
public extension Motor {
    public VersorOdd bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float4(right_dual.e1234 * this.e415, right_dual.e1234 * this.e425, right_dual.e1234 * this.e435, -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)) + float4((right_dual.group0.xyz * this.group0.www), right_dual.e1234 * this.e5), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e1234 * this.e235, right_dual.e1234 * this.e315, right_dual.e1234 * this.e125, -(right_dual.e4235 * this.e415) - (right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(this.e12345) * right_dual.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e3215) * this.group0.xyz) + (float3(this.e12345) * right_dual.group2.xyz) + (right_dual.group3.zxy * this.group1.yzx) - (right_dual.group3.yzx * this.group1.zxy)), right_dual.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension Motor {
    public Motor bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_dual.group0.xy.yy, right_dual.e12345, (right_dual.e5 * this.e12345) + (right_dual.e12345 * this.e5)) * float4(this.group1.xyz, 1.0)
        );
    }
}
public extension Motor {
    public FlatPoint bulk_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
public extension Motor {
    public Flector bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(this.e12345) * right_dual.group0) - (right_dual.group1.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
public extension Motor {
    public Motor bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e12345) * right_dual.group1.xyz), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435))
        );
    }
}
public extension Motor {
    public Motor bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(right_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_dual.group0.xyz)), right_dual.e12345 * this.e12345), 
            /* e235, e315, e125, e5 */
            (float4(right_dual.e12345) * this.group1) + (float4(this.e12345) * right_dual.group1) + float4(float3(0.0), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435))
        );
    }
}
public extension Motor {
    public Flector bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0
        );
    }
}
public extension Motor {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension Motor {
    public AntiCircleRotor bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435))
        );
    }
}
public extension Motor {
    public AntiCircleRotor bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435))
        );
    }
}
public extension Motor {
    public AntiDipoleInversion bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e412 * this.e315) + (right_dual.e321 * this.e415), (right_dual.e423 * this.e125) + (right_dual.e321 * this.e425), (right_dual.e431 * this.e235) + (right_dual.e321 * this.e435), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group1.zxy), right_dual.e235 * this.e415)
        );
    }
}
public extension Motor {
    public AntiDipoleInversion bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.e4 * this.e12345) - (right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e321 * this.e415) + (right_dual.e1 * this.e12345), (right_dual.e321 * this.e425) + (right_dual.e2 * this.e12345), (right_dual.e321 * this.e435) + (right_dual.e3 * this.e12345), -(right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + float4((right_dual.group0.xyz.zxy * this.group1.yzx), right_dual.e5 * this.e12345) - (this.group1.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e415))
        );
    }
}
public extension Motor {
    public AntiMotor bulk_contraction(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_dual.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e3215) * this.group0
        );
    }
}
public extension Motor {
    public AntiFlector bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual.group0.www * this.group0.xyz), -(right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435))
        );
    }
}
public extension Motor {
    public AntiFlector bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual.e1 * this.e12345, right_dual.e2 * this.e12345, right_dual.e3 * this.e12345, -(right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + (this.group0 * float4(right_dual.group0.www, right_dual.e5))
        );
    }
}
public extension Motor {
    public AntiMotor bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_dual.group0.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Motor {
    public AntiMotor bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_dual.group0.xyz), (right_dual.scalar * this.e12345) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual.e3215) * this.group0.xyz) + (float3(this.e12345) * right_dual.group1.xyz)), right_dual.e3215 * this.e12345)
        );
    }
}
public extension Motor {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((this.e12345 * right_dual.scalar) + (this.e5 * right_dual.e1234) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), this.e12345 * right_dual.e12345), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_dual.e321) + (this.e12345 * right_dual.e1), (this.e425 * right_dual.e321) + (this.e12345 * right_dual.e2), (this.e435 * right_dual.e321) + (this.e12345 * right_dual.e3), -(this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) + float4((right_dual.group7.xyz.zxy * this.group1.yzx), this.e12345 * right_dual.e4) - float4((right_dual.group7.xyz.yzx * this.group1.zxy), this.e415 * right_dual.e423), 
            /* e5 */
            (this.e12345 * right_dual.e5) + (this.e5 * right_dual.e12345) - (this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e12345 * right_dual.e15) + (this.e315 * right_dual.e4125), (this.e12345 * right_dual.e25) + (this.e125 * right_dual.e4235), (this.e12345 * right_dual.e35) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) + (this.group0 * float4(right_dual.group9.www, right_dual.e45)) - (right_dual.group9.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * right_dual.group4.xyz) + (float3(right_dual.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e12345) * right_dual.group5.xyz) + (float3(right_dual.e1234) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * right_dual.group6.xyz) + (float3(right_dual.e12345) * this.group0.xyz)), this.e12345 * right_dual.e321), 
            /* e423, e431, e412 */
            float3(this.e12345) * right_dual.group7.xyz, 
            /* e235, e315, e125 */
            (float3(this.e12345) * right_dual.group8.xyz) + (float3(right_dual.e12345) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group9, 
            /* e1234 */
            this.e12345 * right_dual.e1234
        );
    }
}
public extension Motor {
    public AntiPlane bulk_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension Motor {
    public VersorOdd bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.e1234) * float4(this.group0.xyz, this.e5), 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual.e1234) * this.group1.xyz), -(this.e415 * right_dual.e4235) - (this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e3215) * this.group0.xyz) + (this.group1.yzx * right_dual.group0.zxy) - (this.group1.zxy * right_dual.group0.yzx)), this.e12345 * right_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0
        );
    }
}
public extension Motor {
    public Motor bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
public extension Motor {
    public RoundPoint bulk_contraction(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5 */
            this.e12345 * right_dual.e5
        );
    }
}
public extension Motor {
    public VersorOdd bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            (this.group0 * float4(right_dual.group2.www, right_dual.scalar)) + float4(float3(0.0), -(this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) + float4((this.group0.www * right_dual.group0.xyz), this.e5 * right_dual.e1234), 
            /* e23, e31, e12, e45 */
            float4(this.e235 * right_dual.e1234, this.e315 * right_dual.e1234, this.e125 * right_dual.e1234, -(this.e415 * right_dual.e4235) - (this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) + (float4(this.e12345) * right_dual.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * right_dual.group2.xyz) + (float3(right_dual.e3215) * this.group0.xyz) + (this.group1.yzx * right_dual.group3.zxy) - (this.group1.zxy * right_dual.group3.yzx)), this.e12345 * right_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension Motor {
    public VersorEven bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(this.e12345) * right_dual.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * right_dual.group1.xyz) + (float3(right_dual.e12345) * this.group0.xyz)), this.e12345 * right_dual.e321), 
            /* e235, e315, e125, e5 */
            (float4(this.e12345) * right_dual.group2) + (float4(right_dual.e12345) * this.group1) + float4(float3(0.0), -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)), 
            /* e1, e2, e3, e4 */
            float4((this.e12345 * right_dual.e1) + (this.e315 * right_dual.e412), (this.e12345 * right_dual.e2) + (this.e125 * right_dual.e423), (this.e12345 * right_dual.e3) + (this.e235 * right_dual.e431), -(this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) + (this.group0 * float4(right_dual.group1.www, right_dual.e4)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e415))
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<MultiVector> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<MultiVector>(this);
    }
}
public extension BulkContractionInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public MultiVector operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual.e12345 * this.scalar) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), right_dual.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412) + (right_dual.e12345 * this.e1), (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423) + (right_dual.e12345 * this.e2), (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431) + (right_dual.e12345 * this.e3), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) + float4((right_dual.group0.xyz.zxy * this.group8.xyz.yzx), right_dual.e12345 * this.e4) - float4((right_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_dual.e423 * this.e415) - float4((this.group7.xyz.yzx * right_dual.group2.zxy), right_dual.e431 * this.e425), 
            /* e5 */
            (right_dual.e12345 * this.e5) - (right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e315 * this.e4125) + (right_dual.e12345 * this.e15), (right_dual.e125 * this.e4235) + (right_dual.e12345 * this.e25), (right_dual.e235 * this.e4315) + (right_dual.e12345 * this.e35), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) + float4((this.group9.www * right_dual.group1.xyz), right_dual.e12345 * this.e45) - (this.group9.yzxx * float4(right_dual.group2.zxy, right_dual.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual.e12345) * this.group4.xyz) + (float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group9.yzx) - (right_dual.group0.xyz.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e12345) * this.group5.xyz) + (float3(this.e3215) * right_dual.group0.xyz) + (float3(this.e1234) * right_dual.group2.xyz) - (float3(right_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(right_dual.e12345) * this.group6) + (float4(this.e12345) * right_dual.group1), 
            /* e423, e431, e412 */
            (float3(right_dual.e12345) * this.group7.xyz) + (float3(this.e12345) * right_dual.group0.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual.e12345) * this.group8.xyz) + (float3(this.e12345) * right_dual.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group9, 
            /* e1234 */
            right_dual.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float2((right_dual.e1234 * this.e5) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_dual.group2.xyz, right_dual.e45)) + (this.group9.yzxz * float4(right_dual.group1.zxy, right_dual.e43)) + float4((this.group4.xyz * right_dual.group3.www), right_dual.e41 * this.e4235) + float4((this.group5.xyz.yzx * right_dual.group3.zxy), right_dual.e42 * this.e4315) - (float4(right_dual.e1234) * this.group3) - (right_dual.group3.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((right_dual.group0.xyz * this.group9.www), right_dual.e4235 * this.e41) - float4((right_dual.group1.yzx * this.group9.zxy), right_dual.e4125 * this.e43), 
            /* e5 */
            (right_dual.e4235 * this.e15) + (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(this.e12345) * float4(right_dual.group2.xyz, right_dual.e45)) - float4((this.group8.xyz.zxy * right_dual.group3.yzx), right_dual.e4235 * this.e415), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * this.group6.xyz) + (float3(this.e12345) * right_dual.group0.xyz) + (this.group7.xyz.zxy * right_dual.group3.yzx) - (this.group7.xyz.yzx * right_dual.group3.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e1234) * this.group8.xyz) + (float3(right_dual.e3215) * this.group7.xyz) + (float3(this.e12345) * right_dual.group1.xyz) - (float3(this.e321) * right_dual.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_dual.group3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3, 
            /* e1234 */
            right_dual.e1234 * this.e12345
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual.e5 * this.e1234) + (right_dual.e12345 * this.scalar), right_dual.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group1, 
            /* e5 */
            (right_dual.e5 * this.e12345) + (right_dual.e12345 * this.e5), 
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group9, 
            /* e1234 */
            right_dual.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e5 */
            -(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_dual.group0, 
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
    public MultiVector bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e15 * this.e1234) + (right_dual.e4125 * this.e31), (right_dual.e25 * this.e1234) + (right_dual.e4235 * this.e12), (right_dual.e35 * this.e1234) + (right_dual.e4315 * this.e23), -(right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) + float4((this.group4.xyz * right_dual.group1.www), right_dual.e45 * this.e1234) - (right_dual.group1.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5 */
            (right_dual.e4235 * this.e15) + (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(this.e12345) * right_dual.group0) - float4((this.group8.xyz.zxy * right_dual.group1.yzx), right_dual.e4235 * this.e415), 
            /* e41, e42, e43 */
            (this.group7.xyz.zxy * right_dual.group1.yzx) - (this.group7.xyz.yzx * right_dual.group1.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_dual.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group1.yzx * this.group9.zxy) - (right_dual.group1.zxy * this.group9.yzx)), right_dual.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual.group1.xyz, 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_dual.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e415 * this.e321) + (right_dual.e315 * this.e412), (right_dual.e425 * this.e321) + (right_dual.e125 * this.e423), (right_dual.e435 * this.e321) + (right_dual.e235 * this.e431), -(right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) - float4((right_dual.group1.xyz.zxy * this.group7.xyz.yzx), right_dual.e415 * this.e423), 
            /* e5 */
            -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual.e12345 * this.scalar) + (right_dual.e5 * this.e1234) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), right_dual.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e415 * this.e321) + (right_dual.e12345 * this.e1), (right_dual.e425 * this.e321) + (right_dual.e12345 * this.e2), (right_dual.e435 * this.e321) + (right_dual.e12345 * this.e3), -(right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) + float4((this.group7.xyz.zxy * right_dual.group1.yzx), right_dual.e12345 * this.e4) - float4((this.group7.xyz.yzx * right_dual.group1.zxy), right_dual.e415 * this.e423), 
            /* e5 */
            (right_dual.e12345 * this.e5) + (right_dual.e5 * this.e12345) - (right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e12345 * this.e15) + (right_dual.e315 * this.e4125), (right_dual.e12345 * this.e25) + (right_dual.e125 * this.e4235), (right_dual.e12345 * this.e35) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) + (right_dual.group0 * float4(this.group9.www, this.e45)) - (this.group9.yzxx * float4(right_dual.group1.zxy, right_dual.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual.e12345) * this.group4.xyz) + (float3(this.e1234) * right_dual.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual.e12345) * this.group5.xyz) + (float3(this.e1234) * right_dual.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual.e12345) * this.group6.xyz) + (float3(this.e12345) * right_dual.group0.xyz)), right_dual.e12345 * this.e321), 
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            (float3(right_dual.e12345) * this.group8.xyz) + (float3(this.e12345) * right_dual.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group9, 
            /* e1234 */
            right_dual.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5 */
            (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (this.group7.xyz.zxy * right_dual.group0.yzx) - (this.group7.xyz.yzx * right_dual.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_dual.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((this.group9.zxy * right_dual.group0.yzx) - (this.group9.yzx * right_dual.group0.zxy)), this.e1234 * right_dual.e3215), 
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual.group0.xyz, 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_dual.group0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group9.yzxy * float4(right_dual.group1.zxy, right_dual.e42)) + float4((float3(this.e1234) * right_dual.group2.xyz), right_dual.e41 * this.e4235), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(right_dual.group2.xyz, right_dual.e45), 
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * right_dual.group1.xyz, 
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
    public MultiVector bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group9.yzxx * float4(right_dual.group1.zxy, right_dual.e41)) + float4((float3(this.e1234) * right_dual.group2.xyz), right_dual.e42 * this.e4315), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(right_dual.group2.xyz, right_dual.e45), 
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * right_dual.group1.xyz, 
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
    public MultiVector bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) - float4((right_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_dual.e423 * this.e415) - float4((right_dual.group2.xyz.zxy * this.group7.xyz.yzx), right_dual.e431 * this.e425), 
            /* e5 */
            -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_dual.group2.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group9.yzx) - (right_dual.group0.xyz.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_dual.group0.xyz) + (float3(this.e1234) * right_dual.group2.xyz) - (float3(right_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e423, e431, e412 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float2((right_dual.e4 * this.e3215) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) + (float4(this.e12345) * float4(right_dual.group3.xyz, right_dual.e4)) - float4((right_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_dual.e423 * this.e415) - float4((this.group7.xyz.yzx * right_dual.group2.zxy), right_dual.e431 * this.e425), 
            /* e5 */
            (right_dual.e5 * this.e12345) - (right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_dual.group2.zxy, right_dual.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group9.yzx) - (right_dual.group0.xyz.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_dual.group0.xyz) + (float3(this.e1234) * right_dual.group2.xyz) - (float3(right_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e423, e431, e412 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual.e3215 * this.e4) + (right_dual.scalar * this.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            right_dual.e3215 * this.e45, 
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_dual.e3215) * this.group7.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual.e3215) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual.e3215 * this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual.e321) * this.group6.xyz), 0.0) + float4((this.group7.xyz.zxy * right_dual.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e5 */
            -(right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(((right_dual.group0.yzx * this.group9.zxy) - (right_dual.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.e321 * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual.e321) * this.group6.xyz), 0.0) + float4((float3(this.e12345) * right_dual.group1.xyz), 0.0) + float4((this.group7.xyz.zxy * right_dual.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e5 */
            (right_dual.e5 * this.e12345) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(((right_dual.group0.yzx * this.group9.zxy) - (right_dual.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.e321 * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e1234) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.xyz.zxy * this.group9.yzx), 0.0) - float4((right_dual.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            -(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e12345) * right_dual.group0.xyz, 
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
    public MultiVector bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual.scalar * this.e12345) + (right_dual.e3215 * this.e4) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual.e3215) * this.group4.xyz), 0.0) + float4((float3(this.e1234) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.zxy * this.group9.yzx), 0.0) - float4((right_dual.group0.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            (right_dual.e3215 * this.e45) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.e3215) * this.group6.xyz) + (float3(this.e12345) * right_dual.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(right_dual.e3215) * this.group7.xyz) + (float3(this.e12345) * right_dual.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual.e3215) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual.e3215 * this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((right_dual.scalar * this.e12345) + (right_dual.e12345 * this.scalar) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), right_dual.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e15 * this.e1234) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e4125 * this.e31), (right_dual.e25 * this.e1234) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e4235 * this.e12), (right_dual.e35 * this.e1234) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e4315 * this.e23), -(right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412) - (right_dual.e1234 * this.e45)) + (float4(right_dual.e12345) * this.group1) + (float4(this.e12345) * right_dual.group1) + (this.group9.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)) + float4((this.group4.xyz * right_dual.group9.www), right_dual.e45 * this.e1234) + float4((right_dual.group7.xyz.zxy * this.group8.xyz.yzx), right_dual.e42 * this.e4315) + float4((right_dual.group8.xyz.yzx * this.group7.xyz.zxy), right_dual.e43 * this.e4125) - (right_dual.group9.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((float3(right_dual.e1234) * this.group3.xyz), right_dual.e4125 * this.e43) - float4((right_dual.group4.xyz * this.group9.www), right_dual.e423 * this.e415) - float4((right_dual.group5.xyz.yzx * this.group9.zxy), right_dual.e431 * this.e425) - float4((right_dual.group7.xyz.yzx * this.group8.xyz.zxy), right_dual.e412 * this.e435) - float4((right_dual.group8.xyz.zxy * this.group7.xyz.yzx), right_dual.e4235 * this.e41), 
            /* e5 */
            (right_dual.e12345 * this.e5) + (right_dual.e5 * this.e12345) + (right_dual.e4235 * this.e15) + (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215) - (right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125) + (right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235) + (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315) + (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e435 * this.e4125) - (right_dual.e4235 * this.e415) - (right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(right_dual.e12345) * this.group3) + (float4(this.e12345) * right_dual.group3) - (this.group9.yzxx * float4(right_dual.group8.xyz.zxy, right_dual.e415)) - float4((this.group8.xyz.zxy * right_dual.group9.yzx), right_dual.e425 * this.e4315), 
            /* e41, e42, e43 */
            (float3(right_dual.e12345) * this.group4.xyz) + (float3(right_dual.e1234) * this.group6.xyz) + (float3(this.e12345) * right_dual.group4.xyz) + (float3(this.e1234) * right_dual.group6.xyz) + (right_dual.group7.xyz.zxy * this.group9.yzx) + (this.group7.xyz.zxy * right_dual.group9.yzx) - (right_dual.group7.xyz.yzx * this.group9.zxy) - (this.group7.xyz.yzx * right_dual.group9.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e12345) * this.group5.xyz) + (float3(right_dual.e3215) * this.group7.xyz) + (float3(right_dual.e1234) * this.group8.xyz) + (float3(this.e12345) * right_dual.group5.xyz) + (float3(this.e3215) * right_dual.group7.xyz) + (float3(this.e1234) * right_dual.group8.xyz) - (float3(right_dual.e321) * this.group9.xyz) - (float3(this.e321) * right_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(right_dual.e12345) * this.group6) + (float4(this.e12345) * right_dual.group6) + (right_dual.group9.yzxw * float4(this.group9.zxy, this.e1234)) - float4((right_dual.group9.zxy * this.group9.yzx), right_dual.e1234 * this.e3215), 
            /* e423, e431, e412 */
            (float3(right_dual.e12345) * this.group7.xyz) + (float3(this.e12345) * right_dual.group7.xyz) + (float3(this.e1234) * right_dual.group9.xyz) - (float3(right_dual.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual.e12345) * this.group8.xyz) + (float3(right_dual.e3215) * this.group9.xyz) + (float3(this.e12345) * right_dual.group8.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e12345) * this.group9) + (float4(this.e12345) * right_dual.group9), 
            /* e1234 */
            (right_dual.e12345 * this.e1234) + (right_dual.e1234 * this.e12345)
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            right_dual.e5 * this.e12345, 
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
    public MultiVector bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e45 * right_dual.e1234) - (this.e43 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((float3(right_dual.e1234) * this.group3.xyz), this.e42 * right_dual.e4315), 
            /* e5 */
            (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * right_dual.group0.yzx) - (this.group7.xyz.yzx * right_dual.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e3215) * this.group7.xyz) + (float3(right_dual.e1234) * this.group8.xyz) - (float3(this.e321) * right_dual.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * right_dual.group0.yzx), this.e1234 * right_dual.e3215) - (this.group9.yzxw * float4(right_dual.group0.zxy, right_dual.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_dual.group0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0, 
            /* e1234 */
            this.e12345 * right_dual.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual.e12345) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group1, 
            /* e5 */
            right_dual.e12345 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(right_dual.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(right_dual.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group9, 
            /* e1234 */
            right_dual.e12345 * this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) + (this.e1234 * right_dual.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5 */
            this.e12345 * right_dual.e5, 
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
    public MultiVector bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float2((this.e12345 * right_dual.scalar) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_dual.group2.xyz, right_dual.e45)) + (this.group9.yzxz * float4(right_dual.group1.zxy, right_dual.e43)) + float4((this.group4.xyz * right_dual.group3.www), this.e4235 * right_dual.e41) + float4((this.group5.xyz.yzx * right_dual.group3.zxy), this.e4315 * right_dual.e42) - (right_dual.group3.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((this.group9.zxy * right_dual.group1.yzx), this.e43 * right_dual.e4125) - float4((this.group9.www * right_dual.group0.xyz), this.e45 * right_dual.e1234) - float4((right_dual.group2.www * this.group3.xyz), this.e42 * right_dual.e4315), 
            /* e5 */
            (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215) - (this.e4235 * right_dual.e15) - (this.e4315 * right_dual.e25) - (this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) + (float4(this.e12345) * float4(right_dual.group2.xyz, right_dual.e45)) - (right_dual.group3.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * right_dual.group0.xyz) + (float3(right_dual.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * right_dual.group3.yzx) - (this.group7.xyz.yzx * right_dual.group3.zxy), 
            /* e23, e31, e12 */
            (float3(this.e12345) * right_dual.group1.xyz) + (float3(right_dual.e1234) * this.group8.xyz) + (float3(right_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_dual.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * right_dual.group3.yzx), this.e1234 * right_dual.e3215) - (this.group9.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_dual.group3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3, 
            /* e1234 */
            this.e12345 * right_dual.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float2((this.scalar * right_dual.e12345) + (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) + (this.e1234 * right_dual.e5) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435), this.e12345 * right_dual.e12345), 
            /* e1, e2, e3, e4 */
            float4((this.e1 * right_dual.e12345) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e2 * right_dual.e12345) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e3 * right_dual.e12345) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412) - (this.e412 * right_dual.e435)) + (float4(this.e12345) * right_dual.group3) + float4((this.group7.xyz.zxy * right_dual.group2.yzx), this.e4 * right_dual.e12345) - float4((this.group7.xyz.yzx * right_dual.group2.zxy), this.e423 * right_dual.e415) - float4((this.group8.xyz.zxy * right_dual.group0.yzx), this.e431 * right_dual.e425), 
            /* e5 */
            (this.e12345 * right_dual.e5) + (this.e5 * right_dual.e12345) - (this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) + (float4(right_dual.e12345) * this.group3) - (this.group9.yzxx * float4(right_dual.group2.zxy, right_dual.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual.group1.xyz) + (float3(right_dual.e12345) * this.group4.xyz) + (this.group9.yzx * right_dual.group0.zxy) - (this.group9.zxy * right_dual.group0.yzx), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_dual.group0.xyz) + (float3(this.e1234) * right_dual.group2.xyz) + (float3(right_dual.e12345) * this.group5.xyz) - (float3(right_dual.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * right_dual.group1) + (float4(right_dual.e12345) * this.group6), 
            /* e423, e431, e412 */
            (float3(this.e12345) * right_dual.group0.xyz) + (float3(right_dual.e12345) * this.group7.xyz), 
            /* e235, e315, e125 */
            (float3(this.e12345) * right_dual.group2.xyz) + (float3(right_dual.e12345) * this.group8.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group9, 
            /* e1234 */
            this.e1234 * right_dual.e12345
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<Plane> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<Plane>(this);
    }
}
public extension BulkContractionInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public DipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension Plane {
    public DipoleInversion bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * this.e3215, right_dual.e431 * this.e3215, right_dual.e412 * this.e3215, -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * right_dual.group1.xyz), 0.0) + float4((right_dual.group2.yzx * this.group0.zxy), 0.0) - float4((right_dual.group2.zxy * this.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0
        );
    }
}
public extension Plane {
    public AntiDipoleInversion bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            float3(right_dual.e1234) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group3.yzx * this.group0.zxy) - (right_dual.group3.zxy * this.group0.yzx)), right_dual.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(right_dual.e4235 * this.e3215 * -1.0, right_dual.e4315 * this.e3215 * -1.0, right_dual.e4125 * this.e3215 * -1.0, (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * this.e4315, right_dual.e23 * this.e4125, right_dual.e31 * this.e4235, -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (this.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15))
        );
    }
}
public extension Plane {
    public Plane bulk_contraction(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane {
    public DualNum bulk_contraction(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 0.0)
        );
    }
}
public extension Plane {
    public Motor bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_dual.group1.yzx * this.group0.zxy) - (right_dual.group1.zxy * this.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e3215 * this.e4235, right_dual.e3215 * this.e4315, right_dual.e3215 * this.e4125, -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215)) - (this.group0.wwwx * float4(right_dual.group1.xyz, right_dual.e15))
        );
    }
}
public extension Plane {
    public FlatPoint bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e415))
        );
    }
}
public extension Plane {
    public Flector bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0
        );
    }
}
public extension Plane {
    public Line bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line(
            /* e415, e425, e435 */
            (right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group0.xyz)
        );
    }
}
public extension Plane {
    public RoundPoint bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)
        );
    }
}
public extension Plane {
    public RoundPoint bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)
        );
    }
}
public extension Plane {
    public Dipole bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            (right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * this.e3215, right_dual.e431 * this.e3215, right_dual.e412 * this.e3215, -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(this.e3215) * right_dual.group1.xyz) + (right_dual.group2.xyz.yzx * this.group0.zxy) - (right_dual.group2.xyz.zxy * this.group0.yzx)
        );
    }
}
public extension Plane {
    public AntiCircleRotor bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            (right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * this.e3215, right_dual.e431 * this.e3215, right_dual.e412 * this.e3215, -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(right_dual.e125 * this.e4315 * -1.0, right_dual.e235 * this.e4125 * -1.0, right_dual.e315 * this.e4235 * -1.0, (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125)) + (float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (this.group0.zxyx * float4(right_dual.group2.yzx, right_dual.e1))
        );
    }
}
public extension Plane {
    public AntiFlatPoint bulk_contraction(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension Plane {
    public AntiLine bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual.e321) * this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx)
        );
    }
}
public extension Plane {
    public AntiMotor bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_dual.group0.www * this.group0.xyz * float3(-1.0)), (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Plane {
    public AntiPlane bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * this.e4315, right_dual.e23 * this.e4125, right_dual.e31 * this.e4235, -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
public extension Plane {
    public AntiFlector bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * this.e4315, right_dual.e23 * this.e4125, right_dual.e31 * this.e4235, -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
public extension Plane {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)), 
            /* e5 */
            -(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_dual.group8.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43 */
            (right_dual.group7.xyz.zxy * this.group0.yzx) - (right_dual.group7.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_dual.group7.xyz) - (float3(right_dual.e321) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group9.yzx * this.group0.zxy) - (right_dual.group9.zxy * this.group0.yzx)), right_dual.e1234 * this.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(right_dual.e1234) * this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension Plane {
    public Scalar bulk_contraction(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125)
        );
    }
}
public extension Plane {
    public Circle bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual.e1234) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * right_dual.group0.yzx) - (this.group0.yzx * right_dual.group0.zxy)), this.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group0.xyz)
        );
    }
}
public extension Plane {
    public Plane bulk_contraction(Scalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
public extension Plane {
    public Scalar bulk_contraction(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Scalar(
            /* scalar */
            (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4)
        );
    }
}
public extension Plane {
    public AntiDipoleInversion bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            float3(right_dual.e1234) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * right_dual.group3.yzx) - (this.group0.yzx * right_dual.group3.zxy)), this.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * right_dual.e4235 * -1.0, this.e3215 * right_dual.e4315 * -1.0, this.e3215 * right_dual.e4125 * -1.0, (this.e4315 * right_dual.e42) + (this.e4125 * right_dual.e43)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e4315 * right_dual.e12, this.e4125 * right_dual.e23, this.e4235 * right_dual.e31, -(this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45)) - (this.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - (this.group0.wwwy * float4(right_dual.group0.xyz, right_dual.e25))
        );
    }
}
public extension Plane {
    public VersorOdd bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(this.e4125 * right_dual.e431 * -1.0, this.e4235 * right_dual.e412 * -1.0, this.e4315 * right_dual.e423 * -1.0, (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4)) + (this.group0.yzxx * float4(right_dual.group0.zxy, right_dual.e1)), 
            /* e23, e31, e12, e45 */
            float4(this.e3215 * right_dual.e423, this.e3215 * right_dual.e431, this.e3215 * right_dual.e412, -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) - (this.group0.xyzx * right_dual.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * right_dual.group1.xyz), 0.0) + float4((this.group0.zxy * right_dual.group2.yzx), 0.0) - float4((this.group0.yzx * right_dual.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0
        );
    }
}
public extension RoundPoint {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<RoundPoint> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<RoundPoint>(this);
    }
}
public extension BulkContractionInfixPartial<RoundPoint> {
    // Fancy infix trick (second half)
    public RoundPoint operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension RoundPoint {
    public RoundPoint bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group0, 
            /* e5 */
            right_dual.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public Scalar bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            (right_dual.e1234 * this.e5) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4)
        );
    }
}
public extension RoundPoint {
    public RoundPoint bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group0, 
            /* e5 */
            right_dual.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public Scalar bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4)
        );
    }
}
public extension RoundPoint {
    public RoundPoint bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group0, 
            /* e5 */
            right_dual.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public Scalar bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar(
            /* scalar */
            (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4)
        );
    }
}
public extension RoundPoint {
    public Scalar bulk_contraction(DualNum other) {
        return Scalar(
            /* scalar */
            other.e5 * this.e4 * -1.0
        );
    }
}
public extension RoundPoint {
    public Scalar bulk_contraction(Motor other) {
        return Scalar(
            /* scalar */
            float4(other.group1.xyz, other.e5 * -1.0)[3] * this.e4
        );
    }
}
public extension RoundPoint {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group0, 
            /* e5 */
            right_dual.e12345 * this.e5, 
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
    public Scalar bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Scalar(
            /* scalar */
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234)
        );
    }
}
public extension RoundPoint {
    public RoundPoint bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group0, 
            /* e5 */
            right_dual.e12345 * this.e5
        );
    }
}
public extension RoundPoint {
    public Scalar bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234)
        );
    }
}
public extension RoundPoint {
    public RoundPoint bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(right_dual.e12345) * this.group0, 
            /* e5 */
            this.e5 * right_dual.e12345
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<Scalar> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<Scalar>(this);
    }
}
public extension BulkContractionInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension Scalar {
    public Scalar bulk_contraction(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar bulk_contraction(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar bulk_contraction(AntiMotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar bulk_contraction(MultiVector other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar bulk_contraction(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar bulk_contraction(VersorOdd other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension Sphere {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<Sphere> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<Sphere>(this);
    }
}
public extension BulkContractionInfixPartial<Sphere> {
    // Fancy infix trick (second half)
    public DipoleInversion operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension Sphere {
    public DipoleInversion bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_dual.group1.xyz) + (right_dual.group2.yzx * this.group0.zxy) - (right_dual.group2.zxy * this.group0.yzx)), right_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0
        );
    }
}
public extension Sphere {
    public AntiDipoleInversion bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            (float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(right_dual.e4235 * this.e3215 * -1.0, right_dual.e4315 * this.e3215 * -1.0, right_dual.e4125 * this.e3215 * -1.0, (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (this.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15))
        );
    }
}
public extension Sphere {
    public VersorOdd bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), right_dual.e5 * this.e1234), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), right_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0
        );
    }
}
public extension Sphere {
    public RoundPoint bulk_contraction(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e5 */
            -(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215)
        );
    }
}
public extension Sphere {
    public AntiDipoleInversion bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group1.yzx * this.group0.zxy) - (right_dual.group1.zxy * this.group0.yzx)), right_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group1.xyz)), right_dual.e45 * this.e1234), 
            /* e1, e2, e3, e5 */
            float4((float3(this.e1234) * right_dual.group0.xyz), -(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215))
        );
    }
}
public extension Sphere {
    public Dipole bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_dual.group1.xyz), -(right_dual.e415 * this.e4235) - (right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)), 
            /* e15, e25, e35 */
            (float3(this.e3215) * right_dual.group0.xyz) + (right_dual.group1.xyz.yzx * this.group0.zxy) - (right_dual.group1.xyz.zxy * this.group0.yzx)
        );
    }
}
public extension Sphere {
    public VersorOdd bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e1234) * float4(right_dual.group0.xyz, right_dual.e5), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_dual.group1.xyz), -(right_dual.e415 * this.e4235) - (right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_dual.group0.xyz) + (right_dual.group1.yzx * this.group0.zxy) - (right_dual.group1.zxy * this.group0.yzx)), right_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0
        );
    }
}
public extension Sphere {
    public Circle bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx)), right_dual.e3215 * this.e1234), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group0.xyz)
        );
    }
}
public extension Sphere {
    public RoundPoint bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group0.yzxy * float4(right_dual.group1.zxy, right_dual.e42)) + float4((float3(this.e1234) * right_dual.group2.xyz), right_dual.e41 * this.e4235), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)
        );
    }
}
public extension Sphere {
    public RoundPoint bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)) + float4((float3(this.e1234) * right_dual.group2.xyz), right_dual.e42 * this.e4315), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)
        );
    }
}
public extension Sphere {
    public Dipole bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(this.e3215) * right_dual.group1.xyz) + (right_dual.group2.xyz.yzx * this.group0.zxy) - (right_dual.group2.xyz.zxy * this.group0.yzx)
        );
    }
}
public extension Sphere {
    public AntiCircleRotor bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            (float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(right_dual.e125 * this.e4315 * -1.0, right_dual.e235 * this.e4125 * -1.0, right_dual.e315 * this.e4235 * -1.0, (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)) + (float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (this.group0.zxyx * float4(right_dual.group2.yzx, right_dual.e1))
        );
    }
}
public extension Sphere {
    public AntiFlatPoint bulk_contraction(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e1234)
        );
    }
}
public extension Sphere {
    public AntiLine bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            (float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e321) * this.group0.xyz), 
            /* e15, e25, e35 */
            (right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx)
        );
    }
}
public extension Sphere {
    public AntiMotor bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.e321 * this.e4235 * -1.0, right_dual.e321 * this.e4315 * -1.0, right_dual.e321 * this.e4125 * -1.0, (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)) + float4((float3(this.e1234) * right_dual.group0.xyz), right_dual.e1 * this.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
public extension Sphere {
    public AntiPlane bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
public extension Sphere {
    public AntiFlector bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.e3215) * float4(this.group0.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
public extension Sphere {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e45 * this.e1234) + (right_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)) + float4((float3(this.e1234) * right_dual.group3.xyz), right_dual.e42 * this.e4315), 
            /* e5 */
            -(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_dual.group8.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual.group6.xyz) + (right_dual.group7.xyz.zxy * this.group0.yzx) - (right_dual.group7.xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * right_dual.group7.xyz) + (float3(this.e1234) * right_dual.group8.xyz) - (float3(right_dual.e321) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group9.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_dual.group9.zxy * this.group0.yzx), right_dual.e1234 * this.e3215), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual.group9.xyz) - (float3(right_dual.e1234) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0, 
            /* e1234 */
            right_dual.e12345 * this.e1234
        );
    }
}
public extension Sphere {
    public Scalar bulk_contraction(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public Circle bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Circle(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group0.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_dual.group0.zxy * this.group0.yzx), right_dual.e1234 * this.e3215), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group0.xyz)
        );
    }
}
public extension Sphere {
    public Sphere bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0, 
            /* e1234 */
            right_dual.e12345 * this.e1234
        );
    }
}
public extension Sphere {
    public Scalar bulk_contraction(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Scalar(
            /* scalar */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234)
        );
    }
}
public extension Sphere {
    public AntiDipoleInversion bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            (float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.group0.zxy * right_dual.group3.yzx), this.e1234 * right_dual.e3215) - (this.group0.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * right_dual.e4235 * -1.0, this.e3215 * right_dual.e4315 * -1.0, this.e3215 * right_dual.e4125 * -1.0, (this.e4315 * right_dual.e42) + (this.e4125 * right_dual.e43) + (this.e1234 * right_dual.e45)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4((this.e4315 * right_dual.e12) + (this.e1234 * right_dual.e15), (this.e4125 * right_dual.e23) + (this.e1234 * right_dual.e25), (this.e4235 * right_dual.e31) + (this.e1234 * right_dual.e35), -(this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45)) - (this.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - (this.group0.wwwy * float4(right_dual.group0.xyz, right_dual.e25))
        );
    }
}
public extension Sphere {
    public VersorOdd bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(this.e4125 * right_dual.e431 * -1.0, this.e4235 * right_dual.e412 * -1.0, this.e4315 * right_dual.e423 * -1.0, (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) + (this.e1234 * right_dual.e5)) + (this.group0.yzxx * float4(right_dual.group0.zxy, right_dual.e1)) + float4((float3(this.e1234) * right_dual.group1.xyz), this.e4315 * right_dual.e2), 
            /* e23, e31, e12, e45 */
            float4((this.e3215 * right_dual.e423) + (this.e1234 * right_dual.e235), (this.e3215 * right_dual.e431) + (this.e1234 * right_dual.e315), (this.e3215 * right_dual.e412) + (this.e1234 * right_dual.e125), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) - (this.group0.xyzx * right_dual.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_dual.group1.xyz) + (this.group0.zxy * right_dual.group2.yzx) - (this.group0.yzx * right_dual.group2.zxy)), this.e1234 * right_dual.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0
        );
    }
}
public extension VersorEven {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<VersorEven> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<VersorEven>(this);
    }
}
public extension BulkContractionInfixPartial<VersorEven> {
    // Fancy infix trick (second half)
    public VersorEven operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension VersorEven {
    public VersorEven bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_dual.group0.xyz)), right_dual.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(right_dual.e12345) * this.group1) + (float4(this.e12345) * right_dual.group1), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e12345 * this.e235, right_dual.e12345 * this.e315, right_dual.e12345 * this.e125, -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + (right_dual.group2 * float4(this.group0.www, this.e5)), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412) + (right_dual.e12345 * this.e1), (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423) + (right_dual.e12345 * this.e2), (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431) + (right_dual.e12345 * this.e3), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) + float4((right_dual.group0.xyz.zxy * this.group2.yzx), right_dual.e12345 * this.e4) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.e423 * this.e415) - float4((right_dual.group2.zxy * this.group0.yzx), right_dual.e431 * this.e425)
        );
    }
}
public extension VersorEven {
    public VersorOdd bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            (right_dual.group3.yzxy * float4(this.group0.zxy, this.e2)) + float4(float3(0.0), (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) + float4((right_dual.group0.xyz * this.group0.www), right_dual.e1234 * this.e5) + float4((right_dual.group2.www * this.group1.xyz), right_dual.e4235 * this.e1) - float4((right_dual.group3.zxy * this.group0.yzx), right_dual.e41 * this.e235), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e1234 * this.e235) + (right_dual.e3215 * this.e423), (right_dual.e1234 * this.e315) + (right_dual.e3215 * this.e431), (right_dual.e1234 * this.e125) + (right_dual.e3215 * this.e412), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(this.e12345) * right_dual.group1) - (right_dual.group3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e3215) * this.group1.xyz) + (float3(this.e12345) * right_dual.group2.xyz) + (right_dual.group3.zxy * this.group2.yzx) - (right_dual.group3.yzx * this.group2.zxy)), right_dual.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension VersorEven {
    public VersorEven bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_dual.group0.xy.yy, right_dual.e12345, (right_dual.e5 * this.e12345) + (right_dual.e12345 * this.e5)) * float4(this.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension VersorEven {
    public AntiCircleRotor bulk_contraction(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_dual.e45 * this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group0.xyz), -(right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321))
        );
    }
}
public extension VersorEven {
    public VersorOdd bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual.group1.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e15)), 
            /* e23, e31, e12, e45 */
            (this.group0 * float4(right_dual.group1.www, right_dual.e45)) + float4(float3(0.0), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) - (right_dual.group1.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.e3215) * this.group1.xyz), 0.0) + float4((float3(this.e12345) * right_dual.group0.xyz), 0.0) + float4((right_dual.group1.zxy * this.group2.yzx), 0.0) - float4((right_dual.group1.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group1.xyz), -(right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e415 * this.e321) + (right_dual.e315 * this.e412), (right_dual.e425 * this.e321) + (right_dual.e125 * this.e423), (right_dual.e435 * this.e321) + (right_dual.e235 * this.e431), -(right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((right_dual.group1.xyz.zxy * this.group0.yzx), right_dual.e415 * this.e235)
        );
    }
}
public extension VersorEven {
    public VersorEven bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual.e12345) * this.group1.xyz) + (float3(this.e12345) * right_dual.group0.xyz)), right_dual.e12345 * this.e321), 
            /* e235, e315, e125, e5 */
            (float4(right_dual.e12345) * this.group2) + (float4(this.e12345) * right_dual.group1) + float4(float3(0.0), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e12345 * this.e1) + (right_dual.e315 * this.e412), (right_dual.e12345 * this.e2) + (right_dual.e125 * this.e423), (right_dual.e12345 * this.e3) + (right_dual.e235 * this.e431), -(right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) + (right_dual.group0 * float4(this.group1.www, this.e4)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e415))
        );
    }
}
public extension VersorEven {
    public VersorOdd bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.e4125 * this.e431 * -1.0, right_dual.e4235 * this.e412 * -1.0, right_dual.e4315 * this.e423 * -1.0, (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4)) + (right_dual.group0.yzxx * float4(this.group0.zxy, this.e1)), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e3215 * this.e423, right_dual.e3215 * this.e431, right_dual.e3215 * this.e412, -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) - (right_dual.group0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.e3215) * this.group1.xyz), 0.0) + float4((right_dual.group0.zxy * this.group2.yzx), 0.0) - float4((right_dual.group0.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0
        );
    }
}
public extension VersorEven {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension VersorEven {
    public AntiCircleRotor bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
public extension VersorEven {
    public AntiCircleRotor bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e125 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.e235 * this.e415) - float4((right_dual.group2.xyz.zxy * this.group0.yzx), right_dual.e315 * this.e425)
        );
    }
}
public extension VersorEven {
    public AntiDipoleInversion bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            float3(this.e12345) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.e4 * this.e12345) - (right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412) + (right_dual.e1 * this.e12345), (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423) + (right_dual.e2 * this.e12345), (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431) + (right_dual.e3 * this.e12345), -(right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + float4((right_dual.group0.xyz.zxy * this.group2.yzx), right_dual.e5 * this.e12345) - (this.group2.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e415)) - float4((right_dual.group2.zxy * this.group0.yzx), right_dual.e425 * this.e315)
        );
    }
}
public extension VersorEven {
    public AntiMotor bulk_contraction(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.group0.xy.xx, right_dual.e3215, (right_dual.e3215 * this.e4) + (right_dual.scalar * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
public extension VersorEven {
    public AntiFlector bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual.e315 * this.e412) + (right_dual.e321 * this.e415), (right_dual.e125 * this.e423) + (right_dual.e321 * this.e425), (right_dual.e235 * this.e431) + (right_dual.e321 * this.e435), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - (right_dual.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
public extension VersorEven {
    public AntiFlector bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual.e321 * this.e415) + (right_dual.e1 * this.e12345), (right_dual.e321 * this.e425) + (right_dual.e2 * this.e12345), (right_dual.e321 * this.e435) + (right_dual.e3 * this.e12345), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + (this.group0.zxyw * float4(right_dual.group0.yzx, right_dual.e5)) - (right_dual.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
public extension VersorEven {
    public AntiMotor bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_dual.group0.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
public extension VersorEven {
    public AntiMotor bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(right_dual.e3215) * float4(this.group0.xyz, this.e4)) + (float4(this.e12345) * right_dual.group0) + float4(float3(0.0), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual.e3215) * this.group1.xyz) + (float3(this.e12345) * right_dual.group1.xyz)), right_dual.e3215 * this.e12345)
        );
    }
}
public extension VersorEven {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((right_dual.scalar * this.e12345) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435), right_dual.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e1 * this.e12345) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e2 * this.e12345) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e3 * this.e12345) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412) - (right_dual.e412 * this.e435)) + (float4(right_dual.e12345) * this.group3) + float4((right_dual.group7.xyz.zxy * this.group2.yzx), right_dual.e4 * this.e12345) - float4((right_dual.group7.xyz.yzx * this.group2.zxy), right_dual.e423 * this.e415) - float4((right_dual.group8.xyz.zxy * this.group0.yzx), right_dual.e431 * this.e425), 
            /* e5 */
            (right_dual.e12345 * this.e5) + (right_dual.e5 * this.e12345) - (right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(this.e12345) * right_dual.group3) - (right_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual.e1234) * this.group1.xyz) + (float3(this.e12345) * right_dual.group4.xyz) + (right_dual.group9.yzx * this.group0.zxy) - (right_dual.group9.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual.e3215) * this.group0.xyz) + (float3(right_dual.e1234) * this.group2.xyz) + (float3(this.e12345) * right_dual.group5.xyz) - (float3(this.e321) * right_dual.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(right_dual.e12345) * this.group1) + (float4(this.e12345) * right_dual.group6), 
            /* e423, e431, e412 */
            (float3(right_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_dual.group7.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual.e12345) * this.group2.xyz) + (float3(this.e12345) * right_dual.group8.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group9, 
            /* e1234 */
            right_dual.e1234 * this.e12345
        );
    }
}
public extension VersorEven {
    public AntiPlane bulk_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
public extension VersorEven {
    public VersorOdd bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.e4125 * this.e431 * -1.0, right_dual.e4235 * this.e412 * -1.0, right_dual.e4315 * this.e423 * -1.0, (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5)) + (right_dual.group0.yzxx * float4(this.group0.zxy, this.e1)) + float4((float3(right_dual.e1234) * this.group1.xyz), right_dual.e4315 * this.e2), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e3215 * this.e423) + (right_dual.e1234 * this.e235), (right_dual.e3215 * this.e431) + (right_dual.e1234 * this.e315), (right_dual.e3215 * this.e412) + (right_dual.e1234 * this.e125), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) - (right_dual.group0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e3215) * this.group1.xyz) + (right_dual.group0.zxy * this.group2.yzx) - (right_dual.group0.yzx * this.group2.zxy)), right_dual.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0
        );
    }
}
public extension VersorEven {
    public VersorEven bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_dual.e12345) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension VersorEven {
    public RoundPoint bulk_contraction(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5 */
            right_dual.e5 * this.e12345
        );
    }
}
public extension VersorEven {
    public VersorOdd bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            (this.group0.zxyw * float4(right_dual.group3.yzx, right_dual.scalar)) + float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) + float4((this.group0.www * right_dual.group0.xyz), this.e5 * right_dual.e1234) + float4((right_dual.group2.www * this.group1.xyz), this.e1 * right_dual.e4235) - (this.group0.yzxx * float4(right_dual.group3.zxy, right_dual.e15)), 
            /* e23, e31, e12, e45 */
            float4((this.e12345 * right_dual.e23) + (this.e235 * right_dual.e1234), (this.e12345 * right_dual.e31) + (this.e315 * right_dual.e1234), (this.e12345 * right_dual.e12) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) + (this.group0 * float4(right_dual.group3.www, right_dual.e45)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * right_dual.group2.xyz) + (float3(right_dual.e3215) * this.group1.xyz) + (this.group2.yzx * right_dual.group3.zxy) - (this.group2.zxy * right_dual.group3.yzx)), this.e12345 * right_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
public extension VersorEven {
    public VersorEven bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            float4(((float3(right_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_dual.group0.xyz)), right_dual.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(right_dual.e12345) * this.group1) + (float4(this.e12345) * right_dual.group1), 
            /* e235, e315, e125, e5 */
            (float4(right_dual.e12345) * this.group2) + (float4(this.e12345) * right_dual.group2) + float4(float3(0.0), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412) + (right_dual.e1 * this.e12345), (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423) + (right_dual.e2 * this.e12345), (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431) + (right_dual.e3 * this.e12345), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) + (right_dual.group0.zxyw * float4(this.group2.yzx, this.e4)) + float4((right_dual.group0.www * this.group3.xyz), right_dual.e4 * this.e12345) - (right_dual.group0.yzxx * float4(this.group2.zxy, this.e415)) - float4((right_dual.group2.zxy * this.group0.yzx), right_dual.e431 * this.e425)
        );
    }
}
public extension VersorOdd {
    // Fancy infix trick (first half)
    public BulkContractionInfixPartial<VersorOdd> operator /(BulkContractionInfix rhs) {
        return BulkContractionInfixPartial<VersorOdd>(this);
    }
}
public extension BulkContractionInfixPartial<VersorOdd> {
    // Fancy infix trick (second half)
    public VersorOdd operator /(AntiCircleRotor rhs) {
        return this.a.bulk_contraction(rhs);
    }
}
public extension VersorOdd {
    public VersorOdd bulk_contraction(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual.e415 * this.e1234) + (right_dual.e12345 * this.e41), (right_dual.e425 * this.e1234) + (right_dual.e12345 * this.e42), (right_dual.e435 * this.e1234) + (right_dual.e12345 * this.e43), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) + float4((right_dual.group0.xyz.zxy * this.group3.yzx), right_dual.e12345 * this.scalar) - float4((right_dual.group0.xyz.yzx * this.group3.zxy), right_dual.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e235 * this.e1234) + (right_dual.e12345 * this.e23), (right_dual.e315 * this.e1234) + (right_dual.e12345 * this.e31), (right_dual.e125 * this.e1234) + (right_dual.e12345 * this.e12), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) + float4((right_dual.group0.xyz * this.group3.www), right_dual.e12345 * this.e45) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e12345) * this.group2.xyz) + (float3(this.e3215) * right_dual.group1.xyz) + (right_dual.group2.yzx * this.group3.zxy) - (right_dual.group2.zxy * this.group3.yzx)), right_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion bulk_contraction(AntiDipoleInversion other) {
        DipoleInversion right_dual = DipoleInversion(
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
            (float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e41)) + float4(float3(0.0), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234) - (right_dual.e4235 * this.e41) - (right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) - float4((this.group3.www * right_dual.group3.xyz), right_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_dual.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_dual.group2.xyz, right_dual.e4315)) + float4((right_dual.group1.zxy * this.group3.yzx), right_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - float4((right_dual.group3.yzx * this.group1.zxy), right_dual.e35 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public VersorOdd bulk_contraction(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.group0.xy.yy, right_dual.e12345, (right_dual.e5 * this.e1234) + (right_dual.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public RoundPoint bulk_contraction(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e5 */
            -(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215)
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion bulk_contraction(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group1.yzx * this.group3.zxy) - (right_dual.group1.zxy * this.group3.yzx)), right_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) + float4((right_dual.group1.www * this.group3.xyz), right_dual.e45 * this.e1234) - (right_dual.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (right_dual.group1.zxyy * float4(this.group1.yzx, this.e25)) + (right_dual.group1.wwwz * float4(this.group0.xyz, this.e35)) + (this.group2.wwwx * float4(right_dual.group0.xyz, right_dual.e4235)) + float4(float3(0.0), (right_dual.e3215 * this.e45) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215)) - float4((right_dual.group1.yzx * this.group1.zxy), right_dual.e15 * this.e4235)
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor bulk_contraction(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e1234) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_dual.group1.xyz), -(right_dual.e415 * this.e4235) - (right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((right_dual.group1.xyz.zxy * this.group3.yzx), right_dual.e415 * this.e23)
        );
    }
}
public extension VersorOdd {
    public VersorOdd bulk_contraction(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual.group0 * float4(this.group2.www, this.scalar)) + float4(float3(0.0), -(right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) + float4((right_dual.group0.www * this.group0.xyz), right_dual.e5 * this.e1234), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e235 * this.e1234, right_dual.e315 * this.e1234, right_dual.e125 * this.e1234, -(right_dual.e415 * this.e4235) - (right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) + (float4(right_dual.e12345) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e12345) * this.group2.xyz) + (float3(this.e3215) * right_dual.group0.xyz) + (right_dual.group1.yzx * this.group3.zxy) - (right_dual.group1.zxy * this.group3.yzx)), right_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion bulk_contraction(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx)), right_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(right_dual.e3215 * this.e4235, right_dual.e3215 * this.e4315, right_dual.e3215 * this.e4125, -(right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e4315 * this.e12 * -1.0, right_dual.e4125 * this.e23 * -1.0, right_dual.e4235 * this.e31 * -1.0, (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45)) + (right_dual.group0.zxyx * float4(this.group1.yzx, this.e15)) + (right_dual.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
public extension VersorOdd {
    public RoundPoint bulk_contraction(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group3.yzxy * float4(right_dual.group1.zxy, right_dual.e42)) + float4((right_dual.group2.xyz * this.group2.www), right_dual.e41 * this.e4235), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public RoundPoint bulk_contraction(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e41)) + float4((this.group2.www * right_dual.group2.xyz), right_dual.e42 * this.e4315), 
            /* e5 */
            -(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor bulk_contraction(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group3.yzx) - (right_dual.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((right_dual.group2.xyz.zxy * this.group3.yzx), right_dual.e423 * this.e15)
        );
    }
}
public extension VersorOdd {
    public AntiCircleRotor bulk_contraction(DipoleInversion other) {
        AntiDipoleInversion right_dual = AntiDipoleInversion(
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
            (float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group3.yzx) - (right_dual.group0.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (this.group3.zxyx * float4(right_dual.group2.yzx, right_dual.e1)) + float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((right_dual.group2.zxy * this.group3.yzx), right_dual.e423 * this.e15)
        );
    }
}
public extension VersorOdd {
    public AntiFlector bulk_contraction(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
public extension VersorOdd {
    public AntiMotor bulk_contraction(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.e235 * this.e1234, right_dual.e315 * this.e1234, right_dual.e125 * this.e1234, -(right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension VersorOdd {
    public AntiMotor bulk_contraction(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) + float4((this.group2.www * right_dual.group0.xyz), right_dual.e1 * this.e4235) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
public extension VersorOdd {
    public AntiPlane bulk_contraction(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group3.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
public extension VersorOdd {
    public AntiFlector bulk_contraction(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e15 * this.e1234) + (right_dual.e3215 * this.e41), (right_dual.e25 * this.e1234) + (right_dual.e3215 * this.e42), (right_dual.e35 * this.e1234) + (right_dual.e3215 * this.e43), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) + float4((right_dual.group0.zxy * this.group3.yzx), right_dual.e3215 * this.e45) - (this.group3.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
public extension VersorOdd {
    public MultiVector bulk_contraction(MultiVector other) {
        MultiVector right_dual = MultiVector(
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
            float2((right_dual.e12345 * this.scalar) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            (this.group3.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)) + float4((right_dual.group9.zxy * this.group1.yzx), right_dual.e43 * this.e4125) + float4((right_dual.group9.www * this.group0.xyz), right_dual.e45 * this.e1234) + float4((this.group2.www * right_dual.group3.xyz), right_dual.e42 * this.e4315) - (float4(right_dual.e1234) * float4(this.group2.xyz, this.e45)) - (right_dual.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((right_dual.group4.xyz * this.group3.www), right_dual.e4235 * this.e41) - float4((right_dual.group5.xyz.yzx * this.group3.zxy), right_dual.e4315 * this.e42), 
            /* e5 */
            (right_dual.e4235 * this.e15) + (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) + (float4(right_dual.e12345) * float4(this.group2.xyz, this.e45)) - (this.group3.yzxx * float4(right_dual.group8.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual.e12345) * this.group0.xyz) + (float3(this.e1234) * right_dual.group6.xyz) + (right_dual.group7.xyz.zxy * this.group3.yzx) - (right_dual.group7.xyz.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual.e12345) * this.group1.xyz) + (float3(this.e1234) * right_dual.group8.xyz) + (float3(this.e3215) * right_dual.group7.xyz) - (float3(right_dual.e321) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group9.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_dual.group9.zxy * this.group3.yzx), right_dual.e1234 * this.e3215), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual.group9.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * this.group3.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3, 
            /* e1234 */
            right_dual.e12345 * this.e1234
        );
    }
}
public extension VersorOdd {
    public Scalar bulk_contraction(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion bulk_contraction(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group0.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_dual.group0.zxy * this.group3.yzx), right_dual.e1234 * this.e3215), 
            /* e235, e315, e125, e4 */
            float4(right_dual.e3215 * this.e4235, right_dual.e3215 * this.e4315, right_dual.e3215 * this.e4125, -(right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43) - (right_dual.e1234 * this.e45)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(right_dual.e4315 * this.e12) - (right_dual.e1234 * this.e15), -(right_dual.e4125 * this.e23) - (right_dual.e1234 * this.e25), -(right_dual.e4235 * this.e31) - (right_dual.e1234 * this.e35), (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45)) + (right_dual.group0.zxyx * float4(this.group1.yzx, this.e15)) + (right_dual.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
public extension VersorOdd {
    public VersorOdd bulk_contraction(Scalar other) {
        AntiScalar right_dual = AntiScalar(
            /* e12345 */
            other.scalar
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.e12345) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
public extension VersorOdd {
    public Scalar bulk_contraction(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Scalar(
            /* scalar */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234)
        );
    }
}
public extension VersorOdd {
    public AntiDipoleInversion bulk_contraction(VersorEven other) {
        VersorOdd right_dual = VersorOdd(
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
            (float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e41)) + float4(float3(0.0), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234) - (right_dual.e4235 * this.e41) - (right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) - float4((this.group3.www * right_dual.group3.xyz), right_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_dual.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_dual.group2.xyz, right_dual.e4315)) + float4((right_dual.group1.zxy * this.group3.yzx), right_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - float4((right_dual.group3.yzx * this.group1.zxy), right_dual.e35 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public VersorOdd bulk_contraction(VersorOdd other) {
        VersorEven right_dual = VersorEven(
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
            (right_dual.group0.zxyw * float4(this.group3.yzx, this.scalar)) + float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) + float4((right_dual.group0.www * this.group0.xyz), right_dual.e5 * this.e1234) + float4((this.group2.www * right_dual.group1.xyz), right_dual.e1 * this.e4235) - (right_dual.group0.yzxx * float4(this.group3.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e12345 * this.e23) + (right_dual.e235 * this.e1234), (right_dual.e12345 * this.e31) + (right_dual.e315 * this.e1234), (right_dual.e12345 * this.e12) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) + (right_dual.group0 * float4(this.group3.www, this.e45)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e12345) * this.group2.xyz) + (float3(this.e3215) * right_dual.group1.xyz) + (right_dual.group2.yzx * this.group3.zxy) - (right_dual.group2.zxy * this.group3.yzx)), right_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3
        );
    }
}

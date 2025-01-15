implementing cga3d;
using traits;
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor bulk_expansion(AntiCircleRotor other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * right_dual.group2.xyz), (this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion bulk_expansion(AntiDipoleInversion other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.www * right_dual.group2.xyz), (this.scalar * right_dual.e1234) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42) + (this.scalar * right_dual.e4235), (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43) + (this.scalar * right_dual.e4315), (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41) + (this.scalar * right_dual.e4125), -(this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + float4((this.group0.xyz.yzx * right_dual.group2.zxy), this.scalar * right_dual.e3215) - (right_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((right_dual.group0.xyz.zxy * this.group2.yzx), this.e31 * right_dual.e25)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Motor bulk_expansion(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.group0.xy.xx, right_dual.e5, right_dual.e12345) * float4(this.group0.xyz, this.scalar), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Flector bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Flector bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.scalar * right_dual.e4235), (this.e31 * right_dual.e45) + (this.scalar * right_dual.e4315), (this.e12 * right_dual.e45) + (this.scalar * right_dual.e4125), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) + float4((this.group0.xyz.yzx * right_dual.group0.zxy), this.scalar * right_dual.e3215) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Motor bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_dual.group0.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Motor bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar * right_dual.e415, this.scalar * right_dual.e425, this.scalar * right_dual.e435, -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)) + float4((this.group0.xyz * right_dual.group1.www), this.scalar * right_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_dual.group1.xyz) + (float3(right_dual.e5) * this.group1.xyz)), this.scalar * right_dual.e5)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Plane bulk_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_dual.scalar) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41), -(this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group0.xyz.zxy * right_dual.group2.xyz.yzx), this.e23 * right_dual.e15) - float4((right_dual.group0.xyz.zxy * this.group2.yzx), this.e31 * right_dual.e25)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_dual.group0.xyz)), right_dual.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(right_dual.scalar) * this.group1) + (float4(this.scalar) * right_dual.group1), 
            /* e15, e25, e35, e1234 */
            float4((right_dual.e15 * this.scalar) + (right_dual.scalar * this.e15), (right_dual.e25 * this.scalar) + (right_dual.scalar * this.e25), (right_dual.e35 * this.scalar) + (right_dual.scalar * this.e35), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - (this.group2.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23)) - float4((this.group0.xyz.zxy * right_dual.group2.yzx), right_dual.e31 * this.e25)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorEven bulk_expansion(DipoleInversion other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.e42 * right_dual.e3) + (this.e23 * right_dual.e4) + (this.scalar * right_dual.e423), (this.e43 * right_dual.e1) + (this.e31 * right_dual.e4) + (this.scalar * right_dual.e431), (this.e41 * right_dual.e2) + (this.e12 * right_dual.e4) + (this.scalar * right_dual.e412), -(this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) - float4((this.group0.xyz.zxy * right_dual.group3.yzx), this.e41 * right_dual.e235), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_dual.e4) + (this.scalar * right_dual.e415), (this.e25 * right_dual.e4) + (this.scalar * right_dual.e425), (this.e35 * right_dual.e4) + (this.scalar * right_dual.e435), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + float4((this.group0.xyz * right_dual.group3.www), this.scalar * right_dual.e321) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_dual.group2.xyz) + (float3(right_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_dual.group3.yzx) - (this.group2.yzx * right_dual.group3.zxy)), this.scalar * right_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_dual.group3.xyz, right_dual.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.scalar) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.group0.xy.yy, right_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group2.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.scalar * right_dual.e3215)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor bulk_expansion(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.scalar * right_dual.e321), 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * right_dual.group0.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e45 * right_dual.e321))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorEven bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e42 * right_dual.e3, this.e43 * right_dual.e1, this.e41 * right_dual.e2, -(this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e45 * right_dual.e321)) - float4((this.group0.xyz.zxy * right_dual.group1.yzx), this.e41 * right_dual.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + float4((this.group0.xyz * right_dual.group1.www), this.scalar * right_dual.e321) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_dual.group0.xyz) + (float3(right_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_dual.group1.yzx) - (this.group2.yzx * right_dual.group1.zxy)), this.scalar * right_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group2.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group1.xyz), -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e45 * right_dual.e23), (this.e43 * right_dual.e15) + (this.e45 * right_dual.e31), (this.e41 * right_dual.e25) + (this.e45 * right_dual.e12), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group0.xyz.zxy * right_dual.group1.xyz.yzx), this.e15 * right_dual.e23)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.scalar) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(this.scalar) * right_dual.group0.xyz) + (float3(right_dual.scalar) * this.group1.xyz)), this.e45 * right_dual.scalar), 
            /* e15, e25, e35, e1234 */
            float4((this.e15 * right_dual.scalar) + (this.scalar * right_dual.e15), (this.e25 * right_dual.scalar) + (this.scalar * right_dual.e25), (this.e35 * right_dual.scalar) + (this.scalar * right_dual.e35), -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e45 * right_dual.e23, this.e45 * right_dual.e31, this.e45 * right_dual.e12, -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + (right_dual.group1.zxyw * float4(this.group0.xyz.yzx, this.scalar)) - (right_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(this.scalar * right_dual.scalar, (this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e5 */
            this.scalar * right_dual.e5, 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * right_dual.group3) + (float4(right_dual.scalar) * float4(this.group2.xyz, this.e45)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * right_dual.group4.xyz) + (float3(right_dual.scalar) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_dual.e4) + (this.scalar * right_dual.e415), (this.e25 * right_dual.e4) + (this.scalar * right_dual.e425), (this.e35 * right_dual.e4) + (this.scalar * right_dual.e435), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + float4((float3(right_dual.e5) * this.group0.xyz), this.scalar * right_dual.e321) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.scalar) * right_dual.group7.xyz) + (float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group1.zxy) - (this.group0.xyz.zxy * right_dual.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.scalar) * right_dual.group8.xyz) + (float3(right_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_dual.group1.yzx) - (this.group2.yzx * right_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42) + (this.scalar * right_dual.e4235), (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43) + (this.scalar * right_dual.e4315), (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41) + (this.scalar * right_dual.e4125), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e35 * right_dual.e12)) + float4((this.group0.xyz.yzx * right_dual.group3.zxy), this.scalar * right_dual.e3215) - (this.group2.yzxy * float4(right_dual.group4.xyz.zxy, right_dual.e31)) - float4((this.group0.xyz.zxy * right_dual.group3.yzx), this.e15 * right_dual.e23), 
            /* e1234 */
            (this.scalar * right_dual.e1234) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiDipoleInversion bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * right_dual.e5, this.e42 * right_dual.e5, this.e43 * right_dual.e5, -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * right_dual.group0.yzx), 0.0) - float4((this.group2.yzx * right_dual.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Sphere bulk_expansion(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group0, 
            /* e1234 */
            this.scalar * right_dual.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiDipoleInversion bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_dual.group0.yzx) - (this.group2.yzx * right_dual.group0.zxy)), this.scalar * right_dual.e4), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(right_dual.group0.xyz, right_dual.e5)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd bulk_expansion(VersorEven other) {
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
            float4(((float3(this.scalar) * right_dual.group0.xyz) + (float3(right_dual.scalar) * this.group0.xyz)), this.scalar * right_dual.scalar), 
            /* e23, e31, e12, e45 */
            (float4(this.scalar) * right_dual.group1) + (float4(right_dual.scalar) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar * right_dual.e15, this.scalar * right_dual.e25, this.scalar * right_dual.e35, -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)) + (this.group2 * float4(right_dual.group0.www, right_dual.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42) + (this.scalar * right_dual.e4235), (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43) + (this.scalar * right_dual.e4315), (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41) + (this.scalar * right_dual.e4125), -(this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + float4((this.group0.xyz.yzx * right_dual.group2.zxy), this.scalar * right_dual.e3215) - (right_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((this.group2.yzx * right_dual.group0.zxy), this.e31 * right_dual.e25)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorEven bulk_expansion(VersorOdd other) {
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
            float4((this.e23 * right_dual.e4) + (this.scalar * right_dual.e423), (this.e31 * right_dual.e4) + (this.scalar * right_dual.e431), (this.e12 * right_dual.e4) + (this.scalar * right_dual.e412), -(this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) + float4((this.group0.xyz.yzx * right_dual.group3.zxy), this.scalar * right_dual.e12345) - float4((this.group0.xyz.zxy * right_dual.group3.yzx), this.e41 * right_dual.e235), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_dual.e4) + (this.scalar * right_dual.e415), (this.e25 * right_dual.e4) + (this.scalar * right_dual.e425), (this.e35 * right_dual.e4) + (this.scalar * right_dual.e435), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + float4((this.group0.xyz * right_dual.group2.www), this.scalar * right_dual.e321) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_dual.group2.xyz) + (float3(right_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_dual.group3.yzx) - (this.group2.yzx * right_dual.group3.zxy)), this.scalar * right_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group3
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Sphere bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_dual.e235) + (this.e3 * right_dual.e425), (this.e4 * right_dual.e315) + (this.e1 * right_dual.e435), (this.e4 * right_dual.e125) + (this.e2 * right_dual.e415), -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group3.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((right_dual.group0.xyz * this.group3.www), this.e1 * right_dual.e235), 
            /* e1234 */
            (this.e4 * right_dual.e321) + (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor bulk_expansion(AntiDipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group3.zxy) - (right_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_dual.e15) + (this.e5 * right_dual.e41), (this.e4 * right_dual.e25) + (this.e5 * right_dual.e42), (this.e4 * right_dual.e35) + (this.e5 * right_dual.e43), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group3.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group3.wwwx * float4(right_dual.group1.xyz, right_dual.e4235)) + float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) + float4((this.group3.yzx * right_dual.group2.zxy), this.e4 * right_dual.e3215) - (right_dual.group2.yzxx * float4(this.group3.zxy, this.e423))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Flector bulk_expansion(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Motor bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e4 * right_dual.e15, this.e4 * right_dual.e25, this.e4 * right_dual.e35, -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * right_dual.group0.zxy) - (this.group3.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Motor bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e4) * float4(right_dual.group0.xyz, right_dual.e3215)) + float4(float3(0.0), (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * right_dual.group0.zxy) - (this.group3.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Plane bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_dual.e235) + (this.e3 * right_dual.e425), (this.e4 * right_dual.e315) + (this.e1 * right_dual.e435), (this.e4 * right_dual.e125) + (this.e2 * right_dual.e415), -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) - (this.group3.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Flector bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_dual.e235) + (this.e3 * right_dual.e425), (this.e4 * right_dual.e315) + (this.e1 * right_dual.e435), (this.e4 * right_dual.e125) + (this.e2 * right_dual.e415), -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) - (this.group3.yzxx * float4(right_dual.group0.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar bulk_expansion(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            (this.e4 * right_dual.e3215) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_dual.scalar) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group3.zxy) - (right_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_dual.e15) + (this.e5 * right_dual.e41), (this.e4 * right_dual.e25) + (this.e5 * right_dual.e42), (this.e4 * right_dual.e35) + (this.e5 * right_dual.e43), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group3.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_dual.e35) + (this.e5 * right_dual.e23), (this.e3 * right_dual.e15) + (this.e5 * right_dual.e31), (this.e1 * right_dual.e25) + (this.e5 * right_dual.e12), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((right_dual.group2.xyz.yzx * this.group3.zxy), this.e423 * right_dual.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((right_dual.e42 * this.e3) + (right_dual.e23 * this.e4) + (right_dual.scalar * this.e423), (right_dual.e43 * this.e1) + (right_dual.e31 * this.e4) + (right_dual.scalar * this.e431), (right_dual.e41 * this.e2) + (right_dual.e12 * this.e4) + (right_dual.scalar * this.e412), -(right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) - float4((right_dual.group0.xyz.zxy * this.group3.yzx), right_dual.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e15 * this.e4) + (right_dual.scalar * this.e415), (right_dual.e25 * this.e4) + (right_dual.scalar * this.e425), (right_dual.e35 * this.e4) + (right_dual.scalar * this.e435), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) + float4((right_dual.group0.xyz * this.group3.www), right_dual.scalar * this.e321) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.zxy * this.group3.yzx) - (right_dual.group2.yzx * this.group3.zxy)), right_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Sphere bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_dual.e235) + (this.e3 * right_dual.e425), (this.e4 * right_dual.e315) + (this.e1 * right_dual.e435), (this.e4 * right_dual.e125) + (this.e2 * right_dual.e415), -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group3.wwwx * float4(right_dual.group0.xyz, right_dual.e235)) - float4((this.group3.yzx * right_dual.group1.zxy), this.e2 * right_dual.e315), 
            /* e1234 */
            (this.e4 * right_dual.e321) + (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bulk_expansion(DipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (right_dual.group3.zxyw * float4(this.group3.yzx, this.e4)) - (this.group3.zxyw * float4(right_dual.group3.yzx, right_dual.e4)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e423)) + float4(float3(0.0), (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e4 * this.e321) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)) - float4((this.group3.www * right_dual.group3.xyz), right_dual.e1 * this.e423), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group3.yzxw * this.group1.zxyw) + (this.group2.wwwz * float4(right_dual.group2.xyz, right_dual.e3)) + float4((this.group0.xyz * right_dual.group3.www), right_dual.e1 * this.e235) + float4((right_dual.group1.yzx * this.group3.zxy), right_dual.e2 * this.e315) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e235)) - float4((right_dual.group3.zxy * this.group1.yzx), right_dual.e125 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.group0.xy.yy, right_dual.scalar, right_dual.e3215) * float4(this.group0.xyz, this.e4), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_dual.scalar) * float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Sphere bulk_expansion(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group2.www * right_dual.group0.xyz), -(this.e1 * right_dual.e235) - (this.e2 * right_dual.e315) - (this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)), 
            /* e1234 */
            this.e4 * right_dual.e321
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_dual.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * right_dual.group1.zxy) - (this.group3.zxy * right_dual.group1.yzx)), this.e4 * right_dual.e5), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(this.e431 * right_dual.e2) - (this.e412 * right_dual.e3)) + float4((right_dual.group1.www * this.group3.xyz), this.e4 * right_dual.e321) - (right_dual.group1.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(right_dual.group0.xyz, right_dual.e2)) + (right_dual.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * right_dual.e3) - (this.e2 * right_dual.e315) - (this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - float4((this.group1.yzx * right_dual.group1.zxy), this.e1 * right_dual.e235)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), -(this.e1 * right_dual.e23) - (this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_dual.e35) + (this.e5 * right_dual.e23), (this.e3 * right_dual.e15) + (this.e5 * right_dual.e31), (this.e1 * right_dual.e25) + (this.e5 * right_dual.e12), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12)) - float4((right_dual.group1.xyz.yzx * this.group3.zxy), this.e423 * right_dual.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e4 * right_dual.e23, this.e4 * right_dual.e31, this.e4 * right_dual.e12, -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12)) + float4((this.group0.xyz * right_dual.group0.www), this.e4 * right_dual.e3215), 
            /* e415, e425, e435, e321 */
            float4(this.e4 * right_dual.e15, this.e4 * right_dual.e25, this.e4 * right_dual.e35, -(this.e1 * right_dual.e23) - (this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) + (float4(right_dual.scalar) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_dual.group0.xyz) + (float3(right_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_dual.group1.zxy) - (this.group3.zxy * right_dual.group1.yzx)), this.e5 * right_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (this.e4 * right_dual.e3215) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * float4(this.group3.xyz, this.e4), 
            /* e5 */
            this.e5 * right_dual.scalar, 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * float4(this.group3.xyz, this.e4)) - (float4(this.e5) * right_dual.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12 */
            (this.group3.yzx * right_dual.group1.zxy) - (this.group3.zxy * right_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_dual.e15) + (this.e5 * right_dual.e41), (this.e4 * right_dual.e25) + (this.e5 * right_dual.e42), (this.e4 * right_dual.e35) + (this.e5 * right_dual.e43), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) + (float4(right_dual.scalar) * this.group1) - float4((right_dual.group3.www * this.group3.xyz), this.e1 * right_dual.e23), 
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group0.xyz) + (right_dual.group4.xyz.yzx * this.group3.zxy) - (right_dual.group4.xyz.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_dual.group3.zxy) - (this.group3.zxy * right_dual.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_dual.group8.xyz * this.group2.www), this.e235 * right_dual.e1) + float4((this.group3.zxy * right_dual.group6.yzx), this.e125 * right_dual.e3) - (this.group3.yzxw * right_dual.group6.zxyw) - (this.group3.wwwx * float4(right_dual.group7.xyz, right_dual.e235)) - float4((this.group1.yzx * right_dual.group1.zxy), this.e2 * right_dual.e315) - float4((right_dual.group1.www * this.group2.xyz), this.e3 * right_dual.e125), 
            /* e1234 */
            (this.e4 * right_dual.e321) + (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) - (this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * right_dual.group0.zxy) - (this.group3.zxy * right_dual.group0.yzx)), this.e4 * right_dual.e5), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * right_dual.e5, this.e2 * right_dual.e5, this.e3 * right_dual.e5, -(this.e431 * right_dual.e2) - (this.e412 * right_dual.e3)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * right_dual.e3 * -1.0, this.e435 * right_dual.e1 * -1.0, this.e415 * right_dual.e2 * -1.0, (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar bulk_expansion(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiScalar(
            /* e12345 */
            (this.e4 * right_dual.e3215) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * right_dual.group0.zxy), this.e4 * right_dual.e5) - (this.group3.zxyw * right_dual.group0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * right_dual.e5, this.e2 * right_dual.e5, this.e3 * right_dual.e5, -(this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bulk_expansion(VersorEven other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group3.zxyy * float4(right_dual.group0.yzx, right_dual.e4315)) + float4(float3(0.0), (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) + float4((this.group0.xyz * right_dual.group0.www), this.e4 * right_dual.e3215) + float4((this.group2.www * right_dual.group1.xyz), this.e1 * right_dual.e4235) - float4((this.group3.yzx * right_dual.group0.zxy), this.e423 * right_dual.e15), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_dual.e15) + (this.e5 * right_dual.e41), (this.e4 * right_dual.e25) + (this.e5 * right_dual.e42), (this.e4 * right_dual.e35) + (this.e5 * right_dual.e43), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) + (float4(right_dual.scalar) * this.group1) - (this.group3.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_dual.group2.zxy) - (this.group3.zxy * right_dual.group2.yzx)), this.e5 * right_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bulk_expansion(VersorOdd other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * right_dual.group3.zxy), this.e4 * right_dual.e5) - (this.group3.zxyw * right_dual.group3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)) + float4((right_dual.group2.www * this.group3.xyz), this.e4 * right_dual.e321) - (right_dual.group3.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(right_dual.group2.xyz, right_dual.e2)) + (right_dual.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4((this.group3.zxy * right_dual.group1.yzx), this.e125 * right_dual.e3) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group3.yzxz * float4(right_dual.group1.zxy, right_dual.e125)) - float4((this.group1.yzx * right_dual.group3.zxy), this.e1 * right_dual.e235) - float4((right_dual.group3.www * this.group2.xyz), this.e2 * right_dual.e315)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public CircleRotor bulk_expansion(AntiCircleRotor other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * right_dual.group2
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public DipoleInversion bulk_expansion(AntiDipoleInversion other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group3
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public DualNum bulk_expansion(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public FlatPoint bulk_expansion(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Flector bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Line bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_dual.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Motor bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * right_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Plane bulk_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiDualNum bulk_expansion(AntiScalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Dipole bulk_expansion(Circle other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_dual.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public VersorOdd bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(right_dual.group0.xyz, right_dual.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xy.yy, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group2.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual.scalar * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Circle bulk_expansion(Dipole other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_dual.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public VersorEven bulk_expansion(DipoleInversion other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xy.yy, this.scalar, this.e3215) * float4(right_dual.group0.xyz, right_dual.e4), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * float4(right_dual.group2.xyz, right_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_dual.group3.xyz, right_dual.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiDualNum bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2((right_dual.e3215 * this.scalar) + (right_dual.scalar * this.e3215), right_dual.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiFlatPoint bulk_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiFlector bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiLine bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_dual.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiMotor bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_dual.scalar) + (this.scalar * right_dual.e3215)) * float4(right_dual.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(this.scalar * right_dual.scalar, (this.e3215 * right_dual.e4) + (this.scalar * right_dual.e12345)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e5 */
            this.scalar * right_dual.e5, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group3, 
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group6, 
            /* e423, e431, e412 */
            float3(this.scalar) * right_dual.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_dual.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_dual.scalar) + (this.scalar * right_dual.e3215)) * float4(right_dual.group9.xyz, 1.0), 
            /* e1234 */
            this.scalar * right_dual.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiPlane bulk_expansion(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Sphere bulk_expansion(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group0, 
            /* e1234 */
            this.scalar * right_dual.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public VersorEven bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e3215 * right_dual.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.scalar * right_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public VersorOdd bulk_expansion(VersorEven other) {
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
            float4(this.scalar) * right_dual.group0, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_dual.scalar) + (this.scalar * right_dual.e3215)) * float4(right_dual.group3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public VersorEven bulk_expansion(VersorOdd other) {
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
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_dual.e4) + (this.scalar * right_dual.e12345)) * float4(right_dual.group0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_dual.group2, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group3
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar bulk_expansion(AntiDipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            -(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum bulk_expansion(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e3215 * this.e321, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar bulk_expansion(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar bulk_expansion(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum bulk_expansion(AntiMotor other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e321 * other.e3215, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint bulk_expansion(AntiScalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
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
            float4(float3(0.0), right_dual.scalar * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.group2.www * this.group0.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e45 * this.e321))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public Sphere bulk_expansion(DipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.group2.www * this.group0.xyz * float3(-1.0)), (right_dual.e1 * this.e235) + (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)), 
            /* e1234 */
            right_dual.e4 * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint bulk_expansion(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint bulk_expansion(Motor other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, -(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)), 
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
            float4(float3(0.0), this.e321 * right_dual.scalar), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5)), 
            /* e1234 */
            this.e321 * right_dual.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public Sphere bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5)), 
            /* e1234 */
            this.e321 * right_dual.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor bulk_expansion(VersorEven other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_dual.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(right_dual.scalar) * this.group0.xyz), -(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public Sphere bulk_expansion(VersorOdd other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5)), 
            /* e1234 */
            this.e321 * right_dual.e4 * -1.0
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiFlector> operator /(AntiFlector lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Sphere bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group1.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((right_dual.group0.xyz * this.group1.www), this.e1 * right_dual.e235), 
            /* e1234 */
            (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public CircleRotor bulk_expansion(AntiDipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (right_dual.group0.xyz.yzx * this.group1.zxy) - (right_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_dual.e41, this.e5 * right_dual.e42, this.e5 * right_dual.e43, -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group1.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group1.yzxx * float4(right_dual.group2.zxy, right_dual.e4235)) + (this.group1.wwwy * float4(right_dual.group1.xyz, right_dual.e4315)) + float4(float3(0.0), (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) - float4((this.group1.zxy * right_dual.group2.yzx), this.e235 * right_dual.e41)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Flector bulk_expansion(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.xx, right_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e321 * right_dual.e5)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Motor bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e45) * float4(this.group1.xyz, this.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * right_dual.group0.zxy) - (this.group1.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Motor bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.e45) * this.group1.xyz * float3(-1.0)), (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) - (this.e321 * right_dual.e45)), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * right_dual.group0.zxy) - (this.group1.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Plane bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) - (this.group1.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Flector bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) + float4((this.group1.zxy * right_dual.group0.yzx), this.e321 * right_dual.e5) - (this.group1.yzxx * float4(right_dual.group0.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiScalar bulk_expansion(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlector bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public CircleRotor bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (right_dual.group0.xyz.yzx * this.group1.zxy) - (right_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_dual.e41, this.e5 * right_dual.e42, this.e5 * right_dual.e43, -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group1.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_dual.e35) + (this.e5 * right_dual.e23), (this.e3 * right_dual.e15) + (this.e5 * right_dual.e31), (this.e1 * right_dual.e25) + (this.e5 * right_dual.e12), -(this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) - float4((right_dual.group2.xyz.yzx * this.group1.zxy), this.e235 * right_dual.e41)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public VersorEven bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.e42 * this.e3, right_dual.e43 * this.e1, right_dual.e41 * this.e2, -(right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e45 * this.e321)) - float4((right_dual.group0.xyz.zxy * this.group1.yzx), right_dual.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) + float4((right_dual.group0.xyz * this.group1.www), right_dual.scalar * this.e321) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.scalar) * this.group0.xyz) + (float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.zxy * this.group1.yzx) - (right_dual.group2.yzx * this.group1.zxy)), right_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(right_dual.group2.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Sphere bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group1.wwwx * float4(right_dual.group0.xyz, right_dual.e235)) - float4((this.group1.yzx * right_dual.group1.zxy), this.e2 * right_dual.e315), 
            /* e1234 */
            (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public DipoleInversion bulk_expansion(DipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group3.zxy * this.group1.yzx) - (right_dual.group3.yzx * this.group1.zxy)), right_dual.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(right_dual.group3.www, right_dual.e423)) + float4(float3(0.0), (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3)) - float4((this.group1.www * right_dual.group3.xyz), right_dual.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321) - (right_dual.e125 * this.e3)) + float4((right_dual.group1.yzx * this.group1.zxy), right_dual.e1 * this.e235) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (right_dual.group2.wwwy * float4(this.group0.xyz, this.e2)) - (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlector bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiDualNum bulk_expansion(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiMotor bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((right_dual.group1.zxy * this.group1.yzx) - (right_dual.group1.yzx * this.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (right_dual.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)) - (this.group1.wwwx * float4(right_dual.group1.xyz, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e2 * right_dual.e35) + (this.e5 * right_dual.e23), (this.e3 * right_dual.e15) + (this.e5 * right_dual.e31), (this.e1 * right_dual.e25) + (this.e5 * right_dual.e12), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group1.zxyx * float4(right_dual.group1.xyz.yzx, right_dual.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlector bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.e2 * right_dual.e35) + (this.e5 * right_dual.e23), (this.e3 * right_dual.e15) + (this.e5 * right_dual.e31), (this.e1 * right_dual.e25) + (this.e5 * right_dual.e12), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) + (float4(right_dual.scalar) * this.group0) - (this.group1.zxyx * float4(right_dual.group1.yzx, right_dual.e23)), 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.group0.xy.xx, right_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            this.e5 * right_dual.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_dual.group1.xyz)), this.e5 * right_dual.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(right_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (this.group1.yzx * right_dual.group1.zxy) - (this.group1.zxy * right_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) + float4((right_dual.group4.xyz * this.group1.www), this.e321 * right_dual.scalar) - float4((right_dual.group3.www * this.group1.xyz), this.e1 * right_dual.e23), 
            /* e423, e431, e412 */
            (right_dual.group4.xyz.yzx * this.group1.zxy) - (right_dual.group4.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group0.xyz) + (this.group1.yzx * right_dual.group3.zxy) - (this.group1.zxy * right_dual.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5) - (this.e5 * right_dual.e321)) + float4((this.group1.zxy * right_dual.group6.yzx), this.e235 * right_dual.e1) - (this.group1.wwwx * float4(right_dual.group7.xyz, right_dual.e235)) - float4((this.group1.yzx * right_dual.group6.zxy), this.e3 * right_dual.e125) - float4((right_dual.group1.www * this.group0.xyz), this.e2 * right_dual.e315), 
            /* e1234 */
            (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) - (this.e321 * right_dual.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiMotor bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((this.group1.yzx * right_dual.group0.zxy) - (this.group1.zxy * right_dual.group0.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e5 * right_dual.e1 * -1.0, this.e5 * right_dual.e2 * -1.0, this.e5 * right_dual.e3 * -1.0, (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5)) + (right_dual.group0.wwwx * float4(this.group1.xyz, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiScalar bulk_expansion(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiScalar(
            /* e12345 */
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public DipoleInversion bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * right_dual.group0.zxy) - (this.group1.zxy * right_dual.group0.yzx)), this.e5 * right_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_dual.group0.xyz)), this.e321 * right_dual.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public VersorEven bulk_expansion(VersorEven other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group1.zxyx * float4(right_dual.group0.yzx, right_dual.e4235)) + float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) - (right_dual.group0.zxyx * float4(this.group1.yzx, this.e235)), 
            /* e415, e425, e435, e321 */
            (right_dual.group0 * float4(this.group1.www, this.e321)) + float4(float3(0.0), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group1.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group0.xyz) + (this.group1.yzx * right_dual.group2.zxy) - (this.group1.zxy * right_dual.group2.yzx)), this.e5 * right_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public DipoleInversion bulk_expansion(VersorOdd other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * right_dual.group3.zxy) - (this.group1.zxy * right_dual.group3.yzx)), this.e5 * right_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(right_dual.group2.www, right_dual.e423)) + float4(float3(0.0), (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412)) - (right_dual.group3 * float4(this.group1.www, this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5) - (this.e5 * right_dual.e321)) + float4((this.group1.zxy * right_dual.group1.yzx), this.e235 * right_dual.e1) - (this.group1.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - (this.group1.wwwz * float4(right_dual.group0.xyz, right_dual.e125)) - float4((right_dual.group3.www * this.group0.xyz), this.e1 * right_dual.e235)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiLine> operator /(AntiLine lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiScalar bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Sphere bulk_expansion(AntiDipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e35 * right_dual.e42), (this.e31 * right_dual.e45) + (this.e15 * right_dual.e43), (this.e12 * right_dual.e45) + (this.e25 * right_dual.e41), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group1.xyz.yzx * right_dual.group0.xyz.zxy), this.e23 * right_dual.e15), 
            /* e1234 */
            -(this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiFlatPoint bulk_expansion(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(right_dual.group0.xy.xx, right_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Plane bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e45) * this.group0.xyz), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Plane bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e45) * this.group0.xyz), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiScalar bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Motor bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiLine bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_dual.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Sphere bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e35 * right_dual.e42), (this.e31 * right_dual.e45) + (this.e15 * right_dual.e43), (this.e12 * right_dual.e45) + (this.e25 * right_dual.e41), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group1.xyz.yzx * right_dual.group0.xyz.zxy), this.e23 * right_dual.e15), 
            /* e1234 */
            -(this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public DipoleInversion bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
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
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group1.xyz), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e45 * this.e23), (right_dual.e43 * this.e15) + (right_dual.e45 * this.e31), (right_dual.e41 * this.e25) + (right_dual.e45 * this.e12), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group1.xyz.yzx), right_dual.e15 * this.e23)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiScalar bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public CircleRotor bulk_expansion(DipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_dual.e4) * this.group1.xyz), -(right_dual.e1 * this.e23) - (right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.e2 * this.e35) + (right_dual.e5 * this.e23), (right_dual.e3 * this.e15) + (right_dual.e5 * this.e31), (right_dual.e1 * this.e25) + (right_dual.e5 * this.e12), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12)) - float4((this.group1.xyz.yzx * right_dual.group3.zxy), right_dual.e423 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkExpansionInfixPartial<AntiLine> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiLine bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_dual.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiFlatPoint bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((right_dual.e2 * this.e35) + (right_dual.e5 * this.e23), (right_dual.e3 * this.e15) + (right_dual.e5 * this.e31), (right_dual.e1 * this.e25) + (right_dual.e5 * this.e12), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiDualNum bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiMotor bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(right_dual.scalar) * this.group1.xyz), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.xx, right_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_dual.e4) * this.group1.xyz), -(this.e23 * right_dual.e1) - (this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)), 
            /* e423, e431, e412 */
            float3(right_dual.e4) * this.group0.xyz, 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group0.xyz) + (this.group1.xyz.zxy * right_dual.group1.yzx) - (this.group1.xyz.yzx * right_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e35 * right_dual.e42), (this.e31 * right_dual.e45) + (this.e15 * right_dual.e43), (this.e12 * right_dual.e45) + (this.e25 * right_dual.e41), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group1.xyz.yzx * right_dual.group4.xyz.zxy), this.e23 * right_dual.e15), 
            /* e1234 */
            -(this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiLine> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiFlatPoint bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e23 * right_dual.e5) + (this.e35 * right_dual.e2), (this.e31 * right_dual.e5) + (this.e15 * right_dual.e3), (this.e12 * right_dual.e5) + (this.e25 * right_dual.e1), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (right_dual.group0.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<AntiLine> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Circle bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_dual.e4) * this.group1.xyz), -(this.e23 * right_dual.e1) - (this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)), 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group0.xyz) + (this.group1.xyz.zxy * right_dual.group0.yzx) - (this.group1.xyz.yzx * right_dual.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public DipoleInversion bulk_expansion(VersorEven other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group1.xyz), -(this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e35 * right_dual.e42), (this.e31 * right_dual.e45) + (this.e15 * right_dual.e43), (this.e12 * right_dual.e45) + (this.e25 * right_dual.e41), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group1.xyz.yzx * right_dual.group0.zxy), this.e23 * right_dual.e15)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public CircleRotor bulk_expansion(VersorOdd other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_dual.e4) * this.group1.xyz), -(this.e23 * right_dual.e1) - (this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * right_dual.e5) + (this.e35 * right_dual.e2), (this.e31 * right_dual.e5) + (this.e15 * right_dual.e3), (this.e12 * right_dual.e5) + (this.e25 * right_dual.e1), -(this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) - float4((this.group1.xyz.yzx * right_dual.group3.zxy), this.e23 * right_dual.e415)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiMotor> operator /(AntiMotor lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public CircleRotor bulk_expansion(AntiCircleRotor other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group0.www * right_dual.group2.xyz), (this.scalar * right_dual.e12345) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public DipoleInversion bulk_expansion(AntiDipoleInversion other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group0.www * right_dual.group2.xyz), (this.scalar * right_dual.e1234) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.scalar * right_dual.e4235), (this.e31 * right_dual.e45) + (this.scalar * right_dual.e4315), (this.e12 * right_dual.e45) + (this.scalar * right_dual.e4125), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + float4((right_dual.group0.xyz.yzx * this.group1.zxy), this.scalar * right_dual.e3215) - float4((right_dual.group0.xyz.zxy * this.group1.yzx), this.e23 * right_dual.e15)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Motor bulk_expansion(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), this.scalar * right_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e5) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Flector bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e45) * this.group0.xyz), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Flector bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * right_dual.e4235, this.scalar * right_dual.e4315, this.scalar * right_dual.e4125, -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) + (this.group0 * float4(right_dual.group0.www, right_dual.e3215))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Motor bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_dual.group0.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Motor bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.www * right_dual.group0.xyz), (this.scalar * right_dual.e12345) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_dual.group1.xyz) + (float3(right_dual.e5) * this.group0.xyz)), this.scalar * right_dual.e5)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Plane bulk_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public DipoleInversion bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e35 * right_dual.e42), (this.e31 * right_dual.e45) + (this.e15 * right_dual.e43), (this.e12 * right_dual.e45) + (this.e25 * right_dual.e41), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((right_dual.group0.xyz.zxy * this.group1.yzx), this.e23 * right_dual.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public VersorOdd bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(right_dual.group0.xyz, right_dual.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(right_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_dual.group1.xyz)), right_dual.e45 * this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((right_dual.e15 * this.scalar) + (right_dual.scalar * this.e15), (right_dual.e25 * this.scalar) + (right_dual.scalar * this.e25), (right_dual.e35 * this.scalar) + (right_dual.scalar * this.e35), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e45 * this.e23, right_dual.e45 * this.e31, right_dual.e45 * this.e12, -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + (this.group1.zxyw * float4(right_dual.group0.xyz.yzx, right_dual.scalar)) - (this.group1.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public CircleRotor bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public VersorEven bulk_expansion(DipoleInversion other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.e4 * this.e23, right_dual.e4 * this.e31, right_dual.e4 * this.e12, -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12)) + float4((right_dual.group0.xyz * this.group0.www), right_dual.e4 * this.e3215), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e4 * this.e15, right_dual.e4 * this.e25, right_dual.e4 * this.e35, -(right_dual.e1 * this.e23) - (right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + (float4(this.scalar) * right_dual.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.e5) * this.group0.xyz) + (float3(this.scalar) * right_dual.group2.xyz) + (right_dual.group3.yzx * this.group1.zxy) - (right_dual.group3.zxy * this.group1.yzx)), right_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_dual.group3.xyz, right_dual.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_dual.group0.xy.yy, right_dual.scalar, (right_dual.e3215 * this.scalar) + (right_dual.scalar * this.e3215)) * float4(this.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiFlatPoint bulk_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiFlector bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_dual.e2 * this.e35) + (right_dual.e5 * this.e23), (right_dual.e3 * this.e15) + (right_dual.e5 * this.e31), (right_dual.e1 * this.e25) + (right_dual.e5 * this.e12), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + (float4(this.scalar) * right_dual.group0) - (right_dual.group1.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.scalar) * right_dual.group1.xyz), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((float3(right_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_dual.group0.xyz)), right_dual.scalar * this.scalar), 
            /* e15, e25, e35, e3215 */
            (float4(right_dual.scalar) * this.group1) + (float4(this.scalar) * right_dual.group1) + float4(float3(0.0), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(this.scalar * right_dual.scalar, (this.scalar * right_dual.e12345) + (this.e3215 * right_dual.e4) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e5 */
            this.scalar * right_dual.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(this.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * this.group1.xyz)), this.scalar * right_dual.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group4.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.e15 * right_dual.e4, this.e25 * right_dual.e4, this.e35 * right_dual.e4, -(this.e23 * right_dual.e1) - (this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + (float4(this.scalar) * right_dual.group6), 
            /* e423, e431, e412 */
            (float3(this.scalar) * right_dual.group7.xyz) + (float3(right_dual.e4) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(this.scalar) * right_dual.group8.xyz) + (float3(right_dual.e5) * this.group0.xyz) + (this.group1.zxy * right_dual.group1.yzx) - (this.group1.yzx * right_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * right_dual.e4235, this.scalar * right_dual.e4315, this.scalar * right_dual.e4125, -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + (this.group0 * float4(right_dual.group3.www, right_dual.e3215)) + (this.group1.zxyw * float4(right_dual.group4.xyz.yzx, right_dual.scalar)) - (this.group1.yzxx * float4(right_dual.group4.xyz.zxy, right_dual.e23)), 
            /* e1234 */
            (this.scalar * right_dual.e1234) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiFlector bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.e23 * right_dual.e5) + (this.e35 * right_dual.e2), (this.e31 * right_dual.e5) + (this.e15 * right_dual.e3), (this.e12 * right_dual.e5) + (this.e25 * right_dual.e1), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (right_dual.group0.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Sphere bulk_expansion(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group0, 
            /* e1234 */
            this.scalar * right_dual.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public VersorEven bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.e4) * float4(this.group0.xyz, this.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(right_dual.e4) * this.group1.xyz), -(this.e23 * right_dual.e1) - (this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.e5) * this.group0.xyz) + (this.group1.zxy * right_dual.group0.yzx) - (this.group1.yzx * right_dual.group0.zxy)), this.scalar * right_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public VersorOdd bulk_expansion(VersorEven other) {
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
            float4(this.scalar) * right_dual.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(this.scalar) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group0.xyz)), this.scalar * right_dual.e45), 
            /* e15, e25, e35, e1234 */
            float4(this.e15 * right_dual.scalar, this.e25 * right_dual.scalar, this.e35 * right_dual.scalar, -(this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)) + (float4(this.scalar) * right_dual.group2), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e35 * right_dual.e42, this.e15 * right_dual.e43, this.e25 * right_dual.e41, -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + (this.group0 * float4(right_dual.group1.www, right_dual.e3215)) + float4((this.group0.www * right_dual.group3.xyz), this.e3215 * right_dual.scalar) - float4((this.group1.yzx * right_dual.group0.zxy), this.e23 * right_dual.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public VersorEven bulk_expansion(VersorOdd other) {
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
            (this.group0 * float4(right_dual.group3.www, right_dual.e12345)) + float4(float3(0.0), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) + float4((this.group0.www * right_dual.group0.xyz), this.e3215 * right_dual.e4), 
            /* e415, e425, e435, e321 */
            float4(this.e15 * right_dual.e4, this.e25 * right_dual.e4, this.e35 * right_dual.e4, -(this.e23 * right_dual.e1) - (this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + (float4(this.scalar) * right_dual.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_dual.group2.xyz) + (float3(right_dual.e5) * this.group0.xyz) + (this.group1.zxy * right_dual.group3.yzx) - (this.group1.yzx * right_dual.group3.zxy)), this.scalar * right_dual.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group3
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiPlane> operator /(AntiPlane lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Sphere bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group0.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((right_dual.group0.xyz * this.group0.www), this.e1 * right_dual.e235), 
            /* e1234 */
            (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public CircleRotor bulk_expansion(AntiDipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_dual.e41, this.e5 * right_dual.e42, this.e5 * right_dual.e43, -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group0.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(this.e3 * right_dual.e25 * -1.0, this.e1 * right_dual.e35 * -1.0, this.e2 * right_dual.e15 * -1.0, (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)) + (this.group0.yzxx * float4(right_dual.group2.zxy, right_dual.e4235)) + (this.group0.wwwy * float4(right_dual.group1.xyz, right_dual.e4315))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public FlatPoint bulk_expansion(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.xx, right_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Line bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_dual.e45) * this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Motor bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.e45) * this.group0.xyz * float3(-1.0)), (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Plane bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Flector bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) - (this.group0.yzxx * float4(right_dual.group0.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiScalar bulk_expansion(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiPlane bulk_expansion(AntiScalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Circle bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Circle(
            /* e423, e431, e412 */
            (right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_dual.e41, this.e5 * right_dual.e42, this.e5 * right_dual.e43, -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group0.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.xyz.zxy * this.group0.yzx) - (right_dual.group2.xyz.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<AntiPlane> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e41 * this.e5, right_dual.e42 * this.e5, right_dual.e43 * this.e5, -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e5) * right_dual.group1.xyz), 0.0) + float4((right_dual.group2.zxy * this.group0.yzx), 0.0) - float4((right_dual.group2.yzx * this.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Sphere bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e235)) - float4((this.group0.yzx * right_dual.group1.zxy), this.e2 * right_dual.e315), 
            /* e1234 */
            (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiPlane> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public DipoleInversion bulk_expansion(DipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group3.zxy * this.group0.yzx) - (right_dual.group3.yzx * this.group0.zxy)), right_dual.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e1 * this.e5 * -1.0, right_dual.e2 * this.e5 * -1.0, right_dual.e3 * this.e5 * -1.0, (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e425 * this.e3, right_dual.e435 * this.e1, right_dual.e415 * this.e2, -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkExpansionInfixPartial<AntiPlane> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiPlane bulk_expansion(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiPlane> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiDualNum bulk_expansion(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiMotor bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((right_dual.group1.zxy * this.group0.yzx) - (right_dual.group1.yzx * this.group0.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e5 * this.e1, right_dual.e5 * this.e2, right_dual.e5 * this.e3, -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)) - (this.group0.wwwx * float4(right_dual.group1.xyz, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiFlatPoint bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((right_dual.e23 * this.e5) + (right_dual.e35 * this.e2), (right_dual.e31 * this.e5) + (right_dual.e15 * this.e3), (right_dual.e12 * this.e5) + (right_dual.e25 * this.e1), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (this.group0.zxyx * float4(right_dual.group1.xyz.yzx, right_dual.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiFlector bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_dual.e23 * this.e5) + (right_dual.e35 * this.e2), (right_dual.e31 * this.e5) + (right_dual.e15 * this.e3), (right_dual.e12 * this.e5) + (right_dual.e25 * this.e1), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (this.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e23)), 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.group0.xy.xx, right_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            this.e5 * right_dual.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group1.xyz)), this.e5 * right_dual.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(right_dual.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (this.group0.yzx * right_dual.group1.zxy) - (this.group0.zxy * right_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_dual.e41, this.e5 * right_dual.e42, this.e5 * right_dual.e43, -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - float4((right_dual.group3.www * this.group0.xyz), this.e1 * right_dual.e23), 
            /* e423, e431, e412 */
            (right_dual.group4.xyz.yzx * this.group0.zxy) - (right_dual.group4.xyz.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group5.xyz) + (this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group0.wwwx * float4(right_dual.group7.xyz, right_dual.e235)) - float4((this.group0.yzx * right_dual.group6.zxy), this.e2 * right_dual.e315), 
            /* e1234 */
            (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiLine bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            (right_dual.group0.zxy * this.group0.yzx) - (right_dual.group0.yzx * this.group0.zxy), 
            /* e15, e25, e35 */
            (float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiScalar bulk_expansion(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiScalar(
            /* e12345 */
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Dipole bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_dual.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx)), this.e5 * right_dual.e4 * -1.0), 
            /* e15, e25, e35 */
            (float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public VersorEven bulk_expansion(VersorEven other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e2 * right_dual.e43 * -1.0, this.e3 * right_dual.e41 * -1.0, this.e1 * right_dual.e42 * -1.0, (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)) + (this.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e4235)), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_dual.e41, this.e5 * right_dual.e42, this.e5 * right_dual.e43, -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group0.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_dual.group1.xyz) + (this.group0.yzx * right_dual.group2.zxy) - (this.group0.zxy * right_dual.group2.yzx)), this.e5 * right_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public DipoleInversion bulk_expansion(VersorOdd other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx)), this.e5 * right_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * right_dual.e1 * -1.0, this.e5 * right_dual.e2 * -1.0, this.e5 * right_dual.e3 * -1.0, (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412)) + (this.group0.xyzx * float4(right_dual.group2.www, right_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e235)) - (this.group0.wwwy * float4(right_dual.group0.xyz, right_dual.e315))
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiScalar> operator /(AntiScalar lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            float4(other.group2.xyz, other.e12345 * -1.0)[3] * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(DualNum other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(Motor other) {
        return AntiScalar(
            /* e12345 */
            float4(other.group0.xyz, other.e12345 * -1.0)[3] * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            float4(other.group0.xyz, other.e12345 * -1.0)[3] * this.e12345
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Circle> operator /(Circle lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public AntiScalar bulk_expansion(AntiDipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Plane bulk_expansion(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public AntiScalar bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public AntiScalar bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Plane bulk_expansion(AntiMotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<Circle> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Circle bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(right_dual.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public AntiScalar bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public CircleRotor bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Sphere bulk_expansion(DipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual.e4 * this.e235) - (right_dual.e3 * this.e425), -(right_dual.e4 * this.e315) - (right_dual.e1 * this.e435), -(right_dual.e4 * this.e125) - (right_dual.e2 * this.e415), (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group3.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_dual.group3.yzx * this.group1.zxy), right_dual.e2 * this.e315), 
            /* e1234 */
            -(right_dual.e4 * this.e321) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Circle bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(right_dual.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Sphere bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group1.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_dual.group1.yzx * this.group1.zxy), right_dual.e2 * this.e315), 
            /* e1234 */
            -(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public AntiScalar bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public CircleRotor bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 
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
            float4(right_dual.scalar) * this.group1, 
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.scalar) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e321 * right_dual.e5) + (this.e125 * right_dual.e3)) + (right_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(right_dual.e5) * this.group0.xyz), this.e235 * right_dual.e1), 
            /* e1234 */
            -(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Circle> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Sphere bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group0.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_dual.group0.yzx * this.group1.zxy), right_dual.e2 * this.e315), 
            /* e1234 */
            -(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Circle> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Sphere bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e321 * right_dual.e5) + (this.e125 * right_dual.e3)) + (right_dual.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(right_dual.e5) * this.group0.xyz), this.e235 * right_dual.e1), 
            /* e1234 */
            -(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public CircleRotor bulk_expansion(VersorEven other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Sphere bulk_expansion(VersorOdd other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e321 * right_dual.e5) + (this.e125 * right_dual.e3)) + (right_dual.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_dual.group2.www), this.e235 * right_dual.e1), 
            /* e1234 */
            -(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<CircleRotor> operator /(CircleRotor lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar bulk_expansion(AntiDipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Plane bulk_expansion(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Plane bulk_expansion(AntiMotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_dual.scalar) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((right_dual.group2.www * this.group2.xyz), (right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Sphere bulk_expansion(DipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual.e4 * this.e235) - (right_dual.e3 * this.e425), -(right_dual.e4 * this.e315) - (right_dual.e1 * this.e435), -(right_dual.e4 * this.e125) - (right_dual.e2 * this.e415), (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_dual.group3.www), right_dual.e1 * this.e235), 
            /* e1234 */
            -(right_dual.e4 * this.e321) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_dual.scalar) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Sphere bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_dual.group1.www), right_dual.e1 * this.e235), 
            /* e1234 */
            -(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((right_dual.group0.www * this.group2.xyz), (right_dual.scalar * this.e12345) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (this.e12345 * right_dual.scalar) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 
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
            float4(right_dual.scalar) * this.group1, 
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.scalar) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group1.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Sphere bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_dual.group0.www), right_dual.e1 * this.e235), 
            /* e1234 */
            -(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Sphere bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group0.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor bulk_expansion(VersorEven other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_dual.scalar) * this.group2.xyz), (this.e12345 * right_dual.scalar) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Sphere bulk_expansion(VersorOdd other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group3.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Dipole> operator /(Dipole lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public AntiScalar bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Sphere bulk_expansion(AntiDipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e35 * right_dual.e12)) - float4((this.group0.xyz.zxy * right_dual.group2.yzx), this.e15 * right_dual.e23) - float4((this.group2.xyz.yzx * right_dual.group0.xyz.zxy), this.e25 * right_dual.e31), 
            /* e1234 */
            -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Line bulk_expansion(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_dual.e5) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.e5) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Plane bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Plane bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public AntiScalar bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Motor bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.e5) * this.group0.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Dipole bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(right_dual.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Dipole> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Sphere bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group2.xyz.yzx), right_dual.e15 * this.e23) - float4((right_dual.group2.xyz.yzx * this.group0.xyz.zxy), right_dual.e25 * this.e31), 
            /* e1234 */
            -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<Dipole> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public DipoleInversion bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group2.xyz.yzx), right_dual.e23 * this.e15) - float4((this.group0.xyz.zxy * right_dual.group2.yzx), right_dual.e31 * this.e25)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public AntiScalar bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public CircleRotor bulk_expansion(DipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group3.zxy) - (this.group0.xyz.zxy * right_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e4 * this.e15) + (right_dual.e5 * this.e41), (right_dual.e4 * this.e25) + (right_dual.e5 * this.e42), (right_dual.e4 * this.e35) + (right_dual.e5 * this.e43), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.e2 * this.e35) + (right_dual.e5 * this.e23), (right_dual.e3 * this.e15) + (right_dual.e5 * this.e31), (right_dual.e1 * this.e25) + (right_dual.e5 * this.e12), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((this.group2.xyz.yzx * right_dual.group3.zxy), right_dual.e423 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Dipole bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(right_dual.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public AntiScalar bulk_expansion(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public CircleRotor bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_dual.group1.zxy) - (this.group0.xyz.zxy * right_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e5 * this.e41, right_dual.e5 * this.e42, right_dual.e5 * this.e43, -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.e2 * this.e35) + (right_dual.e5 * this.e23), (right_dual.e3 * this.e15) + (right_dual.e5 * this.e31), (right_dual.e1 * this.e25) + (right_dual.e5 * this.e12), -(right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) - float4((this.group2.xyz.yzx * right_dual.group1.zxy), right_dual.e235 * this.e41)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Dipole> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Sphere bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e35 * this.e42), (right_dual.e31 * this.e45) + (right_dual.e15 * this.e43), (right_dual.e12 * this.e45) + (right_dual.e25 * this.e41), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group1.xyz.yzx * this.group0.xyz.zxy), right_dual.e23 * this.e15), 
            /* e1234 */
            -(right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public DipoleInversion bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e35 * this.e42), (right_dual.e31 * this.e45) + (right_dual.e15 * this.e43), (right_dual.e12 * this.e45) + (right_dual.e25 * this.e41), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((this.group0.xyz.zxy * right_dual.group1.yzx), right_dual.e23 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.scalar) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group1.zxy) - (this.group0.xyz.zxy * right_dual.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_dual.group1.yzx) - (this.group2.xyz.yzx * right_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e35 * right_dual.e12)) - float4((this.group0.xyz.zxy * right_dual.group3.yzx), this.e15 * right_dual.e23) - float4((this.group2.xyz.yzx * right_dual.group4.xyz.zxy), this.e25 * right_dual.e31), 
            /* e1234 */
            -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<Dipole> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Circle bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Circle(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e5 * this.e41, right_dual.e5 * this.e42, right_dual.e5 * this.e43, -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_dual.group0.yzx) - (this.group2.xyz.yzx * right_dual.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<Dipole> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Circle bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Circle(
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_dual.group0.yzx) - (this.group2.xyz.yzx * right_dual.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public DipoleInversion bulk_expansion(VersorEven other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e35 * right_dual.e12)) - float4((this.group0.xyz.zxy * right_dual.group2.yzx), this.e15 * right_dual.e23) - float4((this.group2.xyz.yzx * right_dual.group0.zxy), this.e25 * right_dual.e31)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public CircleRotor bulk_expansion(VersorOdd other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group3.zxy) - (this.group0.xyz.zxy * right_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * right_dual.e5) + (this.e35 * right_dual.e2), (this.e31 * right_dual.e5) + (this.e15 * right_dual.e3), (this.e12 * right_dual.e5) + (this.e25 * right_dual.e1), -(this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) - float4((this.group2.xyz.yzx * right_dual.group3.zxy), this.e41 * right_dual.e235)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Sphere bulk_expansion(AntiDipoleInversion other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - (this.group2.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23)) - float4((this.group0.xyz.zxy * right_dual.group2.yzx), right_dual.e31 * this.e25), 
            /* e1234 */
            -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Motor bulk_expansion(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e5) * float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4(right_dual.group0.xy.xx, right_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Plane bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Plane bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Motor bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.e5) * this.group0.xyz), (this.e1234 * right_dual.e5) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Sphere bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group2.yzx), right_dual.e15 * this.e23) - float4((right_dual.group2.xyz.yzx * this.group0.xyz.zxy), right_dual.e25 * this.e31), 
            /* e1234 */
            -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((right_dual.group2.www * this.group2.xyz), (right_dual.scalar * this.e1234) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42) + (right_dual.scalar * this.e4235), (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43) + (right_dual.scalar * this.e4315), (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41) + (right_dual.scalar * this.e4125), -(right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.scalar * this.e3215) - (this.group2.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23)) - float4((this.group0.xyz.zxy * right_dual.group2.yzx), right_dual.e31 * this.e25)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor bulk_expansion(DipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group3.zxy) - (this.group0.xyz.zxy * right_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e4 * this.e15) + (right_dual.e5 * this.e41), (right_dual.e4 * this.e25) + (right_dual.e5 * this.e42), (right_dual.e4 * this.e35) + (right_dual.e5 * this.e43), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (right_dual.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) + float4((right_dual.group3.yzx * this.group2.zxy), right_dual.e4 * this.e3215) - (this.group2.yzxx * float4(right_dual.group3.zxy, right_dual.e423))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar bulk_expansion(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_dual.group1.zxy) - (this.group0.xyz.zxy * right_dual.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e5 * this.e41, right_dual.e5 * this.e42, right_dual.e5 * this.e43, -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (right_dual.group1.yzxx * float4(this.group2.zxy, this.e4235)) + (right_dual.group1.wwwy * float4(this.group1.xyz, this.e4315)) + float4(float3(0.0), (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) - float4((right_dual.group1.zxy * this.group2.yzx), right_dual.e235 * this.e41)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Sphere bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e35 * this.e42), (right_dual.e31 * this.e45) + (right_dual.e15 * this.e43), (right_dual.e12 * this.e45) + (right_dual.e25 * this.e41), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group1.xyz.yzx * this.group0.xyz.zxy), right_dual.e23 * this.e15), 
            /* e1234 */
            -(right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((right_dual.group0.www * this.group2.xyz), (right_dual.scalar * this.e1234) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.scalar * this.e4235), (right_dual.e31 * this.e45) + (right_dual.scalar * this.e4315), (right_dual.e12 * this.e45) + (right_dual.scalar * this.e4125), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + float4((this.group0.xyz.yzx * right_dual.group1.zxy), right_dual.scalar * this.e3215) - float4((this.group0.xyz.zxy * right_dual.group1.yzx), right_dual.e23 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (this.e1234 * right_dual.e5) + (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.scalar) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group1.zxy) - (this.group0.xyz.zxy * right_dual.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_dual.group1.yzx) - (this.group2.yzx * right_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e35 * right_dual.e12)) + (float4(right_dual.scalar) * this.group3) - (this.group2.yzxy * float4(right_dual.group4.xyz.zxy, right_dual.e31)) - float4((this.group0.xyz.zxy * right_dual.group3.yzx), this.e15 * right_dual.e23), 
            /* e1234 */
            (this.e1234 * right_dual.scalar) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e5 * this.e41, right_dual.e5 * this.e42, right_dual.e5 * this.e43, -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(right_dual.e3 * this.e25 * -1.0, right_dual.e1 * this.e35 * -1.0, right_dual.e2 * this.e15 * -1.0, (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)) + (right_dual.group0.yzxx * float4(this.group2.zxy, this.e4235)) + (right_dual.group0.wwwy * float4(this.group1.xyz, this.e4315))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(this.e25 * right_dual.e3 * -1.0, this.e35 * right_dual.e1 * -1.0, this.e15 * right_dual.e2 * -1.0, (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4)) + (float4(right_dual.e5) * float4(this.group1.xyz, this.e1234)) + (right_dual.group0.yzxx * float4(this.group2.zxy, this.e4235))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion bulk_expansion(VersorEven other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group2.xyz), (this.e1234 * right_dual.scalar) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42) + (this.e4235 * right_dual.scalar), (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43) + (this.e4315 * right_dual.scalar), (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41) + (this.e4125 * right_dual.scalar), -(this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + float4((this.group0.xyz.yzx * right_dual.group2.zxy), this.e3215 * right_dual.scalar) - (right_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((this.group2.yzx * right_dual.group0.zxy), this.e31 * right_dual.e25)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor bulk_expansion(VersorOdd other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group3.zxy) - (this.group0.xyz.zxy * right_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(right_dual.e5) * float4(this.group1.xyz, this.e1234)) + (right_dual.group3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) - float4((this.group2.yzx * right_dual.group3.zxy), this.e41 * right_dual.e235)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<DualNum> operator /(DualNum lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Plane bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Motor bulk_expansion(AntiDipoleInversion other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e1234), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkExpansionInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public DualNum bulk_expansion(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Line bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.e5) * right_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e5) * right_dual.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Motor bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, this.e12345) * float4(right_dual.group0.xyz, right_dual.scalar), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(right_dual.group1.xyz, right_dual.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Plane bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Flector bulk_expansion(DipoleInversion other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(right_dual.group3.xyz, right_dual.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkExpansionInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public DualNum bulk_expansion(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public AntiDualNum bulk_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e5 * other.e45, 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Flector bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual.e321 * this.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public AntiFlatPoint bulk_expansion(Line other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Motor bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), right_dual.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * right_dual.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (this.e5 * right_dual.e1234) + (this.e12345 * right_dual.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5 * right_dual.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_dual.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e5) * right_dual.group5.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group7.xyz, right_dual.e321) * float4(-1.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public FlatPoint bulk_expansion(Plane other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215).xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public AntiScalar bulk_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public FlatPoint bulk_expansion(Sphere other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e1234) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Motor bulk_expansion(VersorEven other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, (this.e5 * right_dual.e1234) + (this.e12345 * right_dual.scalar)) * float4(right_dual.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(right_dual.group1.xyz, right_dual.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Flector bulk_expansion(VersorOdd other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_dual.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<FlatPoint> operator /(FlatPoint lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public AntiScalar bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Plane bulk_expansion(AntiDipoleInversion other) {
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
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<FlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public FlatPoint bulk_expansion(AntiScalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Plane bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<FlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Flector bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public AntiScalar bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Motor bulk_expansion(DipoleInversion other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e4 * this.e15, right_dual.e4 * this.e25, right_dual.e4 * this.e35, -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group3.yzx * this.group0.zxy) - (right_dual.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<FlatPoint> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public FlatPoint bulk_expansion(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<FlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public AntiScalar bulk_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Motor bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45) * float4(right_dual.group1.xyz, right_dual.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group1.yzx * this.group0.zxy) - (right_dual.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Plane bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_dual.group0.xyz), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Flector bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_dual.group0.xyz), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, -(this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_dual.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (this.group0.zxy * right_dual.group1.yzx) - (this.group0.yzx * right_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_dual.e42) + (this.e45 * right_dual.e23), (this.e15 * right_dual.e43) + (this.e45 * right_dual.e31), (this.e25 * right_dual.e41) + (this.e45 * right_dual.e12), -(this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - (this.group0.yzxx * float4(right_dual.group4.xyz.zxy, right_dual.e23)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Line bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.e45) * right_dual.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Line bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Line(
            /* e415, e425, e435 */
            (float3(right_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_dual.group0.xyz), 
            /* e235, e315, e125 */
            (this.group0.zxy * right_dual.group0.yzx) - (this.group0.yzx * right_dual.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Flector bulk_expansion(VersorEven other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_dual.e42) + (this.e45 * right_dual.e23), (this.e15 * right_dual.e43) + (this.e45 * right_dual.e31), (this.e25 * right_dual.e41) + (this.e45 * right_dual.e12), -(this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - (this.group0.yzxx * float4(right_dual.group0.zxy, right_dual.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Motor bulk_expansion(VersorOdd other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e15 * right_dual.e4, this.e25 * right_dual.e4, this.e35 * right_dual.e4, -(this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321)) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_dual.group3.yzx) - (this.group0.yzx * right_dual.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Flector> operator /(Flector lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public AntiScalar bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Plane bulk_expansion(AntiDipoleInversion other) {
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
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Flector bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Plane bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Flector bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.scalar * this.e4235), (right_dual.e31 * this.e45) + (right_dual.scalar * this.e4315), (right_dual.e12 * this.e45) + (right_dual.scalar * this.e4125), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) + float4((right_dual.group0.xyz.yzx * this.group0.zxy), right_dual.scalar * this.e3215) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public AntiScalar bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Motor bulk_expansion(DipoleInversion other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(right_dual.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group3.yzx * this.group0.zxy) - (right_dual.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Flector bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public AntiScalar bulk_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Motor bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * right_dual.group1.xyz * float3(-1.0)), (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) - (right_dual.e321 * this.e45)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group1.yzx * this.group0.zxy) - (right_dual.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Plane bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_dual.group0.xyz), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Flector bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar * this.e4235, right_dual.scalar * this.e4315, right_dual.scalar * this.e4125, -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) + (right_dual.group0 * float4(this.group0.www, this.e3215))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_dual.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (this.group0.zxy * right_dual.group1.yzx) - (this.group0.yzx * right_dual.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_dual.e42) + (this.e45 * right_dual.e23), (this.e15 * right_dual.e43) + (this.e45 * right_dual.e31), (this.e25 * right_dual.e41) + (this.e45 * right_dual.e12), -(this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + (float4(right_dual.scalar) * this.group1) - (this.group0.yzxx * float4(right_dual.group4.xyz.zxy, right_dual.e23)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Motor bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * right_dual.group0.xyz * float3(-1.0)), (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Motor bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45 * right_dual.e1 * -1.0, this.e45 * right_dual.e2 * -1.0, this.e45 * right_dual.e3 * -1.0, (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4)) + (right_dual.group0.wwwx * float4(this.group0.xyz, this.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_dual.group0.yzx) - (this.group0.yzx * right_dual.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Flector bulk_expansion(VersorEven other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e45 * right_dual.e23) + (this.e4235 * right_dual.scalar), (this.e45 * right_dual.e31) + (this.e4315 * right_dual.scalar), (this.e45 * right_dual.e12) + (this.e4125 * right_dual.scalar), -(this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + (right_dual.group0.yzxw * float4(this.group0.zxy, this.e3215)) - (this.group0.yzxx * float4(right_dual.group0.zxy, right_dual.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Motor bulk_expansion(VersorOdd other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            (right_dual.group3.wwwx * float4(this.group0.xyz, this.e4235)) + float4(float3(0.0), (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321)) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_dual.group3.yzx) - (this.group0.yzx * right_dual.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Line> operator /(Line lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public AntiScalar bulk_expansion(AntiDipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Line bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public AntiScalar bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Motor bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.scalar) * this.group0.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Plane bulk_expansion(DipoleInversion other) {
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
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual.e4 * this.e235) - (right_dual.e3 * this.e425), -(right_dual.e4 * this.e315) - (right_dual.e1 * this.e435), -(right_dual.e4 * this.e125) - (right_dual.e2 * this.e415), (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) + (right_dual.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Line bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(right_dual.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Plane bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) + (right_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public AntiScalar bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiScalar(
            /* e12345 */
            -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Motor bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.scalar) * this.group0.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, -(this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 
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
            float4(right_dual.group0.xy.xx, right_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual.scalar) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e235)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Plane bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) + (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Line> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Plane bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Motor bulk_expansion(VersorEven other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.scalar) * this.group0.xyz), -(this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Plane bulk_expansion(VersorOdd other) {
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
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Motor> operator /(Motor lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Plane bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(AntiDipoleInversion other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * right_dual.group0.xyz), (right_dual.e1234 * this.e5) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * right_dual.group0.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.scalar * this.e415, right_dual.scalar * this.e425, right_dual.scalar * this.e435, -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)) + float4((right_dual.group0.xyz * this.group1.www), right_dual.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.scalar) * this.group1.xyz) + (float3(this.e5) * right_dual.group1.xyz)), right_dual.scalar * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Plane bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Flector bulk_expansion(DipoleInversion other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(right_dual.group3.xyz, right_dual.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual.e4 * this.e235) - (right_dual.e3 * this.e425), -(right_dual.e4 * this.e315) - (right_dual.e1 * this.e435), -(right_dual.e4 * this.e125) - (right_dual.e2 * this.e415), (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) + (right_dual.group3.yzxx * float4(this.group0.zxy, this.e235)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public AntiDualNum bulk_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e45 * this.e5, 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Flector bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) - float4((right_dual.group1.zxy * this.group0.yzx), right_dual.e321 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_dual.group0.www * this.group0.xyz), (right_dual.scalar * this.e12345) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.scalar) * this.group1.xyz) + (float3(this.e5) * right_dual.group0.xyz)), right_dual.scalar * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (this.e12345 * right_dual.scalar) + (this.e5 * right_dual.e1234) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5 * right_dual.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_dual.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * right_dual.group4.xyz) + (float3(right_dual.scalar) * this.group0.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group1.yzxx * float4(this.group0.zxy, this.e235)) - float4((right_dual.group7.xyz * this.group1.www), this.e5 * right_dual.e321), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Flector bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) + (right_dual.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public AntiScalar bulk_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Flector bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_dual.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(VersorEven other) {
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
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e1234)) + (float4(right_dual.scalar) * this.group0) + float4(float3(0.0), -(this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group1.xyz)), this.e5 * right_dual.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Flector bulk_expansion(VersorOdd other) {
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
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_dual.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e235 * right_dual.e4) - (this.e5 * right_dual.e423), -(this.e315 * right_dual.e4) - (this.e5 * right_dual.e431), -(this.e125 * right_dual.e4) - (this.e5 * right_dual.e412), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group3.yzxx * float4(this.group0.zxy, this.e235)) - float4((this.group0.yzx * right_dual.group3.zxy), this.e5 * right_dual.e321)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<MultiVector> operator /(MultiVector lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiCircleRotor other) {
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
            float2(0.0, (right_dual.e12345 * this.scalar) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 
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
            float4(this.scalar) * right_dual.group1, 
            /* e423, e431, e412 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_dual.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e235)), 
            /* e1234 */
            (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiDipoleInversion other) {
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
            float2(0.0, (right_dual.e1234 * this.e5) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(right_dual.group2.xyz, right_dual.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group1.zxy) - (right_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.zxy * this.group1.yzx) - (right_dual.group2.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) + (float4(this.scalar) * right_dual.group3) - (right_dual.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((right_dual.group0.xyz.zxy * this.group3.yzx), right_dual.e15 * this.e23), 
            /* e1234 */
            (right_dual.e1234 * this.scalar) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual.e5 * this.e1234) + (right_dual.e12345 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            right_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.group0.xy.xx, right_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual.e5) * this.group5.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e5) * float4(this.group7.xyz, this.e321), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (right_dual.group0.zxy * this.group1.yzx) - (right_dual.group0.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e35 * this.e42) + (right_dual.e45 * this.e23), (right_dual.e15 * this.e43) + (right_dual.e45 * this.e31), (right_dual.e25 * this.e41) + (right_dual.e45 * this.e12), -(right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - (right_dual.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (right_dual.group0.zxy * this.group1.yzx) - (right_dual.group0.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e35 * this.e42) + (right_dual.e45 * this.e23), (right_dual.e15 * this.e43) + (right_dual.e45 * this.e31), (right_dual.e25 * this.e41) + (right_dual.e45 * this.e12), -(right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + (float4(this.scalar) * right_dual.group1) - (right_dual.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 
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
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.scalar) * right_dual.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (this.group1.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual.e12345 * this.scalar) + (right_dual.e5 * this.e1234) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            right_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual.e5) * this.group4.xyz) + (float3(this.scalar) * right_dual.group0.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_dual.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) + float4((this.group7.xyz * right_dual.group1.www), right_dual.e5 * this.e321) - (this.group1.yzxx * float4(right_dual.group0.zxy, right_dual.e235)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215)), 
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
            float4(this.scalar) * right_dual.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual.scalar) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group1, 
            /* e5 */
            this.e5 * right_dual.scalar, 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group3, 
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.scalar) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group6, 
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.scalar) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group9, 
            /* e1234 */
            this.e1234 * right_dual.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Circle other) {
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
            float2(0.0, -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(right_dual.group2.xyz, right_dual.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group1.zxy) - (right_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.xyz.zxy * this.group1.yzx) - (right_dual.group2.xyz.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group3.yzx), right_dual.e15 * this.e23) - float4((right_dual.group2.xyz.yzx * this.group4.xyz.zxy), right_dual.e25 * this.e31), 
            /* e1234 */
            -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(CircleRotor other) {
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
            float2(right_dual.scalar * this.scalar, (right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group1, 
            /* e5 */
            right_dual.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_dual.scalar) * this.group3) + (float4(this.scalar) * float4(right_dual.group2.xyz, right_dual.e45)), 
            /* e41, e42, e43 */
            (float3(right_dual.scalar) * this.group4.xyz) + (float3(this.scalar) * right_dual.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_dual.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e15 * this.e4) + (right_dual.scalar * this.e415), (right_dual.e25 * this.e4) + (right_dual.scalar * this.e425), (right_dual.e35 * this.e4) + (right_dual.scalar * this.e435), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) + float4((float3(this.e5) * right_dual.group0.xyz), right_dual.scalar * this.e321) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(right_dual.scalar) * this.group7.xyz) + (float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group1.zxy) - (right_dual.group0.xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual.scalar) * this.group8.xyz) + (float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.zxy * this.group1.yzx) - (right_dual.group2.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42) + (right_dual.scalar * this.e4235), (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43) + (right_dual.scalar * this.e4315), (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41) + (right_dual.scalar * this.e4125), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) + float4((right_dual.group0.xyz.yzx * this.group3.zxy), right_dual.scalar * this.e3215) - (right_dual.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((right_dual.group0.xyz.zxy * this.group3.yzx), right_dual.e15 * this.e23), 
            /* e1234 */
            (right_dual.scalar * this.e1234) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Dipole other) {
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
            float2(0.0, -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 
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
            float4(this.scalar) * right_dual.group1, 
            /* e423, e431, e412 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_dual.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e321 * this.e5) - (right_dual.e125 * this.e3)) - (this.group1.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((float3(this.e5) * right_dual.group0.xyz), right_dual.e235 * this.e1), 
            /* e1234 */
            (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(DipoleInversion other) {
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
            float2(0.0, (right_dual.e4 * this.e3215) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_dual.group3.xyz, right_dual.e4), 
            /* e5 */
            right_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * this.group1) - (float4(this.e5) * float4(right_dual.group3.xyz, right_dual.e4)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (right_dual.group3.zxy * this.group1.yzx) - (right_dual.group3.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e4 * this.e15) + (right_dual.e5 * this.e41), (right_dual.e4 * this.e25) + (right_dual.e5 * this.e42), (right_dual.e4 * this.e35) + (right_dual.e5 * this.e43), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + (float4(this.scalar) * right_dual.group1) - float4((this.group3.www * right_dual.group3.xyz), right_dual.e1 * this.e23), 
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group5.xyz) + (float3(this.scalar) * right_dual.group0.xyz) + (this.group4.xyz.yzx * right_dual.group3.zxy) - (this.group4.xyz.zxy * right_dual.group3.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_dual.group2.xyz) + (right_dual.group3.yzx * this.group3.zxy) - (right_dual.group3.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group3.yzxw * this.group6.zxyw) + (right_dual.group3.wwwx * float4(this.group7.xyz, this.e235)) + float4((right_dual.group1.yzx * this.group1.zxy), right_dual.e2 * this.e315) + float4((this.group1.www * right_dual.group2.xyz), right_dual.e3 * this.e125) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group1.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((this.group8.xyz * right_dual.group2.www), right_dual.e235 * this.e1) - float4((right_dual.group3.zxy * this.group6.yzx), right_dual.e125 * this.e3), 
            /* e1234 */
            (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e4 * this.e321) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual.scalar * this.scalar, (right_dual.e3215 * this.e4) + (right_dual.scalar * this.e12345)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group1, 
            /* e5 */
            right_dual.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group3, 
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_dual.scalar) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group6, 
            /* e423, e431, e412 */
            float3(right_dual.scalar) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_dual.scalar) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.group0.xy.yy, right_dual.scalar, (right_dual.e3215 * this.scalar) + (right_dual.scalar * this.e3215)) * float4(this.group9.xyz, 1.0), 
            /* e1234 */
            right_dual.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)), 
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
            float4(float3(0.0), right_dual.e321 * this.scalar), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_dual.group0.xyz), -(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)), 
            /* e1234 */
            right_dual.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            right_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_dual.group1.xyz)), right_dual.e5 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * right_dual.group1.xyz, 
            /* e23, e31, e12 */
            (right_dual.group1.zxy * this.group1.yzx) - (right_dual.group1.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + float4((this.group4.xyz * right_dual.group1.www), right_dual.e321 * this.scalar) - float4((this.group3.www * right_dual.group1.xyz), right_dual.e1 * this.e23), 
            /* e423, e431, e412 */
            (this.group4.xyz.yzx * right_dual.group1.zxy) - (this.group4.xyz.zxy * right_dual.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_dual.group0.xyz) + (right_dual.group1.yzx * this.group3.zxy) - (right_dual.group1.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group1.wwwx * float4(this.group7.xyz, this.e235)) + float4(float3(0.0), (right_dual.e5 * this.e321) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)) + float4((right_dual.group1.yzx * this.group6.zxy), right_dual.e3 * this.e125) + float4((this.group1.www * right_dual.group0.xyz), right_dual.e2 * this.e315) - float4((right_dual.group1.zxy * this.group6.yzx), right_dual.e235 * this.e1), 
            /* e1234 */
            (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), -(right_dual.e23 * this.e1) - (right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)), 
            /* e423, e431, e412 */
            float3(this.e4) * right_dual.group0.xyz, 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group0.xyz) + (right_dual.group1.xyz.zxy * this.group1.yzx) - (right_dual.group1.xyz.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e35 * this.e42), (right_dual.e31 * this.e45) + (right_dual.e15 * this.e43), (right_dual.e12 * this.e45) + (right_dual.e25 * this.e41), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group1.xyz.yzx * this.group4.xyz.zxy), right_dual.e23 * this.e15), 
            /* e1234 */
            -(right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual.scalar * this.scalar, (right_dual.scalar * this.e12345) + (right_dual.e3215 * this.e4) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group1, 
            /* e5 */
            right_dual.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.scalar) * this.group3.xyz) + (float3(this.scalar) * right_dual.group1.xyz)), right_dual.scalar * this.e45), 
            /* e41, e42, e43 */
            float3(right_dual.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            (float3(right_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_dual.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e15 * this.e4, right_dual.e25 * this.e4, right_dual.e35 * this.e4, -(right_dual.e23 * this.e1) - (right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) + (float4(right_dual.scalar) * this.group6), 
            /* e423, e431, e412 */
            (float3(right_dual.scalar) * this.group7.xyz) + (float3(this.e4) * right_dual.group0.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual.scalar) * this.group8.xyz) + (float3(this.e5) * right_dual.group0.xyz) + (right_dual.group1.zxy * this.group1.yzx) - (right_dual.group1.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar * this.e4235, right_dual.scalar * this.e4315, right_dual.scalar * this.e4125, -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + (right_dual.group0 * float4(this.group3.www, this.e3215)) + (right_dual.group1.zxyw * float4(this.group4.xyz.yzx, this.scalar)) - (right_dual.group1.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234 */
            (right_dual.scalar * this.e1234) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(right_dual.scalar * this.scalar, (right_dual.scalar * this.e12345) + (right_dual.e12345 * this.scalar) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(right_dual.scalar) * this.group1) + (float4(this.scalar) * right_dual.group1), 
            /* e5 */
            (right_dual.scalar * this.e5) + (right_dual.e5 * this.scalar), 
            /* e15, e25, e35, e45 */
            (float4(right_dual.scalar) * this.group3) + (float4(right_dual.e5) * this.group1) + (float4(this.scalar) * right_dual.group3) - (float4(this.e5) * right_dual.group1), 
            /* e41, e42, e43 */
            (float3(right_dual.scalar) * this.group4.xyz) + (float3(this.scalar) * right_dual.group4.xyz) + (float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_dual.group5.xyz) + (right_dual.group1.zxy * this.group1.yzx) - (right_dual.group1.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e4 * this.e15) + (right_dual.e5 * this.e41) + (right_dual.e15 * this.e4) + (right_dual.e41 * this.e5), (right_dual.e4 * this.e25) + (right_dual.e5 * this.e42) + (right_dual.e25 * this.e4) + (right_dual.e42 * this.e5), (right_dual.e4 * this.e35) + (right_dual.e5 * this.e43) + (right_dual.e35 * this.e4) + (right_dual.e43 * this.e5), -(right_dual.e1 * this.e23) - (right_dual.e2 * this.e31) - (right_dual.e3 * this.e12) - (right_dual.e12 * this.e3)) + (float4(right_dual.scalar) * this.group6) + (float4(this.scalar) * right_dual.group6) - (this.group1.xyzy * float4(right_dual.group3.www, right_dual.e31)) - float4((this.group3.www * right_dual.group1.xyz), right_dual.e23 * this.e1), 
            /* e423, e431, e412 */
            (float3(right_dual.scalar) * this.group7.xyz) + (float3(right_dual.e4) * this.group5.xyz) + (float3(this.scalar) * right_dual.group7.xyz) + (float3(this.e4) * right_dual.group5.xyz) + (right_dual.group4.xyz.yzx * this.group1.zxy) + (this.group4.xyz.yzx * right_dual.group1.zxy) - (right_dual.group4.xyz.zxy * this.group1.yzx) - (this.group4.xyz.zxy * right_dual.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual.scalar) * this.group8.xyz) + (float3(right_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_dual.group8.xyz) + (float3(this.e5) * right_dual.group5.xyz) + (right_dual.group1.yzx * this.group3.zxy) + (right_dual.group3.zxy * this.group1.yzx) - (right_dual.group1.zxy * this.group3.yzx) - (right_dual.group3.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e2 * this.e435) + (right_dual.e5 * this.e423) + (right_dual.e45 * this.e23) + (right_dual.e425 * this.e3), (right_dual.e3 * this.e415) + (right_dual.e5 * this.e431) + (right_dual.e45 * this.e31) + (right_dual.e435 * this.e1), (right_dual.e1 * this.e425) + (right_dual.e5 * this.e412) + (right_dual.e45 * this.e12) + (right_dual.e415 * this.e2), -(right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12) - (right_dual.e321 * this.e5)) + (float4(right_dual.scalar) * this.group9) + (float4(this.scalar) * right_dual.group9) + float4((right_dual.group5.xyz * this.group3.www), right_dual.e2 * this.e315) + float4((right_dual.group8.xyz * this.group1.www), right_dual.e3 * this.e125) + float4((right_dual.group4.xyz.yzx * this.group3.zxy), right_dual.e1 * this.e235) + float4((this.group4.xyz.yzx * right_dual.group3.zxy), right_dual.e5 * this.e321) - (this.group1.yzxz * float4(right_dual.group6.zxy, right_dual.e125)) - (this.group3.yzxx * float4(right_dual.group4.xyz.zxy, right_dual.e23)) - float4((float3(this.e5) * right_dual.group7.xyz), right_dual.e31 * this.e25) - float4((this.group8.xyz * right_dual.group1.www), right_dual.e235 * this.e1) - float4((this.group4.xyz.zxy * right_dual.group3.yzx), right_dual.e12 * this.e35) - float4((right_dual.group1.zxy * this.group6.yzx), right_dual.e315 * this.e2), 
            /* e1234 */
            (right_dual.scalar * this.e1234) + (right_dual.e321 * this.e4) + (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e1234 * this.scalar) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5 */
            right_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_dual.group0.xyz)), right_dual.e5 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * right_dual.group0.xyz, 
            /* e23, e31, e12 */
            (right_dual.group0.zxy * this.group1.yzx) - (right_dual.group0.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e5 * this.e41, right_dual.e5 * this.e42, right_dual.e5 * this.e43, -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - float4((this.group3.www * right_dual.group0.xyz), right_dual.e1 * this.e23), 
            /* e423, e431, e412 */
            (this.group4.xyz.yzx * right_dual.group0.zxy) - (this.group4.xyz.zxy * right_dual.group0.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group5.xyz) + (right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group0.wwwx * float4(this.group7.xyz, this.e235)) + float4((right_dual.group0.yzx * this.group6.zxy), right_dual.e2 * this.e315), 
            /* e1234 */
            -(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234)), 
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
            float4(this.scalar) * right_dual.group0, 
            /* e1234 */
            this.scalar * right_dual.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) + (this.e1234 * right_dual.e5)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0, 
            /* e5 */
            this.scalar * right_dual.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * this.group1) - (float4(this.e5) * right_dual.group0), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * right_dual.group0.zxy) - (this.group1.zxy * right_dual.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_dual.e4) + (this.e41 * right_dual.e5), (this.e25 * right_dual.e4) + (this.e42 * right_dual.e5), (this.e35 * right_dual.e4) + (this.e43 * right_dual.e5), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group5.xyz) + (this.group4.xyz.yzx * right_dual.group0.zxy) - (this.group4.xyz.zxy * right_dual.group0.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group5.xyz) + (this.group3.zxy * right_dual.group0.yzx) - (this.group3.yzx * right_dual.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e321 * right_dual.e5) + (this.e125 * right_dual.e3)) + (right_dual.group0.yzxy * float4(this.group6.zxy, this.e315)) + float4((float3(right_dual.e5) * this.group7.xyz), this.e235 * right_dual.e1), 
            /* e1234 */
            -(this.e321 * right_dual.e4) - (this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(VersorEven other) {
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
            float2(this.scalar * right_dual.scalar, (this.e12345 * right_dual.scalar) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group1, 
            /* e5 */
            this.e5 * right_dual.scalar, 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * float4(right_dual.group2.xyz, right_dual.e45)) + (float4(right_dual.scalar) * this.group3), 
            /* e41, e42, e43 */
            (float3(this.scalar) * right_dual.group0.xyz) + (float3(right_dual.scalar) * this.group4.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group5.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.e5 * right_dual.e41) + (this.e415 * right_dual.scalar), (this.e5 * right_dual.e42) + (this.e425 * right_dual.scalar), (this.e5 * right_dual.e43) + (this.e435 * right_dual.scalar), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) + float4((this.group1.www * right_dual.group2.xyz), this.e321 * right_dual.scalar) - (this.group1.xyzx * right_dual.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group7.xyz) + (this.group1.zxy * right_dual.group0.yzx) - (this.group1.yzx * right_dual.group0.zxy), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group8.xyz) + (this.group1.yzx * right_dual.group2.zxy) - (this.group1.zxy * right_dual.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_dual.e42) + (this.e45 * right_dual.e23) + (this.e23 * right_dual.e45) + (this.e4235 * right_dual.scalar), (this.e15 * right_dual.e43) + (this.e45 * right_dual.e31) + (this.e31 * right_dual.e45) + (this.e4315 * right_dual.scalar), (this.e25 * right_dual.e41) + (this.e45 * right_dual.e12) + (this.e12 * right_dual.e45) + (this.e4125 * right_dual.scalar), -(this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12) - (this.e12 * right_dual.e35)) + (float4(this.scalar) * right_dual.group3) + float4((this.group4.xyz.yzx * right_dual.group2.zxy), this.e3215 * right_dual.scalar) - (right_dual.group2.yzxx * float4(this.group4.xyz.zxy, this.e23)) - float4((this.group3.yzx * right_dual.group0.zxy), this.e31 * right_dual.e25), 
            /* e1234 */
            (this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(VersorOdd other) {
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
            float2(0.0, (this.scalar * right_dual.e12345) + (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) + (this.e1234 * right_dual.e5) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group3, 
            /* e5 */
            this.scalar * right_dual.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * this.group1) - (float4(this.e5) * right_dual.group3), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * right_dual.group3.zxy) - (this.group1.zxy * right_dual.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_dual.e4) + (this.e41 * right_dual.e5), (this.e25 * right_dual.e4) + (this.e42 * right_dual.e5), (this.e35 * right_dual.e4) + (this.e43 * right_dual.e5), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + (float4(this.scalar) * right_dual.group1) - (right_dual.group3.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412 */
            (float3(this.scalar) * right_dual.group0.xyz) + (float3(right_dual.e4) * this.group5.xyz) + (this.group4.xyz.yzx * right_dual.group3.zxy) - (this.group4.xyz.zxy * right_dual.group3.yzx), 
            /* e235, e315, e125 */
            (float3(this.scalar) * right_dual.group2.xyz) + (float3(right_dual.e5) * this.group5.xyz) + (this.group3.zxy * right_dual.group3.yzx) - (this.group3.yzx * right_dual.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (this.group6.zxyw * float4(right_dual.group3.yzx, right_dual.e5)) + float4((this.group7.xyz * right_dual.group2.www), this.e235 * right_dual.e1) + float4((this.group1.zxy * right_dual.group1.yzx), this.e315 * right_dual.e2) + float4((this.group1.www * right_dual.group2.xyz), this.e125 * right_dual.e3) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group1.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((this.group8.xyz * right_dual.group3.www), this.e1 * right_dual.e235) - float4((this.group6.yzx * right_dual.group3.zxy), this.e3 * right_dual.e125), 
            /* e1234 */
            (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321) - (this.e321 * right_dual.e4) - (this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Plane> operator /(Plane lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public Plane bulk_expansion(AntiScalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public Plane bulk_expansion(CircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group2.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public AntiScalar bulk_expansion(DipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            (right_dual.e4 * this.e3215) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public Plane bulk_expansion(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public AntiScalar bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public Plane bulk_expansion(Motor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215)), 
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
            float4(right_dual.scalar) * this.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public AntiScalar bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public AntiScalar bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return AntiScalar(
            /* e12345 */
            (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public Plane bulk_expansion(VersorEven other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public AntiScalar bulk_expansion(VersorOdd other) {
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
        return AntiScalar(
            /* e12345 */
            (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<RoundPoint> operator /(RoundPoint lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Sphere bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e235)), 
            /* e1234 */
            (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public CircleRotor bulk_expansion(AntiDipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(right_dual.e25 * this.e3 * -1.0, right_dual.e35 * this.e1 * -1.0, right_dual.e15 * this.e2 * -1.0, (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4)) + (float4(this.e5) * float4(right_dual.group1.xyz, right_dual.e1234)) + (this.group0.yzxx * float4(right_dual.group2.zxy, right_dual.e4235))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public FlatPoint bulk_expansion(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Line bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line(
            /* e415, e425, e435 */
            (float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e45) * this.group0.xyz), 
            /* e235, e315, e125 */
            (right_dual.group0.zxy * this.group0.yzx) - (right_dual.group0.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Motor bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e45 * this.e1 * -1.0, right_dual.e45 * this.e2 * -1.0, right_dual.e45 * this.e3 * -1.0, (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4)) + (this.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group0.zxy * this.group0.yzx) - (right_dual.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Plane bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Flector bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (this.group0.yzxx * float4(right_dual.group0.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public AntiScalar bulk_expansion(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public RoundPoint bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group0, 
            /* e5 */
            this.e5 * right_dual.scalar
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Circle bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return Circle(
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.xyz.zxy * this.group0.yzx) - (right_dual.group2.xyz.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.zxy * this.group0.yzx) - (right_dual.group2.yzx * this.group0.zxy)), right_dual.scalar * this.e4), 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * float4(this.group0.xyz, this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Sphere bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e321 * this.e5) - (right_dual.e125 * this.e3)) - (this.group0.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((float3(this.e5) * right_dual.group0.xyz), right_dual.e235 * this.e1), 
            /* e1234 */
            (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public DipoleInversion bulk_expansion(DipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (right_dual.group3.zxyw * this.group0.yzxw) - float4((right_dual.group3.yzx * this.group0.zxy), right_dual.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e1 * this.e5 * -1.0, right_dual.e2 * this.e5 * -1.0, right_dual.e3 * this.e5 * -1.0, (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public VersorEven bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), right_dual.e3215 * this.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), right_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<RoundPoint> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Sphere bulk_expansion(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_dual.group0.xyz), -(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)), 
            /* e1234 */
            right_dual.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public DipoleInversion bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_dual.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group1.zxy * this.group0.yzx) - (right_dual.group1.yzx * this.group0.zxy)), right_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group1.xyz)), right_dual.e321 * this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_dual.group0.xyz), -(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Circle bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return Circle(
            /* e423, e431, e412 */
            float3(this.e4) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), -(right_dual.e23 * this.e1) - (right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group0.xyz) + (right_dual.group1.xyz.zxy * this.group0.yzx) - (right_dual.group1.xyz.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public VersorEven bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e4) * float4(right_dual.group0.xyz, right_dual.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), -(right_dual.e23 * this.e1) - (right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_dual.group0.xyz) + (right_dual.group1.zxy * this.group0.yzx) - (right_dual.group1.yzx * this.group0.zxy)), right_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group0, 
            /* e5 */
            right_dual.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * this.group0) - (float4(this.e5) * right_dual.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (right_dual.group1.zxy * this.group0.yzx) - (right_dual.group1.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e15 * this.e4) + (right_dual.e41 * this.e5), (right_dual.e25 * this.e4) + (right_dual.e42 * this.e5), (right_dual.e35 * this.e4) + (right_dual.e43 * this.e5), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e23)), 
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group5.xyz) + (right_dual.group4.xyz.yzx * this.group0.zxy) - (right_dual.group4.xyz.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * right_dual.group5.xyz) + (right_dual.group3.zxy * this.group0.yzx) - (right_dual.group3.yzx * this.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e321 * this.e5) - (right_dual.e125 * this.e3)) - (this.group0.yzxy * float4(right_dual.group6.zxy, right_dual.e315)) - float4((float3(this.e5) * right_dual.group7.xyz), right_dual.e235 * this.e1), 
            /* e1234 */
            (right_dual.e321 * this.e4) + (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Dipole bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e4) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group0.zxy * this.group0.yzx) - (right_dual.group0.yzx * this.group0.zxy)), right_dual.e5 * this.e4), 
            /* e15, e25, e35 */
            (float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public AntiScalar bulk_expansion(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiScalar(
            /* e12345 */
            (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Dipole bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_dual.group0.zxy * this.group0.yzx), right_dual.e5 * this.e4) - (right_dual.group0.yzxw * float4(this.group0.zxy, this.e5)), 
            /* e15, e25, e35 */
            (float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public VersorEven bulk_expansion(VersorEven other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e2 * right_dual.e43 * -1.0, this.e3 * right_dual.e41 * -1.0, this.e1 * right_dual.e42 * -1.0, (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234)) + (this.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e4235)) + (this.group0.wwwy * float4(right_dual.group1.xyz, right_dual.e4315)), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_dual.e15) + (this.e5 * right_dual.e41), (this.e4 * right_dual.e25) + (this.e5 * right_dual.e42), (this.e4 * right_dual.e35) + (this.e5 * right_dual.e43), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group0.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_dual.group1.xyz) + (this.group0.yzx * right_dual.group2.zxy) - (this.group0.zxy * right_dual.group2.yzx)), this.e5 * right_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public DipoleInversion bulk_expansion(VersorOdd other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(right_dual.group3.zxy, right_dual.e5)) - float4((this.group0.zxy * right_dual.group3.yzx), this.e5 * right_dual.e4), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * right_dual.e1 * -1.0, this.e5 * right_dual.e2 * -1.0, this.e5 * right_dual.e3 * -1.0, (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321)) + (this.group0.xyzx * float4(right_dual.group2.www, right_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e3 * right_dual.e425) + (this.e4 * right_dual.e235), (this.e1 * right_dual.e435) + (this.e4 * right_dual.e315), (this.e2 * right_dual.e415) + (this.e4 * right_dual.e125), -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e235)) - float4((float3(this.e5) * right_dual.group0.xyz), this.e2 * right_dual.e315)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Scalar> operator /(Scalar lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public CircleRotor bulk_expansion(AntiCircleRotor other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * right_dual.group2
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public DipoleInversion bulk_expansion(AntiDipoleInversion other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group3
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public DualNum bulk_expansion(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public FlatPoint bulk_expansion(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Flector bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Line bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_dual.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Motor bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * right_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Plane bulk_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Scalar bulk_expansion(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<Scalar> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Dipole bulk_expansion(Circle other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_dual.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkExpansionInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiCircleRotor bulk_expansion(CircleRotor other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.scalar) * right_dual.group2
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<Scalar> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Circle bulk_expansion(Dipole other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_dual.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<Scalar> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiDipoleInversion bulk_expansion(DipoleInversion other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.scalar) * right_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiDualNum bulk_expansion(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<Scalar> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiFlatPoint bulk_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiFlector bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkExpansionInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiLine bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * right_dual.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiMotor bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(this.scalar) * right_dual.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e5 */
            right_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group3, 
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * right_dual.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group6, 
            /* e423, e431, e412 */
            float3(this.scalar) * right_dual.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * right_dual.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group9, 
            /* e1234 */
            right_dual.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkExpansionInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiPlane bulk_expansion(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Scalar> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Sphere bulk_expansion(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group0, 
            /* e1234 */
            this.scalar * right_dual.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkExpansionInfixPartial<Scalar> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public RoundPoint bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0, 
            /* e5 */
            right_dual.e5 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public VersorOdd bulk_expansion(VersorEven other) {
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
            float4(this.scalar) * right_dual.group0, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public VersorEven bulk_expansion(VersorOdd other) {
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
            float4(this.scalar) * right_dual.group0, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_dual.group2, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group3
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Sphere> operator /(Sphere lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Sphere> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public Sphere bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group0, 
            /* e1234 */
            right_dual.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public Sphere bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group0, 
            /* e1234 */
            right_dual.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(DipoleInversion other) {
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
        return AntiScalar(
            /* e12345 */
            (right_dual.e4 * this.e3215) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public Sphere bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group0, 
            /* e1234 */
            right_dual.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public Sphere bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group0, 
            /* e1234 */
            right_dual.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234)), 
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
            float4(right_dual.scalar) * this.group0, 
            /* e1234 */
            right_dual.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return AntiScalar(
            /* e12345 */
            (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public Sphere bulk_expansion(VersorEven other) {
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group0, 
            /* e1234 */
            this.e1234 * right_dual.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(VersorOdd other) {
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
        return AntiScalar(
            /* e12345 */
            (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) + (this.e1234 * right_dual.e5)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<VersorEven> operator /(VersorEven lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Sphere bulk_expansion(AntiCircleRotor other) {
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e235)), 
            /* e1234 */
            (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public CircleRotor bulk_expansion(AntiDipoleInversion other) {
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
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group3.zxy) - (right_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(right_dual.group1.xyz, right_dual.e1234)) + (this.group3.yzxx * float4(right_dual.group2.zxy, right_dual.e4235)) + float4(float3(0.0), (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) - float4((right_dual.group2.yzx * this.group3.zxy), right_dual.e41 * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Flector bulk_expansion(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Motor bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e15 * this.e4, right_dual.e25 * this.e4, right_dual.e35 * this.e4, -(right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321)) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group0.zxy * this.group3.yzx) - (right_dual.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Motor bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            (this.group3.wwwx * float4(right_dual.group0.xyz, right_dual.e4235)) + float4(float3(0.0), (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321)) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group0.zxy * this.group3.yzx) - (right_dual.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Plane bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (this.group3.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Flector bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e235 * this.e4) + (right_dual.e5 * this.e423), (right_dual.e315 * this.e4) + (right_dual.e5 * this.e431), (right_dual.e125 * this.e4) + (right_dual.e5 * this.e412), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) + float4((right_dual.group0.yzx * this.group3.zxy), right_dual.e5 * this.e321) - (this.group3.yzxx * float4(right_dual.group0.zxy, right_dual.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public AntiScalar bulk_expansion(AntiPlane other) {
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar(
            /* e12345 */
            (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public VersorEven bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.scalar) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_dual.scalar) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public CircleRotor bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group3.zxy) - (right_dual.group0.xyz.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.e23 * this.e5) + (right_dual.e35 * this.e2), (right_dual.e31 * this.e5) + (right_dual.e15 * this.e3), (right_dual.e12 * this.e5) + (right_dual.e25 * this.e1), -(right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) - float4((right_dual.group2.xyz.yzx * this.group3.zxy), right_dual.e41 * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public VersorEven bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((right_dual.e23 * this.e4) + (right_dual.scalar * this.e423), (right_dual.e31 * this.e4) + (right_dual.scalar * this.e431), (right_dual.e12 * this.e4) + (right_dual.scalar * this.e412), -(right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) + float4((right_dual.group0.xyz.yzx * this.group3.zxy), right_dual.scalar * this.e12345) - float4((right_dual.group0.xyz.zxy * this.group3.yzx), right_dual.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e15 * this.e4) + (right_dual.scalar * this.e415), (right_dual.e25 * this.e4) + (right_dual.scalar * this.e425), (right_dual.e35 * this.e4) + (right_dual.scalar * this.e435), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) + float4((right_dual.group0.xyz * this.group2.www), right_dual.scalar * this.e321) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.zxy * this.group3.yzx) - (right_dual.group2.yzx * this.group3.zxy)), right_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Sphere bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e321 * this.e5) - (right_dual.e125 * this.e3)) - (this.group3.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((right_dual.group0.xyz * this.group2.www), right_dual.e235 * this.e1), 
            /* e1234 */
            (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion bulk_expansion(DipoleInversion other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (right_dual.group3.zxyw * this.group3.yzxw) - float4((right_dual.group3.yzx * this.group3.zxy), right_dual.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e423)) + float4(float3(0.0), (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)) - float4((this.group2.www * right_dual.group3.xyz), right_dual.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_dual.group1.yzx * this.group3.zxy), right_dual.e1 * this.e235) + float4((this.group3.www * right_dual.group2.xyz), right_dual.e2 * this.e315) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e235)) - float4((right_dual.group3.zxy * this.group1.yzx), right_dual.e125 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public VersorEven bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.group0.xy.yy, right_dual.scalar, (right_dual.e3215 * this.e4) + (right_dual.scalar * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_dual.scalar) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Sphere bulk_expansion(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_dual.group0.xyz), -(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)), 
            /* e1234 */
            right_dual.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_dual.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group1.zxy * this.group3.yzx) - (right_dual.group1.yzx * this.group3.zxy)), right_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            (this.group3 * float4(right_dual.group1.www, right_dual.e321)) + float4(float3(0.0), -(right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)) - (right_dual.group1.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + (right_dual.group1.wwwz * float4(this.group0.xyz, this.e125)) + float4(float3(0.0), (right_dual.e5 * this.e321) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)) + float4((this.group3.www * right_dual.group0.xyz), right_dual.e1 * this.e235) - float4((right_dual.group1.zxy * this.group1.yzx), right_dual.e235 * this.e1)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public CircleRotor bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), -(right_dual.e23 * this.e1) - (right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.e23 * this.e5) + (right_dual.e35 * this.e2), (right_dual.e31 * this.e5) + (right_dual.e15 * this.e3), (right_dual.e12 * this.e5) + (right_dual.e25 * this.e1), -(right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) - float4((right_dual.group1.xyz.yzx * this.group3.zxy), right_dual.e23 * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public VersorEven bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_dual.group0 * float4(this.group3.www, this.e12345)) + float4(float3(0.0), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) + float4((right_dual.group0.www * this.group0.xyz), right_dual.e3215 * this.e4), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e15 * this.e4, right_dual.e25 * this.e4, right_dual.e35 * this.e4, -(right_dual.e23 * this.e1) - (right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) + (float4(right_dual.scalar) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_dual.group0.xyz) + (right_dual.group1.zxy * this.group3.yzx) - (right_dual.group1.yzx * this.group3.zxy)), right_dual.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(0.0, (right_dual.scalar * this.e12345) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group3, 
            /* e5 */
            right_dual.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * this.group3) - (float4(this.e5) * right_dual.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12 */
            (right_dual.group1.zxy * this.group3.yzx) - (right_dual.group1.yzx * this.group3.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e15 * this.e4) + (right_dual.e41 * this.e5), (right_dual.e25 * this.e4) + (right_dual.e42 * this.e5), (right_dual.e35 * this.e4) + (right_dual.e43 * this.e5), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) + (float4(right_dual.scalar) * this.group1) - (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e23)), 
            /* e423, e431, e412 */
            (float3(right_dual.scalar) * this.group0.xyz) + (float3(this.e4) * right_dual.group5.xyz) + (right_dual.group4.xyz.yzx * this.group3.zxy) - (right_dual.group4.xyz.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_dual.group5.xyz) + (right_dual.group3.zxy * this.group3.yzx) - (right_dual.group3.yzx * this.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_dual.group8.xyz * this.group3.www), right_dual.e1 * this.e235) + float4((right_dual.group6.yzx * this.group3.zxy), right_dual.e3 * this.e125) - (right_dual.group6.zxyw * float4(this.group3.yzx, this.e5)) - float4((right_dual.group7.xyz * this.group2.www), right_dual.e235 * this.e1) - float4((right_dual.group1.zxy * this.group1.yzx), right_dual.e315 * this.e2) - float4((right_dual.group1.www * this.group2.xyz), right_dual.e125 * this.e3), 
            /* e1234 */
            (right_dual.e321 * this.e4) + (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group0.zxy * this.group3.yzx) - (right_dual.group0.yzx * this.group3.zxy)), right_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e5 * this.e1, right_dual.e5 * this.e2, right_dual.e5 * this.e3, -(right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)) - (right_dual.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group0.yzxx * float4(this.group1.zxy, this.e235)) + (right_dual.group0.wwwy * float4(this.group0.xyz, this.e315))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public AntiScalar bulk_expansion(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return AntiScalar(
            /* e12345 */
            (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_dual.group0.zxy * this.group3.yzx), right_dual.e5 * this.e4) - (right_dual.group0.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e5 * this.e1, right_dual.e5 * this.e2, right_dual.e5 * this.e3, -(right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)) - (right_dual.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual.e3 * this.e425) - (right_dual.e4 * this.e235), -(right_dual.e1 * this.e435) - (right_dual.e4 * this.e315), -(right_dual.e2 * this.e415) - (right_dual.e4 * this.e125), (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4((float3(right_dual.e5) * this.group0.xyz), right_dual.e2 * this.e315)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public VersorEven bulk_expansion(VersorEven other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_dual.scalar) * this.group0) + (this.group3.wwwx * float4(right_dual.group1.xyz, right_dual.e4235)) + float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) + float4((this.group3.zxy * right_dual.group0.yzx), this.e5 * right_dual.e1234) - float4((this.group3.yzx * right_dual.group0.zxy), this.e423 * right_dual.e15), 
            /* e415, e425, e435, e321 */
            float4((this.e5 * right_dual.e41) + (this.e4 * right_dual.e15), (this.e5 * right_dual.e42) + (this.e4 * right_dual.e25), (this.e5 * right_dual.e43) + (this.e4 * right_dual.e35), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) + (float4(right_dual.scalar) * this.group1) - (this.group3.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_dual.group2.zxy) - (this.group3.zxy * right_dual.group2.yzx)), this.e5 * right_dual.scalar), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion bulk_expansion(VersorOdd other) {
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
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (this.group3.yzxw * float4(right_dual.group3.zxy, right_dual.e5)) - (right_dual.group3.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_dual.group2.www, right_dual.e423)) + float4(float3(0.0), (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)) - (right_dual.group3.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_dual.group1.yzx * this.group3.zxy), right_dual.e5 * this.e321) + float4((right_dual.group2.www * this.group0.xyz), right_dual.e2 * this.e315) + float4((this.group3.www * right_dual.group2.xyz), right_dual.e1 * this.e235) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e235)) - float4((right_dual.group3.zxy * this.group1.yzx), right_dual.e315 * this.e2) - float4((right_dual.group3.www * this.group2.xyz), right_dual.e125 * this.e3)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<VersorOdd> operator /(VersorOdd lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public CircleRotor bulk_expansion(AntiCircleRotor other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_dual.group2.xyz), (right_dual.e12345 * this.scalar) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public DipoleInversion bulk_expansion(AntiDipoleInversion other) {
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
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group2.xyz), (right_dual.e1234 * this.scalar) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42) + (right_dual.e4235 * this.scalar), (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43) + (right_dual.e4315 * this.scalar), (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41) + (right_dual.e4125 * this.scalar), -(right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23)) - float4((right_dual.group2.yzx * this.group0.zxy), right_dual.e31 * this.e25)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Motor bulk_expansion(AntiDualNum other) {
        DualNum right_dual = DualNum(
            /* e5, e12345 */
            other.group0.xy
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual.group0.xy.xx, right_dual.e5, (right_dual.e5 * this.e1234) + (right_dual.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Flector bulk_expansion(AntiFlatPoint other) {
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e35 * this.e42) + (right_dual.e45 * this.e23), (right_dual.e15 * this.e43) + (right_dual.e45 * this.e31), (right_dual.e25 * this.e41) + (right_dual.e45 * this.e12), -(right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - (right_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Flector bulk_expansion(AntiFlector other) {
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e45 * this.e23) + (right_dual.e4235 * this.scalar), (right_dual.e45 * this.e31) + (right_dual.e4315 * this.scalar), (right_dual.e45 * this.e12) + (right_dual.e4125 * this.scalar), -(right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + (this.group0.yzxw * float4(right_dual.group0.zxy, right_dual.e3215)) - (right_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Motor bulk_expansion(AntiLine other) {
        Line right_dual = Line(
            /* e415, e425, e435 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_dual.group0.xyz), -(right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Motor bulk_expansion(AntiMotor other) {
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e1234)) + (float4(this.scalar) * right_dual.group0) + float4(float3(0.0), -(right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_dual.group1.xyz)), right_dual.e5 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Plane bulk_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_expansion(AntiScalar other) {
        Scalar right_dual = Scalar(
            /* scalar */
            other.e12345 * -1.0
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public DipoleInversion bulk_expansion(Circle other) {
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            other.group2.xyz
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group2.yzx), right_dual.e15 * this.e23) - float4((right_dual.group2.xyz.yzx * this.group0.zxy), right_dual.e25 * this.e31)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_expansion(CircleRotor other) {
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_dual.group0.xyz)), right_dual.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(right_dual.scalar) * this.group1) + (float4(this.scalar) * right_dual.group1), 
            /* e15, e25, e35, e1234 */
            float4(right_dual.scalar * this.e15, right_dual.scalar * this.e25, right_dual.scalar * this.e35, -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)) + (right_dual.group2 * float4(this.group0.www, this.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42) + (right_dual.scalar * this.e4235), (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43) + (right_dual.scalar * this.e4315), (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41) + (right_dual.scalar * this.e4125), -(right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.scalar * this.e3215) - (this.group2.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23)) - float4((right_dual.group2.yzx * this.group0.zxy), right_dual.e31 * this.e25)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public CircleRotor bulk_expansion(Dipole other) {
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * right_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven bulk_expansion(DipoleInversion other) {
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
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_dual.group3.zxyy * float4(this.group0.yzx, this.e4315)) + float4(float3(0.0), (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) + float4((right_dual.group0.xyz * this.group0.www), right_dual.e4 * this.e3215) + float4((right_dual.group2.www * this.group1.xyz), right_dual.e1 * this.e4235) - float4((right_dual.group3.yzx * this.group0.zxy), right_dual.e423 * this.e15), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e4 * this.e15) + (right_dual.e5 * this.e41), (right_dual.e4 * this.e25) + (right_dual.e5 * this.e42), (right_dual.e4 * this.e35) + (right_dual.e5 * this.e43), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + (float4(this.scalar) * right_dual.group1) - (right_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_dual.group2.xyz) + (right_dual.group3.yzx * this.group2.zxy) - (right_dual.group3.zxy * this.group2.yzx)), right_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_dual.group3.xyz, right_dual.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_expansion(DualNum other) {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            other.group0.xy * float2(-1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.group0.xy.yy, right_dual.scalar, (right_dual.e3215 * this.scalar) + (right_dual.scalar * this.e3215)) * float4(this.group3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public CircleRotor bulk_expansion(FlatPoint other) {
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.e321 * this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_dual.group0.xyz), -(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven bulk_expansion(Flector other) {
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_dual.group1.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) - (this.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e235)), 
            /* e415, e425, e435, e321 */
            (this.group0 * float4(right_dual.group1.www, right_dual.e321)) + float4(float3(0.0), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_dual.group0.xyz) + (right_dual.group1.yzx * this.group2.zxy) - (right_dual.group1.zxy * this.group2.yzx)), right_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public DipoleInversion bulk_expansion(Line other) {
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group1.xyz), -(right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e35 * this.e42), (right_dual.e31 * this.e45) + (right_dual.e15 * this.e43), (right_dual.e12 * this.e45) + (right_dual.e25 * this.e41), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group1.xyz.yzx * this.group0.zxy), right_dual.e23 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_expansion(Motor other) {
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(right_dual.scalar) * this.group1.xyz) + (float3(this.scalar) * right_dual.group0.xyz)), right_dual.scalar * this.e45), 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e15 * this.scalar, right_dual.e25 * this.scalar, right_dual.e35 * this.scalar, -(right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)) + (float4(right_dual.scalar) * this.group2), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e35 * this.e42, right_dual.e15 * this.e43, right_dual.e25 * this.e41, -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + (right_dual.group0 * float4(this.group1.www, this.e3215)) + float4((right_dual.group0.www * this.group3.xyz), right_dual.e3215 * this.scalar) - float4((right_dual.group1.yzx * this.group0.zxy), right_dual.e23 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public MultiVector bulk_expansion(MultiVector other) {
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
            float2(right_dual.scalar * this.scalar, (right_dual.e12345 * this.scalar) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e5 */
            right_dual.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            (float4(right_dual.scalar) * float4(this.group2.xyz, this.e45)) + (float4(this.scalar) * right_dual.group3), 
            /* e41, e42, e43 */
            (float3(right_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_dual.group4.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual.scalar) * this.group1.xyz) + (float3(this.scalar) * right_dual.group5.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e5 * this.e41) + (right_dual.e415 * this.scalar), (right_dual.e5 * this.e42) + (right_dual.e425 * this.scalar), (right_dual.e5 * this.e43) + (right_dual.e435 * this.scalar), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + float4((right_dual.group1.www * this.group2.xyz), right_dual.e321 * this.scalar) - (right_dual.group1.xyzx * this.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(right_dual.e4) * this.group1.xyz) + (float3(this.scalar) * right_dual.group7.xyz) + (right_dual.group1.zxy * this.group0.yzx) - (right_dual.group1.yzx * this.group0.zxy), 
            /* e235, e315, e125 */
            (float3(right_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_dual.group8.xyz) + (right_dual.group1.yzx * this.group2.zxy) - (right_dual.group1.zxy * this.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e35 * this.e42) + (right_dual.e45 * this.e23) + (right_dual.e23 * this.e45) + (right_dual.e4235 * this.scalar), (right_dual.e15 * this.e43) + (right_dual.e45 * this.e31) + (right_dual.e31 * this.e45) + (right_dual.e4315 * this.scalar), (right_dual.e25 * this.e41) + (right_dual.e45 * this.e12) + (right_dual.e12 * this.e45) + (right_dual.e4125 * this.scalar), -(right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12) - (right_dual.e12 * this.e35)) + (float4(right_dual.scalar) * this.group3) + float4((right_dual.group4.xyz.yzx * this.group2.zxy), right_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_dual.group4.xyz.zxy, right_dual.e23)) - float4((right_dual.group3.yzx * this.group0.zxy), right_dual.e31 * this.e25), 
            /* e1234 */
            (right_dual.scalar * this.e1234) + (right_dual.e1234 * this.scalar) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven bulk_expansion(Plane other) {
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.e2 * this.e43 * -1.0, right_dual.e3 * this.e41 * -1.0, right_dual.e1 * this.e42 * -1.0, (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)) + (right_dual.group0.zxyx * float4(this.group0.yzx, this.e4235)), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e5 * this.e41, right_dual.e5 * this.e42, right_dual.e5 * this.e43, -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.e5) * this.group1.xyz) + (right_dual.group0.yzx * this.group2.zxy) - (right_dual.group0.zxy * this.group2.yzx)), right_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Sphere bulk_expansion(RoundPoint other) {
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * -1.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group0, 
            /* e1234 */
            right_dual.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven bulk_expansion(Sphere other) {
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual.e2 * this.e43 * -1.0, right_dual.e3 * this.e41 * -1.0, right_dual.e1 * this.e42 * -1.0, (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234)) + (right_dual.group0.zxyx * float4(this.group0.yzx, this.e4235)) + (right_dual.group0.wwwy * float4(this.group1.xyz, this.e4315)), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e4 * this.e15) + (right_dual.e5 * this.e41), (right_dual.e4 * this.e25) + (right_dual.e5 * this.e42), (right_dual.e4 * this.e35) + (right_dual.e5 * this.e43), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.e5) * this.group1.xyz) + (right_dual.group0.yzx * this.group2.zxy) - (right_dual.group0.zxy * this.group2.yzx)), right_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_expansion(VersorEven other) {
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
            float4(((float3(right_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_dual.group0.xyz)), right_dual.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(right_dual.scalar) * this.group1) + (float4(this.scalar) * right_dual.group1), 
            /* e15, e25, e35, e1234 */
            (float4(right_dual.scalar) * this.group2) + (float4(this.scalar) * right_dual.group2) + float4(float3(0.0), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42) + (right_dual.e4235 * this.scalar), (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43) + (right_dual.e4315 * this.scalar), (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41) + (right_dual.e4125 * this.scalar), -(right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + (right_dual.group0.yzxw * float4(this.group2.zxy, this.e3215)) + float4((right_dual.group0.www * this.group3.xyz), right_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_dual.group0.zxy, right_dual.e23)) - float4((right_dual.group2.yzx * this.group0.zxy), right_dual.e31 * this.e25)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven bulk_expansion(VersorOdd other) {
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
            (float4(this.scalar) * right_dual.group0) + (right_dual.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) + float4((right_dual.group3.zxy * this.group0.yzx), right_dual.e5 * this.e1234) - float4((right_dual.group3.yzx * this.group0.zxy), right_dual.e423 * this.e15), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e5 * this.e41) + (right_dual.e4 * this.e15), (right_dual.e5 * this.e42) + (right_dual.e4 * this.e25), (right_dual.e5 * this.e43) + (right_dual.e4 * this.e35), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + (float4(this.scalar) * right_dual.group1) - (right_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_dual.group2.xyz) + (right_dual.group3.yzx * this.group2.zxy) - (right_dual.group3.zxy * this.group2.yzx)), right_dual.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group3
        );
    }
}

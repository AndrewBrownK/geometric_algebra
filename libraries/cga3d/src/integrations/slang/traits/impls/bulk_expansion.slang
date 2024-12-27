extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<AntiCircleRotor> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension AntiCircleRotor: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * right_dual.group2.xyz), (this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
extension AntiCircleRotor: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.www * right_dual.group2.xyz), (this.scalar * right_dual.e1234) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42) + (this.scalar * right_dual.e4235), (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43) + (this.scalar * right_dual.e4315), (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41) + (this.scalar * right_dual.e4125), -(this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + float4((this.group0.xyz.yzx * right_dual.group2.zxy), this.scalar * right_dual.e3215) - (right_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((right_dual.group0.xyz.zxy * this.group2.yzx), this.e31 * right_dual.e25)
        );
    }
}
extension AntiCircleRotor: BulkExpansion<AntiDualNum> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.group0.xy.xx, right_dual.e5, right_dual.e12345) * float4(this.group0.xyz, this.scalar), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
extension AntiCircleRotor: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension AntiCircleRotor: BulkExpansion<AntiFlector> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.scalar * right_dual.e4235), (this.e31 * right_dual.e45) + (this.scalar * right_dual.e4315), (this.e12 * right_dual.e45) + (this.scalar * right_dual.e4125), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) + float4((this.group0.xyz.yzx * right_dual.group0.zxy), this.scalar * right_dual.e3215) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension AntiCircleRotor: BulkExpansion<AntiLine> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_dual.group0.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.scalar * right_dual.e415, this.scalar * right_dual.e425, this.scalar * right_dual.e435, -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)) + float4((this.group0.xyz * right_dual.group1.www), this.scalar * right_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_dual.group1.xyz) + (float3(right_dual.e5) * this.group1.xyz)), this.scalar * right_dual.e5)
        );
    }
}
extension AntiCircleRotor: BulkExpansion<AntiPlane> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension AntiCircleRotor: BulkExpansion<AntiScalar> {
    associatedtype Output = AntiCircleRotor;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_dual.scalar) * this.group2
        );
    }
}
extension AntiCircleRotor: BulkExpansion<Circle> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41), -(this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group0.xyz.zxy * right_dual.group2.xyz.yzx), this.e23 * right_dual.e15) - float4((right_dual.group0.xyz.zxy * this.group2.yzx), this.e31 * right_dual.e25)
        );
    }
}
extension AntiCircleRotor: BulkExpansion<CircleRotor> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension AntiCircleRotor: BulkExpansion<Dipole> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
extension AntiCircleRotor: BulkExpansion<DipoleInversion> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension AntiCircleRotor: BulkExpansion<DualNum> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension AntiCircleRotor: BulkExpansion<FlatPoint> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.scalar * right_dual.e321), 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * right_dual.group0.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e45 * right_dual.e321))
        );
    }
}
extension AntiCircleRotor: BulkExpansion<Flector> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension AntiCircleRotor: BulkExpansion<Line> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group1.xyz), -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e45 * right_dual.e23), (this.e43 * right_dual.e15) + (this.e45 * right_dual.e31), (this.e41 * right_dual.e25) + (this.e45 * right_dual.e12), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group0.xyz.zxy * right_dual.group1.xyz.yzx), this.e15 * right_dual.e23)
        );
    }
}
extension AntiCircleRotor: BulkExpansion<Motor> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension AntiCircleRotor: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar * right_dual.scalar, (this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * right_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * right_dual.group3) + (float4(right_dual.scalar) * float4(this.group2.xyz, this.e45)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * right_dual.group4.xyz) + (float3(right_dual.scalar) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_dual.e4) + (this.scalar * right_dual.e415), (this.e25 * right_dual.e4) + (this.scalar * right_dual.e425), (this.e35 * right_dual.e4) + (this.scalar * right_dual.e435), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + float4((float3(right_dual.e5) * this.group0.xyz), this.scalar * right_dual.e321) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.scalar) * right_dual.group7.xyz) + (float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group1.zxy) - (this.group0.xyz.zxy * right_dual.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.scalar) * right_dual.group8.xyz) + (float3(right_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_dual.group1.yzx) - (this.group2.yzx * right_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42) + (this.scalar * right_dual.e4235), (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43) + (this.scalar * right_dual.e4315), (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41) + (this.scalar * right_dual.e4125), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e35 * right_dual.e12)) + float4((this.group0.xyz.yzx * right_dual.group3.zxy), this.scalar * right_dual.e3215) - (this.group2.yzxy * float4(right_dual.group4.xyz.zxy, right_dual.e31)) - float4((this.group0.xyz.zxy * right_dual.group3.yzx), this.e15 * right_dual.e23), 
            /* e1234, 0, 0, 0 */
            float4((this.scalar * right_dual.e1234) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkExpansion<Plane> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * right_dual.e5, this.e42 * right_dual.e5, this.e43 * right_dual.e5, -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * right_dual.group0.yzx), 0.0) - float4((this.group2.yzx * right_dual.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group0
        );
    }
}
extension AntiCircleRotor: BulkExpansion<RoundPoint> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkExpansion<Sphere> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_dual.group0.yzx) - (this.group2.yzx * right_dual.group0.zxy)), this.scalar * right_dual.e4), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(right_dual.group0.xyz, right_dual.e5)
        );
    }
}
extension AntiCircleRotor: BulkExpansion<VersorEven> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension AntiCircleRotor: BulkExpansion<VersorOdd> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<AntiDipoleInversion> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension AntiDipoleInversion: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_dual.e235) + (this.e3 * right_dual.e425), (this.e4 * right_dual.e315) + (this.e1 * right_dual.e435), (this.e4 * right_dual.e125) + (this.e2 * right_dual.e415), -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group3.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((right_dual.group0.xyz * this.group3.www), this.e1 * right_dual.e235), 
            /* e1234, 0, 0, 0 */
            float4((this.e4 * right_dual.e321) + (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group3.zxy) - (right_dual.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_dual.e15) + (this.e5 * right_dual.e41), (this.e4 * right_dual.e25) + (this.e5 * right_dual.e42), (this.e4 * right_dual.e35) + (this.e5 * right_dual.e43), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group3.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group3.wwwx * float4(right_dual.group1.xyz, right_dual.e4235)) + float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) + float4((this.group3.yzx * right_dual.group2.zxy), this.e4 * right_dual.e3215) - (right_dual.group2.yzxx * float4(this.group3.zxy, this.e423))
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<AntiDualNum> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e4 * right_dual.e15, this.e4 * right_dual.e25, this.e4 * right_dual.e35, -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * right_dual.group0.zxy) - (this.group3.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<AntiFlector> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (float4(this.e4) * float4(right_dual.group0.xyz, right_dual.e3215)) + float4(float3(0.0), (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * right_dual.group0.zxy) - (this.group3.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<AntiLine> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_dual.e235) + (this.e3 * right_dual.e425), (this.e4 * right_dual.e315) + (this.e1 * right_dual.e435), (this.e4 * right_dual.e125) + (this.e2 * right_dual.e415), -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) - (this.group3.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235))
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<AntiMotor> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_dual.e235) + (this.e3 * right_dual.e425), (this.e4 * right_dual.e315) + (this.e1 * right_dual.e435), (this.e4 * right_dual.e125) + (this.e2 * right_dual.e415), -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) - (this.group3.yzxx * float4(right_dual.group0.zxy, right_dual.e235))
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<AntiPlane> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4 * right_dual.e3215) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<AntiScalar> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_dual.scalar) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<Circle> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group3.zxy) - (right_dual.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_dual.e15) + (this.e5 * right_dual.e41), (this.e4 * right_dual.e25) + (this.e5 * right_dual.e42), (this.e4 * right_dual.e35) + (this.e5 * right_dual.e43), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group3.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_dual.e35) + (this.e5 * right_dual.e23), (this.e3 * right_dual.e15) + (this.e5 * right_dual.e31), (this.e1 * right_dual.e25) + (this.e5 * right_dual.e12), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((right_dual.group2.xyz.yzx * this.group3.zxy), this.e423 * right_dual.e15)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<CircleRotor> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension AntiDipoleInversion: BulkExpansion<Dipole> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_dual.e235) + (this.e3 * right_dual.e425), (this.e4 * right_dual.e315) + (this.e1 * right_dual.e435), (this.e4 * right_dual.e125) + (this.e2 * right_dual.e415), -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group3.wwwx * float4(right_dual.group0.xyz, right_dual.e235)) - float4((this.group3.yzx * right_dual.group1.zxy), this.e2 * right_dual.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e4 * right_dual.e321) + (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (right_dual.group3.zxyw * float4(this.group3.yzx, this.e4)) - (this.group3.zxyw * float4(right_dual.group3.yzx, right_dual.e4)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e423)) + float4(float3(0.0), (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e4 * this.e321) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)) - float4((this.group3.www * right_dual.group3.xyz), right_dual.e1 * this.e423), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group3.yzxw * this.group1.zxyw) + (this.group2.wwwz * float4(right_dual.group2.xyz, right_dual.e3)) + float4((this.group0.xyz * right_dual.group3.www), right_dual.e1 * this.e235) + float4((right_dual.group1.yzx * this.group3.zxy), right_dual.e2 * this.e315) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e235)) - float4((right_dual.group3.zxy * this.group1.yzx), right_dual.e125 * this.e3)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<DualNum> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension AntiDipoleInversion: BulkExpansion<FlatPoint> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group2.www * right_dual.group0.xyz), -(this.e1 * right_dual.e235) - (this.e2 * right_dual.e315) - (this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)), 
            /* e1234, 0, 0, 0 */
            float4(this.e4 * right_dual.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<Flector> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_dual.group1.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * right_dual.group1.zxy) - (this.group3.zxy * right_dual.group1.yzx)), this.e4 * right_dual.e5), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(this.e431 * right_dual.e2) - (this.e412 * right_dual.e3)) + float4((right_dual.group1.www * this.group3.xyz), this.e4 * right_dual.e321) - (right_dual.group1.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(right_dual.group0.xyz, right_dual.e2)) + (right_dual.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * right_dual.e3) - (this.e2 * right_dual.e315) - (this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - float4((this.group1.yzx * right_dual.group1.zxy), this.e1 * right_dual.e235)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<Line> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), -(this.e1 * right_dual.e23) - (this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_dual.e35) + (this.e5 * right_dual.e23), (this.e3 * right_dual.e15) + (this.e5 * right_dual.e31), (this.e1 * right_dual.e25) + (this.e5 * right_dual.e12), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12)) - float4((right_dual.group1.xyz.yzx * this.group3.zxy), this.e423 * right_dual.e15)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<Motor> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension AntiDipoleInversion: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e4 * right_dual.e3215) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * float4(this.group3.xyz, this.e4), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * float4(this.group3.xyz, this.e4)) - (float4(this.e5) * right_dual.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group3.yzx * right_dual.group1.zxy) - (this.group3.zxy * right_dual.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_dual.e15) + (this.e5 * right_dual.e41), (this.e4 * right_dual.e25) + (this.e5 * right_dual.e42), (this.e4 * right_dual.e35) + (this.e5 * right_dual.e43), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) + (float4(right_dual.scalar) * this.group1) - float4((right_dual.group3.www * this.group3.xyz), this.e1 * right_dual.e23), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group0.xyz) + (right_dual.group4.xyz.yzx * this.group3.zxy) - (right_dual.group4.xyz.zxy * this.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_dual.group3.zxy) - (this.group3.zxy * right_dual.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_dual.group8.xyz * this.group2.www), this.e235 * right_dual.e1) + float4((this.group3.zxy * right_dual.group6.yzx), this.e125 * right_dual.e3) - (this.group3.yzxw * right_dual.group6.zxyw) - (this.group3.wwwx * float4(right_dual.group7.xyz, right_dual.e235)) - float4((this.group1.yzx * right_dual.group1.zxy), this.e2 * right_dual.e315) - float4((right_dual.group1.www * this.group2.xyz), this.e3 * right_dual.e125), 
            /* e1234, 0, 0, 0 */
            float4((this.e4 * right_dual.e321) + (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) - (this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<Plane> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * right_dual.group0.zxy) - (this.group3.zxy * right_dual.group0.yzx)), this.e4 * right_dual.e5), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * right_dual.e5, this.e2 * right_dual.e5, this.e3 * right_dual.e5, -(this.e431 * right_dual.e2) - (this.e412 * right_dual.e3)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * right_dual.e3 * -1.0, this.e435 * right_dual.e1 * -1.0, this.e415 * right_dual.e2 * -1.0, (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4 * right_dual.e3215) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<Sphere> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * right_dual.group0.zxy), this.e4 * right_dual.e5) - (this.group3.zxyw * right_dual.group0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * right_dual.e5, this.e2 * right_dual.e5, this.e3 * right_dual.e5, -(this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
extension AntiDipoleInversion: BulkExpansion<VersorEven> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension AntiDipoleInversion: BulkExpansion<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * right_dual.group3.zxy), this.e4 * right_dual.e5) - (this.group3.zxyw * right_dual.group3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4)) + float4((right_dual.group2.www * this.group3.xyz), this.e4 * right_dual.e321) - (right_dual.group3.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(right_dual.group2.xyz, right_dual.e2)) + (right_dual.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4((this.group3.zxy * right_dual.group1.yzx), this.e125 * right_dual.e3) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group3.yzxz * float4(right_dual.group1.zxy, right_dual.e125)) - float4((this.group1.yzx * right_dual.group3.zxy), this.e1 * right_dual.e235) - float4((right_dual.group3.www * this.group2.xyz), this.e2 * right_dual.e315)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<AntiDualNum> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension AntiDualNum: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * right_dual.group2
        );
    }
}
extension AntiDualNum: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group3
        );
    }
}
extension AntiDualNum: BulkExpansion<AntiDualNum> {
    associatedtype Output = DualNum;
    func bulk_expansion(other: AntiDualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.scalar) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = FlatPoint;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<AntiFlector> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
extension AntiDualNum: BulkExpansion<AntiLine> {
    associatedtype Output = Line;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_dual.group1.xyz, 0.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * right_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
extension AntiDualNum: BulkExpansion<AntiPlane> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<AntiScalar> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: AntiScalar) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.e12345 * -1.0) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<Circle> {
    associatedtype Output = Dipole;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * right_dual.group2.xyz, 0.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<CircleRotor> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension AntiDualNum: BulkExpansion<Dipole> {
    associatedtype Output = Circle;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_dual.group2.xyz, 0.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<DipoleInversion> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension AntiDualNum: BulkExpansion<DualNum> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2((right_dual.e3215 * this.scalar) + (right_dual.scalar * this.e3215), right_dual.scalar * this.scalar), 0.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<FlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: FlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
extension AntiDualNum: BulkExpansion<Flector> {
    associatedtype Output = AntiFlector;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
extension AntiDualNum: BulkExpansion<Line> {
    associatedtype Output = AntiLine;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * right_dual.group1.xyz, 0.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_dual.scalar) + (this.scalar * right_dual.e3215)) * float4(right_dual.group1.xyz, 1.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar * right_dual.scalar, (this.e3215 * right_dual.e4) + (this.scalar * right_dual.e12345)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * right_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_dual.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_dual.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_dual.scalar) + (this.scalar * right_dual.e3215)) * float4(right_dual.group9.xyz, 1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<Plane> {
    associatedtype Output = AntiPlane;
    func bulk_expansion(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension AntiDualNum: BulkExpansion<RoundPoint> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkExpansion<Sphere> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension AntiDualNum: BulkExpansion<VersorEven> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension AntiDualNum: BulkExpansion<VersorOdd> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<AntiFlatPoint> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension AntiFlatPoint: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkExpansion<AntiDualNum> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: AntiDualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.e3215 * this.e321, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkExpansion<AntiFlector> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkExpansion<AntiMotor> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: AntiMotor) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e321 * other.e3215, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkExpansion<AntiScalar> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: AntiScalar) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension AntiFlatPoint: BulkExpansion<Circle> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkExpansion<CircleRotor> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.scalar * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.group2.www * this.group0.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e45 * this.e321))
        );
    }
}
extension AntiFlatPoint: BulkExpansion<DipoleInversion> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.group2.www * this.group0.xyz * float3(-1.0)), (right_dual.e1 * this.e235) + (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)), 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e4 * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkExpansion<DualNum> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: DualNum) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension AntiFlatPoint: BulkExpansion<Flector> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2((this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkExpansion<Motor> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: Motor) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
extension AntiFlatPoint: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_dual.scalar), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5)), 
            /* e1234, 0, 0, 0 */
            float4(this.e321 * right_dual.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkExpansion<Plane> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2((this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkExpansion<Sphere> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5)), 
            /* e1234, 0, 0, 0 */
            float4(this.e321 * right_dual.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkExpansion<VersorEven> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_dual.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(right_dual.scalar) * this.group0.xyz), -(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45))
        );
    }
}
extension AntiFlatPoint: BulkExpansion<VersorOdd> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5)), 
            /* e1234, 0, 0, 0 */
            float4(this.e321 * right_dual.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<AntiFlector> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension AntiFlector: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group1.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((right_dual.group0.xyz * this.group1.www), this.e1 * right_dual.e235), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((right_dual.group0.xyz.yzx * this.group1.zxy) - (right_dual.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_dual.e41, this.e5 * right_dual.e42, this.e5 * right_dual.e43, -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group1.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group1.yzxx * float4(right_dual.group2.zxy, right_dual.e4235)) + (this.group1.wwwy * float4(right_dual.group1.xyz, right_dual.e4315)) + float4(float3(0.0), (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) - float4((this.group1.zxy * right_dual.group2.yzx), this.e235 * right_dual.e41)
        );
    }
}
extension AntiFlector: BulkExpansion<AntiDualNum> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.xx, right_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e321 * right_dual.e5)
        );
    }
}
extension AntiFlector: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e45) * float4(this.group1.xyz, this.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * right_dual.group0.zxy) - (this.group1.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: BulkExpansion<AntiFlector> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.e45) * this.group1.xyz * float3(-1.0)), (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) - (this.e321 * right_dual.e45)), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * right_dual.group0.zxy) - (this.group1.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: BulkExpansion<AntiLine> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) - (this.group1.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235))
        );
    }
}
extension AntiFlector: BulkExpansion<AntiMotor> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) + float4((this.group1.zxy * right_dual.group0.yzx), this.e321 * right_dual.e5) - (this.group1.yzxx * float4(right_dual.group0.zxy, right_dual.e235))
        );
    }
}
extension AntiFlector: BulkExpansion<AntiPlane> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: BulkExpansion<AntiScalar> {
    associatedtype Output = AntiFlector;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
extension AntiFlector: BulkExpansion<Circle> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((right_dual.group0.xyz.yzx * this.group1.zxy) - (right_dual.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_dual.e41, this.e5 * right_dual.e42, this.e5 * right_dual.e43, -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group1.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_dual.e35) + (this.e5 * right_dual.e23), (this.e3 * right_dual.e15) + (this.e5 * right_dual.e31), (this.e1 * right_dual.e25) + (this.e5 * right_dual.e12), -(this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) - float4((right_dual.group2.xyz.yzx * this.group1.zxy), this.e235 * right_dual.e41)
        );
    }
}
extension AntiFlector: BulkExpansion<CircleRotor> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension AntiFlector: BulkExpansion<Dipole> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group1.wwwx * float4(right_dual.group0.xyz, right_dual.e235)) - float4((this.group1.yzx * right_dual.group1.zxy), this.e2 * right_dual.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: BulkExpansion<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group3.zxy * this.group1.yzx) - (right_dual.group3.yzx * this.group1.zxy)), right_dual.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(right_dual.group3.www, right_dual.e423)) + float4(float3(0.0), (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3)) - float4((this.group1.www * right_dual.group3.xyz), right_dual.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321) - (right_dual.e125 * this.e3)) + float4((right_dual.group1.yzx * this.group1.zxy), right_dual.e1 * this.e235) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (right_dual.group2.wwwy * float4(this.group0.xyz, this.e2)) - (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e235))
        );
    }
}
extension AntiFlector: BulkExpansion<DualNum> {
    associatedtype Output = AntiFlector;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
extension AntiFlector: BulkExpansion<FlatPoint> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(-(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlector: BulkExpansion<Flector> {
    associatedtype Output = AntiMotor;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((right_dual.group1.zxy * this.group1.yzx) - (right_dual.group1.yzx * this.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (right_dual.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)) - (this.group1.wwwx * float4(right_dual.group1.xyz, right_dual.e235))
        );
    }
}
extension AntiFlector: BulkExpansion<Line> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e2 * right_dual.e35) + (this.e5 * right_dual.e23), (this.e3 * right_dual.e15) + (this.e5 * right_dual.e31), (this.e1 * right_dual.e25) + (this.e5 * right_dual.e12), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group1.zxyx * float4(right_dual.group1.xyz.yzx, right_dual.e23))
        );
    }
}
extension AntiFlector: BulkExpansion<Motor> {
    associatedtype Output = AntiFlector;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e2 * right_dual.e35) + (this.e5 * right_dual.e23), (this.e3 * right_dual.e15) + (this.e5 * right_dual.e31), (this.e1 * right_dual.e25) + (this.e5 * right_dual.e12), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) + (float4(right_dual.scalar) * this.group0) - (this.group1.zxyx * float4(right_dual.group1.yzx, right_dual.e23)), 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
extension AntiFlector: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.group0.xy.xx, right_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_dual.group1.xyz)), this.e5 * right_dual.e4 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * right_dual.group1.zxy) - (this.group1.zxy * right_dual.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) + float4((right_dual.group4.xyz * this.group1.www), this.e321 * right_dual.scalar) - float4((right_dual.group3.www * this.group1.xyz), this.e1 * right_dual.e23), 
            /* e423, e431, e412, 0 */
            float4((right_dual.group4.xyz.yzx * this.group1.zxy) - (right_dual.group4.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group0.xyz) + (this.group1.yzx * right_dual.group3.zxy) - (this.group1.zxy * right_dual.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5) - (this.e5 * right_dual.e321)) + float4((this.group1.zxy * right_dual.group6.yzx), this.e235 * right_dual.e1) - (this.group1.wwwx * float4(right_dual.group7.xyz, right_dual.e235)) - float4((this.group1.yzx * right_dual.group6.zxy), this.e3 * right_dual.e125) - float4((right_dual.group1.www * this.group0.xyz), this.e2 * right_dual.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) - (this.e321 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: BulkExpansion<Plane> {
    associatedtype Output = AntiMotor;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((this.group1.yzx * right_dual.group0.zxy) - (this.group1.zxy * right_dual.group0.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e5 * right_dual.e1 * -1.0, this.e5 * right_dual.e2 * -1.0, this.e5 * right_dual.e3 * -1.0, (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5)) + (right_dual.group0.wwwx * float4(this.group1.xyz, this.e235))
        );
    }
}
extension AntiFlector: BulkExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: BulkExpansion<Sphere> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * right_dual.group0.zxy) - (this.group1.zxy * right_dual.group0.yzx)), this.e5 * right_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_dual.group0.xyz)), this.e321 * right_dual.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_dual.e1) + (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5))
        );
    }
}
extension AntiFlector: BulkExpansion<VersorEven> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension AntiFlector: BulkExpansion<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * right_dual.group3.zxy) - (this.group1.zxy * right_dual.group3.yzx)), this.e5 * right_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(right_dual.group2.www, right_dual.e423)) + float4(float3(0.0), (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412)) - (right_dual.group3 * float4(this.group1.www, this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3) + (this.e321 * right_dual.e5) - (this.e5 * right_dual.e321)) + float4((this.group1.zxy * right_dual.group1.yzx), this.e235 * right_dual.e1) - (this.group1.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - (this.group1.wwwz * float4(right_dual.group0.xyz, right_dual.e125)) - float4((right_dual.group3.www * this.group0.xyz), this.e1 * right_dual.e235)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<AntiLine> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension AntiLine: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e35 * right_dual.e42), (this.e31 * right_dual.e45) + (this.e15 * right_dual.e43), (this.e12 * right_dual.e45) + (this.e25 * right_dual.e41), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group1.xyz.yzx * right_dual.group0.xyz.zxy), this.e23 * right_dual.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<AntiDualNum> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.group0.xy.xx, right_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e45) * this.group0.xyz), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35))
        );
    }
}
extension AntiLine: BulkExpansion<AntiFlector> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e45) * this.group0.xyz), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35))
        );
    }
}
extension AntiLine: BulkExpansion<AntiLine> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<AntiScalar> {
    associatedtype Output = AntiLine;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(right_dual.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<Circle> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e35 * right_dual.e42), (this.e31 * right_dual.e45) + (this.e15 * right_dual.e43), (this.e12 * right_dual.e45) + (this.e25 * right_dual.e41), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group1.xyz.yzx * right_dual.group0.xyz.zxy), this.e23 * right_dual.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<CircleRotor> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group1.xyz), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e45 * this.e23), (right_dual.e43 * this.e15) + (right_dual.e45 * this.e31), (right_dual.e41 * this.e25) + (right_dual.e45 * this.e12), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group1.xyz.yzx), right_dual.e15 * this.e23)
        );
    }
}
extension AntiLine: BulkExpansion<Dipole> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<DipoleInversion> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e4) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(right_dual.e4) * this.group1.xyz), -(right_dual.e1 * this.e23) - (right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.e2 * this.e35) + (right_dual.e5 * this.e23), (right_dual.e3 * this.e15) + (right_dual.e5 * this.e31), (right_dual.e1 * this.e25) + (right_dual.e5 * this.e12), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12)) - float4((this.group1.xyz.yzx * right_dual.group3.zxy), right_dual.e423 * this.e15)
        );
    }
}
extension AntiLine: BulkExpansion<DualNum> {
    associatedtype Output = AntiLine;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(right_dual.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<Flector> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((right_dual.e2 * this.e35) + (right_dual.e5 * this.e23), (right_dual.e3 * this.e15) + (right_dual.e5 * this.e31), (right_dual.e1 * this.e25) + (right_dual.e5 * this.e12), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
extension AntiLine: BulkExpansion<Line> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(-(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(right_dual.scalar) * this.group1.xyz), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12))
        );
    }
}
extension AntiLine: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.xx, right_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(right_dual.e4) * this.group1.xyz), -(this.e23 * right_dual.e1) - (this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)), 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e4) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group0.xyz) + (this.group1.xyz.zxy * right_dual.group1.yzx) - (this.group1.xyz.yzx * right_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e35 * right_dual.e42), (this.e31 * right_dual.e45) + (this.e15 * right_dual.e43), (this.e12 * right_dual.e45) + (this.e25 * right_dual.e41), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group1.xyz.yzx * right_dual.group4.xyz.zxy), this.e23 * right_dual.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<Plane> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e23 * right_dual.e5) + (this.e35 * right_dual.e2), (this.e31 * right_dual.e5) + (this.e15 * right_dual.e3), (this.e12 * right_dual.e5) + (this.e25 * right_dual.e1), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (right_dual.group0.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
extension AntiLine: BulkExpansion<Sphere> {
    associatedtype Output = Circle;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e4) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(right_dual.e4) * this.group1.xyz), -(this.e23 * right_dual.e1) - (this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group0.xyz) + (this.group1.xyz.zxy * right_dual.group0.yzx) - (this.group1.xyz.yzx * right_dual.group0.zxy), 0.0)
        );
    }
}
extension AntiLine: BulkExpansion<VersorEven> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group1.xyz), -(this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e35 * right_dual.e42), (this.e31 * right_dual.e45) + (this.e15 * right_dual.e43), (this.e12 * right_dual.e45) + (this.e25 * right_dual.e41), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((this.group1.xyz.yzx * right_dual.group0.zxy), this.e23 * right_dual.e15)
        );
    }
}
extension AntiLine: BulkExpansion<VersorOdd> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e4) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(right_dual.e4) * this.group1.xyz), -(this.e23 * right_dual.e1) - (this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * right_dual.e5) + (this.e35 * right_dual.e2), (this.e31 * right_dual.e5) + (this.e15 * right_dual.e3), (this.e12 * right_dual.e5) + (this.e25 * right_dual.e1), -(this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) - float4((this.group1.xyz.yzx * right_dual.group3.zxy), this.e23 * right_dual.e415)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<AntiMotor> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension AntiMotor: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group0.www * right_dual.group2.xyz), (this.scalar * right_dual.e12345) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
extension AntiMotor: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group0.www * right_dual.group2.xyz), (this.scalar * right_dual.e1234) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.scalar * right_dual.e4235), (this.e31 * right_dual.e45) + (this.scalar * right_dual.e4315), (this.e12 * right_dual.e45) + (this.scalar * right_dual.e4125), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + float4((right_dual.group0.xyz.yzx * this.group1.zxy), this.scalar * right_dual.e3215) - float4((right_dual.group0.xyz.zxy * this.group1.yzx), this.e23 * right_dual.e15)
        );
    }
}
extension AntiMotor: BulkExpansion<AntiDualNum> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), this.scalar * right_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e5) * this.group0
        );
    }
}
extension AntiMotor: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_dual.e45) * this.group0.xyz), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35))
        );
    }
}
extension AntiMotor: BulkExpansion<AntiFlector> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * right_dual.e4235, this.scalar * right_dual.e4315, this.scalar * right_dual.e4125, -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) + (this.group0 * float4(right_dual.group0.www, right_dual.e3215))
        );
    }
}
extension AntiMotor: BulkExpansion<AntiLine> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_dual.group0.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: BulkExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((this.group0.www * right_dual.group0.xyz), (this.scalar * right_dual.e12345) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_dual.group1.xyz) + (float3(right_dual.e5) * this.group0.xyz)), this.scalar * right_dual.e5)
        );
    }
}
extension AntiMotor: BulkExpansion<AntiPlane> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension AntiMotor: BulkExpansion<AntiScalar> {
    associatedtype Output = AntiMotor;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
extension AntiMotor: BulkExpansion<Circle> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e35 * right_dual.e42), (this.e31 * right_dual.e45) + (this.e15 * right_dual.e43), (this.e12 * right_dual.e45) + (this.e25 * right_dual.e41), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - float4((right_dual.group0.xyz.zxy * this.group1.yzx), this.e23 * right_dual.e15)
        );
    }
}
extension AntiMotor: BulkExpansion<CircleRotor> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension AntiMotor: BulkExpansion<Dipole> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
extension AntiMotor: BulkExpansion<DipoleInversion> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension AntiMotor: BulkExpansion<DualNum> {
    associatedtype Output = AntiMotor;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_dual.group0.xy.yy, right_dual.scalar, (right_dual.e3215 * this.scalar) + (right_dual.scalar * this.e3215)) * float4(this.group1.xyz, 1.0)
        );
    }
}
extension AntiMotor: BulkExpansion<FlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: FlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
extension AntiMotor: BulkExpansion<Flector> {
    associatedtype Output = AntiFlector;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((right_dual.e2 * this.e35) + (right_dual.e5 * this.e23), (right_dual.e3 * this.e15) + (right_dual.e5 * this.e31), (right_dual.e1 * this.e25) + (right_dual.e5 * this.e12), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + (float4(this.scalar) * right_dual.group0) - (right_dual.group1.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
extension AntiMotor: BulkExpansion<Line> {
    associatedtype Output = AntiMotor;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.scalar) * right_dual.group1.xyz), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12))
        );
    }
}
extension AntiMotor: BulkExpansion<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((float3(right_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_dual.group0.xyz)), right_dual.scalar * this.scalar), 
            /* e15, e25, e35, e3215 */
            (float4(right_dual.scalar) * this.group1) + (float4(this.scalar) * right_dual.group1) + float4(float3(0.0), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12))
        );
    }
}
extension AntiMotor: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar * right_dual.scalar, (this.scalar * right_dual.e12345) + (this.e3215 * right_dual.e4) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * right_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(this.scalar) * right_dual.group3.xyz) + (float3(right_dual.scalar) * this.group1.xyz)), this.scalar * right_dual.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e15 * right_dual.e4, this.e25 * right_dual.e4, this.e35 * right_dual.e4, -(this.e23 * right_dual.e1) - (this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + (float4(this.scalar) * right_dual.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(this.scalar) * right_dual.group7.xyz) + (float3(right_dual.e4) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.scalar) * right_dual.group8.xyz) + (float3(right_dual.e5) * this.group0.xyz) + (this.group1.zxy * right_dual.group1.yzx) - (this.group1.yzx * right_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * right_dual.e4235, this.scalar * right_dual.e4315, this.scalar * right_dual.e4125, -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + (this.group0 * float4(right_dual.group3.www, right_dual.e3215)) + (this.group1.zxyw * float4(right_dual.group4.xyz.yzx, right_dual.scalar)) - (this.group1.yzxx * float4(right_dual.group4.xyz.zxy, right_dual.e23)), 
            /* e1234, 0, 0, 0 */
            float4((this.scalar * right_dual.e1234) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: BulkExpansion<Plane> {
    associatedtype Output = AntiFlector;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e23 * right_dual.e5) + (this.e35 * right_dual.e2), (this.e31 * right_dual.e5) + (this.e15 * right_dual.e3), (this.e12 * right_dual.e5) + (this.e25 * right_dual.e1), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (right_dual.group0.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group0
        );
    }
}
extension AntiMotor: BulkExpansion<RoundPoint> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: BulkExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: BulkExpansion<Sphere> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension AntiMotor: BulkExpansion<VersorEven> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension AntiMotor: BulkExpansion<VersorOdd> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<AntiPlane> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension AntiPlane: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group0.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((right_dual.group0.xyz * this.group0.www), this.e1 * right_dual.e235), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_dual.e41, this.e5 * right_dual.e42, this.e5 * right_dual.e43, -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group0.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(this.e3 * right_dual.e25 * -1.0, this.e1 * right_dual.e35 * -1.0, this.e2 * right_dual.e15 * -1.0, (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)) + (this.group0.yzxx * float4(right_dual.group2.zxy, right_dual.e4235)) + (this.group0.wwwy * float4(right_dual.group1.xyz, right_dual.e4315))
        );
    }
}
extension AntiPlane: BulkExpansion<AntiDualNum> {
    associatedtype Output = FlatPoint;
    func bulk_expansion(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.xx, right_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = Line;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_dual.e45) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx), 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<AntiFlector> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.e45) * this.group0.xyz * float3(-1.0)), (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<AntiLine> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235))
        );
    }
}
extension AntiPlane: BulkExpansion<AntiMotor> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e2 * right_dual.e315) - (this.e3 * right_dual.e125)) - (this.group0.yzxx * float4(right_dual.group0.zxy, right_dual.e235))
        );
    }
}
extension AntiPlane: BulkExpansion<AntiPlane> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<AntiScalar> {
    associatedtype Output = AntiPlane;
    func bulk_expansion(other: AntiScalar) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension AntiPlane: BulkExpansion<Circle> {
    associatedtype Output = Circle;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_dual.e41, this.e5 * right_dual.e42, this.e5 * right_dual.e43, -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - (this.group0.xyzx * right_dual.group1.wwwx), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.xyz.zxy * this.group0.yzx) - (right_dual.group2.xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<CircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e41 * this.e5, right_dual.e42 * this.e5, right_dual.e43 * this.e5, -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e5) * right_dual.group1.xyz), 0.0) + float4((right_dual.group2.zxy * this.group0.yzx), 0.0) - float4((right_dual.group2.yzx * this.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group0
        );
    }
}
extension AntiPlane: BulkExpansion<Dipole> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e235)) - float4((this.group0.yzx * right_dual.group1.zxy), this.e2 * right_dual.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group3.zxy * this.group0.yzx) - (right_dual.group3.yzx * this.group0.zxy)), right_dual.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e1 * this.e5 * -1.0, right_dual.e2 * this.e5 * -1.0, right_dual.e3 * this.e5 * -1.0, (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e425 * this.e3, right_dual.e435 * this.e1, right_dual.e415 * this.e2, -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e235))
        );
    }
}
extension AntiPlane: BulkExpansion<DualNum> {
    associatedtype Output = AntiPlane;
    func bulk_expansion(other: DualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension AntiPlane: BulkExpansion<FlatPoint> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(-(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<Flector> {
    associatedtype Output = AntiMotor;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((right_dual.group1.zxy * this.group0.yzx) - (right_dual.group1.yzx * this.group0.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e5 * this.e1, right_dual.e5 * this.e2, right_dual.e5 * this.e3, -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)) - (this.group0.wwwx * float4(right_dual.group1.xyz, right_dual.e235))
        );
    }
}
extension AntiPlane: BulkExpansion<Line> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((right_dual.e23 * this.e5) + (right_dual.e35 * this.e2), (right_dual.e31 * this.e5) + (right_dual.e15 * this.e3), (right_dual.e12 * this.e5) + (right_dual.e25 * this.e1), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (this.group0.zxyx * float4(right_dual.group1.xyz.yzx, right_dual.e23))
        );
    }
}
extension AntiPlane: BulkExpansion<Motor> {
    associatedtype Output = AntiFlector;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((right_dual.e23 * this.e5) + (right_dual.e35 * this.e2), (right_dual.e31 * this.e5) + (right_dual.e15 * this.e3), (right_dual.e12 * this.e5) + (right_dual.e25 * this.e1), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (this.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e23)), 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * this.group0
        );
    }
}
extension AntiPlane: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.group0.xy.xx, right_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group1.xyz)), this.e5 * right_dual.e4 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group0.yzx * right_dual.group1.zxy) - (this.group0.zxy * right_dual.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_dual.e41, this.e5 * right_dual.e42, this.e5 * right_dual.e43, -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) - float4((right_dual.group3.www * this.group0.xyz), this.e1 * right_dual.e23), 
            /* e423, e431, e412, 0 */
            float4((right_dual.group4.xyz.yzx * this.group0.zxy) - (right_dual.group4.xyz.zxy * this.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group5.xyz) + (this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group0.wwwx * float4(right_dual.group7.xyz, right_dual.e235)) - float4((this.group0.yzx * right_dual.group6.zxy), this.e2 * right_dual.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<Plane> {
    associatedtype Output = AntiLine;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4((right_dual.group0.zxy * this.group0.yzx) - (right_dual.group0.yzx * this.group0.zxy), 0.0), 
            /* e15, e25, e35, 0 */
            float4((float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group0.xyz), 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<Sphere> {
    associatedtype Output = Dipole;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx)), this.e5 * right_dual.e4 * -1.0), 
            /* e15, e25, e35, 0 */
            float4((float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group0.xyz), 0.0)
        );
    }
}
extension AntiPlane: BulkExpansion<VersorEven> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension AntiPlane: BulkExpansion<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx)), this.e5 * right_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * right_dual.e1 * -1.0, this.e5 * right_dual.e2 * -1.0, this.e5 * right_dual.e3 * -1.0, (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412)) + (this.group0.xyzx * float4(right_dual.group2.www, right_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_dual.e425, this.e1 * right_dual.e435, this.e2 * right_dual.e415, -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e235)) - (this.group0.wwwy * float4(right_dual.group0.xyz, right_dual.e315))
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<AntiScalar> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension AntiScalar: BulkExpansion<AntiScalar> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkExpansion<CircleRotor> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(float4(other.group2.xyz, other.e12345 * -1.0)[3] * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkExpansion<DualNum> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkExpansion<Motor> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3] * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkExpansion<MultiVector> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkExpansion<VersorEven> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3] * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<Circle> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension Circle: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkExpansion<AntiDualNum> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiDualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension Circle: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkExpansion<AntiFlector> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkExpansion<AntiMotor> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiMotor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension Circle: BulkExpansion<AntiScalar> {
    associatedtype Output = Circle;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: BulkExpansion<Circle> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkExpansion<CircleRotor> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
extension Circle: BulkExpansion<DipoleInversion> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual.e4 * this.e235) - (right_dual.e3 * this.e425), -(right_dual.e4 * this.e315) - (right_dual.e1 * this.e435), -(right_dual.e4 * this.e125) - (right_dual.e2 * this.e415), (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group3.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_dual.group3.yzx * this.group1.zxy), right_dual.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e4 * this.e321) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkExpansion<DualNum> {
    associatedtype Output = Circle;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: BulkExpansion<Flector> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group1.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_dual.group1.yzx * this.group1.zxy), right_dual.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkExpansion<Line> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkExpansion<Motor> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
extension Circle: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.scalar) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e321 * right_dual.e5) + (this.e125 * right_dual.e3)) + (right_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(right_dual.e5) * this.group0.xyz), this.e235 * right_dual.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkExpansion<Plane> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group0.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_dual.group0.yzx * this.group1.zxy), right_dual.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkExpansion<Sphere> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e321 * right_dual.e5) + (this.e125 * right_dual.e3)) + (right_dual.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(right_dual.e5) * this.group0.xyz), this.e235 * right_dual.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkExpansion<VersorEven> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43))
        );
    }
}
extension Circle: BulkExpansion<VersorOdd> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e321 * right_dual.e5) + (this.e125 * right_dual.e3)) + (right_dual.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_dual.group2.www), this.e235 * right_dual.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<CircleRotor> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension CircleRotor: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkExpansion<AntiDualNum> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiDualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension CircleRotor: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkExpansion<AntiFlector> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkExpansion<AntiMotor> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiMotor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension CircleRotor: BulkExpansion<AntiScalar> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_dual.scalar) * this.group2
        );
    }
}
extension CircleRotor: BulkExpansion<Circle> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkExpansion<CircleRotor> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((right_dual.group2.www * this.group2.xyz), (right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
extension CircleRotor: BulkExpansion<DipoleInversion> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual.e4 * this.e235) - (right_dual.e3 * this.e425), -(right_dual.e4 * this.e315) - (right_dual.e1 * this.e435), -(right_dual.e4 * this.e125) - (right_dual.e2 * this.e415), (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_dual.group3.www), right_dual.e1 * this.e235), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e4 * this.e321) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkExpansion<DualNum> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_dual.scalar) * this.group2
        );
    }
}
extension CircleRotor: BulkExpansion<Flector> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_dual.group1.www), right_dual.e1 * this.e235), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkExpansion<Line> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkExpansion<Motor> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((right_dual.group0.www * this.group2.xyz), (right_dual.scalar * this.e12345) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
extension CircleRotor: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e12345 * right_dual.scalar) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.scalar) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group1.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkExpansion<Plane> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_dual.group0.www), right_dual.e1 * this.e235), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkExpansion<Sphere> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group0.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkExpansion<VersorEven> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_dual.scalar) * this.group2.xyz), (this.e12345 * right_dual.scalar) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43))
        );
    }
}
extension CircleRotor: BulkExpansion<VersorOdd> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group3.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3) - (this.e321 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<Dipole> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension Dipole: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e35 * right_dual.e12)) - float4((this.group0.xyz.zxy * right_dual.group2.yzx), this.e15 * right_dual.e23) - float4((this.group2.xyz.yzx * right_dual.group0.xyz.zxy), this.e25 * right_dual.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<AntiDualNum> {
    associatedtype Output = Line;
    func bulk_expansion(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_dual.e5) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e5) * this.group1.xyz, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension Dipole: BulkExpansion<AntiFlector> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension Dipole: BulkExpansion<AntiLine> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.e5) * this.group0.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<AntiScalar> {
    associatedtype Output = Dipole;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(right_dual.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<Circle> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group2.xyz.yzx), right_dual.e15 * this.e23) - float4((right_dual.group2.xyz.yzx * this.group0.xyz.zxy), right_dual.e25 * this.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<CircleRotor> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group2.xyz.yzx), right_dual.e23 * this.e15) - float4((this.group0.xyz.zxy * right_dual.group2.yzx), right_dual.e31 * this.e25)
        );
    }
}
extension Dipole: BulkExpansion<Dipole> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<DipoleInversion> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group3.zxy) - (this.group0.xyz.zxy * right_dual.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e4 * this.e15) + (right_dual.e5 * this.e41), (right_dual.e4 * this.e25) + (right_dual.e5 * this.e42), (right_dual.e4 * this.e35) + (right_dual.e5 * this.e43), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.e2 * this.e35) + (right_dual.e5 * this.e23), (right_dual.e3 * this.e15) + (right_dual.e5 * this.e31), (right_dual.e1 * this.e25) + (right_dual.e5 * this.e12), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((this.group2.xyz.yzx * right_dual.group3.zxy), right_dual.e423 * this.e15)
        );
    }
}
extension Dipole: BulkExpansion<DualNum> {
    associatedtype Output = Dipole;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(right_dual.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<FlatPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<Flector> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * right_dual.group1.zxy) - (this.group0.xyz.zxy * right_dual.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e5 * this.e41, right_dual.e5 * this.e42, right_dual.e5 * this.e43, -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.e2 * this.e35) + (right_dual.e5 * this.e23), (right_dual.e3 * this.e15) + (right_dual.e5 * this.e31), (right_dual.e1 * this.e25) + (right_dual.e5 * this.e12), -(right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) - float4((this.group2.xyz.yzx * right_dual.group1.zxy), right_dual.e235 * this.e41)
        );
    }
}
extension Dipole: BulkExpansion<Line> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e35 * this.e42), (right_dual.e31 * this.e45) + (right_dual.e15 * this.e43), (right_dual.e12 * this.e45) + (right_dual.e25 * this.e41), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group1.xyz.yzx * this.group0.xyz.zxy), right_dual.e23 * this.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<Motor> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e35 * this.e42), (right_dual.e31 * this.e45) + (right_dual.e15 * this.e43), (right_dual.e12 * this.e45) + (right_dual.e25 * this.e41), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((this.group0.xyz.zxy * right_dual.group1.yzx), right_dual.e23 * this.e15)
        );
    }
}
extension Dipole: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.scalar) * this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group1.zxy) - (this.group0.xyz.zxy * right_dual.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_dual.group1.yzx) - (this.group2.xyz.yzx * right_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e35 * right_dual.e12)) - float4((this.group0.xyz.zxy * right_dual.group3.yzx), this.e15 * right_dual.e23) - float4((this.group2.xyz.yzx * right_dual.group4.xyz.zxy), this.e25 * right_dual.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkExpansion<Plane> {
    associatedtype Output = Circle;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e5 * this.e41, right_dual.e5 * this.e42, right_dual.e5 * this.e43, -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_dual.group0.yzx) - (this.group2.xyz.yzx * right_dual.group0.zxy), 0.0)
        );
    }
}
extension Dipole: BulkExpansion<Sphere> {
    associatedtype Output = Circle;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_dual.group0.yzx) - (this.group2.xyz.yzx * right_dual.group0.zxy), 0.0)
        );
    }
}
extension Dipole: BulkExpansion<VersorEven> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group2.xyz), -(this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e35 * right_dual.e12)) - float4((this.group0.xyz.zxy * right_dual.group2.yzx), this.e15 * right_dual.e23) - float4((this.group2.xyz.yzx * right_dual.group0.zxy), this.e25 * right_dual.e31)
        );
    }
}
extension Dipole: BulkExpansion<VersorOdd> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group3.zxy) - (this.group0.xyz.zxy * right_dual.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * right_dual.e5) + (this.e35 * right_dual.e2), (this.e31 * right_dual.e5) + (this.e15 * right_dual.e3), (this.e12 * right_dual.e5) + (this.e25 * right_dual.e1), -(this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) - float4((this.group2.xyz.yzx * right_dual.group3.zxy), this.e41 * right_dual.e235)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<DipoleInversion> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension DipoleInversion: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - (this.group2.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23)) - float4((this.group0.xyz.zxy * right_dual.group2.yzx), right_dual.e31 * this.e25), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkExpansion<AntiDualNum> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e5) * float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4(right_dual.group0.xy.xx, right_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension DipoleInversion: BulkExpansion<AntiFlector> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45), -(this.e31 * right_dual.e25) - (this.e12 * right_dual.e35)) - (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension DipoleInversion: BulkExpansion<AntiLine> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.e5) * this.group0.xyz), (this.e1234 * right_dual.e5) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkExpansion<AntiScalar> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
extension DipoleInversion: BulkExpansion<Circle> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group2.yzx), right_dual.e15 * this.e23) - float4((right_dual.group2.xyz.yzx * this.group0.xyz.zxy), right_dual.e25 * this.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkExpansion<CircleRotor> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((right_dual.group2.www * this.group2.xyz), (right_dual.scalar * this.e1234) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42) + (right_dual.scalar * this.e4235), (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43) + (right_dual.scalar * this.e4315), (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41) + (right_dual.scalar * this.e4125), -(right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.scalar * this.e3215) - (this.group2.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23)) - float4((this.group0.xyz.zxy * right_dual.group2.yzx), right_dual.e31 * this.e25)
        );
    }
}
extension DipoleInversion: BulkExpansion<Dipole> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkExpansion<DipoleInversion> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group3.zxy) - (this.group0.xyz.zxy * right_dual.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e4 * this.e15) + (right_dual.e5 * this.e41), (right_dual.e4 * this.e25) + (right_dual.e5 * this.e42), (right_dual.e4 * this.e35) + (right_dual.e5 * this.e43), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (right_dual.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) + float4((right_dual.group3.yzx * this.group2.zxy), right_dual.e4 * this.e3215) - (this.group2.yzxx * float4(right_dual.group3.zxy, right_dual.e423))
        );
    }
}
extension DipoleInversion: BulkExpansion<DualNum> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group3
        );
    }
}
extension DipoleInversion: BulkExpansion<FlatPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkExpansion<Flector> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * right_dual.group1.zxy) - (this.group0.xyz.zxy * right_dual.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e5 * this.e41, right_dual.e5 * this.e42, right_dual.e5 * this.e43, -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (right_dual.group1.yzxx * float4(this.group2.zxy, this.e4235)) + (right_dual.group1.wwwy * float4(this.group1.xyz, this.e4315)) + float4(float3(0.0), (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) - float4((right_dual.group1.zxy * this.group2.yzx), right_dual.e235 * this.e41)
        );
    }
}
extension DipoleInversion: BulkExpansion<Line> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e35 * this.e42), (right_dual.e31 * this.e45) + (right_dual.e15 * this.e43), (right_dual.e12 * this.e45) + (right_dual.e25 * this.e41), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group1.xyz.yzx * this.group0.xyz.zxy), right_dual.e23 * this.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkExpansion<Motor> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((right_dual.group0.www * this.group2.xyz), (right_dual.scalar * this.e1234) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.scalar * this.e4235), (right_dual.e31 * this.e45) + (right_dual.scalar * this.e4315), (right_dual.e12 * this.e45) + (right_dual.scalar * this.e4125), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + float4((this.group0.xyz.yzx * right_dual.group1.zxy), right_dual.scalar * this.e3215) - float4((this.group0.xyz.zxy * right_dual.group1.yzx), right_dual.e23 * this.e15)
        );
    }
}
extension DipoleInversion: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1234 * right_dual.e5) + (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.scalar) * this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group1.zxy) - (this.group0.xyz.zxy * right_dual.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_dual.group1.yzx) - (this.group2.yzx * right_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_dual.e35) + (this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42), (this.e43 * right_dual.e15) + (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43), (this.e41 * right_dual.e25) + (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41), -(this.e23 * right_dual.e15) - (this.e31 * right_dual.e25) - (this.e12 * right_dual.e35) - (this.e35 * right_dual.e12)) + (float4(right_dual.scalar) * this.group3) - (this.group2.yzxy * float4(right_dual.group4.xyz.zxy, right_dual.e31)) - float4((this.group0.xyz.zxy * right_dual.group3.yzx), this.e15 * right_dual.e23), 
            /* e1234, 0, 0, 0 */
            float4((this.e1234 * right_dual.scalar) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkExpansion<Plane> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e5 * this.e41, right_dual.e5 * this.e42, right_dual.e5 * this.e43, -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - (right_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(right_dual.e3 * this.e25 * -1.0, right_dual.e1 * this.e35 * -1.0, right_dual.e2 * this.e15 * -1.0, (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)) + (right_dual.group0.yzxx * float4(this.group2.zxy, this.e4235)) + (right_dual.group0.wwwy * float4(this.group1.xyz, this.e4315))
        );
    }
}
extension DipoleInversion: BulkExpansion<Sphere> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group0.zxy) - (this.group0.xyz.zxy * right_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(this.e25 * right_dual.e3 * -1.0, this.e35 * right_dual.e1 * -1.0, this.e15 * right_dual.e2 * -1.0, (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4)) + (float4(right_dual.e5) * float4(this.group1.xyz, this.e1234)) + (right_dual.group0.yzxx * float4(this.group2.zxy, this.e4235))
        );
    }
}
extension DipoleInversion: BulkExpansion<VersorEven> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.scalar) * this.group2.xyz), (this.e1234 * right_dual.scalar) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_dual.e45) + (this.e45 * right_dual.e23) + (this.e35 * right_dual.e42) + (this.e4235 * right_dual.scalar), (this.e31 * right_dual.e45) + (this.e45 * right_dual.e31) + (this.e15 * right_dual.e43) + (this.e4315 * right_dual.scalar), (this.e12 * right_dual.e45) + (this.e45 * right_dual.e12) + (this.e25 * right_dual.e41) + (this.e4125 * right_dual.scalar), -(this.e12 * right_dual.e35) - (this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + float4((this.group0.xyz.yzx * right_dual.group2.zxy), this.e3215 * right_dual.scalar) - (right_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((this.group2.yzx * right_dual.group0.zxy), this.e31 * right_dual.e25)
        );
    }
}
extension DipoleInversion: BulkExpansion<VersorOdd> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_dual.group3.zxy) - (this.group0.xyz.zxy * right_dual.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_dual.e5) + (this.e15 * right_dual.e4), (this.e42 * right_dual.e5) + (this.e25 * right_dual.e4), (this.e43 * right_dual.e5) + (this.e35 * right_dual.e4), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(right_dual.e5) * float4(this.group1.xyz, this.e1234)) + (right_dual.group3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) - float4((this.group2.yzx * right_dual.group3.zxy), this.e41 * right_dual.e235)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<DualNum> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension DualNum: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
extension DualNum: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e1234), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DualNum: BulkExpansion<AntiScalar> {
    associatedtype Output = DualNum;
    func bulk_expansion(other: AntiScalar) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.e12345 * -1.0) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: BulkExpansion<Circle> {
    associatedtype Output = Line;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e5) * right_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e5) * right_dual.group1.xyz, 0.0)
        );
    }
}
extension DualNum: BulkExpansion<CircleRotor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, this.e12345) * float4(right_dual.group0.xyz, right_dual.scalar), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(right_dual.group1.xyz, right_dual.scalar)
        );
    }
}
extension DualNum: BulkExpansion<Dipole> {
    associatedtype Output = Plane;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
extension DualNum: BulkExpansion<DipoleInversion> {
    associatedtype Output = Flector;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(right_dual.group3.xyz, right_dual.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
extension DualNum: BulkExpansion<DualNum> {
    associatedtype Output = DualNum;
    func bulk_expansion(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.e12345 * -1.0) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: BulkExpansion<FlatPoint> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: FlatPoint) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e5 * other.e45, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension DualNum: BulkExpansion<Flector> {
    associatedtype Output = Flector;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual.e321 * this.e5 * -1.0)
        );
    }
}
extension DualNum: BulkExpansion<Line> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: Line) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DualNum: BulkExpansion<Motor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), right_dual.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * right_dual.group0
        );
    }
}
extension DualNum: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e5 * right_dual.e1234) + (this.e12345 * right_dual.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_dual.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e5) * right_dual.group5.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group7.xyz, right_dual.e321) * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension DualNum: BulkExpansion<Plane> {
    associatedtype Output = FlatPoint;
    func bulk_expansion(other: Plane) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215).xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension DualNum: BulkExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e5 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: BulkExpansion<Sphere> {
    associatedtype Output = FlatPoint;
    func bulk_expansion(other: Sphere) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e1234) * float4(-1.0)
        );
    }
}
extension DualNum: BulkExpansion<VersorEven> {
    associatedtype Output = Motor;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, (this.e5 * right_dual.e1234) + (this.e12345 * right_dual.scalar)) * float4(right_dual.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(right_dual.group1.xyz, right_dual.scalar)
        );
    }
}
extension DualNum: BulkExpansion<VersorOdd> {
    associatedtype Output = Flector;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_dual.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<FlatPoint> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension FlatPoint: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
extension FlatPoint: BulkExpansion<AntiScalar> {
    associatedtype Output = FlatPoint;
    func bulk_expansion(other: AntiScalar) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension FlatPoint: BulkExpansion<Circle> {
    associatedtype Output = Plane;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
extension FlatPoint: BulkExpansion<CircleRotor> {
    associatedtype Output = Flector;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
extension FlatPoint: BulkExpansion<Dipole> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkExpansion<DipoleInversion> {
    associatedtype Output = Motor;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e4 * this.e15, right_dual.e4 * this.e25, right_dual.e4 * this.e35, -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group3.yzx * this.group0.zxy) - (right_dual.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension FlatPoint: BulkExpansion<DualNum> {
    associatedtype Output = FlatPoint;
    func bulk_expansion(other: DualNum) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension FlatPoint: BulkExpansion<FlatPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e45 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkExpansion<Flector> {
    associatedtype Output = Motor;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e45) * float4(right_dual.group1.xyz, right_dual.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group1.yzx * this.group0.zxy) - (right_dual.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension FlatPoint: BulkExpansion<Line> {
    associatedtype Output = Plane;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_dual.group0.xyz), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35))
        );
    }
}
extension FlatPoint: BulkExpansion<Motor> {
    associatedtype Output = Flector;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_dual.group0.xyz), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35))
        );
    }
}
extension FlatPoint: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_dual.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.zxy * right_dual.group1.yzx) - (this.group0.yzx * right_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_dual.e42) + (this.e45 * right_dual.e23), (this.e15 * right_dual.e43) + (this.e45 * right_dual.e31), (this.e25 * right_dual.e41) + (this.e45 * right_dual.e12), -(this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - (this.group0.yzxx * float4(right_dual.group4.xyz.zxy, right_dual.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension FlatPoint: BulkExpansion<Plane> {
    associatedtype Output = Line;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e45) * right_dual.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension FlatPoint: BulkExpansion<Sphere> {
    associatedtype Output = Line;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4((float3(right_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_dual.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.zxy * right_dual.group0.yzx) - (this.group0.yzx * right_dual.group0.zxy), 0.0)
        );
    }
}
extension FlatPoint: BulkExpansion<VersorEven> {
    associatedtype Output = Flector;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_dual.e42) + (this.e45 * right_dual.e23), (this.e15 * right_dual.e43) + (this.e45 * right_dual.e31), (this.e25 * right_dual.e41) + (this.e45 * right_dual.e12), -(this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) - (this.group0.yzxx * float4(right_dual.group0.zxy, right_dual.e23))
        );
    }
}
extension FlatPoint: BulkExpansion<VersorOdd> {
    associatedtype Output = Motor;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e15 * right_dual.e4, this.e25 * right_dual.e4, this.e35 * right_dual.e4, -(this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321)) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_dual.group3.yzx) - (this.group0.yzx * right_dual.group3.zxy)), 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<Flector> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension Flector: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
extension Flector: BulkExpansion<AntiScalar> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
extension Flector: BulkExpansion<Circle> {
    associatedtype Output = Plane;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
extension Flector: BulkExpansion<CircleRotor> {
    associatedtype Output = Flector;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.scalar * this.e4235), (right_dual.e31 * this.e45) + (right_dual.scalar * this.e4315), (right_dual.e12 * this.e45) + (right_dual.scalar * this.e4125), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) + float4((right_dual.group0.xyz.yzx * this.group0.zxy), right_dual.scalar * this.e3215) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23))
        );
    }
}
extension Flector: BulkExpansion<Dipole> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: BulkExpansion<DipoleInversion> {
    associatedtype Output = Motor;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (float4(right_dual.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group3.yzx * this.group0.zxy) - (right_dual.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Flector: BulkExpansion<DualNum> {
    associatedtype Output = Flector;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
extension Flector: BulkExpansion<FlatPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e45 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: BulkExpansion<Flector> {
    associatedtype Output = Motor;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * right_dual.group1.xyz * float3(-1.0)), (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) - (right_dual.e321 * this.e45)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group1.yzx * this.group0.zxy) - (right_dual.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Flector: BulkExpansion<Line> {
    associatedtype Output = Plane;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_dual.group0.xyz), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35))
        );
    }
}
extension Flector: BulkExpansion<Motor> {
    associatedtype Output = Flector;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar * this.e4235, right_dual.scalar * this.e4315, right_dual.scalar * this.e4125, -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35)) + (right_dual.group0 * float4(this.group0.www, this.e3215))
        );
    }
}
extension Flector: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_dual.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.zxy * right_dual.group1.yzx) - (this.group0.yzx * right_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_dual.e42) + (this.e45 * right_dual.e23), (this.e15 * right_dual.e43) + (this.e45 * right_dual.e31), (this.e25 * right_dual.e41) + (this.e45 * right_dual.e12), -(this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + (float4(right_dual.scalar) * this.group1) - (this.group0.yzxx * float4(right_dual.group4.xyz.zxy, right_dual.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: BulkExpansion<Plane> {
    associatedtype Output = Motor;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * right_dual.group0.xyz * float3(-1.0)), (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Flector: BulkExpansion<Sphere> {
    associatedtype Output = Motor;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e45 * right_dual.e1 * -1.0, this.e45 * right_dual.e2 * -1.0, this.e45 * right_dual.e3 * -1.0, (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4)) + (right_dual.group0.wwwx * float4(this.group0.xyz, this.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_dual.group0.yzx) - (this.group0.yzx * right_dual.group0.zxy)), 0.0)
        );
    }
}
extension Flector: BulkExpansion<VersorEven> {
    associatedtype Output = Flector;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e45 * right_dual.e23) + (this.e4235 * right_dual.scalar), (this.e45 * right_dual.e31) + (this.e4315 * right_dual.scalar), (this.e45 * right_dual.e12) + (this.e4125 * right_dual.scalar), -(this.e25 * right_dual.e31) - (this.e35 * right_dual.e12)) + (right_dual.group0.yzxw * float4(this.group0.zxy, this.e3215)) - (this.group0.yzxx * float4(right_dual.group0.zxy, right_dual.e23))
        );
    }
}
extension Flector: BulkExpansion<VersorOdd> {
    associatedtype Output = Motor;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (right_dual.group3.wwwx * float4(this.group0.xyz, this.e4235)) + float4(float3(0.0), (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321)) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_dual.group3.yzx) - (this.group0.yzx * right_dual.group3.zxy)), 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<Line> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension Line: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: BulkExpansion<AntiScalar> {
    associatedtype Output = Line;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: BulkExpansion<Circle> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: BulkExpansion<CircleRotor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.scalar) * this.group0.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: BulkExpansion<DipoleInversion> {
    associatedtype Output = Plane;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual.e4 * this.e235) - (right_dual.e3 * this.e425), -(right_dual.e4 * this.e315) - (right_dual.e1 * this.e435), -(right_dual.e4 * this.e125) - (right_dual.e2 * this.e415), (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) + (right_dual.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: BulkExpansion<DualNum> {
    associatedtype Output = Line;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_dual.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: BulkExpansion<Flector> {
    associatedtype Output = Plane;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) + (right_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: BulkExpansion<Line> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: BulkExpansion<Motor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.scalar) * this.group0.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.group0.xy.xx, right_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.scalar) * this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Line: BulkExpansion<Plane> {
    associatedtype Output = Plane;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) + (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: BulkExpansion<Sphere> {
    associatedtype Output = Plane;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: BulkExpansion<VersorEven> {
    associatedtype Output = Motor;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual.scalar) * this.group0.xyz), -(this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: BulkExpansion<VersorOdd> {
    associatedtype Output = Plane;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<Motor> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension Motor: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
extension Motor: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * right_dual.group0.xyz), (right_dual.e1234 * this.e5) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: BulkExpansion<AntiScalar> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
extension Motor: BulkExpansion<Circle> {
    associatedtype Output = Motor;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * right_dual.group0.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: BulkExpansion<CircleRotor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.scalar * this.e415, right_dual.scalar * this.e425, right_dual.scalar * this.e435, -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)) + float4((right_dual.group0.xyz * this.group1.www), right_dual.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.scalar) * this.group1.xyz) + (float3(this.e5) * right_dual.group1.xyz)), right_dual.scalar * this.e5)
        );
    }
}
extension Motor: BulkExpansion<Dipole> {
    associatedtype Output = Plane;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321) * float4(-1.0)
        );
    }
}
extension Motor: BulkExpansion<DipoleInversion> {
    associatedtype Output = Flector;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(right_dual.group3.xyz, right_dual.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual.e4 * this.e235) - (right_dual.e3 * this.e425), -(right_dual.e4 * this.e315) - (right_dual.e1 * this.e435), -(right_dual.e4 * this.e125) - (right_dual.e2 * this.e415), (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) + (right_dual.group3.yzxx * float4(this.group0.zxy, this.e235)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321))
        );
    }
}
extension Motor: BulkExpansion<DualNum> {
    associatedtype Output = Motor;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_dual.scalar) * this.group1
        );
    }
}
extension Motor: BulkExpansion<FlatPoint> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: FlatPoint) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.e45 * this.e5, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension Motor: BulkExpansion<Flector> {
    associatedtype Output = Flector;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) - float4((right_dual.group1.zxy * this.group0.yzx), right_dual.e321 * this.e5)
        );
    }
}
extension Motor: BulkExpansion<Line> {
    associatedtype Output = Motor;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: BulkExpansion<Motor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((right_dual.group0.www * this.group0.xyz), (right_dual.scalar * this.e12345) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.scalar) * this.group1.xyz) + (float3(this.e5) * right_dual.group0.xyz)), right_dual.scalar * this.e5)
        );
    }
}
extension Motor: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e12345 * right_dual.scalar) + (this.e5 * right_dual.e1234) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_dual.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * right_dual.group4.xyz) + (float3(right_dual.scalar) * this.group0.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group5.xyz) + (float3(right_dual.scalar) * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group1.yzxx * float4(this.group0.zxy, this.e235)) - float4((right_dual.group7.xyz * this.group1.www), this.e5 * right_dual.e321), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Motor: BulkExpansion<Plane> {
    associatedtype Output = Flector;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e2 * this.e315) + (right_dual.e3 * this.e125)) + (right_dual.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
extension Motor: BulkExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e5 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: BulkExpansion<Sphere> {
    associatedtype Output = Flector;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_dual.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
extension Motor: BulkExpansion<VersorEven> {
    associatedtype Output = Motor;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e1234)) + (float4(right_dual.scalar) * this.group0) + float4(float3(0.0), -(this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group1.xyz)), this.e5 * right_dual.scalar)
        );
    }
}
extension Motor: BulkExpansion<VersorOdd> {
    associatedtype Output = Flector;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_dual.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e235 * right_dual.e4) - (this.e5 * right_dual.e423), -(this.e315 * right_dual.e4) - (this.e5 * right_dual.e431), -(this.e125 * right_dual.e4) - (this.e5 * right_dual.e412), (this.e315 * right_dual.e2) + (this.e125 * right_dual.e3)) + (right_dual.group3.yzxx * float4(this.group0.zxy, this.e235)) - float4((this.group0.yzx * right_dual.group3.zxy), this.e5 * right_dual.e321)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<MultiVector> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension MultiVector: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.e12345 * this.scalar) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_dual.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e235)), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.e1234 * this.e5) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(right_dual.group2.xyz, right_dual.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group1.zxy) - (right_dual.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.zxy * this.group1.yzx) - (right_dual.group2.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) + (float4(this.scalar) * right_dual.group3) - (right_dual.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((right_dual.group0.xyz.zxy * this.group3.yzx), right_dual.e15 * this.e23), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e1234 * this.scalar) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<AntiDualNum> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.e5 * this.e1234) + (right_dual.e12345 * this.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(right_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.group0.xy.xx, right_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e5) * this.group5.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e5) * float4(this.group7.xyz, this.e321), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((right_dual.group0.zxy * this.group1.yzx) - (right_dual.group0.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e35 * this.e42) + (right_dual.e45 * this.e23), (right_dual.e15 * this.e43) + (right_dual.e45 * this.e31), (right_dual.e25 * this.e41) + (right_dual.e45 * this.e12), -(right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - (right_dual.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkExpansion<AntiFlector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((right_dual.group0.zxy * this.group1.yzx) - (right_dual.group0.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e35 * this.e42) + (right_dual.e45 * this.e23), (right_dual.e15 * this.e43) + (right_dual.e45 * this.e31), (right_dual.e25 * this.e41) + (right_dual.e45 * this.e12), -(right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + (float4(this.scalar) * right_dual.group1) - (right_dual.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkExpansion<AntiLine> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_dual.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (this.group1.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkExpansion<AntiMotor> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.e12345 * this.scalar) + (right_dual.e5 * this.e1234) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(right_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual.e5) * this.group4.xyz) + (float3(this.scalar) * right_dual.group0.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_dual.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) + float4((this.group7.xyz * right_dual.group1.www), right_dual.e5 * this.e321) - (this.group1.yzxx * float4(right_dual.group0.zxy, right_dual.e235)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkExpansion<AntiPlane> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
            float4(this.scalar) * right_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkExpansion<AntiScalar> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_dual.scalar) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.scalar) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.scalar) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * right_dual.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<Circle> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(right_dual.group2.xyz, right_dual.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group1.zxy) - (right_dual.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.xyz.zxy * this.group1.yzx) - (right_dual.group2.xyz.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group3.yzx), right_dual.e15 * this.e23) - float4((right_dual.group2.xyz.yzx * this.group4.xyz.zxy), right_dual.e25 * this.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<CircleRotor> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_dual.scalar * this.scalar, (right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(right_dual.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_dual.scalar) * this.group3) + (float4(this.scalar) * float4(right_dual.group2.xyz, right_dual.e45)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.scalar) * this.group4.xyz) + (float3(this.scalar) * right_dual.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_dual.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e15 * this.e4) + (right_dual.scalar * this.e415), (right_dual.e25 * this.e4) + (right_dual.scalar * this.e425), (right_dual.e35 * this.e4) + (right_dual.scalar * this.e435), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) + float4((float3(this.e5) * right_dual.group0.xyz), right_dual.scalar * this.e321) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.scalar) * this.group7.xyz) + (float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group1.zxy) - (right_dual.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.scalar) * this.group8.xyz) + (float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.zxy * this.group1.yzx) - (right_dual.group2.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42) + (right_dual.scalar * this.e4235), (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43) + (right_dual.scalar * this.e4315), (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41) + (right_dual.scalar * this.e4125), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) + float4((right_dual.group0.xyz.yzx * this.group3.zxy), right_dual.scalar * this.e3215) - (right_dual.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((right_dual.group0.xyz.zxy * this.group3.yzx), right_dual.e15 * this.e23), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.scalar * this.e1234) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<Dipole> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_dual.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e321 * this.e5) - (right_dual.e125 * this.e3)) - (this.group1.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((float3(this.e5) * right_dual.group0.xyz), right_dual.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<DipoleInversion> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.e4 * this.e3215) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_dual.group3.xyz, right_dual.e4), 
            /* e5, 0, 0, 0 */
            float4(right_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * this.group1) - (float4(this.e5) * float4(right_dual.group3.xyz, right_dual.e4)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_dual.group3.zxy * this.group1.yzx) - (right_dual.group3.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e4 * this.e15) + (right_dual.e5 * this.e41), (right_dual.e4 * this.e25) + (right_dual.e5 * this.e42), (right_dual.e4 * this.e35) + (right_dual.e5 * this.e43), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + (float4(this.scalar) * right_dual.group1) - float4((this.group3.www * right_dual.group3.xyz), right_dual.e1 * this.e23), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group5.xyz) + (float3(this.scalar) * right_dual.group0.xyz) + (this.group4.xyz.yzx * right_dual.group3.zxy) - (this.group4.xyz.zxy * right_dual.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_dual.group2.xyz) + (right_dual.group3.yzx * this.group3.zxy) - (right_dual.group3.zxy * this.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group3.yzxw * this.group6.zxyw) + (right_dual.group3.wwwx * float4(this.group7.xyz, this.e235)) + float4((right_dual.group1.yzx * this.group1.zxy), right_dual.e2 * this.e315) + float4((this.group1.www * right_dual.group2.xyz), right_dual.e3 * this.e125) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group1.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((this.group8.xyz * right_dual.group2.www), right_dual.e235 * this.e1) - float4((right_dual.group3.zxy * this.group6.yzx), right_dual.e125 * this.e3), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e4 * this.e321) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<DualNum> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_dual.scalar * this.scalar, (right_dual.e3215 * this.e4) + (right_dual.scalar * this.e12345)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(right_dual.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.scalar) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.scalar) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.scalar) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.scalar) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.scalar) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.group0.xy.yy, right_dual.scalar, (right_dual.e3215 * this.scalar) + (right_dual.scalar * this.e3215)) * float4(this.group9.xyz, 1.0), 
            /* e1234, 0, 0, 0 */
            float4(right_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<FlatPoint> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.e321 * this.scalar), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_dual.group0.xyz), -(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)), 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<Flector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(right_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_dual.group1.xyz)), right_dual.e5 * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_dual.group1.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_dual.group1.zxy * this.group1.yzx) - (right_dual.group1.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + float4((this.group4.xyz * right_dual.group1.www), right_dual.e321 * this.scalar) - float4((this.group3.www * right_dual.group1.xyz), right_dual.e1 * this.e23), 
            /* e423, e431, e412, 0 */
            float4((this.group4.xyz.yzx * right_dual.group1.zxy) - (this.group4.xyz.zxy * right_dual.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_dual.group0.xyz) + (right_dual.group1.yzx * this.group3.zxy) - (right_dual.group1.zxy * this.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group1.wwwx * float4(this.group7.xyz, this.e235)) + float4(float3(0.0), (right_dual.e5 * this.e321) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)) + float4((right_dual.group1.yzx * this.group6.zxy), right_dual.e3 * this.e125) + float4((this.group1.www * right_dual.group0.xyz), right_dual.e2 * this.e315) - float4((right_dual.group1.zxy * this.group6.yzx), right_dual.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<Line> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), -(right_dual.e23 * this.e1) - (right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * right_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group0.xyz) + (right_dual.group1.xyz.zxy * this.group1.yzx) - (right_dual.group1.xyz.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e35 * this.e42), (right_dual.e31 * this.e45) + (right_dual.e15 * this.e43), (right_dual.e12 * this.e45) + (right_dual.e25 * this.e41), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group1.xyz.yzx * this.group4.xyz.zxy), right_dual.e23 * this.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<Motor> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_dual.scalar * this.scalar, (right_dual.scalar * this.e12345) + (right_dual.e3215 * this.e4) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(right_dual.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.scalar) * this.group3.xyz) + (float3(this.scalar) * right_dual.group1.xyz)), right_dual.scalar * this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.scalar) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_dual.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e15 * this.e4, right_dual.e25 * this.e4, right_dual.e35 * this.e4, -(right_dual.e23 * this.e1) - (right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) + (float4(right_dual.scalar) * this.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.scalar) * this.group7.xyz) + (float3(this.e4) * right_dual.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.scalar) * this.group8.xyz) + (float3(this.e5) * right_dual.group0.xyz) + (right_dual.group1.zxy * this.group1.yzx) - (right_dual.group1.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar * this.e4235, right_dual.scalar * this.e4315, right_dual.scalar * this.e4125, -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + (right_dual.group0 * float4(this.group3.www, this.e3215)) + (right_dual.group1.zxyw * float4(this.group4.xyz.yzx, this.scalar)) - (right_dual.group1.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.scalar * this.e1234) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_dual.scalar * this.scalar, (right_dual.scalar * this.e12345) + (right_dual.e12345 * this.scalar) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(right_dual.scalar) * this.group1) + (float4(this.scalar) * right_dual.group1), 
            /* e5, 0, 0, 0 */
            float4((right_dual.scalar * this.e5) + (right_dual.e5 * this.scalar), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_dual.scalar) * this.group3) + (float4(right_dual.e5) * this.group1) + (float4(this.scalar) * right_dual.group3) - (float4(this.e5) * right_dual.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.scalar) * this.group4.xyz) + (float3(this.scalar) * right_dual.group4.xyz) + (float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_dual.group5.xyz) + (right_dual.group1.zxy * this.group1.yzx) - (right_dual.group1.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e4 * this.e15) + (right_dual.e5 * this.e41) + (right_dual.e15 * this.e4) + (right_dual.e41 * this.e5), (right_dual.e4 * this.e25) + (right_dual.e5 * this.e42) + (right_dual.e25 * this.e4) + (right_dual.e42 * this.e5), (right_dual.e4 * this.e35) + (right_dual.e5 * this.e43) + (right_dual.e35 * this.e4) + (right_dual.e43 * this.e5), -(right_dual.e1 * this.e23) - (right_dual.e2 * this.e31) - (right_dual.e3 * this.e12) - (right_dual.e12 * this.e3)) + (float4(right_dual.scalar) * this.group6) + (float4(this.scalar) * right_dual.group6) - (this.group1.xyzy * float4(right_dual.group3.www, right_dual.e31)) - float4((this.group3.www * right_dual.group1.xyz), right_dual.e23 * this.e1), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.scalar) * this.group7.xyz) + (float3(right_dual.e4) * this.group5.xyz) + (float3(this.scalar) * right_dual.group7.xyz) + (float3(this.e4) * right_dual.group5.xyz) + (right_dual.group4.xyz.yzx * this.group1.zxy) + (this.group4.xyz.yzx * right_dual.group1.zxy) - (right_dual.group4.xyz.zxy * this.group1.yzx) - (this.group4.xyz.zxy * right_dual.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.scalar) * this.group8.xyz) + (float3(right_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_dual.group8.xyz) + (float3(this.e5) * right_dual.group5.xyz) + (right_dual.group1.yzx * this.group3.zxy) + (right_dual.group3.zxy * this.group1.yzx) - (right_dual.group1.zxy * this.group3.yzx) - (right_dual.group3.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e2 * this.e435) + (right_dual.e5 * this.e423) + (right_dual.e45 * this.e23) + (right_dual.e425 * this.e3), (right_dual.e3 * this.e415) + (right_dual.e5 * this.e431) + (right_dual.e45 * this.e31) + (right_dual.e435 * this.e1), (right_dual.e1 * this.e425) + (right_dual.e5 * this.e412) + (right_dual.e45 * this.e12) + (right_dual.e415 * this.e2), -(right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12) - (right_dual.e321 * this.e5)) + (float4(right_dual.scalar) * this.group9) + (float4(this.scalar) * right_dual.group9) + float4((right_dual.group5.xyz * this.group3.www), right_dual.e2 * this.e315) + float4((right_dual.group8.xyz * this.group1.www), right_dual.e3 * this.e125) + float4((right_dual.group4.xyz.yzx * this.group3.zxy), right_dual.e1 * this.e235) + float4((this.group4.xyz.yzx * right_dual.group3.zxy), right_dual.e5 * this.e321) - (this.group1.yzxz * float4(right_dual.group6.zxy, right_dual.e125)) - (this.group3.yzxx * float4(right_dual.group4.xyz.zxy, right_dual.e23)) - float4((float3(this.e5) * right_dual.group7.xyz), right_dual.e31 * this.e25) - float4((this.group8.xyz * right_dual.group1.www), right_dual.e235 * this.e1) - float4((this.group4.xyz.zxy * right_dual.group3.yzx), right_dual.e12 * this.e35) - float4((right_dual.group1.zxy * this.group6.yzx), right_dual.e315 * this.e2), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.scalar * this.e1234) + (right_dual.e321 * this.e4) + (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e1234 * this.scalar) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<Plane> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(right_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_dual.group0.xyz)), right_dual.e5 * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_dual.group0.zxy * this.group1.yzx) - (right_dual.group0.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e5 * this.e41, right_dual.e5 * this.e42, right_dual.e5 * this.e43, -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) - float4((this.group3.www * right_dual.group0.xyz), right_dual.e1 * this.e23), 
            /* e423, e431, e412, 0 */
            float4((this.group4.xyz.yzx * right_dual.group0.zxy) - (this.group4.xyz.zxy * right_dual.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group5.xyz) + (right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group0.wwwx * float4(this.group7.xyz, this.e235)) + float4((right_dual.group0.yzx * this.group6.zxy), right_dual.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<RoundPoint> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
            float4(this.scalar) * right_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<Sphere> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) + (this.e1234 * right_dual.e5)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * right_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * this.group1) - (float4(this.e5) * right_dual.group0), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * right_dual.group0.zxy) - (this.group1.zxy * right_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_dual.e4) + (this.e41 * right_dual.e5), (this.e25 * right_dual.e4) + (this.e42 * right_dual.e5), (this.e35 * right_dual.e4) + (this.e43 * right_dual.e5), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group5.xyz) + (this.group4.xyz.yzx * right_dual.group0.zxy) - (this.group4.xyz.zxy * right_dual.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group5.xyz) + (this.group3.zxy * right_dual.group0.yzx) - (this.group3.yzx * right_dual.group0.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_dual.e3) - (this.e235 * right_dual.e4), -(this.e435 * right_dual.e1) - (this.e315 * right_dual.e4), -(this.e415 * right_dual.e2) - (this.e125 * right_dual.e4), (this.e321 * right_dual.e5) + (this.e125 * right_dual.e3)) + (right_dual.group0.yzxy * float4(this.group6.zxy, this.e315)) + float4((float3(right_dual.e5) * this.group7.xyz), this.e235 * right_dual.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e321 * right_dual.e4) - (this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<VersorEven> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar * right_dual.scalar, (this.e12345 * right_dual.scalar) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * float4(right_dual.group2.xyz, right_dual.e45)) + (float4(right_dual.scalar) * this.group3), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * right_dual.group0.xyz) + (float3(right_dual.scalar) * this.group4.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group5.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e5 * right_dual.e41) + (this.e415 * right_dual.scalar), (this.e5 * right_dual.e42) + (this.e425 * right_dual.scalar), (this.e5 * right_dual.e43) + (this.e435 * right_dual.scalar), -(this.e2 * right_dual.e31) - (this.e3 * right_dual.e12)) + float4((this.group1.www * right_dual.group2.xyz), this.e321 * right_dual.scalar) - (this.group1.xyzx * right_dual.group1.wwwx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group7.xyz) + (this.group1.zxy * right_dual.group0.yzx) - (this.group1.yzx * right_dual.group0.zxy), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group1.xyz) + (float3(right_dual.scalar) * this.group8.xyz) + (this.group1.yzx * right_dual.group2.zxy) - (this.group1.zxy * right_dual.group2.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_dual.e42) + (this.e45 * right_dual.e23) + (this.e23 * right_dual.e45) + (this.e4235 * right_dual.scalar), (this.e15 * right_dual.e43) + (this.e45 * right_dual.e31) + (this.e31 * right_dual.e45) + (this.e4315 * right_dual.scalar), (this.e25 * right_dual.e41) + (this.e45 * right_dual.e12) + (this.e12 * right_dual.e45) + (this.e4125 * right_dual.scalar), -(this.e15 * right_dual.e23) - (this.e25 * right_dual.e31) - (this.e35 * right_dual.e12) - (this.e12 * right_dual.e35)) + (float4(this.scalar) * right_dual.group3) + float4((this.group4.xyz.yzx * right_dual.group2.zxy), this.e3215 * right_dual.scalar) - (right_dual.group2.yzxx * float4(this.group4.xyz.zxy, this.e23)) - float4((this.group3.yzx * right_dual.group0.zxy), this.e31 * right_dual.e25), 
            /* e1234, 0, 0, 0 */
            float4((this.scalar * right_dual.e1234) + (this.e1234 * right_dual.scalar) - (this.e41 * right_dual.e23) - (this.e42 * right_dual.e31) - (this.e43 * right_dual.e12) - (this.e23 * right_dual.e41) - (this.e31 * right_dual.e42) - (this.e12 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkExpansion<VersorOdd> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.scalar * right_dual.e12345) + (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) + (this.e1234 * right_dual.e5) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group3, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * right_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * this.group1) - (float4(this.e5) * right_dual.group3), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * right_dual.group3.zxy) - (this.group1.zxy * right_dual.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_dual.e4) + (this.e41 * right_dual.e5), (this.e25 * right_dual.e4) + (this.e42 * right_dual.e5), (this.e35 * right_dual.e4) + (this.e43 * right_dual.e5), -(this.e31 * right_dual.e2) - (this.e12 * right_dual.e3)) + (float4(this.scalar) * right_dual.group1) - (right_dual.group3.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.scalar) * right_dual.group0.xyz) + (float3(right_dual.e4) * this.group5.xyz) + (this.group4.xyz.yzx * right_dual.group3.zxy) - (this.group4.xyz.zxy * right_dual.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.scalar) * right_dual.group2.xyz) + (float3(right_dual.e5) * this.group5.xyz) + (this.group3.zxy * right_dual.group3.yzx) - (this.group3.yzx * right_dual.group3.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (this.group6.zxyw * float4(right_dual.group3.yzx, right_dual.e5)) + float4((this.group7.xyz * right_dual.group2.www), this.e235 * right_dual.e1) + float4((this.group1.zxy * right_dual.group1.yzx), this.e315 * right_dual.e2) + float4((this.group1.www * right_dual.group2.xyz), this.e125 * right_dual.e3) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group1.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((this.group8.xyz * right_dual.group3.www), this.e1 * right_dual.e235) - float4((this.group6.yzx * right_dual.group3.zxy), this.e3 * right_dual.e125), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_dual.e423) + (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321) - (this.e321 * right_dual.e4) - (this.e423 * right_dual.e1) - (this.e431 * right_dual.e2) - (this.e412 * right_dual.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<Plane> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension Plane: BulkExpansion<AntiScalar> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiScalar) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension Plane: BulkExpansion<CircleRotor> {
    associatedtype Output = Plane;
    func bulk_expansion(other: CircleRotor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group2.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
extension Plane: BulkExpansion<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_dual.e4 * this.e3215) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: BulkExpansion<DualNum> {
    associatedtype Output = Plane;
    func bulk_expansion(other: DualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension Plane: BulkExpansion<Flector> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: BulkExpansion<Motor> {
    associatedtype Output = Plane;
    func bulk_expansion(other: Motor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
extension Plane: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
            float4(right_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane: BulkExpansion<Plane> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: BulkExpansion<Sphere> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: BulkExpansion<VersorEven> {
    associatedtype Output = Plane;
    func bulk_expansion(other: VersorEven) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
extension Plane: BulkExpansion<VersorOdd> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<RoundPoint> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension RoundPoint: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e235)), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(right_dual.e25 * this.e3 * -1.0, right_dual.e35 * this.e1 * -1.0, right_dual.e15 * this.e2 * -1.0, (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4)) + (float4(this.e5) * float4(right_dual.group1.xyz, right_dual.e1234)) + (this.group0.yzxx * float4(right_dual.group2.zxy, right_dual.e4235))
        );
    }
}
extension RoundPoint: BulkExpansion<AntiDualNum> {
    associatedtype Output = FlatPoint;
    func bulk_expansion(other: AntiDualNum) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group0
        );
    }
}
extension RoundPoint: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = Line;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4((float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e45) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((right_dual.group0.zxy * this.group0.yzx) - (right_dual.group0.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<AntiFlector> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e45 * this.e1 * -1.0, right_dual.e45 * this.e2 * -1.0, right_dual.e45 * this.e3 * -1.0, (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4)) + (this.group0.wwwx * float4(right_dual.group0.xyz, right_dual.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group0.zxy * this.group0.yzx) - (right_dual.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<AntiLine> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (this.group0.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235))
        );
    }
}
extension RoundPoint: BulkExpansion<AntiMotor> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (this.group0.yzxx * float4(right_dual.group0.zxy, right_dual.e235))
        );
    }
}
extension RoundPoint: BulkExpansion<AntiPlane> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<AntiScalar> {
    associatedtype Output = RoundPoint;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_dual.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<Circle> {
    associatedtype Output = Circle;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.xyz.zxy * this.group0.yzx) - (right_dual.group2.xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<CircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group0.zxy) - (right_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(this.e5) * right_dual.group1.xyz) + (right_dual.group2.zxy * this.group0.yzx) - (right_dual.group2.yzx * this.group0.zxy)), right_dual.scalar * this.e4), 
            /* e1, e2, e3, e5 */
            float4(right_dual.scalar) * float4(this.group0.xyz, this.e5)
        );
    }
}
extension RoundPoint: BulkExpansion<Dipole> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e321 * this.e5) - (right_dual.e125 * this.e3)) - (this.group0.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((float3(this.e5) * right_dual.group0.xyz), right_dual.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (right_dual.group3.zxyw * this.group0.yzxw) - float4((right_dual.group3.yzx * this.group0.zxy), right_dual.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e1 * this.e5 * -1.0, right_dual.e2 * this.e5 * -1.0, right_dual.e3 * this.e5 * -1.0, (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e235))
        );
    }
}
extension RoundPoint: BulkExpansion<DualNum> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension RoundPoint: BulkExpansion<FlatPoint> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_dual.group0.xyz), -(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)), 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<Flector> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_dual.group1.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group1.zxy * this.group0.yzx) - (right_dual.group1.yzx * this.group0.zxy)), right_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group1.xyz)), right_dual.e321 * this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_dual.group0.xyz), -(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5))
        );
    }
}
extension RoundPoint: BulkExpansion<Line> {
    associatedtype Output = Circle;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), -(right_dual.e23 * this.e1) - (right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group0.xyz) + (right_dual.group1.xyz.zxy * this.group0.yzx) - (right_dual.group1.xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<Motor> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension RoundPoint: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_dual.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * this.group0) - (float4(this.e5) * right_dual.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_dual.group1.zxy * this.group0.yzx) - (right_dual.group1.yzx * this.group0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e15 * this.e4) + (right_dual.e41 * this.e5), (right_dual.e25 * this.e4) + (right_dual.e42 * this.e5), (right_dual.e35 * this.e4) + (right_dual.e43 * this.e5), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group5.xyz) + (right_dual.group4.xyz.yzx * this.group0.zxy) - (right_dual.group4.xyz.zxy * this.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_dual.group5.xyz) + (right_dual.group3.zxy * this.group0.yzx) - (right_dual.group3.yzx * this.group0.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e321 * this.e5) - (right_dual.e125 * this.e3)) - (this.group0.yzxy * float4(right_dual.group6.zxy, right_dual.e315)) - float4((float3(this.e5) * right_dual.group7.xyz), right_dual.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e321 * this.e4) + (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<Plane> {
    associatedtype Output = Dipole;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group0.zxy * this.group0.yzx) - (right_dual.group0.yzx * this.group0.zxy)), right_dual.e5 * this.e4), 
            /* e15, e25, e35, 0 */
            float4((float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group0.xyz), 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<Sphere> {
    associatedtype Output = Dipole;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_dual.group0.zxy * this.group0.yzx), right_dual.e5 * this.e4) - (right_dual.group0.yzxw * float4(this.group0.zxy, this.e5)), 
            /* e15, e25, e35, 0 */
            float4((float3(right_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_dual.group0.xyz), 0.0)
        );
    }
}
extension RoundPoint: BulkExpansion<VersorEven> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension RoundPoint: BulkExpansion<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(right_dual.group3.zxy, right_dual.e5)) - float4((this.group0.zxy * right_dual.group3.yzx), this.e5 * right_dual.e4), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * right_dual.e1 * -1.0, this.e5 * right_dual.e2 * -1.0, this.e5 * right_dual.e3 * -1.0, (this.e2 * right_dual.e431) + (this.e3 * right_dual.e412) + (this.e4 * right_dual.e321)) + (this.group0.xyzx * float4(right_dual.group2.www, right_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e3 * right_dual.e425) + (this.e4 * right_dual.e235), (this.e1 * right_dual.e435) + (this.e4 * right_dual.e315), (this.e2 * right_dual.e415) + (this.e4 * right_dual.e125), -(this.e3 * right_dual.e125) - (this.e5 * right_dual.e321)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e235)) - float4((float3(this.e5) * right_dual.group0.xyz), this.e2 * right_dual.e315)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<Scalar> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension Scalar: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * right_dual.group2
        );
    }
}
extension Scalar: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group3
        );
    }
}
extension Scalar: BulkExpansion<AntiDualNum> {
    associatedtype Output = DualNum;
    func bulk_expansion(other: AntiDualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.scalar) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension Scalar: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = FlatPoint;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
extension Scalar: BulkExpansion<AntiFlector> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
extension Scalar: BulkExpansion<AntiLine> {
    associatedtype Output = Line;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_dual.group1.xyz, 0.0)
        );
    }
}
extension Scalar: BulkExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * right_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
extension Scalar: BulkExpansion<AntiPlane> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension Scalar: BulkExpansion<AntiScalar> {
    associatedtype Output = Scalar;
    func bulk_expansion(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: BulkExpansion<Circle> {
    associatedtype Output = Dipole;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * right_dual.group2.xyz, 0.0)
        );
    }
}
extension Scalar: BulkExpansion<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.scalar) * right_dual.group2
        );
    }
}
extension Scalar: BulkExpansion<Dipole> {
    associatedtype Output = Circle;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_dual.group2.xyz, 0.0)
        );
    }
}
extension Scalar: BulkExpansion<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.scalar) * right_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group3
        );
    }
}
extension Scalar: BulkExpansion<DualNum> {
    associatedtype Output = AntiDualNum;
    func bulk_expansion(other: DualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.scalar) * float2(other.e5 * -1.0, other.e12345 * -1.0), 0.0, 0.0)
        );
    }
}
extension Scalar: BulkExpansion<FlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func bulk_expansion(other: FlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
extension Scalar: BulkExpansion<Flector> {
    associatedtype Output = AntiFlector;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
extension Scalar: BulkExpansion<Line> {
    associatedtype Output = AntiLine;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * right_dual.group1.xyz, 0.0)
        );
    }
}
extension Scalar: BulkExpansion<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.scalar) * right_dual.group1
        );
    }
}
extension Scalar: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar) * right_dual.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(right_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_dual.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_dual.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: BulkExpansion<Plane> {
    associatedtype Output = AntiPlane;
    func bulk_expansion(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension Scalar: BulkExpansion<RoundPoint> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: BulkExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: BulkExpansion<Sphere> {
    associatedtype Output = RoundPoint;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(right_dual.e5 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: BulkExpansion<VersorEven> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension Scalar: BulkExpansion<VersorOdd> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<Sphere> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension Sphere: BulkExpansion<AntiDualNum> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiDualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e3215 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<AntiMotor> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e3215 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<AntiScalar> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<CircleRotor> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_dual.e4 * this.e3215) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<DualNum> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<Flector> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<Motor> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
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
            float4(right_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<Plane> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<Sphere> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<VersorEven> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * right_dual.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkExpansion<VersorOdd> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) + (this.e1234 * right_dual.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<VersorEven> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension VersorEven: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e235)), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group3.zxy) - (right_dual.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(right_dual.group1.xyz, right_dual.e1234)) + (this.group3.yzxx * float4(right_dual.group2.zxy, right_dual.e4235)) + float4(float3(0.0), (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) - float4((right_dual.group2.yzx * this.group3.zxy), right_dual.e41 * this.e235)
        );
    }
}
extension VersorEven: BulkExpansion<AntiDualNum> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension VersorEven: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e15 * this.e4, right_dual.e25 * this.e4, right_dual.e35 * this.e4, -(right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321)) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group0.zxy * this.group3.yzx) - (right_dual.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
extension VersorEven: BulkExpansion<AntiFlector> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (this.group3.wwwx * float4(right_dual.group0.xyz, right_dual.e4235)) + float4(float3(0.0), (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321)) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual.group0.zxy * this.group3.yzx) - (right_dual.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
extension VersorEven: BulkExpansion<AntiLine> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) - (this.group3.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e235))
        );
    }
}
extension VersorEven: BulkExpansion<AntiMotor> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e235 * this.e4) + (right_dual.e5 * this.e423), (right_dual.e315 * this.e4) + (right_dual.e5 * this.e431), (right_dual.e125 * this.e4) + (right_dual.e5 * this.e412), -(right_dual.e315 * this.e2) - (right_dual.e125 * this.e3)) + float4((right_dual.group0.yzx * this.group3.zxy), right_dual.e5 * this.e321) - (this.group3.yzxx * float4(right_dual.group0.zxy, right_dual.e235))
        );
    }
}
extension VersorEven: BulkExpansion<AntiPlane> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: BulkExpansion<AntiScalar> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: BulkExpansion<Circle> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) + (right_dual.group0.xyz.yzx * this.group3.zxy) - (right_dual.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e41 * this.e5) + (right_dual.e15 * this.e4), (right_dual.e42 * this.e5) + (right_dual.e25 * this.e4), (right_dual.e43 * this.e5) + (right_dual.e35 * this.e4), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.e23 * this.e5) + (right_dual.e35 * this.e2), (right_dual.e31 * this.e5) + (right_dual.e15 * this.e3), (right_dual.e12 * this.e5) + (right_dual.e25 * this.e1), -(right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) - float4((right_dual.group2.xyz.yzx * this.group3.zxy), right_dual.e41 * this.e235)
        );
    }
}
extension VersorEven: BulkExpansion<CircleRotor> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: BulkExpansion<Dipole> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e425 * this.e3) + (right_dual.e235 * this.e4), (right_dual.e435 * this.e1) + (right_dual.e315 * this.e4), (right_dual.e415 * this.e2) + (right_dual.e125 * this.e4), -(right_dual.e321 * this.e5) - (right_dual.e125 * this.e3)) - (this.group3.yzxy * float4(right_dual.group1.zxy, right_dual.e315)) - float4((right_dual.group0.xyz * this.group2.www), right_dual.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: BulkExpansion<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (right_dual.group3.zxyw * this.group3.yzxw) - float4((right_dual.group3.yzx * this.group3.zxy), right_dual.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e423)) + float4(float3(0.0), (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)) - float4((this.group2.www * right_dual.group3.xyz), right_dual.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_dual.group1.yzx * this.group3.zxy), right_dual.e1 * this.e235) + float4((this.group3.www * right_dual.group2.xyz), right_dual.e2 * this.e315) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e235)) - float4((right_dual.group3.zxy * this.group1.yzx), right_dual.e125 * this.e3)
        );
    }
}
extension VersorEven: BulkExpansion<DualNum> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: BulkExpansion<FlatPoint> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_dual.group0.xyz), -(right_dual.e235 * this.e1) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)), 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: BulkExpansion<Flector> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_dual.group1.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group1.zxy * this.group3.yzx) - (right_dual.group1.yzx * this.group3.zxy)), right_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            (this.group3 * float4(right_dual.group1.www, right_dual.e321)) + float4(float3(0.0), -(right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)) - (right_dual.group1.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + (right_dual.group1.wwwz * float4(this.group0.xyz, this.e125)) + float4(float3(0.0), (right_dual.e5 * this.e321) - (right_dual.e315 * this.e2) - (right_dual.e125 * this.e3) - (right_dual.e321 * this.e5)) + float4((this.group3.www * right_dual.group0.xyz), right_dual.e1 * this.e235) - float4((right_dual.group1.zxy * this.group1.yzx), right_dual.e235 * this.e1)
        );
    }
}
extension VersorEven: BulkExpansion<Line> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_dual.group1.xyz), -(right_dual.e23 * this.e1) - (right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)), 
            /* e235, e315, e125, e12345 */
            float4((right_dual.e23 * this.e5) + (right_dual.e35 * this.e2), (right_dual.e31 * this.e5) + (right_dual.e15 * this.e3), (right_dual.e12 * this.e5) + (right_dual.e25 * this.e1), -(right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) - float4((right_dual.group1.xyz.yzx * this.group3.zxy), right_dual.e23 * this.e415)
        );
    }
}
extension VersorEven: BulkExpansion<Motor> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_dual.scalar * this.e12345) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.scalar) * this.group3, 
            /* e5, 0, 0, 0 */
            float4(right_dual.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_dual.e5) * this.group3) - (float4(this.e5) * right_dual.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group1.xyz) - (float3(right_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_dual.group1.zxy * this.group3.yzx) - (right_dual.group1.yzx * this.group3.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e15 * this.e4) + (right_dual.e41 * this.e5), (right_dual.e25 * this.e4) + (right_dual.e42 * this.e5), (right_dual.e35 * this.e4) + (right_dual.e43 * this.e5), -(right_dual.e31 * this.e2) - (right_dual.e12 * this.e3)) + (float4(right_dual.scalar) * this.group1) - (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.scalar) * this.group0.xyz) + (float3(this.e4) * right_dual.group5.xyz) + (right_dual.group4.xyz.yzx * this.group3.zxy) - (right_dual.group4.xyz.zxy * this.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_dual.group5.xyz) + (right_dual.group3.zxy * this.group3.yzx) - (right_dual.group3.yzx * this.group3.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_dual.group8.xyz * this.group3.www), right_dual.e1 * this.e235) + float4((right_dual.group6.yzx * this.group3.zxy), right_dual.e3 * this.e125) - (right_dual.group6.zxyw * float4(this.group3.yzx, this.e5)) - float4((right_dual.group7.xyz * this.group2.www), right_dual.e235 * this.e1) - float4((right_dual.group1.zxy * this.group1.yzx), right_dual.e315 * this.e2) - float4((right_dual.group1.www * this.group2.xyz), right_dual.e125 * this.e3), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e321 * this.e4) + (right_dual.e423 * this.e1) + (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) - (right_dual.e1 * this.e423) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: BulkExpansion<Plane> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_dual.group0.zxy * this.group3.yzx) - (right_dual.group0.yzx * this.group3.zxy)), right_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e5 * this.e1, right_dual.e5 * this.e2, right_dual.e5 * this.e3, -(right_dual.e2 * this.e431) - (right_dual.e3 * this.e412)) - (right_dual.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e3 * this.e425 * -1.0, right_dual.e1 * this.e435 * -1.0, right_dual.e2 * this.e415 * -1.0, (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group0.yzxx * float4(this.group1.zxy, this.e235)) + (right_dual.group0.wwwy * float4(this.group0.xyz, this.e315))
        );
    }
}
extension VersorEven: BulkExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: BulkExpansion<Sphere> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group0.xyz) - (float3(right_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_dual.group0.zxy * this.group3.yzx), right_dual.e5 * this.e4) - (right_dual.group0.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e5 * this.e1, right_dual.e5 * this.e2, right_dual.e5 * this.e3, -(right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)) - (right_dual.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual.e3 * this.e425) - (right_dual.e4 * this.e235), -(right_dual.e1 * this.e435) - (right_dual.e4 * this.e315), -(right_dual.e2 * this.e415) - (right_dual.e4 * this.e125), (right_dual.e3 * this.e125) + (right_dual.e5 * this.e321)) + (right_dual.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4((float3(right_dual.e5) * this.group0.xyz), right_dual.e2 * this.e315)
        );
    }
}
extension VersorEven: BulkExpansion<VersorEven> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: BulkExpansion<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_dual.group3.xyz) - (float3(right_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (this.group3.yzxw * float4(right_dual.group3.zxy, right_dual.e5)) - (right_dual.group3.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_dual.group2.www, right_dual.e423)) + float4(float3(0.0), (right_dual.e431 * this.e2) + (right_dual.e412 * this.e3) + (right_dual.e321 * this.e4) - (right_dual.e2 * this.e431) - (right_dual.e3 * this.e412) - (right_dual.e4 * this.e321)) - (right_dual.group3.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_dual.group1.yzx * this.group3.zxy), right_dual.e5 * this.e321) + float4((right_dual.group2.www * this.group0.xyz), right_dual.e2 * this.e315) + float4((this.group3.www * right_dual.group2.xyz), right_dual.e1 * this.e235) - (float4(this.e5) * float4(right_dual.group0.xyz, right_dual.e321)) - (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e235)) - float4((right_dual.group3.zxy * this.group1.yzx), right_dual.e315 * this.e2) - float4((right_dual.group3.www * this.group2.xyz), right_dual.e125 * this.e3)
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: BulkExpansionInfix) -> BulkExpansionInfixPartial<VersorOdd> {
        return BulkExpansionInfixPartial { a: this };
    }
}
extension VersorOdd: BulkExpansion<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_dual.group2.xyz), (right_dual.e12345 * this.scalar) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43))
        );
    }
}
extension VersorOdd: BulkExpansion<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group2.xyz), (right_dual.e1234 * this.scalar) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42) + (right_dual.e4235 * this.scalar), (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43) + (right_dual.e4315 * this.scalar), (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41) + (right_dual.e4125 * this.scalar), -(right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_dual.group0.xyz.zxy, right_dual.e23)) - float4((right_dual.group2.yzx * this.group0.zxy), right_dual.e31 * this.e25)
        );
    }
}
extension VersorOdd: BulkExpansion<AntiDualNum> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.group0.xy.xx, right_dual.e5, (right_dual.e5 * this.e1234) + (right_dual.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
extension VersorOdd: BulkExpansion<AntiFlatPoint> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e35 * this.e42) + (right_dual.e45 * this.e23), (right_dual.e15 * this.e43) + (right_dual.e45 * this.e31), (right_dual.e25 * this.e41) + (right_dual.e45 * this.e12), -(right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - (right_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension VersorOdd: BulkExpansion<AntiFlector> {
    associatedtype Output = Flector;
    func bulk_expansion(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e45 * this.e23) + (right_dual.e4235 * this.scalar), (right_dual.e45 * this.e31) + (right_dual.e4315 * this.scalar), (right_dual.e45 * this.e12) + (right_dual.e4125 * this.scalar), -(right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) + (this.group0.yzxw * float4(right_dual.group0.zxy, right_dual.e3215)) - (right_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension VersorOdd: BulkExpansion<AntiLine> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_dual.group0.xyz), -(right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension VersorOdd: BulkExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_expansion(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (float4(right_dual.e5) * float4(this.group0.xyz, this.e1234)) + (float4(this.scalar) * right_dual.group0) + float4(float3(0.0), -(right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_dual.group1.xyz)), right_dual.e5 * this.scalar)
        );
    }
}
extension VersorOdd: BulkExpansion<AntiPlane> {
    associatedtype Output = Plane;
    func bulk_expansion(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension VersorOdd: BulkExpansion<AntiScalar> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: AntiScalar) -> Output {
        Scalar right_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: BulkExpansion<Circle> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e42 * this.e35) + (right_dual.e23 * this.e45) + (right_dual.e45 * this.e23) + (right_dual.e35 * this.e42), (right_dual.e43 * this.e15) + (right_dual.e31 * this.e45) + (right_dual.e45 * this.e31) + (right_dual.e15 * this.e43), (right_dual.e41 * this.e25) + (right_dual.e12 * this.e45) + (right_dual.e45 * this.e12) + (right_dual.e25 * this.e41), -(right_dual.e23 * this.e15) - (right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e35 * this.e12)) - float4((right_dual.group0.xyz.zxy * this.group2.yzx), right_dual.e15 * this.e23) - float4((right_dual.group2.xyz.yzx * this.group0.zxy), right_dual.e25 * this.e31)
        );
    }
}
extension VersorOdd: BulkExpansion<CircleRotor> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: BulkExpansion<Dipole> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_dual.group2.xyz), -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43))
        );
    }
}
extension VersorOdd: BulkExpansion<DipoleInversion> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension VersorOdd: BulkExpansion<DualNum> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: BulkExpansion<FlatPoint> {
    associatedtype Output = CircleRotor;
    func bulk_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.e321 * this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_dual.group0.xyz), -(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45))
        );
    }
}
extension VersorOdd: BulkExpansion<Flector> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension VersorOdd: BulkExpansion<Line> {
    associatedtype Output = DipoleInversion;
    func bulk_expansion(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_dual.group1.xyz), -(right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e23 * this.e45) + (right_dual.e35 * this.e42), (right_dual.e31 * this.e45) + (right_dual.e15 * this.e43), (right_dual.e12 * this.e45) + (right_dual.e25 * this.e41), -(right_dual.e31 * this.e25) - (right_dual.e12 * this.e35) - (right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12)) - float4((right_dual.group1.xyz.yzx * this.group0.zxy), right_dual.e23 * this.e15)
        );
    }
}
extension VersorOdd: BulkExpansion<Motor> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: BulkExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_expansion(other: MultiVector) -> Output {
        MultiVector right_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(other.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(other.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_dual.scalar * this.scalar, (right_dual.e12345 * this.scalar) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(right_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_dual.scalar) * float4(this.group2.xyz, this.e45)) + (float4(this.scalar) * right_dual.group3), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_dual.group4.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.scalar) * this.group1.xyz) + (float3(this.scalar) * right_dual.group5.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual.e5 * this.e41) + (right_dual.e415 * this.scalar), (right_dual.e5 * this.e42) + (right_dual.e425 * this.scalar), (right_dual.e5 * this.e43) + (right_dual.e435 * this.scalar), -(right_dual.e2 * this.e31) - (right_dual.e3 * this.e12)) + float4((right_dual.group1.www * this.group2.xyz), right_dual.e321 * this.scalar) - (right_dual.group1.xyzx * this.group1.wwwx), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e4) * this.group1.xyz) + (float3(this.scalar) * right_dual.group7.xyz) + (right_dual.group1.zxy * this.group0.yzx) - (right_dual.group1.yzx * this.group0.zxy), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_dual.group8.xyz) + (right_dual.group1.yzx * this.group2.zxy) - (right_dual.group1.zxy * this.group2.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual.e35 * this.e42) + (right_dual.e45 * this.e23) + (right_dual.e23 * this.e45) + (right_dual.e4235 * this.scalar), (right_dual.e15 * this.e43) + (right_dual.e45 * this.e31) + (right_dual.e31 * this.e45) + (right_dual.e4315 * this.scalar), (right_dual.e25 * this.e41) + (right_dual.e45 * this.e12) + (right_dual.e12 * this.e45) + (right_dual.e4125 * this.scalar), -(right_dual.e15 * this.e23) - (right_dual.e25 * this.e31) - (right_dual.e35 * this.e12) - (right_dual.e12 * this.e35)) + (float4(right_dual.scalar) * this.group3) + float4((right_dual.group4.xyz.yzx * this.group2.zxy), right_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_dual.group4.xyz.zxy, right_dual.e23)) - float4((right_dual.group3.yzx * this.group0.zxy), right_dual.e31 * this.e25), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.scalar * this.e1234) + (right_dual.e1234 * this.scalar) - (right_dual.e41 * this.e23) - (right_dual.e42 * this.e31) - (right_dual.e43 * this.e12) - (right_dual.e23 * this.e41) - (right_dual.e31 * this.e42) - (right_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: BulkExpansion<Plane> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension VersorOdd: BulkExpansion<RoundPoint> {
    associatedtype Output = Sphere;
    func bulk_expansion(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: BulkExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func bulk_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: BulkExpansion<Sphere> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension VersorOdd: BulkExpansion<VersorEven> {
    associatedtype Output = VersorOdd;
    func bulk_expansion(other: VersorEven) -> Output {
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: BulkExpansion<VersorOdd> {
    associatedtype Output = VersorEven;
    func bulk_expansion(other: VersorOdd) -> Output {
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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

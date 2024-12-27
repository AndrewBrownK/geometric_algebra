extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<AntiCircleRotor> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension AntiCircleRotor: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
extension AntiCircleRotor: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e1234) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41) + (this.scalar * right_anti_dual.e4125), -(this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.scalar * right_anti_dual.e3215) - (right_anti_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), this.e31 * right_anti_dual.e25)
        );
    }
}
extension AntiCircleRotor: WeightExpansion<AntiDualNum> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, right_anti_dual.e12345) * float4(this.group0.xyz, this.scalar), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
extension AntiCircleRotor: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension AntiCircleRotor: WeightExpansion<AntiFlector> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4125), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) + float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy), this.scalar * right_anti_dual.e3215) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension AntiCircleRotor: WeightExpansion<AntiLine> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.scalar * right_anti_dual.e415, this.scalar * right_anti_dual.e425, this.scalar * right_anti_dual.e435, -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)) + float4((this.group0.xyz * right_anti_dual.group1.www), this.scalar * right_anti_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e5) * this.group1.xyz)), this.scalar * right_anti_dual.e5)
        );
    }
}
extension AntiCircleRotor: WeightExpansion<AntiPlane> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension AntiCircleRotor: WeightExpansion<AntiScalar> {
    associatedtype Output = AntiCircleRotor;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual.scalar) * this.group2
        );
    }
}
extension AntiCircleRotor: WeightExpansion<Circle> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.xyz.yzx), this.e23 * right_anti_dual.e15) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), this.e31 * right_anti_dual.e25)
        );
    }
}
extension AntiCircleRotor: WeightExpansion<CircleRotor> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension AntiCircleRotor: WeightExpansion<Dipole> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
extension AntiCircleRotor: WeightExpansion<DipoleInversion> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
extension AntiCircleRotor: WeightExpansion<DualNum> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension AntiCircleRotor: WeightExpansion<FlatPoint> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.scalar * right_anti_dual.e321), 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * right_anti_dual.group0.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e45 * right_anti_dual.e321))
        );
    }
}
extension AntiCircleRotor: WeightExpansion<Flector> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension AntiCircleRotor: WeightExpansion<Line> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group1.xyz), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e45 * right_anti_dual.e23), (this.e43 * right_anti_dual.e15) + (this.e45 * right_anti_dual.e31), (this.e41 * right_anti_dual.e25) + (this.e45 * right_anti_dual.e12), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group1.xyz.yzx), this.e15 * right_anti_dual.e23)
        );
    }
}
extension AntiCircleRotor: WeightExpansion<Motor> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension AntiCircleRotor: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(this.scalar * right_anti_dual.scalar, (this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * right_anti_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * right_anti_dual.group3) + (float4(right_anti_dual.scalar) * float4(this.group2.xyz, this.e45)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * right_anti_dual.group4.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e415), (this.e25 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e425), (this.e35 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e435), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + float4((float3(right_anti_dual.e5) * this.group0.xyz), this.scalar * right_anti_dual.e321) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.scalar) * right_anti_dual.group7.xyz) + (float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.scalar) * right_anti_dual.group8.xyz) + (float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group1.yzx) - (this.group2.yzx * right_anti_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41) + (this.scalar * right_anti_dual.e4125), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group3.zxy), this.scalar * right_anti_dual.e3215) - (this.group2.yzxy * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e31)) - float4((this.group0.xyz.zxy * right_anti_dual.group3.yzx), this.e15 * right_anti_dual.e23), 
            /* e1234, 0, 0, 0 */
            float4((this.scalar * right_anti_dual.e1234) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightExpansion<Plane> {
    associatedtype Output = AntiDipoleInversion;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * right_anti_dual.e5, this.e42 * right_anti_dual.e5, this.e43 * right_anti_dual.e5, -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * right_anti_dual.group0.yzx), 0.0) - float4((this.group2.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group0
        );
    }
}
extension AntiCircleRotor: WeightExpansion<RoundPoint> {
    associatedtype Output = Sphere;
    func weight_expansion(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightExpansion<Sphere> {
    associatedtype Output = AntiDipoleInversion;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group0.yzx) - (this.group2.yzx * right_anti_dual.group0.zxy)), this.scalar * right_anti_dual.e4), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(right_anti_dual.group0.xyz, right_anti_dual.e5)
        );
    }
}
extension AntiCircleRotor: WeightExpansion<VersorEven> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
extension AntiCircleRotor: WeightExpansion<VersorOdd> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<AntiDipoleInversion> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension AntiDipoleInversion: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = Sphere;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group3.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group3.www), this.e1 * right_anti_dual.e235), 
            /* e1234, 0, 0, 0 */
            float4((this.e4 * right_anti_dual.e321) + (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4((float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group3.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group3.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e4235)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) + float4((this.group3.yzx * right_anti_dual.group2.zxy), this.e4 * right_anti_dual.e3215) - (right_anti_dual.group2.yzxx * float4(this.group3.zxy, this.e423))
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<AntiDualNum> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e4 * right_anti_dual.e15, this.e4 * right_anti_dual.e25, this.e4 * right_anti_dual.e35, -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * right_anti_dual.group0.zxy) - (this.group3.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<AntiFlector> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (float4(this.e4) * float4(right_anti_dual.group0.xyz, right_anti_dual.e3215)) + float4(float3(0.0), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * right_anti_dual.group0.zxy) - (this.group3.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<AntiLine> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group3.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<AntiMotor> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) - (this.group3.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<AntiPlane> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4 * right_anti_dual.e3215) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<AntiScalar> {
    associatedtype Output = AntiDipoleInversion;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<Circle> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group3.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((right_anti_dual.group2.xyz.yzx * this.group3.zxy), this.e423 * right_anti_dual.e15)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<CircleRotor> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension AntiDipoleInversion: WeightExpansion<Dipole> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * right_anti_dual.e235) + (this.e3 * right_anti_dual.e425), (this.e4 * right_anti_dual.e315) + (this.e1 * right_anti_dual.e435), (this.e4 * right_anti_dual.e125) + (this.e2 * right_anti_dual.e415), -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group3.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e235)) - float4((this.group3.yzx * right_anti_dual.group1.zxy), this.e2 * right_anti_dual.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e4 * right_anti_dual.e321) + (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4((float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group3.zxyw * float4(this.group3.yzx, this.e4)) - (this.group3.zxyw * float4(right_anti_dual.group3.yzx, right_anti_dual.e4)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e4 * this.e321) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - float4((this.group3.www * right_anti_dual.group3.xyz), right_anti_dual.e1 * this.e423), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group3.yzxw * this.group1.zxyw) + (this.group2.wwwz * float4(right_anti_dual.group2.xyz, right_anti_dual.e3)) + float4((this.group0.xyz * right_anti_dual.group3.www), right_anti_dual.e1 * this.e235) + float4((right_anti_dual.group1.yzx * this.group3.zxy), right_anti_dual.e2 * this.e315) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((right_anti_dual.group3.zxy * this.group1.yzx), right_anti_dual.e125 * this.e3)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<DualNum> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension AntiDipoleInversion: WeightExpansion<FlatPoint> {
    associatedtype Output = Sphere;
    func weight_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group2.www * right_anti_dual.group0.xyz), -(this.e1 * right_anti_dual.e235) - (this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)), 
            /* e1234, 0, 0, 0 */
            float4(this.e4 * right_anti_dual.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<Flector> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_anti_dual.group1.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * right_anti_dual.group1.zxy) - (this.group3.zxy * right_anti_dual.group1.yzx)), this.e4 * right_anti_dual.e5), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3)) + float4((right_anti_dual.group1.www * this.group3.xyz), this.e4 * right_anti_dual.e321) - (right_anti_dual.group1.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e2)) + (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * right_anti_dual.e3) - (this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - float4((this.group1.yzx * right_anti_dual.group1.zxy), this.e1 * right_anti_dual.e235)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<Line> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(this.e1 * right_anti_dual.e23) - (this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group3.zxy), this.e423 * right_anti_dual.e15)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<Motor> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension AntiDipoleInversion: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (this.e4 * right_anti_dual.e3215) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * float4(this.group3.xyz, this.e4), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_anti_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * float4(this.group3.xyz, this.e4)) - (float4(this.e5) * right_anti_dual.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group3.yzx * right_anti_dual.group1.zxy) - (this.group3.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e41), (this.e4 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e42), (this.e4 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e43), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group1) - float4((right_anti_dual.group3.www * this.group3.xyz), this.e1 * right_anti_dual.e23), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz) + (right_anti_dual.group4.xyz.yzx * this.group3.zxy) - (right_anti_dual.group4.xyz.zxy * this.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group2.xyz) + (this.group3.yzx * right_anti_dual.group3.zxy) - (this.group3.zxy * right_anti_dual.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_anti_dual.group8.xyz * this.group2.www), this.e235 * right_anti_dual.e1) + float4((this.group3.zxy * right_anti_dual.group6.yzx), this.e125 * right_anti_dual.e3) - (this.group3.yzxw * right_anti_dual.group6.zxyw) - (this.group3.wwwx * float4(right_anti_dual.group7.xyz, right_anti_dual.e235)) - float4((this.group1.yzx * right_anti_dual.group1.zxy), this.e2 * right_anti_dual.e315) - float4((right_anti_dual.group1.www * this.group2.xyz), this.e3 * right_anti_dual.e125), 
            /* e1234, 0, 0, 0 */
            float4((this.e4 * right_anti_dual.e321) + (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) - (this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<Plane> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * right_anti_dual.group0.zxy) - (this.group3.zxy * right_anti_dual.group0.yzx)), this.e4 * right_anti_dual.e5), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * right_anti_dual.e5, this.e2 * right_anti_dual.e5, this.e3 * right_anti_dual.e5, -(this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * right_anti_dual.e3 * -1.0, this.e435 * right_anti_dual.e1 * -1.0, this.e415 * right_anti_dual.e2 * -1.0, (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4 * right_anti_dual.e3215) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<Sphere> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * right_anti_dual.group0.zxy), this.e4 * right_anti_dual.e5) - (this.group3.zxyw * right_anti_dual.group0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * right_anti_dual.e5, this.e2 * right_anti_dual.e5, this.e3 * right_anti_dual.e5, -(this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
extension AntiDipoleInversion: WeightExpansion<VersorEven> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
extension AntiDipoleInversion: WeightExpansion<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4((float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * right_anti_dual.group3.zxy), this.e4 * right_anti_dual.e5) - (this.group3.zxyw * right_anti_dual.group3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4)) + float4((right_anti_dual.group2.www * this.group3.xyz), this.e4 * right_anti_dual.e321) - (right_anti_dual.group3.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(right_anti_dual.group2.xyz, right_anti_dual.e2)) + (right_anti_dual.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4((this.group3.zxy * right_anti_dual.group1.yzx), this.e125 * right_anti_dual.e3) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group3.yzxz * float4(right_anti_dual.group1.zxy, right_anti_dual.e125)) - float4((this.group1.yzx * right_anti_dual.group3.zxy), this.e1 * right_anti_dual.e235) - float4((right_anti_dual.group3.www * this.group2.xyz), this.e2 * right_anti_dual.e315)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<AntiDualNum> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension AntiDualNum: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * right_anti_dual.group2
        );
    }
}
extension AntiDualNum: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}
extension AntiDualNum: WeightExpansion<AntiDualNum> {
    associatedtype Output = DualNum;
    func weight_expansion(other: AntiDualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.scalar) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = FlatPoint;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<AntiFlector> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
extension AntiDualNum: WeightExpansion<AntiLine> {
    associatedtype Output = Line;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_anti_dual.group1.xyz, 0.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
extension AntiDualNum: WeightExpansion<AntiPlane> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<AntiScalar> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: AntiScalar) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.e12345 * -1.0) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<Circle> {
    associatedtype Output = Dipole;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * right_anti_dual.group2.xyz, 0.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<CircleRotor> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension AntiDualNum: WeightExpansion<Dipole> {
    associatedtype Output = Circle;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_anti_dual.group2.xyz, 0.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<DipoleInversion> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
extension AntiDualNum: WeightExpansion<DualNum> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2((right_anti_dual.e3215 * this.scalar) + (right_anti_dual.scalar * this.e3215), right_anti_dual.scalar * this.scalar), 0.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<FlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: FlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
extension AntiDualNum: WeightExpansion<Flector> {
    associatedtype Output = AntiFlector;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
extension AntiDualNum: WeightExpansion<Line> {
    associatedtype Output = AntiLine;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * right_anti_dual.group1.xyz, 0.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<Motor> {
    associatedtype Output = AntiMotor;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e3215)) * float4(right_anti_dual.group1.xyz, 1.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(this.scalar * right_anti_dual.scalar, (this.e3215 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e12345)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * right_anti_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_anti_dual.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_anti_dual.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_anti_dual.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e3215)) * float4(right_anti_dual.group9.xyz, 1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<Plane> {
    associatedtype Output = AntiPlane;
    func weight_expansion(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension AntiDualNum: WeightExpansion<RoundPoint> {
    associatedtype Output = Sphere;
    func weight_expansion(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightExpansion<Sphere> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension AntiDualNum: WeightExpansion<VersorEven> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
extension AntiDualNum: WeightExpansion<VersorOdd> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<AntiFlatPoint> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension AntiFlatPoint: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4(-(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightExpansion<AntiDualNum> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: AntiDualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.e3215 * this.e321, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightExpansion<AntiFlector> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightExpansion<AntiMotor> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: AntiMotor) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e321 * other.e3215, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightExpansion<AntiScalar> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: AntiScalar) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension AntiFlatPoint: WeightExpansion<Circle> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightExpansion<CircleRotor> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
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
            float4(float3(0.0), right_anti_dual.scalar * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.group2.www * this.group0.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e45 * this.e321))
        );
    }
}
extension AntiFlatPoint: WeightExpansion<DipoleInversion> {
    associatedtype Output = Sphere;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4((right_anti_dual.group2.www * this.group0.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e235) + (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)), 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e4 * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightExpansion<DualNum> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: DualNum) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension AntiFlatPoint: WeightExpansion<Flector> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2((this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightExpansion<Motor> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: Motor) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
extension AntiFlatPoint: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, -(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)), 0.0, 0.0), 
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
            float4(float3(0.0), this.e321 * right_anti_dual.scalar), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5)), 
            /* e1234, 0, 0, 0 */
            float4(this.e321 * right_anti_dual.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightExpansion<Plane> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2((this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightExpansion<Sphere> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5)), 
            /* e1234, 0, 0, 0 */
            float4(this.e321 * right_anti_dual.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightExpansion<VersorEven> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4(float3(0.0), this.e321 * right_anti_dual.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45))
        );
    }
}
extension AntiFlatPoint: WeightExpansion<VersorOdd> {
    associatedtype Output = Sphere;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4((float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5)), 
            /* e1234, 0, 0, 0 */
            float4(this.e321 * right_anti_dual.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<AntiFlector> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension AntiFlector: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = Sphere;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group1.www), this.e1 * right_anti_dual.e235), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4((right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group1.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group1.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235)) + (this.group1.wwwy * float4(right_anti_dual.group1.xyz, right_anti_dual.e4315)) + float4(float3(0.0), (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - float4((this.group1.zxy * right_anti_dual.group2.yzx), this.e235 * right_anti_dual.e41)
        );
    }
}
extension AntiFlector: WeightExpansion<AntiDualNum> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e321 * right_anti_dual.e5)
        );
    }
}
extension AntiFlector: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e45) * float4(this.group1.xyz, this.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: WeightExpansion<AntiFlector> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e45) * this.group1.xyz * float3(-1.0)), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) - (this.e321 * right_anti_dual.e45)), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: WeightExpansion<AntiLine> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group1.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
extension AntiFlector: WeightExpansion<AntiMotor> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) + float4((this.group1.zxy * right_anti_dual.group0.yzx), this.e321 * right_anti_dual.e5) - (this.group1.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
extension AntiFlector: WeightExpansion<AntiPlane> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: WeightExpansion<AntiScalar> {
    associatedtype Output = AntiFlector;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
extension AntiFlector: WeightExpansion<Circle> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group1.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - float4((right_anti_dual.group2.xyz.yzx * this.group1.zxy), this.e235 * right_anti_dual.e41)
        );
    }
}
extension AntiFlector: WeightExpansion<CircleRotor> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension AntiFlector: WeightExpansion<Dipole> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group1.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e235)) - float4((this.group1.yzx * right_anti_dual.group1.zxy), this.e2 * right_anti_dual.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: WeightExpansion<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4(float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group3.zxy * this.group1.yzx) - (right_anti_dual.group3.yzx * this.group1.zxy)), right_anti_dual.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3)) - float4((this.group1.www * right_anti_dual.group3.xyz), right_anti_dual.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321) - (right_anti_dual.e125 * this.e3)) + float4((right_anti_dual.group1.yzx * this.group1.zxy), right_anti_dual.e1 * this.e235) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (right_anti_dual.group2.wwwy * float4(this.group0.xyz, this.e2)) - (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235))
        );
    }
}
extension AntiFlector: WeightExpansion<DualNum> {
    associatedtype Output = AntiFlector;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
extension AntiFlector: WeightExpansion<FlatPoint> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(-(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlector: WeightExpansion<Flector> {
    associatedtype Output = AntiMotor;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual.group1.zxy * this.group1.yzx) - (right_anti_dual.group1.yzx * this.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (right_anti_dual.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)) - (this.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e235))
        );
    }
}
extension AntiFlector: WeightExpansion<Line> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group1.zxyx * float4(right_anti_dual.group1.xyz.yzx, right_anti_dual.e23))
        );
    }
}
extension AntiFlector: WeightExpansion<Motor> {
    associatedtype Output = AntiFlector;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e2 * right_anti_dual.e35) + (this.e5 * right_anti_dual.e23), (this.e3 * right_anti_dual.e15) + (this.e5 * right_anti_dual.e31), (this.e1 * right_anti_dual.e25) + (this.e5 * right_anti_dual.e12), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group0) - (this.group1.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e23)), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
extension AntiFlector: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_anti_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_anti_dual.group1.xyz)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * right_anti_dual.group1.zxy) - (this.group1.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + float4((right_anti_dual.group4.xyz * this.group1.www), this.e321 * right_anti_dual.scalar) - float4((right_anti_dual.group3.www * this.group1.xyz), this.e1 * right_anti_dual.e23), 
            /* e423, e431, e412, 0 */
            float4((right_anti_dual.group4.xyz.yzx * this.group1.zxy) - (right_anti_dual.group4.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz) + (this.group1.yzx * right_anti_dual.group3.zxy) - (this.group1.zxy * right_anti_dual.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5) - (this.e5 * right_anti_dual.e321)) + float4((this.group1.zxy * right_anti_dual.group6.yzx), this.e235 * right_anti_dual.e1) - (this.group1.wwwx * float4(right_anti_dual.group7.xyz, right_anti_dual.e235)) - float4((this.group1.yzx * right_anti_dual.group6.zxy), this.e3 * right_anti_dual.e125) - float4((right_anti_dual.group1.www * this.group0.xyz), this.e2 * right_anti_dual.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) - (this.e321 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: WeightExpansion<Plane> {
    associatedtype Output = AntiMotor;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e5 * right_anti_dual.e1 * -1.0, this.e5 * right_anti_dual.e2 * -1.0, this.e5 * right_anti_dual.e3 * -1.0, (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5)) + (right_anti_dual.group0.wwwx * float4(this.group1.xyz, this.e235))
        );
    }
}
extension AntiFlector: WeightExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: WeightExpansion<Sphere> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)), this.e321 * right_anti_dual.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * right_anti_dual.e1) + (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5))
        );
    }
}
extension AntiFlector: WeightExpansion<VersorEven> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
extension AntiFlector: WeightExpansion<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4(float3(right_anti_dual.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * right_anti_dual.group3.zxy) - (this.group1.zxy * right_anti_dual.group3.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)) + float4(float3(0.0), (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)) - (right_anti_dual.group3 * float4(this.group1.www, this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3) + (this.e321 * right_anti_dual.e5) - (this.e5 * right_anti_dual.e321)) + float4((this.group1.zxy * right_anti_dual.group1.yzx), this.e235 * right_anti_dual.e1) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - (this.group1.wwwz * float4(right_anti_dual.group0.xyz, right_anti_dual.e125)) - float4((right_anti_dual.group3.www * this.group0.xyz), this.e1 * right_anti_dual.e235)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<AntiLine> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension AntiLine: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = Sphere;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group0.xyz.zxy), this.e23 * right_anti_dual.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<AntiDualNum> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35))
        );
    }
}
extension AntiLine: WeightExpansion<AntiFlector> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35))
        );
    }
}
extension AntiLine: WeightExpansion<AntiLine> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<AntiScalar> {
    associatedtype Output = AntiLine;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(right_anti_dual.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<Circle> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group0.xyz.zxy), this.e23 * right_anti_dual.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<CircleRotor> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
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
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group1.xyz), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e45 * this.e23), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e45 * this.e31), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e45 * this.e12), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group1.xyz.yzx), right_anti_dual.e15 * this.e23)
        );
    }
}
extension AntiLine: WeightExpansion<Dipole> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<DipoleInversion> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4(float3(right_anti_dual.e4) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(right_anti_dual.e1 * this.e23) - (right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group3.zxy), right_anti_dual.e423 * this.e15)
        );
    }
}
extension AntiLine: WeightExpansion<DualNum> {
    associatedtype Output = AntiLine;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(right_anti_dual.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<Flector> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
extension AntiLine: WeightExpansion<Line> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(-(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<Motor> {
    associatedtype Output = AntiMotor;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(right_anti_dual.scalar) * this.group1.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12))
        );
    }
}
extension AntiLine: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)), 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e4) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz) + (this.group1.xyz.zxy * right_anti_dual.group1.yzx) - (this.group1.xyz.yzx * right_anti_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group4.xyz.zxy), this.e23 * right_anti_dual.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<Plane> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (right_anti_dual.group0.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
extension AntiLine: WeightExpansion<Sphere> {
    associatedtype Output = Circle;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e4) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz) + (this.group1.xyz.zxy * right_anti_dual.group0.yzx) - (this.group1.xyz.yzx * right_anti_dual.group0.zxy), 0.0)
        );
    }
}
extension AntiLine: WeightExpansion<VersorEven> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group1.xyz), -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((this.group1.xyz.yzx * right_anti_dual.group0.zxy), this.e23 * right_anti_dual.e15)
        );
    }
}
extension AntiLine: WeightExpansion<VersorOdd> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4(float3(right_anti_dual.e4) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz), -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) - float4((this.group1.xyz.yzx * right_anti_dual.group3.zxy), this.e23 * right_anti_dual.e415)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<AntiMotor> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension AntiMotor: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group0.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
extension AntiMotor: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group0.www * right_anti_dual.group2.xyz), (this.scalar * right_anti_dual.e1234) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4235), (this.e31 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4315), (this.e12 * right_anti_dual.e45) + (this.scalar * right_anti_dual.e4125), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group1.zxy), this.scalar * right_anti_dual.e3215) - float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx), this.e23 * right_anti_dual.e15)
        );
    }
}
extension AntiMotor: WeightExpansion<AntiDualNum> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), this.scalar * right_anti_dual.e12345), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e5) * this.group0
        );
    }
}
extension AntiMotor: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35))
        );
    }
}
extension AntiMotor: WeightExpansion<AntiFlector> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * right_anti_dual.e4235, this.scalar * right_anti_dual.e4315, this.scalar * right_anti_dual.e4125, -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) + (this.group0 * float4(right_anti_dual.group0.www, right_anti_dual.e3215))
        );
    }
}
extension AntiMotor: WeightExpansion<AntiLine> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: WeightExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((this.group0.www * right_anti_dual.group0.xyz), (this.scalar * right_anti_dual.e12345) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e5) * this.group0.xyz)), this.scalar * right_anti_dual.e5)
        );
    }
}
extension AntiMotor: WeightExpansion<AntiPlane> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension AntiMotor: WeightExpansion<AntiScalar> {
    associatedtype Output = AntiMotor;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
extension AntiMotor: WeightExpansion<Circle> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e35 * right_anti_dual.e42), (this.e31 * right_anti_dual.e45) + (this.e15 * right_anti_dual.e43), (this.e12 * right_anti_dual.e45) + (this.e25 * right_anti_dual.e41), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx), this.e23 * right_anti_dual.e15)
        );
    }
}
extension AntiMotor: WeightExpansion<CircleRotor> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension AntiMotor: WeightExpansion<Dipole> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
extension AntiMotor: WeightExpansion<DipoleInversion> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
extension AntiMotor: WeightExpansion<DualNum> {
    associatedtype Output = AntiMotor;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, (right_anti_dual.e3215 * this.scalar) + (right_anti_dual.scalar * this.e3215)) * float4(this.group1.xyz, 1.0)
        );
    }
}
extension AntiMotor: WeightExpansion<FlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: FlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
extension AntiMotor: WeightExpansion<Flector> {
    associatedtype Output = AntiFlector;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group0) - (right_anti_dual.group1.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
extension AntiMotor: WeightExpansion<Line> {
    associatedtype Output = AntiMotor;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.scalar) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12))
        );
    }
}
extension AntiMotor: WeightExpansion<Motor> {
    associatedtype Output = AntiMotor;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.scalar), 
            /* e15, e25, e35, e3215 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12))
        );
    }
}
extension AntiMotor: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(this.scalar * right_anti_dual.scalar, (this.scalar * right_anti_dual.e12345) + (this.e3215 * right_anti_dual.e4) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * right_anti_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(this.scalar) * right_anti_dual.group3.xyz) + (float3(right_anti_dual.scalar) * this.group1.xyz)), this.scalar * right_anti_dual.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_anti_dual.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e15 * right_anti_dual.e4, this.e25 * right_anti_dual.e4, this.e35 * right_anti_dual.e4, -(this.e23 * right_anti_dual.e1) - (this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + (float4(this.scalar) * right_anti_dual.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(this.scalar) * right_anti_dual.group7.xyz) + (float3(right_anti_dual.e4) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.scalar) * right_anti_dual.group8.xyz) + (float3(right_anti_dual.e5) * this.group0.xyz) + (this.group1.zxy * right_anti_dual.group1.yzx) - (this.group1.yzx * right_anti_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * right_anti_dual.e4235, this.scalar * right_anti_dual.e4315, this.scalar * right_anti_dual.e4125, -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + (this.group0 * float4(right_anti_dual.group3.www, right_anti_dual.e3215)) + (this.group1.zxyw * float4(right_anti_dual.group4.xyz.yzx, right_anti_dual.scalar)) - (this.group1.yzxx * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e23)), 
            /* e1234, 0, 0, 0 */
            float4((this.scalar * right_anti_dual.e1234) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: WeightExpansion<Plane> {
    associatedtype Output = AntiFlector;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group0
        );
    }
}
extension AntiMotor: WeightExpansion<RoundPoint> {
    associatedtype Output = Sphere;
    func weight_expansion(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: WeightExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: WeightExpansion<Sphere> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension AntiMotor: WeightExpansion<VersorEven> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
extension AntiMotor: WeightExpansion<VersorOdd> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<AntiPlane> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension AntiPlane: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = Sphere;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group0.www), this.e1 * right_anti_dual.e235), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(this.e3 * right_anti_dual.e25 * -1.0, this.e1 * right_anti_dual.e35 * -1.0, this.e2 * right_anti_dual.e15 * -1.0, (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)) + (this.group0.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235)) + (this.group0.wwwy * float4(right_anti_dual.group1.xyz, right_anti_dual.e4315))
        );
    }
}
extension AntiPlane: WeightExpansion<AntiDualNum> {
    associatedtype Output = FlatPoint;
    func weight_expansion(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = Line;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_anti_dual.e45) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx), 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<AntiFlector> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e45) * this.group0.xyz * float3(-1.0)), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<AntiLine> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
extension AntiPlane: WeightExpansion<AntiMotor> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e2 * right_anti_dual.e315) - (this.e3 * right_anti_dual.e125)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
extension AntiPlane: WeightExpansion<AntiPlane> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<AntiScalar> {
    associatedtype Output = AntiPlane;
    func weight_expansion(other: AntiScalar) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension AntiPlane: WeightExpansion<Circle> {
    associatedtype Output = Circle;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - (this.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.zxy * this.group0.yzx) - (right_anti_dual.group2.xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<CircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e41 * this.e5, right_anti_dual.e42 * this.e5, right_anti_dual.e43 * this.e5, -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e5) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group2.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual.group2.yzx * this.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group0
        );
    }
}
extension AntiPlane: WeightExpansion<Dipole> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e235)) - float4((this.group0.yzx * right_anti_dual.group1.zxy), this.e2 * right_anti_dual.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4(float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group3.zxy * this.group0.yzx) - (right_anti_dual.group3.yzx * this.group0.zxy)), right_anti_dual.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e1 * this.e5 * -1.0, right_anti_dual.e2 * this.e5 * -1.0, right_anti_dual.e3 * this.e5 * -1.0, (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e425 * this.e3, right_anti_dual.e435 * this.e1, right_anti_dual.e415 * this.e2, -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235))
        );
    }
}
extension AntiPlane: WeightExpansion<DualNum> {
    associatedtype Output = AntiPlane;
    func weight_expansion(other: DualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension AntiPlane: WeightExpansion<FlatPoint> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(-(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<Flector> {
    associatedtype Output = AntiMotor;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e5 * this.e1, right_anti_dual.e5 * this.e2, right_anti_dual.e5 * this.e3, -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)) - (this.group0.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e235))
        );
    }
}
extension AntiPlane: WeightExpansion<Line> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (this.group0.zxyx * float4(right_anti_dual.group1.xyz.yzx, right_anti_dual.e23))
        );
    }
}
extension AntiPlane: WeightExpansion<Motor> {
    associatedtype Output = AntiFlector;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e23)), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * this.group0
        );
    }
}
extension AntiPlane: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_anti_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group1.xyz)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group0.yzx * right_anti_dual.group1.zxy) - (this.group0.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * right_anti_dual.e41, this.e5 * right_anti_dual.e42, this.e5 * right_anti_dual.e43, -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) - float4((right_anti_dual.group3.www * this.group0.xyz), this.e1 * right_anti_dual.e23), 
            /* e423, e431, e412, 0 */
            float4((right_anti_dual.group4.xyz.yzx * this.group0.zxy) - (right_anti_dual.group4.xyz.zxy * this.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group5.xyz) + (this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.wwwx * float4(right_anti_dual.group7.xyz, right_anti_dual.e235)) - float4((this.group0.yzx * right_anti_dual.group6.zxy), this.e2 * right_anti_dual.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<Plane> {
    associatedtype Output = AntiLine;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4((right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy), 0.0), 
            /* e15, e25, e35, 0 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz), 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<Sphere> {
    associatedtype Output = Dipole;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35, 0 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz), 0.0)
        );
    }
}
extension AntiPlane: WeightExpansion<VersorEven> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
extension AntiPlane: WeightExpansion<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4(float3(right_anti_dual.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx)), this.e5 * right_anti_dual.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * right_anti_dual.e1 * -1.0, this.e5 * right_anti_dual.e2 * -1.0, this.e5 * right_anti_dual.e3 * -1.0, (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412)) + (this.group0.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * right_anti_dual.e425, this.e1 * right_anti_dual.e435, this.e2 * right_anti_dual.e415, -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - (this.group0.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e315))
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<AntiScalar> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension AntiScalar: WeightExpansion<AntiScalar> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightExpansion<CircleRotor> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(float4(other.group2.xyz, other.e12345 * -1.0)[3] * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightExpansion<DualNum> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightExpansion<Motor> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3] * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightExpansion<MultiVector> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightExpansion<VersorEven> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3] * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<Circle> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension Circle: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightExpansion<AntiDualNum> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiDualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension Circle: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightExpansion<AntiFlector> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightExpansion<AntiMotor> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiMotor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension Circle: WeightExpansion<AntiScalar> {
    associatedtype Output = Circle;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: WeightExpansion<Circle> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightExpansion<CircleRotor> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
extension Circle: WeightExpansion<DipoleInversion> {
    associatedtype Output = Sphere;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group3.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_anti_dual.group3.yzx * this.group1.zxy), right_anti_dual.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e4 * this.e321) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightExpansion<DualNum> {
    associatedtype Output = Circle;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: WeightExpansion<Flector> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group1.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_anti_dual.group1.yzx * this.group1.zxy), right_anti_dual.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightExpansion<Line> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightExpansion<Motor> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
extension Circle: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 0.0, 0.0), 
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
            float4(right_anti_dual.scalar) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.scalar) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e321 * right_anti_dual.e5) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(right_anti_dual.e5) * this.group0.xyz), this.e235 * right_anti_dual.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightExpansion<Plane> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_anti_dual.group0.yzx * this.group1.zxy), right_anti_dual.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightExpansion<Sphere> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e321 * right_anti_dual.e5) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(right_anti_dual.e5) * this.group0.xyz), this.e235 * right_anti_dual.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightExpansion<VersorEven> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43))
        );
    }
}
extension Circle: WeightExpansion<VersorOdd> {
    associatedtype Output = Sphere;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e321 * right_anti_dual.e5) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group2.www), this.e235 * right_anti_dual.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<CircleRotor> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension CircleRotor: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightExpansion<AntiDualNum> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiDualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension CircleRotor: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightExpansion<AntiFlector> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightExpansion<AntiMotor> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiMotor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension CircleRotor: WeightExpansion<AntiScalar> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.scalar) * this.group2
        );
    }
}
extension CircleRotor: WeightExpansion<Circle> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightExpansion<CircleRotor> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.group2.www * this.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
extension CircleRotor: WeightExpansion<DipoleInversion> {
    associatedtype Output = Sphere;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group3.www), right_anti_dual.e1 * this.e235), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e4 * this.e321) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightExpansion<DualNum> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.scalar) * this.group2
        );
    }
}
extension CircleRotor: WeightExpansion<Flector> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group1.www), right_anti_dual.e1 * this.e235), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightExpansion<Line> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightExpansion<Motor> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.group0.www * this.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
extension CircleRotor: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (this.e12345 * right_anti_dual.scalar) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 0.0, 0.0), 
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
            float4(right_anti_dual.scalar) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.scalar) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightExpansion<Plane> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_anti_dual.group0.www), right_anti_dual.e1 * this.e235), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightExpansion<Sphere> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightExpansion<VersorEven> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), (this.e12345 * right_anti_dual.scalar) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43))
        );
    }
}
extension CircleRotor: WeightExpansion<VersorOdd> {
    associatedtype Output = Sphere;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group3.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<Dipole> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension Dipole: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = Sphere;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), this.e15 * right_anti_dual.e23) - float4((this.group2.xyz.yzx * right_anti_dual.group0.xyz.zxy), this.e25 * right_anti_dual.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<AntiDualNum> {
    associatedtype Output = Line;
    func weight_expansion(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_anti_dual.e5) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e5) * this.group1.xyz, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension Dipole: WeightExpansion<AntiFlector> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension Dipole: WeightExpansion<AntiLine> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<AntiScalar> {
    associatedtype Output = Dipole;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(right_anti_dual.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<Circle> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.xyz.yzx), right_anti_dual.e15 * this.e23) - float4((right_anti_dual.group2.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e25 * this.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<CircleRotor> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.xyz.yzx), right_anti_dual.e23 * this.e15) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), right_anti_dual.e31 * this.e25)
        );
    }
}
extension Dipole: WeightExpansion<Dipole> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<DipoleInversion> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4((float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((this.group2.xyz.yzx * right_anti_dual.group3.zxy), right_anti_dual.e423 * this.e15)
        );
    }
}
extension Dipole: WeightExpansion<DualNum> {
    associatedtype Output = Dipole;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(right_anti_dual.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<FlatPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<Flector> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e2 * this.e35) + (right_anti_dual.e5 * this.e23), (right_anti_dual.e3 * this.e15) + (right_anti_dual.e5 * this.e31), (right_anti_dual.e1 * this.e25) + (right_anti_dual.e5 * this.e12), -(right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - float4((this.group2.xyz.yzx * right_anti_dual.group1.zxy), right_anti_dual.e235 * this.e41)
        );
    }
}
extension Dipole: WeightExpansion<Line> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e23 * this.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<Motor> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx), right_anti_dual.e23 * this.e15)
        );
    }
}
extension Dipole: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.scalar) * this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_anti_dual.group1.yzx) - (this.group2.xyz.yzx * right_anti_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group3.yzx), this.e15 * right_anti_dual.e23) - float4((this.group2.xyz.yzx * right_anti_dual.group4.xyz.zxy), this.e25 * right_anti_dual.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightExpansion<Plane> {
    associatedtype Output = Circle;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_anti_dual.group0.yzx) - (this.group2.xyz.yzx * right_anti_dual.group0.zxy), 0.0)
        );
    }
}
extension Dipole: WeightExpansion<Sphere> {
    associatedtype Output = Circle;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.xyz.zxy * right_anti_dual.group0.yzx) - (this.group2.xyz.yzx * right_anti_dual.group0.zxy), 0.0)
        );
    }
}
extension Dipole: WeightExpansion<VersorEven> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), this.e15 * right_anti_dual.e23) - float4((this.group2.xyz.yzx * right_anti_dual.group0.zxy), this.e25 * right_anti_dual.e31)
        );
    }
}
extension Dipole: WeightExpansion<VersorOdd> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4((float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e2), (this.e31 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e3), (this.e12 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e1), -(this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) - float4((this.group2.xyz.yzx * right_anti_dual.group3.zxy), this.e41 * right_anti_dual.e235)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<DipoleInversion> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension DipoleInversion: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = Sphere;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), right_anti_dual.e31 * this.e25), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightExpansion<AntiDualNum> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension DipoleInversion: WeightExpansion<AntiFlector> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45), -(this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35)) - (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension DipoleInversion: WeightExpansion<AntiLine> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz), (this.e1234 * right_anti_dual.e5) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightExpansion<AntiScalar> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
extension DipoleInversion: WeightExpansion<Circle> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), right_anti_dual.e15 * this.e23) - float4((right_anti_dual.group2.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e25 * this.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightExpansion<CircleRotor> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual.group2.www * this.group2.xyz), (right_anti_dual.scalar * this.e1234) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.scalar * this.e3215) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), right_anti_dual.e31 * this.e25)
        );
    }
}
extension DipoleInversion: WeightExpansion<Dipole> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightExpansion<DipoleInversion> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4((float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (right_anti_dual.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) + float4((right_anti_dual.group3.yzx * this.group2.zxy), right_anti_dual.e4 * this.e3215) - (this.group2.yzxx * float4(right_anti_dual.group3.zxy, right_anti_dual.e423))
        );
    }
}
extension DipoleInversion: WeightExpansion<DualNum> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group3
        );
    }
}
extension DipoleInversion: WeightExpansion<FlatPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightExpansion<Flector> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (right_anti_dual.group1.yzxx * float4(this.group2.zxy, this.e4235)) + (right_anti_dual.group1.wwwy * float4(this.group1.xyz, this.e4315)) + float4(float3(0.0), (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - float4((right_anti_dual.group1.zxy * this.group2.yzx), right_anti_dual.e235 * this.e41)
        );
    }
}
extension DipoleInversion: WeightExpansion<Line> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group0.xyz.zxy), right_anti_dual.e23 * this.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightExpansion<Motor> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual.group0.www * this.group2.xyz), (right_anti_dual.scalar * this.e1234) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group1.zxy), right_anti_dual.scalar * this.e3215) - float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx), right_anti_dual.e23 * this.e15)
        );
    }
}
extension DipoleInversion: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (this.e1234 * right_anti_dual.e5) + (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.scalar) * this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group1.zxy) - (this.group0.xyz.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group1.xyz) + (this.group2.zxy * right_anti_dual.group1.yzx) - (this.group2.yzx * right_anti_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * right_anti_dual.e35) + (this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42), (this.e43 * right_anti_dual.e15) + (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43), (this.e41 * right_anti_dual.e25) + (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41), -(this.e23 * right_anti_dual.e15) - (this.e31 * right_anti_dual.e25) - (this.e12 * right_anti_dual.e35) - (this.e35 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group3) - (this.group2.yzxy * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e31)) - float4((this.group0.xyz.zxy * right_anti_dual.group3.yzx), this.e15 * right_anti_dual.e23), 
            /* e1234, 0, 0, 0 */
            float4((this.e1234 * right_anti_dual.scalar) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightExpansion<Plane> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - (right_anti_dual.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.e3 * this.e25 * -1.0, right_anti_dual.e1 * this.e35 * -1.0, right_anti_dual.e2 * this.e15 * -1.0, (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)) + (right_anti_dual.group0.yzxx * float4(this.group2.zxy, this.e4235)) + (right_anti_dual.group0.wwwy * float4(this.group1.xyz, this.e4315))
        );
    }
}
extension DipoleInversion: WeightExpansion<Sphere> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group0.zxy) - (this.group0.xyz.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(this.e25 * right_anti_dual.e3 * -1.0, this.e35 * right_anti_dual.e1 * -1.0, this.e15 * right_anti_dual.e2 * -1.0, (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)) + (float4(right_anti_dual.e5) * float4(this.group1.xyz, this.e1234)) + (right_anti_dual.group0.yzxx * float4(this.group2.zxy, this.e4235))
        );
    }
}
extension DipoleInversion: WeightExpansion<VersorEven> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz), (this.e1234 * right_anti_dual.scalar) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e23) + (this.e35 * right_anti_dual.e42) + (this.e4235 * right_anti_dual.scalar), (this.e31 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e31) + (this.e15 * right_anti_dual.e43) + (this.e4315 * right_anti_dual.scalar), (this.e12 * right_anti_dual.e45) + (this.e45 * right_anti_dual.e12) + (this.e25 * right_anti_dual.e41) + (this.e4125 * right_anti_dual.scalar), -(this.e12 * right_anti_dual.e35) - (this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.e3215 * right_anti_dual.scalar) - (right_anti_dual.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((this.group2.yzx * right_anti_dual.group0.zxy), this.e31 * right_anti_dual.e25)
        );
    }
}
extension DipoleInversion: WeightExpansion<VersorOdd> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4((float3(right_anti_dual.e4) * this.group1.xyz) + (this.group0.xyz.yzx * right_anti_dual.group3.zxy) - (this.group0.xyz.zxy * right_anti_dual.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * right_anti_dual.e5) + (this.e15 * right_anti_dual.e4), (this.e42 * right_anti_dual.e5) + (this.e25 * right_anti_dual.e4), (this.e43 * right_anti_dual.e5) + (this.e35 * right_anti_dual.e4), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(right_anti_dual.e5) * float4(this.group1.xyz, this.e1234)) + (right_anti_dual.group3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) - float4((this.group2.yzx * right_anti_dual.group3.zxy), this.e41 * right_anti_dual.e235)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<DualNum> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension DualNum: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
extension DualNum: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e1234), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DualNum: WeightExpansion<AntiScalar> {
    associatedtype Output = DualNum;
    func weight_expansion(other: AntiScalar) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.e12345 * -1.0) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: WeightExpansion<Circle> {
    associatedtype Output = Line;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e5) * right_anti_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e5) * right_anti_dual.group1.xyz, 0.0)
        );
    }
}
extension DualNum: WeightExpansion<CircleRotor> {
    associatedtype Output = Motor;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, this.e12345) * float4(right_anti_dual.group0.xyz, right_anti_dual.scalar), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.scalar)
        );
    }
}
extension DualNum: WeightExpansion<Dipole> {
    associatedtype Output = Plane;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
extension DualNum: WeightExpansion<DipoleInversion> {
    associatedtype Output = Flector;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4(this.e5) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
extension DualNum: WeightExpansion<DualNum> {
    associatedtype Output = DualNum;
    func weight_expansion(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.e12345 * -1.0) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: WeightExpansion<FlatPoint> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: FlatPoint) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e5 * other.e45, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension DualNum: WeightExpansion<Flector> {
    associatedtype Output = Flector;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual.e321 * this.e5 * -1.0)
        );
    }
}
extension DualNum: WeightExpansion<Line> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: Line) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DualNum: WeightExpansion<Motor> {
    associatedtype Output = Motor;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), right_anti_dual.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * right_anti_dual.group0
        );
    }
}
extension DualNum: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (this.e5 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_anti_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_anti_dual.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e5) * right_anti_dual.group5.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group7.xyz, right_anti_dual.e321) * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension DualNum: WeightExpansion<Plane> {
    associatedtype Output = FlatPoint;
    func weight_expansion(other: Plane) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215).xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension DualNum: WeightExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e5 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: WeightExpansion<Sphere> {
    associatedtype Output = FlatPoint;
    func weight_expansion(other: Sphere) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e1234) * float4(-1.0)
        );
    }
}
extension DualNum: WeightExpansion<VersorEven> {
    associatedtype Output = Motor;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4(this.group0.xy.xx, this.e5, (this.e5 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.scalar)) * float4(right_anti_dual.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.scalar)
        );
    }
}
extension DualNum: WeightExpansion<VersorOdd> {
    associatedtype Output = Flector;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4(this.e5) * right_anti_dual.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<FlatPoint> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension FlatPoint: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
extension FlatPoint: WeightExpansion<AntiScalar> {
    associatedtype Output = FlatPoint;
    func weight_expansion(other: AntiScalar) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension FlatPoint: WeightExpansion<Circle> {
    associatedtype Output = Plane;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
extension FlatPoint: WeightExpansion<CircleRotor> {
    associatedtype Output = Flector;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
extension FlatPoint: WeightExpansion<Dipole> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightExpansion<DipoleInversion> {
    associatedtype Output = Motor;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4(right_anti_dual.e4 * this.e15, right_anti_dual.e4 * this.e25, right_anti_dual.e4 * this.e35, -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group3.yzx * this.group0.zxy) - (right_anti_dual.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension FlatPoint: WeightExpansion<DualNum> {
    associatedtype Output = FlatPoint;
    func weight_expansion(other: DualNum) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension FlatPoint: WeightExpansion<FlatPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e45 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightExpansion<Flector> {
    associatedtype Output = Motor;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e45) * float4(right_anti_dual.group1.xyz, right_anti_dual.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension FlatPoint: WeightExpansion<Line> {
    associatedtype Output = Plane;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35))
        );
    }
}
extension FlatPoint: WeightExpansion<Motor> {
    associatedtype Output = Flector;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35))
        );
    }
}
extension FlatPoint: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, -(this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_anti_dual.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.zxy * right_anti_dual.group1.yzx) - (this.group0.yzx * right_anti_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_anti_dual.e42) + (this.e45 * right_anti_dual.e23), (this.e15 * right_anti_dual.e43) + (this.e45 * right_anti_dual.e31), (this.e25 * right_anti_dual.e41) + (this.e45 * right_anti_dual.e12), -(this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - (this.group0.yzxx * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension FlatPoint: WeightExpansion<Plane> {
    associatedtype Output = Line;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e45) * right_anti_dual.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension FlatPoint: WeightExpansion<Sphere> {
    associatedtype Output = Line;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4((float3(right_anti_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_anti_dual.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.zxy * right_anti_dual.group0.yzx) - (this.group0.yzx * right_anti_dual.group0.zxy), 0.0)
        );
    }
}
extension FlatPoint: WeightExpansion<VersorEven> {
    associatedtype Output = Flector;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_anti_dual.e42) + (this.e45 * right_anti_dual.e23), (this.e15 * right_anti_dual.e43) + (this.e45 * right_anti_dual.e31), (this.e25 * right_anti_dual.e41) + (this.e45 * right_anti_dual.e12), -(this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e23))
        );
    }
}
extension FlatPoint: WeightExpansion<VersorOdd> {
    associatedtype Output = Motor;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4(this.e15 * right_anti_dual.e4, this.e25 * right_anti_dual.e4, this.e35 * right_anti_dual.e4, -(this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_anti_dual.group3.yzx) - (this.group0.yzx * right_anti_dual.group3.zxy)), 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<Flector> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension Flector: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
extension Flector: WeightExpansion<AntiScalar> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
extension Flector: WeightExpansion<Circle> {
    associatedtype Output = Plane;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
extension Flector: WeightExpansion<CircleRotor> {
    associatedtype Output = Flector;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) + float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), right_anti_dual.scalar * this.e3215) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23))
        );
    }
}
extension Flector: WeightExpansion<Dipole> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: WeightExpansion<DipoleInversion> {
    associatedtype Output = Motor;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            (float4(right_anti_dual.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group3.yzx * this.group0.zxy) - (right_anti_dual.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Flector: WeightExpansion<DualNum> {
    associatedtype Output = Flector;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
extension Flector: WeightExpansion<FlatPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e45 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: WeightExpansion<Flector> {
    associatedtype Output = Motor;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * right_anti_dual.group1.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) - (right_anti_dual.e321 * this.e45)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Flector: WeightExpansion<Line> {
    associatedtype Output = Plane;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35))
        );
    }
}
extension Flector: WeightExpansion<Motor> {
    associatedtype Output = Flector;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar * this.e4235, right_anti_dual.scalar * this.e4315, right_anti_dual.scalar * this.e4125, -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35)) + (right_anti_dual.group0 * float4(this.group0.www, this.e3215))
        );
    }
}
extension Flector: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual.e4) * this.group0.xyz) - (float3(this.e45) * right_anti_dual.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.zxy * right_anti_dual.group1.yzx) - (this.group0.yzx * right_anti_dual.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_anti_dual.e42) + (this.e45 * right_anti_dual.e23), (this.e15 * right_anti_dual.e43) + (this.e45 * right_anti_dual.e31), (this.e25 * right_anti_dual.e41) + (this.e45 * right_anti_dual.e12), -(this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + (float4(right_anti_dual.scalar) * this.group1) - (this.group0.yzxx * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: WeightExpansion<Plane> {
    associatedtype Output = Motor;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * right_anti_dual.group0.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Flector: WeightExpansion<Sphere> {
    associatedtype Output = Motor;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e45 * right_anti_dual.e1 * -1.0, this.e45 * right_anti_dual.e2 * -1.0, this.e45 * right_anti_dual.e3 * -1.0, (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4)) + (right_anti_dual.group0.wwwx * float4(this.group0.xyz, this.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_anti_dual.group0.yzx) - (this.group0.yzx * right_anti_dual.group0.zxy)), 0.0)
        );
    }
}
extension Flector: WeightExpansion<VersorEven> {
    associatedtype Output = Flector;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4(right_anti_dual.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e45 * right_anti_dual.e23) + (this.e4235 * right_anti_dual.scalar), (this.e45 * right_anti_dual.e31) + (this.e4315 * right_anti_dual.scalar), (this.e45 * right_anti_dual.e12) + (this.e4125 * right_anti_dual.scalar), -(this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12)) + (right_anti_dual.group0.yzxw * float4(this.group0.zxy, this.e3215)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e23))
        );
    }
}
extension Flector: WeightExpansion<VersorOdd> {
    associatedtype Output = Motor;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            (right_anti_dual.group3.wwwx * float4(this.group0.xyz, this.e4235)) + float4(float3(0.0), (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * right_anti_dual.group3.yzx) - (this.group0.yzx * right_anti_dual.group3.zxy)), 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<Line> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension Line: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4(-(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: WeightExpansion<AntiScalar> {
    associatedtype Output = Line;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: WeightExpansion<Circle> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: WeightExpansion<CircleRotor> {
    associatedtype Output = Motor;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: WeightExpansion<DipoleInversion> {
    associatedtype Output = Plane;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: WeightExpansion<DualNum> {
    associatedtype Output = Line;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_anti_dual.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: WeightExpansion<Flector> {
    associatedtype Output = Plane;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: WeightExpansion<Line> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: WeightExpansion<Motor> {
    associatedtype Output = Motor;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, -(this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 0.0, 0.0), 
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
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.scalar) * this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group1.yzxx * float4(this.group0.xyz.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Line: WeightExpansion<Plane> {
    associatedtype Output = Plane;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: WeightExpansion<Sphere> {
    associatedtype Output = Plane;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: WeightExpansion<VersorEven> {
    associatedtype Output = Motor;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4((float3(right_anti_dual.scalar) * this.group0.xyz), -(this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: WeightExpansion<VersorOdd> {
    associatedtype Output = Plane;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<Motor> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension Motor: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
extension Motor: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4((float3(this.e5) * right_anti_dual.group0.xyz), (right_anti_dual.e1234 * this.e5) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: WeightExpansion<AntiScalar> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
extension Motor: WeightExpansion<Circle> {
    associatedtype Output = Motor;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * right_anti_dual.group0.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: WeightExpansion<CircleRotor> {
    associatedtype Output = Motor;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.scalar * this.e415, right_anti_dual.scalar * this.e425, right_anti_dual.scalar * this.e435, -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)) + float4((right_anti_dual.group0.xyz * this.group1.www), right_anti_dual.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group1.xyz) + (float3(this.e5) * right_anti_dual.group1.xyz)), right_anti_dual.scalar * this.e5)
        );
    }
}
extension Motor: WeightExpansion<Dipole> {
    associatedtype Output = Plane;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321) * float4(-1.0)
        );
    }
}
extension Motor: WeightExpansion<DipoleInversion> {
    associatedtype Output = Flector;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4(this.e5) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e4 * this.e235) - (right_anti_dual.e3 * this.e425), -(right_anti_dual.e4 * this.e315) - (right_anti_dual.e1 * this.e435), -(right_anti_dual.e4 * this.e125) - (right_anti_dual.e2 * this.e415), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group3.yzxx * float4(this.group0.zxy, this.e235)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321))
        );
    }
}
extension Motor: WeightExpansion<DualNum> {
    associatedtype Output = Motor;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.scalar) * this.group1
        );
    }
}
extension Motor: WeightExpansion<FlatPoint> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: FlatPoint) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.e45 * this.e5, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension Motor: WeightExpansion<Flector> {
    associatedtype Output = Flector;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) - float4((right_anti_dual.group1.zxy * this.group0.yzx), right_anti_dual.e321 * this.e5)
        );
    }
}
extension Motor: WeightExpansion<Line> {
    associatedtype Output = Motor;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: WeightExpansion<Motor> {
    associatedtype Output = Motor;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual.group0.www * this.group0.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.scalar) * this.group1.xyz) + (float3(this.e5) * right_anti_dual.group0.xyz)), right_anti_dual.scalar * this.e5)
        );
    }
}
extension Motor: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (this.e12345 * right_anti_dual.scalar) + (this.e5 * right_anti_dual.e1234) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_anti_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_anti_dual.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * right_anti_dual.group4.xyz) + (float3(right_anti_dual.scalar) * this.group0.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.scalar) * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group1.yzxx * float4(this.group0.zxy, this.e235)) - float4((right_anti_dual.group7.xyz * this.group1.www), this.e5 * right_anti_dual.e321), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Motor: WeightExpansion<Plane> {
    associatedtype Output = Flector;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e2 * this.e315) + (right_anti_dual.e3 * this.e125)) + (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
extension Motor: WeightExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e5 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: WeightExpansion<Sphere> {
    associatedtype Output = Flector;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * right_anti_dual.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
extension Motor: WeightExpansion<VersorEven> {
    associatedtype Output = Motor;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e1234)) + (float4(right_anti_dual.scalar) * this.group0) + float4(float3(0.0), -(this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group1.xyz)), this.e5 * right_anti_dual.scalar)
        );
    }
}
extension Motor: WeightExpansion<VersorOdd> {
    associatedtype Output = Flector;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4(this.e5) * right_anti_dual.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e235 * right_anti_dual.e4) - (this.e5 * right_anti_dual.e423), -(this.e315 * right_anti_dual.e4) - (this.e5 * right_anti_dual.e431), -(this.e125 * right_anti_dual.e4) - (this.e5 * right_anti_dual.e412), (this.e315 * right_anti_dual.e2) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group3.yzxx * float4(this.group0.zxy, this.e235)) - float4((this.group0.yzx * right_anti_dual.group3.zxy), this.e5 * right_anti_dual.e321)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<MultiVector> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension MultiVector: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_anti_dual.e12345 * this.scalar) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 0.0, 0.0), 
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
            float4(this.scalar) * right_anti_dual.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_anti_dual.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4(float2(0.0, (right_anti_dual.e1234 * this.e5) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_anti_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group1.yzx) - (right_anti_dual.group2.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) + (float4(this.scalar) * right_anti_dual.group3) - (right_anti_dual.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((right_anti_dual.group0.xyz.zxy * this.group3.yzx), right_anti_dual.e15 * this.e23), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e1234 * this.scalar) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<AntiDualNum> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_anti_dual.e5 * this.e1234) + (right_anti_dual.e12345 * this.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e5) * this.group5.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e5) * float4(this.group7.xyz, this.e321), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((right_anti_dual.group0.zxy * this.group1.yzx) - (right_anti_dual.group0.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e35 * this.e42) + (right_anti_dual.e45 * this.e23), (right_anti_dual.e15 * this.e43) + (right_anti_dual.e45 * this.e31), (right_anti_dual.e25 * this.e41) + (right_anti_dual.e45 * this.e12), -(right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - (right_anti_dual.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightExpansion<AntiFlector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((right_anti_dual.group0.zxy * this.group1.yzx) - (right_anti_dual.group0.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e35 * this.e42) + (right_anti_dual.e45 * this.e23), (right_anti_dual.e15 * this.e43) + (right_anti_dual.e45 * this.e31), (right_anti_dual.e25 * this.e41) + (right_anti_dual.e45 * this.e12), -(right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (float4(this.scalar) * right_anti_dual.group1) - (right_anti_dual.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightExpansion<AntiLine> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 0.0, 0.0), 
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
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_anti_dual.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (this.group1.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightExpansion<AntiMotor> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual.e5) * this.group4.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) + float4((this.group7.xyz * right_anti_dual.group1.www), right_anti_dual.e5 * this.e321) - (this.group1.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightExpansion<AntiPlane> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215)), 0.0, 0.0), 
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
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightExpansion<AntiScalar> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_anti_dual.scalar) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_anti_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.scalar) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.scalar) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * right_anti_dual.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<Circle> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_anti_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.zxy * this.group1.yzx) - (right_anti_dual.group2.xyz.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group3.yzx), right_anti_dual.e15 * this.e23) - float4((right_anti_dual.group2.xyz.yzx * this.group4.xyz.zxy), right_anti_dual.e25 * this.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<CircleRotor> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_anti_dual.scalar * this.scalar, (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.scalar) * this.group3) + (float4(this.scalar) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.scalar) * this.group4.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * this.e4) + (right_anti_dual.scalar * this.e415), (right_anti_dual.e25 * this.e4) + (right_anti_dual.scalar * this.e425), (right_anti_dual.e35 * this.e4) + (right_anti_dual.scalar * this.e435), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + float4((float3(this.e5) * right_anti_dual.group0.xyz), right_anti_dual.scalar * this.e321) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.scalar) * this.group7.xyz) + (float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group1.zxy) - (right_anti_dual.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.scalar) * this.group8.xyz) + (float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group1.yzx) - (right_anti_dual.group2.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.scalar * this.e4235), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.scalar * this.e4315), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.scalar * this.e4125), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group3.zxy), right_anti_dual.scalar * this.e3215) - (right_anti_dual.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((right_anti_dual.group0.xyz.zxy * this.group3.yzx), right_anti_dual.e15 * this.e23), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.scalar * this.e1234) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<Dipole> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 0.0, 0.0), 
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
            float4(this.scalar) * right_anti_dual.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_anti_dual.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e321 * this.e5) - (right_anti_dual.e125 * this.e3)) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((float3(this.e5) * right_anti_dual.group0.xyz), right_anti_dual.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<DipoleInversion> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4(float2(0.0, (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4), 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * this.group1) - (float4(this.e5) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_anti_dual.group3.zxy * this.group1.yzx) - (right_anti_dual.group3.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + (float4(this.scalar) * right_anti_dual.group1) - float4((this.group3.www * right_anti_dual.group3.xyz), right_anti_dual.e1 * this.e23), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.e4) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz) + (this.group4.xyz.yzx * right_anti_dual.group3.zxy) - (this.group4.xyz.zxy * right_anti_dual.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group2.xyz) + (right_anti_dual.group3.yzx * this.group3.zxy) - (right_anti_dual.group3.zxy * this.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group3.yzxw * this.group6.zxyw) + (right_anti_dual.group3.wwwx * float4(this.group7.xyz, this.e235)) + float4((right_anti_dual.group1.yzx * this.group1.zxy), right_anti_dual.e2 * this.e315) + float4((this.group1.www * right_anti_dual.group2.xyz), right_anti_dual.e3 * this.e125) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((this.group8.xyz * right_anti_dual.group2.www), right_anti_dual.e235 * this.e1) - float4((right_anti_dual.group3.zxy * this.group6.yzx), right_anti_dual.e125 * this.e3), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e4 * this.e321) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<DualNum> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_anti_dual.scalar * this.scalar, (right_anti_dual.e3215 * this.e4) + (right_anti_dual.scalar * this.e12345)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.scalar) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.scalar) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.scalar) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.scalar) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.scalar) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.scalar, (right_anti_dual.e3215 * this.scalar) + (right_anti_dual.scalar * this.e3215)) * float4(this.group9.xyz, 1.0), 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<FlatPoint> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)), 0.0, 0.0), 
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
            float4(float3(0.0), right_anti_dual.e321 * this.scalar), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)), 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<Flector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_anti_dual.group1.xyz)), right_anti_dual.e5 * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_anti_dual.group1.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_anti_dual.group1.zxy * this.group1.yzx) - (right_anti_dual.group1.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + float4((this.group4.xyz * right_anti_dual.group1.www), right_anti_dual.e321 * this.scalar) - float4((this.group3.www * right_anti_dual.group1.xyz), right_anti_dual.e1 * this.e23), 
            /* e423, e431, e412, 0 */
            float4((this.group4.xyz.yzx * right_anti_dual.group1.zxy) - (this.group4.xyz.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.yzx * this.group3.zxy) - (right_anti_dual.group1.zxy * this.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group1.wwwx * float4(this.group7.xyz, this.e235)) + float4(float3(0.0), (right_anti_dual.e5 * this.e321) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)) + float4((right_anti_dual.group1.yzx * this.group6.zxy), right_anti_dual.e3 * this.e125) + float4((this.group1.www * right_anti_dual.group0.xyz), right_anti_dual.e2 * this.e315) - float4((right_anti_dual.group1.zxy * this.group6.yzx), right_anti_dual.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e321 * this.e4) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<Line> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * right_anti_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.xyz.zxy * this.group1.yzx) - (right_anti_dual.group1.xyz.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group4.xyz.zxy), right_anti_dual.e23 * this.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<Motor> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_anti_dual.scalar * this.scalar, (right_anti_dual.scalar * this.e12345) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.scalar) * this.group3.xyz) + (float3(this.scalar) * right_anti_dual.group1.xyz)), right_anti_dual.scalar * this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.scalar) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e15 * this.e4, right_anti_dual.e25 * this.e4, right_anti_dual.e35 * this.e4, -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + (float4(right_anti_dual.scalar) * this.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.scalar) * this.group7.xyz) + (float3(this.e4) * right_anti_dual.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.scalar) * this.group8.xyz) + (float3(this.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.zxy * this.group1.yzx) - (right_anti_dual.group1.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar * this.e4235, right_anti_dual.scalar * this.e4315, right_anti_dual.scalar * this.e4125, -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (right_anti_dual.group0 * float4(this.group3.www, this.e3215)) + (right_anti_dual.group1.zxyw * float4(this.group4.xyz.yzx, this.scalar)) - (right_anti_dual.group1.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.scalar * this.e1234) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(right_anti_dual.scalar * this.scalar, (right_anti_dual.scalar * this.e12345) + (right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual.scalar) * this.group1) + (float4(this.scalar) * right_anti_dual.group1), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.scalar * this.e5) + (right_anti_dual.e5 * this.scalar), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.scalar) * this.group3) + (float4(right_anti_dual.e5) * this.group1) + (float4(this.scalar) * right_anti_dual.group3) - (float4(this.e5) * right_anti_dual.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.scalar) * this.group4.xyz) + (float3(this.scalar) * right_anti_dual.group4.xyz) + (float3(this.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.scalar) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group5.xyz) + (right_anti_dual.group1.zxy * this.group1.yzx) - (right_anti_dual.group1.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e4 * this.e15) + (right_anti_dual.e5 * this.e41) + (right_anti_dual.e15 * this.e4) + (right_anti_dual.e41 * this.e5), (right_anti_dual.e4 * this.e25) + (right_anti_dual.e5 * this.e42) + (right_anti_dual.e25 * this.e4) + (right_anti_dual.e42 * this.e5), (right_anti_dual.e4 * this.e35) + (right_anti_dual.e5 * this.e43) + (right_anti_dual.e35 * this.e4) + (right_anti_dual.e43 * this.e5), -(right_anti_dual.e1 * this.e23) - (right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12) - (right_anti_dual.e12 * this.e3)) + (float4(right_anti_dual.scalar) * this.group6) + (float4(this.scalar) * right_anti_dual.group6) - (this.group1.xyzy * float4(right_anti_dual.group3.www, right_anti_dual.e31)) - float4((this.group3.www * right_anti_dual.group1.xyz), right_anti_dual.e23 * this.e1), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.scalar) * this.group7.xyz) + (float3(right_anti_dual.e4) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group7.xyz) + (float3(this.e4) * right_anti_dual.group5.xyz) + (right_anti_dual.group4.xyz.yzx * this.group1.zxy) + (this.group4.xyz.yzx * right_anti_dual.group1.zxy) - (right_anti_dual.group4.xyz.zxy * this.group1.yzx) - (this.group4.xyz.zxy * right_anti_dual.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.scalar) * this.group8.xyz) + (float3(right_anti_dual.e5) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual.group8.xyz) + (float3(this.e5) * right_anti_dual.group5.xyz) + (right_anti_dual.group1.yzx * this.group3.zxy) + (right_anti_dual.group3.zxy * this.group1.yzx) - (right_anti_dual.group1.zxy * this.group3.yzx) - (right_anti_dual.group3.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e2 * this.e435) + (right_anti_dual.e5 * this.e423) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e425 * this.e3), (right_anti_dual.e3 * this.e415) + (right_anti_dual.e5 * this.e431) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e435 * this.e1), (right_anti_dual.e1 * this.e425) + (right_anti_dual.e5 * this.e412) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e415 * this.e2), -(right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12) - (right_anti_dual.e321 * this.e5)) + (float4(right_anti_dual.scalar) * this.group9) + (float4(this.scalar) * right_anti_dual.group9) + float4((right_anti_dual.group5.xyz * this.group3.www), right_anti_dual.e2 * this.e315) + float4((right_anti_dual.group8.xyz * this.group1.www), right_anti_dual.e3 * this.e125) + float4((right_anti_dual.group4.xyz.yzx * this.group3.zxy), right_anti_dual.e1 * this.e235) + float4((this.group4.xyz.yzx * right_anti_dual.group3.zxy), right_anti_dual.e5 * this.e321) - (this.group1.yzxz * float4(right_anti_dual.group6.zxy, right_anti_dual.e125)) - (this.group3.yzxx * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e23)) - float4((float3(this.e5) * right_anti_dual.group7.xyz), right_anti_dual.e31 * this.e25) - float4((this.group8.xyz * right_anti_dual.group1.www), right_anti_dual.e235 * this.e1) - float4((this.group4.xyz.zxy * right_anti_dual.group3.yzx), right_anti_dual.e12 * this.e35) - float4((right_anti_dual.group1.zxy * this.group6.yzx), right_anti_dual.e315 * this.e2), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.scalar * this.e1234) + (right_anti_dual.e321 * this.e4) + (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e1234 * this.scalar) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<Plane> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz)), right_anti_dual.e5 * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_anti_dual.group0.zxy * this.group1.yzx) - (right_anti_dual.group0.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e5 * this.e41, right_anti_dual.e5 * this.e42, right_anti_dual.e5 * this.e43, -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) - float4((this.group3.www * right_anti_dual.group0.xyz), right_anti_dual.e1 * this.e23), 
            /* e423, e431, e412, 0 */
            float4((this.group4.xyz.yzx * right_anti_dual.group0.zxy) - (this.group4.xyz.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group5.xyz) + (right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.wwwx * float4(this.group7.xyz, this.e235)) + float4((right_anti_dual.group0.yzx * this.group6.zxy), right_anti_dual.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<RoundPoint> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234)), 0.0, 0.0), 
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
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<Sphere> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * right_anti_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * this.group1) - (float4(this.e5) * right_anti_dual.group0), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * right_anti_dual.group0.zxy) - (this.group1.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_anti_dual.e4) + (this.e41 * right_anti_dual.e5), (this.e25 * right_anti_dual.e4) + (this.e42 * right_anti_dual.e5), (this.e35 * right_anti_dual.e4) + (this.e43 * right_anti_dual.e5), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.e4) * this.group5.xyz) + (this.group4.xyz.yzx * right_anti_dual.group0.zxy) - (this.group4.xyz.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group5.xyz) + (this.group3.zxy * right_anti_dual.group0.yzx) - (this.group3.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * right_anti_dual.e3) - (this.e235 * right_anti_dual.e4), -(this.e435 * right_anti_dual.e1) - (this.e315 * right_anti_dual.e4), -(this.e415 * right_anti_dual.e2) - (this.e125 * right_anti_dual.e4), (this.e321 * right_anti_dual.e5) + (this.e125 * right_anti_dual.e3)) + (right_anti_dual.group0.yzxy * float4(this.group6.zxy, this.e315)) + float4((float3(right_anti_dual.e5) * this.group7.xyz), this.e235 * right_anti_dual.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e321 * right_anti_dual.e4) - (this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<VersorEven> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4(float2(this.scalar * right_anti_dual.scalar, (this.e12345 * right_anti_dual.scalar) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_anti_dual.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)) + (float4(right_anti_dual.scalar) * this.group3), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.scalar) * this.group4.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group5.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e5 * right_anti_dual.e41) + (this.e415 * right_anti_dual.scalar), (this.e5 * right_anti_dual.e42) + (this.e425 * right_anti_dual.scalar), (this.e5 * right_anti_dual.e43) + (this.e435 * right_anti_dual.scalar), -(this.e2 * right_anti_dual.e31) - (this.e3 * right_anti_dual.e12)) + float4((this.group1.www * right_anti_dual.group2.xyz), this.e321 * right_anti_dual.scalar) - (this.group1.xyzx * right_anti_dual.group1.wwwx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group7.xyz) + (this.group1.zxy * right_anti_dual.group0.yzx) - (this.group1.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.scalar) * this.group8.xyz) + (this.group1.yzx * right_anti_dual.group2.zxy) - (this.group1.zxy * right_anti_dual.group2.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * right_anti_dual.e42) + (this.e45 * right_anti_dual.e23) + (this.e23 * right_anti_dual.e45) + (this.e4235 * right_anti_dual.scalar), (this.e15 * right_anti_dual.e43) + (this.e45 * right_anti_dual.e31) + (this.e31 * right_anti_dual.e45) + (this.e4315 * right_anti_dual.scalar), (this.e25 * right_anti_dual.e41) + (this.e45 * right_anti_dual.e12) + (this.e12 * right_anti_dual.e45) + (this.e4125 * right_anti_dual.scalar), -(this.e15 * right_anti_dual.e23) - (this.e25 * right_anti_dual.e31) - (this.e35 * right_anti_dual.e12) - (this.e12 * right_anti_dual.e35)) + (float4(this.scalar) * right_anti_dual.group3) + float4((this.group4.xyz.yzx * right_anti_dual.group2.zxy), this.e3215 * right_anti_dual.scalar) - (right_anti_dual.group2.yzxx * float4(this.group4.xyz.zxy, this.e23)) - float4((this.group3.yzx * right_anti_dual.group0.zxy), this.e31 * right_anti_dual.e25), 
            /* e1234, 0, 0, 0 */
            float4((this.scalar * right_anti_dual.e1234) + (this.e1234 * right_anti_dual.scalar) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightExpansion<VersorOdd> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4(float2(0.0, (this.scalar * right_anti_dual.e12345) + (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group3, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * right_anti_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * this.group1) - (float4(this.e5) * right_anti_dual.group3), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * right_anti_dual.group3.zxy) - (this.group1.zxy * right_anti_dual.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * right_anti_dual.e4) + (this.e41 * right_anti_dual.e5), (this.e25 * right_anti_dual.e4) + (this.e42 * right_anti_dual.e5), (this.e35 * right_anti_dual.e4) + (this.e43 * right_anti_dual.e5), -(this.e31 * right_anti_dual.e2) - (this.e12 * right_anti_dual.e3)) + (float4(this.scalar) * right_anti_dual.group1) - (right_anti_dual.group3.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e4) * this.group5.xyz) + (this.group4.xyz.yzx * right_anti_dual.group3.zxy) - (this.group4.xyz.zxy * right_anti_dual.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e5) * this.group5.xyz) + (this.group3.zxy * right_anti_dual.group3.yzx) - (this.group3.yzx * right_anti_dual.group3.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (this.group6.zxyw * float4(right_anti_dual.group3.yzx, right_anti_dual.e5)) + float4((this.group7.xyz * right_anti_dual.group2.www), this.e235 * right_anti_dual.e1) + float4((this.group1.zxy * right_anti_dual.group1.yzx), this.e315 * right_anti_dual.e2) + float4((this.group1.www * right_anti_dual.group2.xyz), this.e125 * right_anti_dual.e3) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group1.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((this.group8.xyz * right_anti_dual.group3.www), this.e1 * right_anti_dual.e235) - float4((this.group6.yzx * right_anti_dual.group3.zxy), this.e3 * right_anti_dual.e125), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321) - (this.e321 * right_anti_dual.e4) - (this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<Plane> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension Plane: WeightExpansion<AntiScalar> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiScalar) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension Plane: WeightExpansion<CircleRotor> {
    associatedtype Output = Plane;
    func weight_expansion(other: CircleRotor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group2.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
extension Plane: WeightExpansion<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4((right_anti_dual.e4 * this.e3215) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: WeightExpansion<DualNum> {
    associatedtype Output = Plane;
    func weight_expansion(other: DualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
extension Plane: WeightExpansion<Flector> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: WeightExpansion<Motor> {
    associatedtype Output = Plane;
    func weight_expansion(other: Motor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
extension Plane: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215)), 0.0, 0.0), 
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
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane: WeightExpansion<Plane> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: WeightExpansion<Sphere> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: WeightExpansion<VersorEven> {
    associatedtype Output = Plane;
    func weight_expansion(other: VersorEven) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float4(other.group0.xyz, other.e12345 * -1.0)[3]) * this.group0
        );
    }
}
extension Plane: WeightExpansion<VersorOdd> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4((this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<RoundPoint> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension RoundPoint: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = Sphere;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4((float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.e25 * this.e3 * -1.0, right_anti_dual.e35 * this.e1 * -1.0, right_anti_dual.e15 * this.e2 * -1.0, (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)) + (float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.e1234)) + (this.group0.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235))
        );
    }
}
extension RoundPoint: WeightExpansion<AntiDualNum> {
    associatedtype Output = FlatPoint;
    func weight_expansion(other: AntiDualNum) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group0
        );
    }
}
extension RoundPoint: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = Line;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e45) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<AntiFlector> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e45 * this.e1 * -1.0, right_anti_dual.e45 * this.e2 * -1.0, right_anti_dual.e45 * this.e3 * -1.0, (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4)) + (this.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<AntiLine> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (this.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
extension RoundPoint: WeightExpansion<AntiMotor> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (this.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
extension RoundPoint: WeightExpansion<AntiPlane> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<AntiScalar> {
    associatedtype Output = RoundPoint;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_anti_dual.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<Circle> {
    associatedtype Output = Circle;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.zxy * this.group0.yzx) - (right_anti_dual.group2.xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<CircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group0.zxy) - (right_anti_dual.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(this.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * this.group0.yzx) - (right_anti_dual.group2.yzx * this.group0.zxy)), right_anti_dual.scalar * this.e4), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.scalar) * float4(this.group0.xyz, this.e5)
        );
    }
}
extension RoundPoint: WeightExpansion<Dipole> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e321 * this.e5) - (right_anti_dual.e125 * this.e3)) - (this.group0.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((float3(this.e5) * right_anti_dual.group0.xyz), right_anti_dual.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4((float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group3.zxyw * this.group0.yzxw) - float4((right_anti_dual.group3.yzx * this.group0.zxy), right_anti_dual.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e1 * this.e5 * -1.0, right_anti_dual.e2 * this.e5 * -1.0, right_anti_dual.e3 * this.e5 * -1.0, (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235))
        );
    }
}
extension RoundPoint: WeightExpansion<DualNum> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension RoundPoint: WeightExpansion<FlatPoint> {
    associatedtype Output = Sphere;
    func weight_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)), 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<Flector> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_anti_dual.group1.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group1.xyz)), right_anti_dual.e321 * this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5))
        );
    }
}
extension RoundPoint: WeightExpansion<Line> {
    associatedtype Output = Circle;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.xyz.zxy * this.group0.yzx) - (right_anti_dual.group1.xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<Motor> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension RoundPoint: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * this.group0) - (float4(this.e5) * right_anti_dual.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * this.e4) + (right_anti_dual.e41 * this.e5), (right_anti_dual.e25 * this.e4) + (right_anti_dual.e42 * this.e5), (right_anti_dual.e35 * this.e4) + (right_anti_dual.e43 * this.e5), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_anti_dual.group5.xyz) + (right_anti_dual.group4.xyz.yzx * this.group0.zxy) - (right_anti_dual.group4.xyz.zxy * this.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * right_anti_dual.group5.xyz) + (right_anti_dual.group3.zxy * this.group0.yzx) - (right_anti_dual.group3.yzx * this.group0.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e321 * this.e5) - (right_anti_dual.e125 * this.e3)) - (this.group0.yzxy * float4(right_anti_dual.group6.zxy, right_anti_dual.e315)) - float4((float3(this.e5) * right_anti_dual.group7.xyz), right_anti_dual.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e321 * this.e4) + (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<Plane> {
    associatedtype Output = Dipole;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group0.zxy * this.group0.yzx) - (right_anti_dual.group0.yzx * this.group0.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35, 0 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz), 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<Sphere> {
    associatedtype Output = Dipole;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.group0.zxy * this.group0.yzx), right_anti_dual.e5 * this.e4) - (right_anti_dual.group0.yzxw * float4(this.group0.zxy, this.e5)), 
            /* e15, e25, e35, 0 */
            float4((float3(right_anti_dual.e5) * this.group0.xyz) - (float3(this.e5) * right_anti_dual.group0.xyz), 0.0)
        );
    }
}
extension RoundPoint: WeightExpansion<VersorEven> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
extension RoundPoint: WeightExpansion<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4((float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e5)) - float4((this.group0.zxy * right_anti_dual.group3.yzx), this.e5 * right_anti_dual.e4), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * right_anti_dual.e1 * -1.0, this.e5 * right_anti_dual.e2 * -1.0, this.e5 * right_anti_dual.e3 * -1.0, (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321)) + (this.group0.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e3 * right_anti_dual.e425) + (this.e4 * right_anti_dual.e235), (this.e1 * right_anti_dual.e435) + (this.e4 * right_anti_dual.e315), (this.e2 * right_anti_dual.e415) + (this.e4 * right_anti_dual.e125), -(this.e3 * right_anti_dual.e125) - (this.e5 * right_anti_dual.e321)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((float3(this.e5) * right_anti_dual.group0.xyz), this.e2 * right_anti_dual.e315)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<Scalar> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension Scalar: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * right_anti_dual.group2
        );
    }
}
extension Scalar: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}
extension Scalar: WeightExpansion<AntiDualNum> {
    associatedtype Output = DualNum;
    func weight_expansion(other: AntiDualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.scalar) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension Scalar: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = FlatPoint;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
extension Scalar: WeightExpansion<AntiFlector> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
extension Scalar: WeightExpansion<AntiLine> {
    associatedtype Output = Line;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_anti_dual.group1.xyz, 0.0)
        );
    }
}
extension Scalar: WeightExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
extension Scalar: WeightExpansion<AntiPlane> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension Scalar: WeightExpansion<AntiScalar> {
    associatedtype Output = Scalar;
    func weight_expansion(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: WeightExpansion<Circle> {
    associatedtype Output = Dipole;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * right_anti_dual.group2.xyz, 0.0)
        );
    }
}
extension Scalar: WeightExpansion<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.scalar) * right_anti_dual.group2
        );
    }
}
extension Scalar: WeightExpansion<Dipole> {
    associatedtype Output = Circle;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_anti_dual.group2.xyz, 0.0)
        );
    }
}
extension Scalar: WeightExpansion<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.scalar) * right_anti_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group3
        );
    }
}
extension Scalar: WeightExpansion<DualNum> {
    associatedtype Output = AntiDualNum;
    func weight_expansion(other: DualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.scalar) * float2(other.e5 * -1.0, other.e12345 * -1.0), 0.0, 0.0)
        );
    }
}
extension Scalar: WeightExpansion<FlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func weight_expansion(other: FlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
extension Scalar: WeightExpansion<Flector> {
    associatedtype Output = AntiFlector;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
extension Scalar: WeightExpansion<Line> {
    associatedtype Output = AntiLine;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * right_anti_dual.group1.xyz, 0.0)
        );
    }
}
extension Scalar: WeightExpansion<Motor> {
    associatedtype Output = AntiMotor;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.scalar) * right_anti_dual.group1
        );
    }
}
extension Scalar: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(this.scalar) * right_anti_dual.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_anti_dual.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * right_anti_dual.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * right_anti_dual.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: WeightExpansion<Plane> {
    associatedtype Output = AntiPlane;
    func weight_expansion(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension Scalar: WeightExpansion<RoundPoint> {
    associatedtype Output = Sphere;
    func weight_expansion(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: WeightExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: WeightExpansion<Sphere> {
    associatedtype Output = RoundPoint;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e5 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: WeightExpansion<VersorEven> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
extension Scalar: WeightExpansion<VersorOdd> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<Sphere> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension Sphere: WeightExpansion<AntiDualNum> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiDualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e3215 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<AntiMotor> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e3215 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<AntiScalar> {
    associatedtype Output = Sphere;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<CircleRotor> {
    associatedtype Output = Sphere;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4((right_anti_dual.e4 * this.e3215) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<DualNum> {
    associatedtype Output = Sphere;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<Flector> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<Motor> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234)), 0.0, 0.0), 
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
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<Plane> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<Sphere> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<VersorEven> {
    associatedtype Output = Sphere;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
            float4(right_anti_dual.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * right_anti_dual.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightExpansion<VersorOdd> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4((this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<VersorEven> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension VersorEven: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = Sphere;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4((float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.e1234)) + (this.group3.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235)) + float4(float3(0.0), (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) - float4((right_anti_dual.group2.yzx * this.group3.zxy), right_anti_dual.e41 * this.e235)
        );
    }
}
extension VersorEven: WeightExpansion<AntiDualNum> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension VersorEven: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e15 * this.e4, right_anti_dual.e25 * this.e4, right_anti_dual.e35 * this.e4, -(right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.zxy * this.group3.yzx) - (right_anti_dual.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
extension VersorEven: WeightExpansion<AntiFlector> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (this.group3.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e4235)) + float4(float3(0.0), (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.zxy * this.group3.yzx) - (right_anti_dual.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
extension VersorEven: WeightExpansion<AntiLine> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) - (this.group3.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
extension VersorEven: WeightExpansion<AntiMotor> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e235 * this.e4) + (right_anti_dual.e5 * this.e423), (right_anti_dual.e315 * this.e4) + (right_anti_dual.e5 * this.e431), (right_anti_dual.e125 * this.e4) + (right_anti_dual.e5 * this.e412), -(right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3)) + float4((right_anti_dual.group0.yzx * this.group3.zxy), right_anti_dual.e5 * this.e321) - (this.group3.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
extension VersorEven: WeightExpansion<AntiPlane> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: WeightExpansion<AntiScalar> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: WeightExpansion<Circle> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * this.group3.zxy) - (right_anti_dual.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * this.e5) + (right_anti_dual.e15 * this.e4), (right_anti_dual.e42 * this.e5) + (right_anti_dual.e25 * this.e4), (right_anti_dual.e43 * this.e5) + (right_anti_dual.e35 * this.e4), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) - float4((right_anti_dual.group2.xyz.yzx * this.group3.zxy), right_anti_dual.e41 * this.e235)
        );
    }
}
extension VersorEven: WeightExpansion<CircleRotor> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: WeightExpansion<Dipole> {
    associatedtype Output = Sphere;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * this.e3) + (right_anti_dual.e235 * this.e4), (right_anti_dual.e435 * this.e1) + (right_anti_dual.e315 * this.e4), (right_anti_dual.e415 * this.e2) + (right_anti_dual.e125 * this.e4), -(right_anti_dual.e321 * this.e5) - (right_anti_dual.e125 * this.e3)) - (this.group3.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((right_anti_dual.group0.xyz * this.group2.www), right_anti_dual.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: WeightExpansion<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
            float4((float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group3.zxyw * this.group3.yzxw) - float4((right_anti_dual.group3.yzx * this.group3.zxy), right_anti_dual.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - float4((this.group2.www * right_anti_dual.group3.xyz), right_anti_dual.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_anti_dual.group1.yzx * this.group3.zxy), right_anti_dual.e1 * this.e235) + float4((this.group3.www * right_anti_dual.group2.xyz), right_anti_dual.e2 * this.e315) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((right_anti_dual.group3.zxy * this.group1.yzx), right_anti_dual.e125 * this.e3)
        );
    }
}
extension VersorEven: WeightExpansion<DualNum> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: WeightExpansion<FlatPoint> {
    associatedtype Output = Sphere;
    func weight_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e1) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)), 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: WeightExpansion<Flector> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_anti_dual.group1.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group1.zxy * this.group3.yzx) - (right_anti_dual.group1.yzx * this.group3.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            (this.group3 * float4(right_anti_dual.group1.www, right_anti_dual.e321)) + float4(float3(0.0), -(right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - (right_anti_dual.group1.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + (right_anti_dual.group1.wwwz * float4(this.group0.xyz, this.e125)) + float4(float3(0.0), (right_anti_dual.e5 * this.e321) - (right_anti_dual.e315 * this.e2) - (right_anti_dual.e125 * this.e3) - (right_anti_dual.e321 * this.e5)) + float4((this.group3.www * right_anti_dual.group0.xyz), right_anti_dual.e1 * this.e235) - float4((right_anti_dual.group1.zxy * this.group1.yzx), right_anti_dual.e235 * this.e1)
        );
    }
}
extension VersorEven: WeightExpansion<Line> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e1) - (right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual.e23 * this.e5) + (right_anti_dual.e35 * this.e2), (right_anti_dual.e31 * this.e5) + (right_anti_dual.e15 * this.e3), (right_anti_dual.e12 * this.e5) + (right_anti_dual.e25 * this.e1), -(right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) - float4((right_anti_dual.group1.xyz.yzx * this.group3.zxy), right_anti_dual.e23 * this.e415)
        );
    }
}
extension VersorEven: WeightExpansion<Motor> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(0.0, (right_anti_dual.scalar * this.e12345) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * this.group3, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * this.group3) - (float4(this.e5) * right_anti_dual.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_anti_dual.group1.zxy * this.group3.yzx) - (right_anti_dual.group1.yzx * this.group3.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * this.e4) + (right_anti_dual.e41 * this.e5), (right_anti_dual.e25 * this.e4) + (right_anti_dual.e42 * this.e5), (right_anti_dual.e35 * this.e4) + (right_anti_dual.e43 * this.e5), -(right_anti_dual.e31 * this.e2) - (right_anti_dual.e12 * this.e3)) + (float4(right_anti_dual.scalar) * this.group1) - (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.e4) * right_anti_dual.group5.xyz) + (right_anti_dual.group4.xyz.yzx * this.group3.zxy) - (right_anti_dual.group4.xyz.zxy * this.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.scalar) * this.group2.xyz) + (float3(this.e5) * right_anti_dual.group5.xyz) + (right_anti_dual.group3.zxy * this.group3.yzx) - (right_anti_dual.group3.yzx * this.group3.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e321)) + (right_anti_dual.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_anti_dual.group8.xyz * this.group3.www), right_anti_dual.e1 * this.e235) + float4((right_anti_dual.group6.yzx * this.group3.zxy), right_anti_dual.e3 * this.e125) - (right_anti_dual.group6.zxyw * float4(this.group3.yzx, this.e5)) - float4((right_anti_dual.group7.xyz * this.group2.www), right_anti_dual.e235 * this.e1) - float4((right_anti_dual.group1.zxy * this.group1.yzx), right_anti_dual.e315 * this.e2) - float4((right_anti_dual.group1.www * this.group2.xyz), right_anti_dual.e125 * this.e3), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e321 * this.e4) + (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: WeightExpansion<Plane> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group0.zxy * this.group3.yzx) - (right_anti_dual.group0.yzx * this.group3.zxy)), right_anti_dual.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e5 * this.e1, right_anti_dual.e5 * this.e2, right_anti_dual.e5 * this.e3, -(right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412)) - (right_anti_dual.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e3 * this.e425 * -1.0, right_anti_dual.e1 * this.e435 * -1.0, right_anti_dual.e2 * this.e415 * -1.0, (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235)) + (right_anti_dual.group0.wwwy * float4(this.group0.xyz, this.e315))
        );
    }
}
extension VersorEven: WeightExpansion<RoundPoint> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: WeightExpansion<Sphere> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.group0.zxy * this.group3.yzx), right_anti_dual.e5 * this.e4) - (right_anti_dual.group0.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e5 * this.e1, right_anti_dual.e5 * this.e2, right_anti_dual.e5 * this.e3, -(right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)) - (right_anti_dual.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_anti_dual.e3 * this.e425) - (right_anti_dual.e4 * this.e235), -(right_anti_dual.e1 * this.e435) - (right_anti_dual.e4 * this.e315), -(right_anti_dual.e2 * this.e415) - (right_anti_dual.e4 * this.e125), (right_anti_dual.e3 * this.e125) + (right_anti_dual.e5 * this.e321)) + (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4((float3(right_anti_dual.e5) * this.group0.xyz), right_anti_dual.e2 * this.e315)
        );
    }
}
extension VersorEven: WeightExpansion<VersorEven> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
extension VersorEven: WeightExpansion<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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
            float4((float3(this.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (this.group3.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e5)) - (right_anti_dual.group3.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)) - (right_anti_dual.group3.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_anti_dual.group1.yzx * this.group3.zxy), right_anti_dual.e5 * this.e321) + float4((right_anti_dual.group2.www * this.group0.xyz), right_anti_dual.e2 * this.e315) + float4((this.group3.www * right_anti_dual.group2.xyz), right_anti_dual.e1 * this.e235) - (float4(this.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((right_anti_dual.group3.zxy * this.group1.yzx), right_anti_dual.e315 * this.e2) - float4((right_anti_dual.group3.www * this.group2.xyz), right_anti_dual.e125 * this.e3)
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: WeightExpansionInfix) -> WeightExpansionInfixPartial<VersorOdd> {
        return WeightExpansionInfixPartial { a: this };
    }
}
extension VersorOdd: WeightExpansion<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), (right_anti_dual.e12345 * this.scalar) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43))
        );
    }
}
extension VersorOdd: WeightExpansion<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: AntiDipoleInversion) -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
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
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), (right_anti_dual.e1234 * this.scalar) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42) + (right_anti_dual.e4235 * this.scalar), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43) + (right_anti_dual.e4315 * this.scalar), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41) + (right_anti_dual.e4125 * this.scalar), -(right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e23)) - float4((right_anti_dual.group2.yzx * this.group0.zxy), right_anti_dual.e31 * this.e25)
        );
    }
}
extension VersorOdd: WeightExpansion<AntiDualNum> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e5, (right_anti_dual.e5 * this.e1234) + (right_anti_dual.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
extension VersorOdd: WeightExpansion<AntiFlatPoint> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e35 * this.e42) + (right_anti_dual.e45 * this.e23), (right_anti_dual.e15 * this.e43) + (right_anti_dual.e45 * this.e31), (right_anti_dual.e25 * this.e41) + (right_anti_dual.e45 * this.e12), -(right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension VersorOdd: WeightExpansion<AntiFlector> {
    associatedtype Output = Flector;
    func weight_expansion(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e45 * this.e23) + (right_anti_dual.e4235 * this.scalar), (right_anti_dual.e45 * this.e31) + (right_anti_dual.e4315 * this.scalar), (right_anti_dual.e45 * this.e12) + (right_anti_dual.e4125 * this.scalar), -(right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) + (this.group0.yzxw * float4(right_anti_dual.group0.zxy, right_anti_dual.e3215)) - (right_anti_dual.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension VersorOdd: WeightExpansion<AntiLine> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension VersorOdd: WeightExpansion<AntiMotor> {
    associatedtype Output = Motor;
    func weight_expansion(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (float4(right_anti_dual.e5) * float4(this.group0.xyz, this.e1234)) + (float4(this.scalar) * right_anti_dual.group0) + float4(float3(0.0), -(right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group1.xyz)), right_anti_dual.e5 * this.scalar)
        );
    }
}
extension VersorOdd: WeightExpansion<AntiPlane> {
    associatedtype Output = Plane;
    func weight_expansion(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension VersorOdd: WeightExpansion<AntiScalar> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: AntiScalar) -> Output {
        Scalar right_anti_dual = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: WeightExpansion<Circle> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e42 * this.e35) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e43 * this.e15) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e41 * this.e25) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e23 * this.e15) - (right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), right_anti_dual.e15 * this.e23) - float4((right_anti_dual.group2.xyz.yzx * this.group0.zxy), right_anti_dual.e25 * this.e31)
        );
    }
}
extension VersorOdd: WeightExpansion<CircleRotor> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: WeightExpansion<Dipole> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group2.xyz), -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43))
        );
    }
}
extension VersorOdd: WeightExpansion<DipoleInversion> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: DipoleInversion) -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
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
extension VersorOdd: WeightExpansion<DualNum> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: WeightExpansion<FlatPoint> {
    associatedtype Output = CircleRotor;
    func weight_expansion(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e321 * this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45))
        );
    }
}
extension VersorOdd: WeightExpansion<Flector> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension VersorOdd: WeightExpansion<Line> {
    associatedtype Output = DipoleInversion;
    func weight_expansion(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e23 * this.e45) + (right_anti_dual.e35 * this.e42), (right_anti_dual.e31 * this.e45) + (right_anti_dual.e15 * this.e43), (right_anti_dual.e12 * this.e45) + (right_anti_dual.e25 * this.e41), -(right_anti_dual.e31 * this.e25) - (right_anti_dual.e12 * this.e35) - (right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12)) - float4((right_anti_dual.group1.xyz.yzx * this.group0.zxy), right_anti_dual.e23 * this.e15)
        );
    }
}
extension VersorOdd: WeightExpansion<Motor> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: WeightExpansion<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_expansion(other: MultiVector) -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
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
            float4(float2(right_anti_dual.scalar * this.scalar, (right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * right_anti_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.scalar) * float4(this.group2.xyz, this.e45)) + (float4(this.scalar) * right_anti_dual.group3), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.scalar) * this.group0.xyz) + (float3(this.scalar) * right_anti_dual.group4.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.scalar) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group5.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e5 * this.e41) + (right_anti_dual.e415 * this.scalar), (right_anti_dual.e5 * this.e42) + (right_anti_dual.e425 * this.scalar), (right_anti_dual.e5 * this.e43) + (right_anti_dual.e435 * this.scalar), -(right_anti_dual.e2 * this.e31) - (right_anti_dual.e3 * this.e12)) + float4((right_anti_dual.group1.www * this.group2.xyz), right_anti_dual.e321 * this.scalar) - (right_anti_dual.group1.xyzx * this.group1.wwwx), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.e4) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group7.xyz) + (right_anti_dual.group1.zxy * this.group0.yzx) - (right_anti_dual.group1.yzx * this.group0.zxy), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e5) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual.group8.xyz) + (right_anti_dual.group1.yzx * this.group2.zxy) - (right_anti_dual.group1.zxy * this.group2.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e35 * this.e42) + (right_anti_dual.e45 * this.e23) + (right_anti_dual.e23 * this.e45) + (right_anti_dual.e4235 * this.scalar), (right_anti_dual.e15 * this.e43) + (right_anti_dual.e45 * this.e31) + (right_anti_dual.e31 * this.e45) + (right_anti_dual.e4315 * this.scalar), (right_anti_dual.e25 * this.e41) + (right_anti_dual.e45 * this.e12) + (right_anti_dual.e12 * this.e45) + (right_anti_dual.e4125 * this.scalar), -(right_anti_dual.e15 * this.e23) - (right_anti_dual.e25 * this.e31) - (right_anti_dual.e35 * this.e12) - (right_anti_dual.e12 * this.e35)) + (float4(right_anti_dual.scalar) * this.group3) + float4((right_anti_dual.group4.xyz.yzx * this.group2.zxy), right_anti_dual.e3215 * this.scalar) - (this.group2.yzxx * float4(right_anti_dual.group4.xyz.zxy, right_anti_dual.e23)) - float4((right_anti_dual.group3.yzx * this.group0.zxy), right_anti_dual.e31 * this.e25), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.scalar * this.e1234) + (right_anti_dual.e1234 * this.scalar) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: WeightExpansion<Plane> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension VersorOdd: WeightExpansion<RoundPoint> {
    associatedtype Output = Sphere;
    func weight_expansion(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * right_anti_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: WeightExpansion<Scalar> {
    associatedtype Output = AntiScalar;
    func weight_expansion(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: WeightExpansion<Sphere> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension VersorOdd: WeightExpansion<VersorEven> {
    associatedtype Output = VersorOdd;
    func weight_expansion(other: VersorEven) -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
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
extension VersorOdd: WeightExpansion<VersorOdd> {
    associatedtype Output = VersorEven;
    func weight_expansion(other: VersorOdd) -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
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

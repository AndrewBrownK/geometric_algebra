extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<AntiCircleRotor> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension AntiCircleRotor: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
extension AntiCircleRotor: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((right_dual.group2.www * this.group2.xyz), this.e42 * right_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkContraction<AntiDualNum> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_dual.e12345) * this.group2
        );
    }
}
extension AntiCircleRotor: BulkContraction<AntiFlector> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkContraction<AntiLine> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkContraction<AntiMotor> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435))
        );
    }
}
extension AntiCircleRotor: BulkContraction<AntiPlane> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkContraction<Dipole> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkContraction<DipoleInversion> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkContraction<DualNum> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: DualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension AntiCircleRotor: BulkContraction<FlatPoint> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e45 * right_dual.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkContraction<Flector> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e45 * right_dual.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkContraction<Motor> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Motor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(float4(other.group1.xyz, other.e5 * -1.0)[3]) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension AntiCircleRotor: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group9.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_dual.e1234) * this.group2.xyz), this.e42 * right_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e12345) * this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiCircleRotor: BulkContraction<RoundPoint> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_dual.e1234) * this.group2.xyz), this.e42 * right_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkContraction<Scalar> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_dual.e12345) * this.group2
        );
    }
}
extension AntiCircleRotor: BulkContraction<VersorEven> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((right_dual.group2.www * this.group2.xyz), this.e42 * right_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: BulkContraction<VersorOdd> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.scalar * right_dual.e12345) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<AntiDipoleInversion> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension AntiDipoleInversion: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.e4 * right_dual.e12345) - (this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412) + (this.e1 * right_dual.e12345), (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423) + (this.e2 * right_dual.e12345), (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431) + (this.e3 * right_dual.e12345), -(this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + (right_dual.group2.yzxw * float4(this.group0.xyz.zxy, this.e5)) - (right_dual.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), this.e425 * right_dual.e315)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group3.yzx) - (this.group0.xyz.yzx * right_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_dual.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((this.group2.zxy * right_dual.group3.yzx), this.e423 * right_dual.e15)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<AntiDualNum> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_dual.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
extension AntiDipoleInversion: BulkContraction<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<AntiFlector> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * right_dual.group1.yzx) - (this.group0.xyz.yzx * right_dual.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_dual.e3215, this.e431 * right_dual.e3215, this.e412 * right_dual.e3215, -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_dual.group1.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)) - float4((this.group2.zxy * right_dual.group1.yzx), this.e423 * right_dual.e15)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<AntiLine> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e315) + (this.e321 * right_dual.e415), (this.e423 * right_dual.e125) + (this.e321 * right_dual.e425), (this.e431 * right_dual.e235) + (this.e321 * right_dual.e435), -(this.e431 * right_dual.e425) - (this.e412 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group1.xyz.zxy), this.e423 * right_dual.e415), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<AntiMotor> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.e4 * right_dual.e12345) - (this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_dual.e415) + (this.e1 * right_dual.e12345), (this.e321 * right_dual.e425) + (this.e2 * right_dual.e12345), (this.e321 * right_dual.e435) + (this.e3 * right_dual.e12345), -(this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + float4((this.group0.xyz.zxy * right_dual.group1.yzx), this.e5 * right_dual.e12345) - (right_dual.group1.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
extension AntiDipoleInversion: BulkContraction<AntiPlane> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_dual.e3215, this.e431 * right_dual.e3215, this.e412 * right_dual.e3215, -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * right_dual.e4315 * -1.0, this.e235 * right_dual.e4125 * -1.0, this.e315 * right_dual.e4235 * -1.0, (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125)) + (float4(right_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_dual.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
extension AntiDipoleInversion: BulkContraction<Circle> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<CircleRotor> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<Dipole> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) - float4((this.group0.xyz.yzx * right_dual.group2.xyz.zxy), this.e423 * right_dual.e415) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), this.e431 * right_dual.e425), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<DipoleInversion> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.e423 * this.e415) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), right_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<DualNum> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.e3215) * float4(this.group0.xyz, this.e4), 
            /* e15, e25, e35, e3215 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<FlatPoint> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321), -(this.e425 * right_dual.e315) - (this.e435 * right_dual.e125)) - (right_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
extension AntiDipoleInversion: BulkContraction<Flector> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321), -(this.e425 * right_dual.e315) - (this.e435 * right_dual.e125)) - (right_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
extension AntiDipoleInversion: BulkContraction<Line> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e3215) * this.group0.xyz), (this.e4 * right_dual.e3215) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e4 * right_dual.e3215) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) + (float4(right_dual.e12345) * float4(this.group3.xyz, this.e4)) - float4((this.group0.xyz.yzx * right_dual.group8.xyz.zxy), this.e423 * right_dual.e415) - float4((right_dual.group7.xyz.yzx * this.group2.zxy), this.e431 * right_dual.e425), 
            /* e5, 0, 0, 0 */
            float4((this.e5 * right_dual.e12345) - (this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group9.yzx) - (this.group0.xyz.yzx * right_dual.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e3215) * this.group0.xyz) + (float3(right_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e12345) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<RoundPoint> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * right_dual.e4315 * -1.0, this.e235 * right_dual.e4125 * -1.0, this.e315 * right_dual.e4235 * -1.0, (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)) + (float4(right_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_dual.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
extension AntiDipoleInversion: BulkContraction<Scalar> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_dual.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
extension AntiDipoleInversion: BulkContraction<VersorEven> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group3.yzx) - (this.group0.xyz.yzx * right_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_dual.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((this.group2.zxy * right_dual.group3.yzx), this.e423 * right_dual.e15)
        );
    }
}
extension AntiDipoleInversion: BulkContraction<VersorOdd> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), (this.e4 * right_dual.e12345) - (this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412) + (this.e1 * right_dual.e12345), (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423) + (this.e2 * right_dual.e12345), (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431) + (this.e3 * right_dual.e12345), -(this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + float4((this.group0.xyz.zxy * right_dual.group2.yzx), this.e5 * right_dual.e12345) - (right_dual.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((this.group2.zxy * right_dual.group0.yzx), this.e425 * right_dual.e315)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<AntiDualNum> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension AntiDualNum: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.xx, this.e3215, this.scalar) * float4(right_dual.group0.xyz, right_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e12345)
        );
    }
}
extension AntiDualNum: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_dual.group3.xyz, right_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45) * float4(-1.0)
        );
    }
}
extension AntiDualNum: BulkContraction<AntiDualNum> {
    associatedtype Output = AntiDualNum;
    func bulk_contraction(other: AntiDualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkContraction<AntiFlatPoint> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e3215, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkContraction<AntiFlector> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e3215 * right_dual.e45 * -1.0)
        );
    }
}
extension AntiDualNum: BulkContraction<AntiLine> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: AntiLine) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkContraction<AntiMotor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.scalar * right_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * right_dual.group0
        );
    }
}
extension AntiDualNum: BulkContraction<AntiPlane> {
    associatedtype Output = AntiFlatPoint;
    func bulk_contraction(other: AntiPlane) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkContraction<Circle> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45) * float4(-1.0)
        );
    }
}
extension AntiDualNum: BulkContraction<CircleRotor> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45) * float4(-1.0)
        );
    }
}
extension AntiDualNum: BulkContraction<Dipole> {
    associatedtype Output = AntiLine;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e3215) * right_dual.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.e3215) * right_dual.group1.xyz, 0.0)
        );
    }
}
extension AntiDualNum: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e4), 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e3215 * right_dual.e4) + (this.scalar * right_dual.e12345), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group4.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e3215 * right_dual.e45 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e3215) * right_dual.group7.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * right_dual.e1234 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e3215) * right_dual.group9.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * right_dual.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: BulkContraction<RoundPoint> {
    associatedtype Output = AntiFlatPoint;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e1234) * float4(-1.0)
        );
    }
}
extension AntiDualNum: BulkContraction<Scalar> {
    associatedtype Output = AntiDualNum;
    func bulk_contraction(other: Scalar) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkContraction<Sphere> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e3215 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: BulkContraction<VersorEven> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_dual.group3.xyz, right_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45) * float4(-1.0)
        );
    }
}
extension AntiDualNum: BulkContraction<VersorOdd> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.xx, this.e3215, (this.e3215 * right_dual.e4) + (this.scalar * right_dual.e12345)) * float4(right_dual.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e12345)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<AntiFlatPoint> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension AntiFlatPoint: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_dual.e412) + (this.e321 * right_dual.e415), (this.e125 * right_dual.e423) + (this.e321 * right_dual.e425), (this.e235 * right_dual.e431) + (this.e321 * right_dual.e435), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((right_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_dual.e415)
        );
    }
}
extension AntiFlatPoint: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e235 * right_dual.e1234, this.e315 * right_dual.e1234, this.e125 * right_dual.e1234, -(this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) - float4((this.group0.www * right_dual.group3.xyz), this.e235 * right_dual.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx)), 0.0)
        );
    }
}
extension AntiFlatPoint: BulkContraction<AntiDualNum> {
    associatedtype Output = AntiFlatPoint;
    func bulk_contraction(other: AntiDualNum) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiFlatPoint: BulkContraction<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkContraction<AntiFlector> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(right_dual.group1.xyz, right_dual.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group1.zxy) - (this.group0.zxy * right_dual.group1.yzx)), 0.0)
        );
    }
}
extension AntiFlatPoint: BulkContraction<AntiLine> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * right_dual.group0.xyz), -(this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435))
        );
    }
}
extension AntiFlatPoint: BulkContraction<AntiMotor> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.group0.www * right_dual.group0.xyz), -(this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435))
        );
    }
}
extension AntiFlatPoint: BulkContraction<AntiPlane> {
    associatedtype Output = AntiLine;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * right_dual.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4((this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx), 0.0)
        );
    }
}
extension AntiFlatPoint: BulkContraction<Circle> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkContraction<CircleRotor> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e45 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: BulkContraction<Dipole> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_dual.e412) + (this.e321 * right_dual.e415), (this.e125 * right_dual.e423) + (this.e321 * right_dual.e425), (this.e235 * right_dual.e431) + (this.e321 * right_dual.e435), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((right_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_dual.e415)
        );
    }
}
extension AntiFlatPoint: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321), -(right_dual.e425 * this.e315) - (right_dual.e435 * this.e125)) - (this.group0.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e415))
        );
    }
}
extension AntiFlatPoint: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_dual.group6.xyz), 0.0) + float4((right_dual.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((right_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4(-(this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * right_dual.group9.zxy) - (this.group0.zxy * right_dual.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_dual.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiFlatPoint: BulkContraction<RoundPoint> {
    associatedtype Output = AntiLine;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_dual.group0.xyz), 0.0), 
            /* e15, e25, e35, 0 */
            float4((this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx), 0.0)
        );
    }
}
extension AntiFlatPoint: BulkContraction<Scalar> {
    associatedtype Output = AntiFlatPoint;
    func bulk_contraction(other: Scalar) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiFlatPoint: BulkContraction<VersorEven> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e235 * right_dual.e1234, this.e315 * right_dual.e1234, this.e125 * right_dual.e1234, -(this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx)), 0.0)
        );
    }
}
extension AntiFlatPoint: BulkContraction<VersorOdd> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_dual.e412) + (this.e321 * right_dual.e415), (this.e125 * right_dual.e423) + (this.e321 * right_dual.e425), (this.e235 * right_dual.e431) + (this.e321 * right_dual.e435), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - (this.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e415))
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<AntiFlector> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension AntiFlector: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_dual.e415) + (this.e1 * right_dual.e12345), (this.e321 * right_dual.e425) + (this.e2 * right_dual.e12345), (this.e321 * right_dual.e435) + (this.e3 * right_dual.e12345), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + float4((right_dual.group0.xyz.zxy * this.group0.yzx), this.e5 * right_dual.e12345) - float4((right_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_dual.e415)
        );
    }
}
extension AntiFlector: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) + float4((right_dual.group2.www * this.group0.xyz), this.e1 * right_dual.e4235) - float4((this.group0.www * right_dual.group3.xyz), this.e235 * right_dual.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx)), 0.0)
        );
    }
}
extension AntiFlector: BulkContraction<AntiDualNum> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
extension AntiFlector: BulkContraction<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: BulkContraction<AntiFlector> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * right_dual.group1.xyz * float3(-1.0)), (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) - (this.e321 * right_dual.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group1.zxy) - (this.group0.zxy * right_dual.group1.yzx)), 0.0)
        );
    }
}
extension AntiFlector: BulkContraction<AntiLine> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * right_dual.group0.xyz), -(this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435))
        );
    }
}
extension AntiFlector: BulkContraction<AntiMotor> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e1 * right_dual.e12345, this.e2 * right_dual.e12345, this.e3 * right_dual.e12345, -(this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + (right_dual.group0 * float4(this.group0.www, this.e5))
        );
    }
}
extension AntiFlector: BulkContraction<AntiPlane> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * right_dual.group0.xyz * float3(-1.0)), (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: BulkContraction<Circle> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: BulkContraction<CircleRotor> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e45 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: BulkContraction<Dipole> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_dual.e412) + (this.e321 * right_dual.e415), (this.e125 * right_dual.e423) + (this.e321 * right_dual.e425), (this.e235 * right_dual.e431) + (this.e321 * right_dual.e435), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((right_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_dual.e415)
        );
    }
}
extension AntiFlector: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321), -(right_dual.e425 * this.e315) - (right_dual.e435 * this.e125)) - (this.group0.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e415))
        );
    }
}
extension AntiFlector: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_dual.group6.xyz), 0.0) + float4((float3(right_dual.e12345) * this.group1.xyz), 0.0) + float4((right_dual.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((right_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4((this.e5 * right_dual.e12345) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * right_dual.group9.zxy) - (this.group0.zxy * right_dual.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_dual.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiFlector: BulkContraction<RoundPoint> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e321 * right_dual.e4235 * -1.0, this.e321 * right_dual.e4315 * -1.0, this.e321 * right_dual.e4125 * -1.0, (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234)) + float4((float3(right_dual.e1234) * this.group0.xyz), this.e1 * right_dual.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group0.zxy) - (this.group0.zxy * right_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: BulkContraction<Scalar> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
extension AntiFlector: BulkContraction<VersorEven> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43) - (this.e321 * right_dual.e45)) + float4((right_dual.group2.www * this.group0.xyz), this.e1 * right_dual.e4235) - (this.group0.wwwx * float4(right_dual.group3.xyz, right_dual.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_dual.group3.zxy) - (this.group0.zxy * right_dual.group3.yzx)), 0.0)
        );
    }
}
extension AntiFlector: BulkContraction<VersorOdd> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_dual.e415) + (this.e1 * right_dual.e12345), (this.e321 * right_dual.e425) + (this.e2 * right_dual.e12345), (this.e321 * right_dual.e435) + (this.e3 * right_dual.e12345), -(this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) + (right_dual.group0.zxyw * float4(this.group0.yzx, this.e5)) - (this.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e415))
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<AntiLine> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension AntiLine: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e12345) * this.group0.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: BulkContraction<AntiDualNum> {
    associatedtype Output = AntiLine;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(right_dual.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: BulkContraction<AntiFlector> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_dual.e4315 * -1.0, this.e23 * right_dual.e4125 * -1.0, this.e31 * right_dual.e4235 * -1.0, (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group1.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: BulkContraction<AntiLine> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: BulkContraction<AntiMotor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e12345) * this.group0.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: BulkContraction<AntiPlane> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_dual.e4315 * -1.0, this.e23 * right_dual.e4125 * -1.0, this.e31 * right_dual.e4235 * -1.0, (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: BulkContraction<Dipole> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: BulkContraction<DipoleInversion> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.xyz.yzx * right_dual.group9.zxy), 0.0) - float4((float3(right_dual.e1234) * this.group1.xyz), 0.0) - float4((this.group0.xyz.zxy * right_dual.group9.yzx), 0.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.yy, right_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiLine: BulkContraction<RoundPoint> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: BulkContraction<Scalar> {
    associatedtype Output = AntiLine;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(right_dual.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: BulkContraction<VersorEven> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: BulkContraction<VersorOdd> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e12345) * this.group0.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<AntiMotor> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension AntiMotor: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e23 * right_dual.e12345, this.e31 * right_dual.e12345, this.e12 * right_dual.e12345, -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)) + float4((right_dual.group0.xyz * this.group1.www), this.scalar * right_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_dual.group1.xyz) + (float3(right_dual.e12345) * this.group1.xyz)), this.e3215 * right_dual.e12345)
        );
    }
}
extension AntiMotor: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_dual.group3.xyz, right_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((right_dual.group0.xyz * this.group1.www), this.e3215 * right_dual.e45)
        );
    }
}
extension AntiMotor: BulkContraction<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
extension AntiMotor: BulkContraction<AntiFlatPoint> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e3215, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiMotor: BulkContraction<AntiFlector> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            (right_dual.group1.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) - float4((this.group0.zxy * right_dual.group1.yzx), this.e3215 * right_dual.e45)
        );
    }
}
extension AntiMotor: BulkContraction<AntiLine> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: BulkContraction<AntiMotor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e12345) * this.group0.xyz), (this.scalar * right_dual.e12345) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_dual.group0.xyz) + (float3(right_dual.e12345) * this.group1.xyz)), this.e3215 * right_dual.e12345)
        );
    }
}
extension AntiMotor: BulkContraction<AntiPlane> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_dual.e4315 * -1.0, this.e23 * right_dual.e4125 * -1.0, this.e31 * right_dual.e4235 * -1.0, (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
extension AntiMotor: BulkContraction<Circle> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_dual.group0.xyz * this.group1.www * float3(-1.0)), this.e3215 * right_dual.e45 * -1.0)
        );
    }
}
extension AntiMotor: BulkContraction<CircleRotor> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45) * float4(-1.0)
        );
    }
}
extension AntiMotor: BulkContraction<Dipole> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * right_dual.group0.xyz), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * right_dual.group0.xyz), (right_dual.e4 * this.e3215) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.scalar * right_dual.e12345) + (this.e3215 * right_dual.e4) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.yzx * right_dual.group9.zxy), 0.0) - float4((float3(this.e3215) * right_dual.group4.xyz), 0.0) - float4((float3(right_dual.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * right_dual.group9.yzx), 0.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) - (this.e3215 * right_dual.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(this.e3215) * right_dual.group6.xyz) + (float3(right_dual.e12345) * this.group1.xyz)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_dual.group7.xyz) + (float3(right_dual.e12345) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * right_dual.e1234 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e3215) * right_dual.group9.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * right_dual.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: BulkContraction<RoundPoint> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
extension AntiMotor: BulkContraction<Scalar> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
extension AntiMotor: BulkContraction<Sphere> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e3215 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: BulkContraction<VersorEven> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_dual.group3.xyz, right_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e15 * right_dual.e1234) - (this.e3215 * right_dual.e41), -(this.e25 * right_dual.e1234) - (this.e3215 * right_dual.e42), -(this.e35 * right_dual.e1234) - (this.e3215 * right_dual.e43), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (right_dual.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((this.group0.zxy * right_dual.group3.yzx), this.e3215 * right_dual.e45)
        );
    }
}
extension AntiMotor: BulkContraction<VersorOdd> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e4)) + (float4(right_dual.e12345) * this.group0) + float4(float3(0.0), -(this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_dual.group1.xyz) + (float3(right_dual.e12345) * this.group1.xyz)), this.e3215 * right_dual.e12345)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<AntiPlane> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension AntiPlane: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: BulkContraction<AntiDualNum> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: AntiDualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: BulkContraction<AntiFlector> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: BulkContraction<AntiMotor> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: AntiMotor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: BulkContraction<AntiPlane> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.group0.xy.yy, right_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_dual.e12345, 0.0, 0.0, 0.0), 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiPlane: BulkContraction<RoundPoint> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: BulkContraction<Scalar> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Scalar) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: BulkContraction<VersorEven> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e5 * right_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: BulkContraction<VersorOdd> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: VersorOdd) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<AntiScalar> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension AntiScalar: BulkContraction<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.e12345) * right_dual.group2
        );
    }
}
extension AntiScalar: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * right_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
extension AntiScalar: BulkContraction<AntiDualNum> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: AntiDualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.e12345) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkContraction<AntiFlatPoint> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
extension AntiScalar: BulkContraction<AntiFlector> {
    associatedtype Output = Flector;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
extension AntiScalar: BulkContraction<AntiLine> {
    associatedtype Output = Line;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_dual.group1.xyz, 0.0)
        );
    }
}
extension AntiScalar: BulkContraction<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * right_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
extension AntiScalar: BulkContraction<AntiPlane> {
    associatedtype Output = Plane;
    func bulk_contraction(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension AntiScalar: BulkContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkContraction<Circle> {
    associatedtype Output = Dipole;
    func bulk_contraction(other: Circle) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * right_dual.group2.xyz, 0.0)
        );
    }
}
extension AntiScalar: BulkContraction<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: CircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * right_dual.group2
        );
    }
}
extension AntiScalar: BulkContraction<Dipole> {
    associatedtype Output = Circle;
    func bulk_contraction(other: Dipole) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_dual.group2.xyz, 0.0)
        );
    }
}
extension AntiScalar: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * right_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
extension AntiScalar: BulkContraction<DualNum> {
    associatedtype Output = AntiDualNum;
    func bulk_contraction(other: DualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0), 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkContraction<FlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func bulk_contraction(other: FlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
extension AntiScalar: BulkContraction<Flector> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
extension AntiScalar: BulkContraction<Line> {
    associatedtype Output = AntiLine;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * right_dual.group1.xyz, 0.0)
        );
    }
}
extension AntiScalar: BulkContraction<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * right_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
extension AntiScalar: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2(this.e12345) * right_dual.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_dual.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_dual.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_dual.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_dual.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_dual.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkContraction<Plane> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension AntiScalar: BulkContraction<RoundPoint> {
    associatedtype Output = Sphere;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkContraction<Scalar> {
    associatedtype Output = AntiScalar;
    func bulk_contraction(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkContraction<Sphere> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_dual.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: BulkContraction<VersorEven> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: VersorEven) -> Output {
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
extension AntiScalar: BulkContraction<VersorOdd> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: VersorOdd) -> Output {
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
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<Circle> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension Circle: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e125 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), this.e235 * right_dual.e415) - float4((this.group2.xyz.zxy * right_dual.group0.xyz.yzx), this.e315 * right_dual.e425)
        );
    }
}
extension Circle: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group3.yzx) - (this.group0.xyz.yzx * right_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((this.group2.xyz.zxy * right_dual.group3.yzx), this.e423 * right_dual.e15)
        );
    }
}
extension Circle: BulkContraction<AntiDualNum> {
    associatedtype Output = Circle;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: BulkContraction<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkContraction<AntiFlector> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * right_dual.group1.yzx) - (this.group0.xyz.yzx * right_dual.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_dual.e3215, this.e431 * right_dual.e3215, this.e412 * right_dual.e3215, -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45)) - float4((this.group2.xyz.zxy * right_dual.group1.yzx), this.e423 * right_dual.e15)
        );
    }
}
extension Circle: BulkContraction<AntiLine> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e315) + (this.e321 * right_dual.e415), (this.e423 * right_dual.e125) + (this.e321 * right_dual.e425), (this.e431 * right_dual.e235) + (this.e321 * right_dual.e435), -(this.e431 * right_dual.e425) - (this.e412 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group1.xyz.zxy), this.e423 * right_dual.e415), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkContraction<AntiMotor> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e321 * right_dual.e415), (this.e423 * right_dual.e125) + (this.e321 * right_dual.e425), (this.e431 * right_dual.e235) + (this.e321 * right_dual.e435), -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group1.zxy), this.e235 * right_dual.e415)
        );
    }
}
extension Circle: BulkContraction<AntiPlane> {
    associatedtype Output = Dipole;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_dual.e3215, this.e431 * right_dual.e3215, this.e412 * right_dual.e3215, -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(right_dual.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * right_dual.group0.zxy) - (this.group2.xyz.zxy * right_dual.group0.yzx), 0.0)
        );
    }
}
extension Circle: BulkContraction<Circle> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkContraction<CircleRotor> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkContraction<Dipole> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) - float4((right_dual.group0.xyz.yzx * this.group2.xyz.zxy), right_dual.e423 * this.e415) - float4((right_dual.group2.xyz.zxy * this.group0.xyz.yzx), right_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkContraction<DipoleInversion> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) - float4((right_dual.group0.xyz.yzx * this.group2.xyz.zxy), right_dual.e423 * this.e415) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), right_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkContraction<DualNum> {
    associatedtype Output = AntiLine;
    func bulk_contraction(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e3215) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(right_dual.e3215) * this.group1.xyz, 0.0)
        );
    }
}
extension Circle: BulkContraction<FlatPoint> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_dual.e315 * this.e412) + (right_dual.e321 * this.e415), (right_dual.e125 * this.e423) + (right_dual.e321 * this.e425), (right_dual.e235 * this.e431) + (right_dual.e321 * this.e435), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_dual.group0.zxy), right_dual.e235 * this.e415)
        );
    }
}
extension Circle: BulkContraction<Flector> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_dual.e315 * this.e412) + (right_dual.e321 * this.e415), (right_dual.e125 * this.e423) + (right_dual.e321 * this.e425), (right_dual.e235 * this.e431) + (right_dual.e321 * this.e435), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_dual.group0.zxy), right_dual.e235 * this.e415)
        );
    }
}
extension Circle: BulkContraction<Line> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: BulkContraction<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual.e3215) * this.group0.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Circle: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) - float4((this.group0.xyz.yzx * right_dual.group8.xyz.zxy), this.e423 * right_dual.e415) - float4((this.group2.xyz.zxy * right_dual.group7.xyz.yzx), this.e431 * right_dual.e425), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group9.yzxx * float4(this.group2.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group9.yzx) - (this.group0.xyz.yzx * right_dual.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e3215) * this.group0.xyz) + (float3(right_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e12345) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Circle: BulkContraction<RoundPoint> {
    associatedtype Output = Dipole;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(right_dual.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * right_dual.group0.zxy) - (this.group2.xyz.zxy * right_dual.group0.yzx), 0.0)
        );
    }
}
extension Circle: BulkContraction<Scalar> {
    associatedtype Output = Circle;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: BulkContraction<VersorEven> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group3.yzx) - (this.group0.xyz.yzx * right_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((this.group2.xyz.zxy * right_dual.group3.yzx), this.e423 * right_dual.e15)
        );
    }
}
extension Circle: BulkContraction<VersorOdd> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e423 * right_dual.e125) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e431 * right_dual.e235) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e125 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), this.e235 * right_dual.e415) - float4((this.group2.xyz.zxy * right_dual.group0.yzx), this.e315 * right_dual.e425)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<CircleRotor> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension CircleRotor: BulkContraction<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension CircleRotor: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return VersorOdd.from_groups(
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
extension CircleRotor: BulkContraction<AntiDualNum> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension CircleRotor: BulkContraction<AntiFlatPoint> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e12345 * right_dual.e45), 
            /* e15, e25, e35, scalar */
            float4((this.group2.www * right_dual.group0.xyz), -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e321 * right_dual.e45))
        );
    }
}
extension CircleRotor: BulkContraction<AntiFlector> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension CircleRotor: BulkContraction<AntiLine> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group1.xyz), -(this.e423 * right_dual.e415) - (this.e431 * right_dual.e425) - (this.e412 * right_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_dual.e315) + (this.e321 * right_dual.e415), (this.e423 * right_dual.e125) + (this.e321 * right_dual.e425), (this.e431 * right_dual.e235) + (this.e321 * right_dual.e435), -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((this.group0.xyz.yzx * right_dual.group1.xyz.zxy), this.e235 * right_dual.e415)
        );
    }
}
extension CircleRotor: BulkContraction<AntiMotor> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension CircleRotor: BulkContraction<AntiPlane> {
    associatedtype Output = DipoleInversion;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_dual.e3215, this.e431 * right_dual.e3215, this.e412 * right_dual.e3215, -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * right_dual.group0.zxy), 0.0) - float4((this.group2.zxy * right_dual.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0
        );
    }
}
extension CircleRotor: BulkContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkContraction<Circle> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43))
        );
    }
}
extension CircleRotor: BulkContraction<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: CircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
extension CircleRotor: BulkContraction<Dipole> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e125 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.e235 * this.e415) - float4((right_dual.group2.xyz.zxy * this.group0.xyz.yzx), right_dual.e315 * this.e425)
        );
    }
}
extension CircleRotor: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.e4 * this.e12345) - (right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412) + (right_dual.e1 * this.e12345), (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423) + (right_dual.e2 * this.e12345), (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431) + (right_dual.e3 * this.e12345), -(right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + (this.group2.yzxw * float4(right_dual.group0.xyz.zxy, right_dual.e5)) - (this.group2.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e415)) - float4((this.group0.xyz.yzx * right_dual.group2.zxy), right_dual.e425 * this.e315)
        );
    }
}
extension CircleRotor: BulkContraction<DualNum> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.group0.xy.xx, right_dual.e3215, right_dual.scalar) * float4(this.group0.xyz, this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
extension CircleRotor: BulkContraction<FlatPoint> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual.e315 * this.e412) + (right_dual.e321 * this.e415), (right_dual.e125 * this.e423) + (right_dual.e321 * this.e425), (right_dual.e235 * this.e431) + (right_dual.e321 * this.e435), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_dual.group0.zxy), right_dual.e235 * this.e415)
        );
    }
}
extension CircleRotor: BulkContraction<Flector> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual.e321 * this.e415) + (right_dual.e1 * this.e12345), (right_dual.e321 * this.e425) + (right_dual.e2 * this.e12345), (right_dual.e321 * this.e435) + (right_dual.e3 * this.e12345), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + float4((this.group0.xyz.zxy * right_dual.group0.yzx), right_dual.e5 * this.e12345) - float4((this.group0.xyz.yzx * right_dual.group0.zxy), right_dual.e235 * this.e415)
        );
    }
}
extension CircleRotor: BulkContraction<Line> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_dual.group0.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension CircleRotor: BulkContraction<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.e23 * this.e12345, right_dual.e31 * this.e12345, right_dual.e12 * this.e12345, -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)) + float4((this.group0.xyz * right_dual.group1.www), right_dual.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual.e3215) * this.group1.xyz) + (float3(this.e12345) * right_dual.group1.xyz)), right_dual.e3215 * this.e12345)
        );
    }
}
extension CircleRotor: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e12345 * right_dual.scalar) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), this.e12345 * right_dual.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412) + (this.e12345 * right_dual.e1), (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423) + (this.e12345 * right_dual.e2), (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431) + (this.e12345 * right_dual.e3), -(this.e412 * right_dual.e435) - (this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) + float4((this.group0.xyz.zxy * right_dual.group8.xyz.yzx), this.e12345 * right_dual.e4) - float4((this.group0.xyz.yzx * right_dual.group8.xyz.zxy), this.e423 * right_dual.e415) - float4((right_dual.group7.xyz.yzx * this.group2.zxy), this.e431 * right_dual.e425), 
            /* e5, 0, 0, 0 */
            float4((this.e12345 * right_dual.e5) - (this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e315 * right_dual.e4125) + (this.e12345 * right_dual.e15), (this.e125 * right_dual.e4235) + (this.e12345 * right_dual.e25), (this.e235 * right_dual.e4315) + (this.e12345 * right_dual.e35), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) + float4((right_dual.group9.www * this.group1.xyz), this.e12345 * right_dual.e45) - (right_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e12345) * right_dual.group4.xyz) + (float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group9.yzx) - (this.group0.xyz.yzx * right_dual.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e12345) * right_dual.group5.xyz) + (float3(right_dual.e3215) * this.group0.xyz) + (float3(right_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * right_dual.group6) + (float4(right_dual.e12345) * this.group1), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e12345) * right_dual.group7.xyz) + (float3(right_dual.e12345) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e12345) * right_dual.group8.xyz) + (float3(right_dual.e12345) * this.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkContraction<Plane> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension CircleRotor: BulkContraction<RoundPoint> {
    associatedtype Output = DipoleInversion;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_dual.group0.yzx) - (this.group0.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_dual.e3215) + (this.e235 * right_dual.e1234), (this.e431 * right_dual.e3215) + (this.e315 * right_dual.e1234), (this.e412 * right_dual.e3215) + (this.e125 * right_dual.e1234), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (this.group1.wwwx * right_dual.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual.e3215) * this.group1.xyz) + (this.group2.yzx * right_dual.group0.zxy) - (this.group2.zxy * right_dual.group0.yzx)), this.e12345 * right_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0
        );
    }
}
extension CircleRotor: BulkContraction<Scalar> {
    associatedtype Output = CircleRotor;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_dual.e12345) * this.group2
        );
    }
}
extension CircleRotor: BulkContraction<Sphere> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_dual.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: BulkContraction<VersorEven> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: VersorEven) -> Output {
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
extension CircleRotor: BulkContraction<VersorOdd> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: VersorOdd) -> Output {
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
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<Dipole> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension Dipole: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43))
        );
    }
}
extension Dipole: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_dual.group2.www), this.e41 * right_dual.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkContraction<AntiDualNum> {
    associatedtype Output = Dipole;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(right_dual.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: BulkContraction<AntiFlector> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkContraction<AntiLine> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkContraction<AntiMotor> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435))
        );
    }
}
extension Dipole: BulkContraction<AntiPlane> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkContraction<Dipole> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkContraction<DipoleInversion> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkContraction<DualNum> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: DualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension Dipole: BulkContraction<FlatPoint> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkContraction<Flector> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkContraction<Motor> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.group0.xyz * right_dual.group1.www), right_dual.e3215 * this.e45)
        );
    }
}
extension Dipole: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group9.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_dual.e1234) * this.group2.xyz), this.e41 * right_dual.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e12345) * this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Dipole: BulkContraction<RoundPoint> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group0.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_dual.e1234) * this.group2.xyz), this.e41 * right_dual.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkContraction<Scalar> {
    associatedtype Output = Dipole;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(right_dual.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: BulkContraction<VersorEven> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_dual.group2.www), this.e41 * right_dual.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: BulkContraction<VersorOdd> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group2.xyz), -(this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412))
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<DipoleInversion> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension DipoleInversion: BulkContraction<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension DipoleInversion: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e41)) + float4(float3(0.0), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234) - (right_dual.e1234 * this.e45) - (right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) - float4((this.group3.www * right_dual.group3.xyz), right_dual.e4235 * this.e41), 
            /* e1, e2, e3, e5 */
            (right_dual.group3.zxyw * this.group1.yzxw) + (this.group2.wwwz * float4(right_dual.group2.xyz, right_dual.e4125)) + float4((this.group0.xyz * right_dual.group3.www), right_dual.e4235 * this.e15) + float4((right_dual.group1.zxy * this.group3.yzx), right_dual.e4315 * this.e25) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - float4((right_dual.group3.yzx * this.group1.zxy), right_dual.e35 * this.e4125)
        );
    }
}
extension DipoleInversion: BulkContraction<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension DipoleInversion: BulkContraction<AntiFlatPoint> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(-(this.e4235 * right_dual.e15) - (this.e4315 * right_dual.e25) - (this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkContraction<AntiFlector> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * right_dual.group1.yzx) - (this.group3.yzx * right_dual.group1.zxy)), this.e1234 * right_dual.e3215), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) + float4((right_dual.group1.www * this.group3.xyz), this.e1234 * right_dual.e45) - (right_dual.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(right_dual.group0.xyz, right_dual.e4315)) + (right_dual.group1.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (this.e35 * right_dual.e4125) - (this.e4315 * right_dual.e25) - (this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45)) - float4((this.group1.zxy * right_dual.group1.yzx), this.e4235 * right_dual.e15)
        );
    }
}
extension DipoleInversion: BulkContraction<AntiLine> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_dual.group1.xyz), -(this.e4235 * right_dual.e415) - (this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)), 
            /* e15, e25, e35, scalar */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435)) - float4((right_dual.group1.xyz.zxy * this.group3.yzx), this.e41 * right_dual.e235)
        );
    }
}
extension DipoleInversion: BulkContraction<AntiMotor> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension DipoleInversion: BulkContraction<AntiPlane> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * right_dual.group0.yzx) - (this.group3.yzx * right_dual.group0.zxy)), this.e1234 * right_dual.e3215), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * right_dual.e3215, this.e4315 * right_dual.e3215, this.e4125 * right_dual.e3215, -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_dual.e4315 * -1.0, this.e23 * right_dual.e4125 * -1.0, this.e31 * right_dual.e4235 * -1.0, (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_dual.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
extension DipoleInversion: BulkContraction<Circle> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group3.yzxy * float4(right_dual.group1.zxy, right_dual.e42)) + float4((right_dual.group2.xyz * this.group2.www), right_dual.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkContraction<CircleRotor> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e41)) + float4((this.group2.www * right_dual.group2.xyz), right_dual.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkContraction<Dipole> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group3.yzx) - (right_dual.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((right_dual.group2.xyz.zxy * this.group3.yzx), right_dual.e423 * this.e15)
        );
    }
}
extension DipoleInversion: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group3.yzx) - (right_dual.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (this.group3.zxyx * float4(right_dual.group2.yzx, right_dual.e1)) + float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((right_dual.group2.zxy * this.group3.yzx), right_dual.e423 * this.e15)
        );
    }
}
extension DipoleInversion: BulkContraction<DualNum> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension DipoleInversion: BulkContraction<FlatPoint> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.e235 * this.e1234, right_dual.e315 * this.e1234, right_dual.e125 * this.e1234, -(right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) - float4((right_dual.group0.www * this.group3.xyz), right_dual.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension DipoleInversion: BulkContraction<Flector> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) + float4((this.group2.www * right_dual.group0.xyz), right_dual.e1 * this.e4235) - float4((right_dual.group0.www * this.group3.xyz), right_dual.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension DipoleInversion: BulkContraction<Line> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group3.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
extension DipoleInversion: BulkContraction<Motor> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) + float4((this.group0.xyz * right_dual.group1.www), right_dual.e3215 * this.e45) - (this.group3.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
extension DipoleInversion: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e1234 * right_dual.e5) + (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435) - (this.e45 * right_dual.e321) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * right_dual.group3) + (this.group3.yzxy * float4(right_dual.group5.xyz.zxy, right_dual.e42)) + float4((this.group0.xyz * right_dual.group9.www), this.e4235 * right_dual.e41) + float4((this.group1.yzx * right_dual.group9.zxy), this.e4125 * right_dual.e43) - (float4(right_dual.e1234) * float4(this.group2.xyz, this.e45)) - (right_dual.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((right_dual.group4.xyz * this.group3.www), this.e41 * right_dual.e4235) - float4((right_dual.group5.xyz.yzx * this.group3.zxy), this.e42 * right_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_dual.e3215) + (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) - (this.e4235 * right_dual.e15) - (this.e4315 * right_dual.e25) - (this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) + (float4(right_dual.e12345) * float4(this.group2.xyz, this.e45)) - float4((right_dual.group8.xyz.zxy * this.group3.yzx), this.e4235 * right_dual.e415), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group6.xyz) + (float3(right_dual.e12345) * this.group0.xyz) + (right_dual.group7.xyz.zxy * this.group3.yzx) - (right_dual.group7.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * right_dual.group8.xyz) + (float3(this.e3215) * right_dual.group7.xyz) + (float3(right_dual.e12345) * this.group1.xyz) - (float3(right_dual.e321) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group9.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group9.zxy, right_dual.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group9.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group3.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * right_dual.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkContraction<Plane> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkContraction<RoundPoint> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group0.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group0.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * right_dual.e3215, this.e4315 * right_dual.e3215, this.e4125 * right_dual.e3215, -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_dual.e4315) - (this.e15 * right_dual.e1234), -(this.e23 * right_dual.e4125) - (this.e25 * right_dual.e1234), -(this.e31 * right_dual.e4235) - (this.e35 * right_dual.e1234), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125)) + (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_dual.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
extension DipoleInversion: BulkContraction<Scalar> {
    associatedtype Output = DipoleInversion;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3
        );
    }
}
extension DipoleInversion: BulkContraction<Sphere> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1234 * right_dual.e5) + (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: BulkContraction<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (this.e4235 * right_dual.e41) + (this.e4315 * right_dual.e42) + (this.e4125 * right_dual.e43) - (this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125) - (this.e45 * right_dual.e1234)) + float4((right_dual.group3.www * this.group3.xyz), this.e1234 * right_dual.e45) - (right_dual.group3.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(right_dual.group2.xyz, right_dual.e4315)) + (right_dual.group3.zxyx * float4(this.group1.yzx, this.e15)) + float4((this.group3.yzx * right_dual.group1.zxy), this.e35 * right_dual.e4125) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (this.group3.zxyz * float4(right_dual.group1.yzx, right_dual.e35)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - float4((this.group1.zxy * right_dual.group3.yzx), this.e4235 * right_dual.e15)
        );
    }
}
extension DipoleInversion: BulkContraction<VersorOdd> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return VersorOdd.from_groups(
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
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<DualNum> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension DualNum: BulkContraction<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension DualNum: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return VersorOdd.from_groups(
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
extension DualNum: BulkContraction<AntiDualNum> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((right_dual.e5 * this.e12345) + (right_dual.e12345 * this.e5), right_dual.e12345 * this.e12345), 0.0, 0.0)
        );
    }
}
extension DualNum: BulkContraction<AntiFlatPoint> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
extension DualNum: BulkContraction<AntiFlector> {
    associatedtype Output = Flector;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
extension DualNum: BulkContraction<AntiLine> {
    associatedtype Output = Line;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_dual.group1.xyz, 0.0)
        );
    }
}
extension DualNum: BulkContraction<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * right_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.yy, this.e12345, (this.e5 * right_dual.e12345) + (this.e12345 * right_dual.e5)) * float4(right_dual.group1.xyz, 1.0)
        );
    }
}
extension DualNum: BulkContraction<AntiPlane> {
    associatedtype Output = Plane;
    func bulk_contraction(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension DualNum: BulkContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: BulkContraction<Circle> {
    associatedtype Output = Dipole;
    func bulk_contraction(other: Circle) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * right_dual.group2.xyz, 0.0)
        );
    }
}
extension DualNum: BulkContraction<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: CircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * right_dual.group2
        );
    }
}
extension DualNum: BulkContraction<Dipole> {
    associatedtype Output = Circle;
    func bulk_contraction(other: Dipole) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_dual.group2.xyz, 0.0)
        );
    }
}
extension DualNum: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * right_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_dual.group3
        );
    }
}
extension DualNum: BulkContraction<DualNum> {
    associatedtype Output = AntiDualNum;
    func bulk_contraction(other: DualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0), 0.0, 0.0)
        );
    }
}
extension DualNum: BulkContraction<FlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func bulk_contraction(other: FlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
extension DualNum: BulkContraction<Flector> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
extension DualNum: BulkContraction<Line> {
    associatedtype Output = AntiLine;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * right_dual.group1.xyz, 0.0)
        );
    }
}
extension DualNum: BulkContraction<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * right_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
extension DualNum: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e5 * right_dual.e1234) + (this.e12345 * right_dual.scalar), this.e12345 * right_dual.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group1, 
            /* e5, 0, 0, 0 */
            float4((this.e5 * right_dual.e12345) + (this.e12345 * right_dual.e5), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_dual.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_dual.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_dual.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_dual.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_dual.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: BulkContraction<Plane> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension DualNum: BulkContraction<RoundPoint> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension DualNum: BulkContraction<Scalar> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: Scalar) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: BulkContraction<Sphere> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_dual.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: BulkContraction<VersorEven> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: VersorEven) -> Output {
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
extension DualNum: BulkContraction<VersorOdd> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: VersorOdd) -> Output {
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
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<FlatPoint> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension FlatPoint: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_dual.e12345 * this.e45), 
            /* e15, e25, e35, scalar */
            float4((right_dual.group2.www * this.group0.xyz), -(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45))
        );
    }
}
extension FlatPoint: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((right_dual.e4235 * this.e15) + (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkContraction<AntiDualNum> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: AntiDualNum) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
extension FlatPoint: BulkContraction<AntiFlector> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkContraction<AntiMotor> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: AntiMotor) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
extension FlatPoint: BulkContraction<AntiPlane> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkContraction<Dipole> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkContraction<DipoleInversion> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkContraction<DualNum> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.e5 * this.e45 * -1.0, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkContraction<FlatPoint> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e45 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkContraction<Flector> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e45 * other.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkContraction<Motor> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: Motor) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(float4(other.group1.xyz, other.e5 * -1.0)[3] * this.e45, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * this.group0, 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension FlatPoint: BulkContraction<RoundPoint> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkContraction<Scalar> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: Scalar) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
extension FlatPoint: BulkContraction<VersorEven> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: BulkContraction<VersorOdd> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45 * right_dual.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual.e12345) * this.group0.xyz), -(this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321))
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<Flector> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension Flector: BulkContraction<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension Flector: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group3.yzx * this.group1.zxy) - (right_dual.group3.zxy * this.group1.yzx)), right_dual.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(right_dual.group3.www, right_dual.e41)) + float4(float3(0.0), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) - float4((this.group1.www * right_dual.group3.xyz), right_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e35 * this.e4125)) + float4((right_dual.group1.zxy * this.group1.yzx), right_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (right_dual.group2.wwwy * float4(this.group0.xyz, this.e4315)) - (this.group1.zxyx * float4(right_dual.group1.yzx, right_dual.e15))
        );
    }
}
extension Flector: BulkContraction<AntiDualNum> {
    associatedtype Output = Flector;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
extension Flector: BulkContraction<AntiFlatPoint> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(-(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension Flector: BulkContraction<AntiFlector> {
    associatedtype Output = Motor;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((right_dual.group1.yzx * this.group1.zxy) - (right_dual.group1.zxy * this.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (right_dual.group1.wwwx * float4(this.group1.xyz, this.e15)) + float4(float3(0.0), (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215)) - (this.group1.wwwx * float4(right_dual.group1.xyz, right_dual.e15))
        );
    }
}
extension Flector: BulkContraction<AntiLine> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) - (this.group1.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e415))
        );
    }
}
extension Flector: BulkContraction<AntiMotor> {
    associatedtype Output = Flector;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) + (float4(right_dual.e12345) * this.group0) - (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
extension Flector: BulkContraction<AntiPlane> {
    associatedtype Output = Motor;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((this.group1.zxy * right_dual.group0.yzx) - (this.group1.yzx * right_dual.group0.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(this.e3215 * right_dual.e4235 * -1.0, this.e3215 * right_dual.e4315 * -1.0, this.e3215 * right_dual.e4125 * -1.0, (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215)) + (right_dual.group0.wwwx * float4(this.group1.xyz, this.e15))
        );
    }
}
extension Flector: BulkContraction<Circle> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: BulkContraction<CircleRotor> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group1.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: BulkContraction<Dipole> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_dual.group0.xyz.zxy * this.group1.yzx) - (right_dual.group0.xyz.yzx * this.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * this.e3215, right_dual.e431 * this.e3215, right_dual.e412 * this.e3215, -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)) - float4((right_dual.group2.xyz.zxy * this.group1.yzx), right_dual.e423 * this.e15)
        );
    }
}
extension Flector: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_dual.group0.xyz.zxy * this.group1.yzx) - (right_dual.group0.xyz.yzx * this.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * this.e3215, right_dual.e431 * this.e3215, right_dual.e412 * this.e3215, -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (this.group1.zxyx * float4(right_dual.group2.yzx, right_dual.e1)) + float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e321 * this.e45)) - float4((right_dual.group2.zxy * this.group1.yzx), right_dual.e423 * this.e15)
        );
    }
}
extension Flector: BulkContraction<DualNum> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), right_dual.e3215 * this.e45)
        );
    }
}
extension Flector: BulkContraction<FlatPoint> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.e321) * float4(this.group1.xyz, this.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group1.zxy) - (right_dual.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
extension Flector: BulkContraction<Flector> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((right_dual.group0.www * this.group1.xyz * float3(-1.0)), (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) - (right_dual.e321 * this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group1.zxy) - (right_dual.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
extension Flector: BulkContraction<Line> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * this.e4315, right_dual.e23 * this.e4125, right_dual.e31 * this.e4235, -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group1.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
extension Flector: BulkContraction<Motor> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) + float4((right_dual.group0.zxy * this.group1.yzx), right_dual.e3215 * this.e45) - (this.group1.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
extension Flector: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(this.e15 * right_dual.e1234) - (this.e4125 * right_dual.e31), -(this.e25 * right_dual.e1234) - (this.e4235 * right_dual.e12), -(this.e35 * right_dual.e1234) - (this.e4315 * right_dual.e23), (this.e4315 * right_dual.e42) + (this.e4125 * right_dual.e43)) + (this.group1.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)) - float4((right_dual.group4.xyz * this.group1.www), this.e45 * right_dual.e1234), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215) - (this.e4235 * right_dual.e15) - (this.e4315 * right_dual.e25) - (this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) + (float4(right_dual.e12345) * this.group0) - float4((right_dual.group8.xyz.zxy * this.group1.yzx), this.e4235 * right_dual.e415), 
            /* e41, e42, e43, 0 */
            float4((right_dual.group7.xyz.zxy * this.group1.yzx) - (right_dual.group7.xyz.yzx * this.group1.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_dual.group7.xyz) - (float3(right_dual.e321) * this.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_dual.group9.yzx) - (this.group1.yzx * right_dual.group9.zxy)), this.e3215 * right_dual.e1234 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group1.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: BulkContraction<Plane> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: BulkContraction<RoundPoint> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_dual.group0.yzx) - (this.group1.yzx * right_dual.group0.zxy)), this.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_dual.e3215) * this.group1.xyz) - (float3(this.e3215) * right_dual.group0.xyz)), this.e45 * right_dual.e1234 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(right_dual.e1234) * this.group0.xyz * float3(-1.0)), (this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215))
        );
    }
}
extension Flector: BulkContraction<Scalar> {
    associatedtype Output = Flector;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
extension Flector: BulkContraction<Sphere> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: BulkContraction<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_dual.group3.yzx) - (this.group1.yzx * right_dual.group3.zxy)), this.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(right_dual.group3.www, right_dual.e41)) + float4(float3(0.0), (this.e4315 * right_dual.e42) + (this.e4125 * right_dual.e43)) - float4((this.group1.www * right_dual.group3.xyz), this.e45 * right_dual.e1234), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215) - (this.e3215 * right_dual.e45)) + float4((this.group1.yzx * right_dual.group1.zxy), this.e15 * right_dual.e4235) - (this.group1.zxyy * float4(right_dual.group1.yzx, right_dual.e25)) - (this.group1.wwwz * float4(right_dual.group0.xyz, right_dual.e35)) - (right_dual.group2.wwwx * float4(this.group0.xyz, this.e4235))
        );
    }
}
extension Flector: BulkContraction<VersorOdd> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return VersorOdd.from_groups(
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
extension Line {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<Line> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension Line: BulkContraction<AntiCircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group1.xyz), -(right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e412 * this.e315) + (right_dual.e321 * this.e415), (right_dual.e423 * this.e125) + (right_dual.e321 * this.e425), (right_dual.e431 * this.e235) + (right_dual.e321 * this.e435), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_dual.e235 * this.e415)
        );
    }
}
extension Line: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual.e1234) * this.group1.xyz), -(right_dual.e4235 * this.e415) - (right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)), 
            /* e15, e25, e35, scalar */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)) - float4((this.group1.xyz.zxy * right_dual.group3.yzx), right_dual.e41 * this.e235)
        );
    }
}
extension Line: BulkContraction<AntiDualNum> {
    associatedtype Output = Line;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: BulkContraction<AntiFlector> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) - (right_dual.group1.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
extension Line: BulkContraction<AntiLine> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(-(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0), 0.0, 0.0)
        );
    }
}
extension Line: BulkContraction<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(right_dual.e12345) * this.group1.xyz), -(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435))
        );
    }
}
extension Line: BulkContraction<AntiPlane> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
extension Line: BulkContraction<Circle> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: BulkContraction<CircleRotor> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: BulkContraction<Dipole> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e321 * this.e415), (right_dual.e423 * this.e125) + (right_dual.e321 * this.e425), (right_dual.e431 * this.e235) + (right_dual.e321 * this.e435), -(right_dual.e431 * this.e425) - (right_dual.e412 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_dual.e423 * this.e415), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: BulkContraction<DipoleInversion> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e321 * this.e415), (right_dual.e423 * this.e125) + (right_dual.e321 * this.e425), (right_dual.e431 * this.e235) + (right_dual.e321 * this.e435), -(right_dual.e431 * this.e425) - (right_dual.e412 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_dual.e423 * this.e415), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: BulkContraction<DualNum> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: BulkContraction<FlatPoint> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(right_dual.e321) * this.group0.xyz), -(right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435))
        );
    }
}
extension Line: BulkContraction<Flector> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(right_dual.e321) * this.group0.xyz), -(right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435))
        );
    }
}
extension Line: BulkContraction<Line> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: BulkContraction<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_dual.e321) + (this.e315 * right_dual.e412), (this.e425 * right_dual.e321) + (this.e125 * right_dual.e423), (this.e435 * right_dual.e321) + (this.e235 * right_dual.e431), -(this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) - float4((this.group1.xyz.zxy * right_dual.group7.xyz.yzx), this.e415 * right_dual.e423), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group9.yzxx * float4(this.group1.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e1234) * this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.group0.xy.yy, right_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e12345) * this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Line: BulkContraction<RoundPoint> {
    associatedtype Output = Dipole;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual.e1234) * this.group1.xyz), -(this.e415 * right_dual.e4235) - (this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)), 
            /* e15, e25, e35, 0 */
            float4((float3(right_dual.e3215) * this.group0.xyz) + (this.group1.xyz.yzx * right_dual.group0.zxy) - (this.group1.xyz.zxy * right_dual.group0.yzx), 0.0)
        );
    }
}
extension Line: BulkContraction<Scalar> {
    associatedtype Output = Line;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_dual.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: BulkContraction<VersorEven> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual.e1234) * this.group1.xyz), -(this.e415 * right_dual.e4235) - (this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43)) - float4((this.group1.xyz.zxy * right_dual.group3.yzx), this.e415 * right_dual.e23)
        );
    }
}
extension Line: BulkContraction<VersorOdd> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual.e12345) * this.group1.xyz), -(this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_dual.e321) + (this.e315 * right_dual.e412), (this.e425 * right_dual.e321) + (this.e125 * right_dual.e423), (this.e435 * right_dual.e321) + (this.e235 * right_dual.e431), -(this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435)) - float4((this.group1.xyz.zxy * right_dual.group0.yzx), this.e415 * right_dual.e235)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<Motor> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension Motor: BulkContraction<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension Motor: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return VersorOdd.from_groups(
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
extension Motor: BulkContraction<AntiDualNum> {
    associatedtype Output = Motor;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_dual.group0.xy.yy, right_dual.e12345, (right_dual.e5 * this.e12345) + (right_dual.e12345 * this.e5)) * float4(this.group1.xyz, 1.0)
        );
    }
}
extension Motor: BulkContraction<AntiFlatPoint> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
extension Motor: BulkContraction<AntiFlector> {
    associatedtype Output = Flector;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(this.e12345) * right_dual.group0) - (right_dual.group1.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group1
        );
    }
}
extension Motor: BulkContraction<AntiLine> {
    associatedtype Output = Motor;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e12345) * right_dual.group1.xyz), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435))
        );
    }
}
extension Motor: BulkContraction<AntiMotor> {
    associatedtype Output = Motor;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((float3(right_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_dual.group0.xyz)), right_dual.e12345 * this.e12345), 
            /* e235, e315, e125, e5 */
            (float4(right_dual.e12345) * this.group1) + (float4(this.e12345) * right_dual.group1) + float4(float3(0.0), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435))
        );
    }
}
extension Motor: BulkContraction<AntiPlane> {
    associatedtype Output = Flector;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0
        );
    }
}
extension Motor: BulkContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: BulkContraction<Circle> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435))
        );
    }
}
extension Motor: BulkContraction<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: CircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435))
        );
    }
}
extension Motor: BulkContraction<Dipole> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e412 * this.e315) + (right_dual.e321 * this.e415), (right_dual.e423 * this.e125) + (right_dual.e321 * this.e425), (right_dual.e431 * this.e235) + (right_dual.e321 * this.e435), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group1.zxy), right_dual.e235 * this.e415)
        );
    }
}
extension Motor: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.e4 * this.e12345) - (right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e321 * this.e415) + (right_dual.e1 * this.e12345), (right_dual.e321 * this.e425) + (right_dual.e2 * this.e12345), (right_dual.e321 * this.e435) + (right_dual.e3 * this.e12345), -(right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + float4((right_dual.group0.xyz.zxy * this.group1.yzx), right_dual.e5 * this.e12345) - (this.group1.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e415))
        );
    }
}
extension Motor: BulkContraction<DualNum> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_dual.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e3215) * this.group0
        );
    }
}
extension Motor: BulkContraction<FlatPoint> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual.group0.www * this.group0.xyz), -(right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435))
        );
    }
}
extension Motor: BulkContraction<Flector> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual.e1 * this.e12345, right_dual.e2 * this.e12345, right_dual.e3 * this.e12345, -(right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + (this.group0 * float4(right_dual.group0.www, right_dual.e5))
        );
    }
}
extension Motor: BulkContraction<Line> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_dual.group0.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: BulkContraction<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_dual.group0.xyz), (right_dual.scalar * this.e12345) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual.e3215) * this.group0.xyz) + (float3(this.e12345) * right_dual.group1.xyz)), right_dual.e3215 * this.e12345)
        );
    }
}
extension Motor: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e12345 * right_dual.scalar) + (this.e5 * right_dual.e1234) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), this.e12345 * right_dual.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_dual.e321) + (this.e12345 * right_dual.e1), (this.e425 * right_dual.e321) + (this.e12345 * right_dual.e2), (this.e435 * right_dual.e321) + (this.e12345 * right_dual.e3), -(this.e425 * right_dual.e431) - (this.e435 * right_dual.e412)) + float4((right_dual.group7.xyz.zxy * this.group1.yzx), this.e12345 * right_dual.e4) - float4((right_dual.group7.xyz.yzx * this.group1.zxy), this.e415 * right_dual.e423), 
            /* e5, 0, 0, 0 */
            float4((this.e12345 * right_dual.e5) + (this.e5 * right_dual.e12345) - (this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e12345 * right_dual.e15) + (this.e315 * right_dual.e4125), (this.e12345 * right_dual.e25) + (this.e125 * right_dual.e4235), (this.e12345 * right_dual.e35) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) + (this.group0 * float4(right_dual.group9.www, right_dual.e45)) - (right_dual.group9.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e12345) * right_dual.group4.xyz) + (float3(right_dual.e1234) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e12345) * right_dual.group5.xyz) + (float3(right_dual.e1234) * this.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * right_dual.group6.xyz) + (float3(right_dual.e12345) * this.group0.xyz)), this.e12345 * right_dual.e321), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_dual.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e12345) * right_dual.group8.xyz) + (float3(right_dual.e12345) * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: BulkContraction<Plane> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension Motor: BulkContraction<RoundPoint> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension Motor: BulkContraction<Scalar> {
    associatedtype Output = Motor;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_dual.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_dual.e12345) * this.group1
        );
    }
}
extension Motor: BulkContraction<Sphere> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_dual.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: BulkContraction<VersorEven> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: VersorEven) -> Output {
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
extension Motor: BulkContraction<VersorOdd> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: VersorOdd) -> Output {
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
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<MultiVector> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension MultiVector: BulkContraction<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
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
            float4(float2((right_dual.e12345 * this.scalar) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), right_dual.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412) + (right_dual.e12345 * this.e1), (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423) + (right_dual.e12345 * this.e2), (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431) + (right_dual.e12345 * this.e3), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) + float4((right_dual.group0.xyz.zxy * this.group8.xyz.yzx), right_dual.e12345 * this.e4) - float4((right_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_dual.e423 * this.e415) - float4((this.group7.xyz.yzx * right_dual.group2.zxy), right_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4((right_dual.e12345 * this.e5) - (right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e315 * this.e4125) + (right_dual.e12345 * this.e15), (right_dual.e125 * this.e4235) + (right_dual.e12345 * this.e25), (right_dual.e235 * this.e4315) + (right_dual.e12345 * this.e35), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) + float4((this.group9.www * right_dual.group1.xyz), right_dual.e12345 * this.e45) - (this.group9.yzxx * float4(right_dual.group2.zxy, right_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e12345) * this.group4.xyz) + (float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group9.yzx) - (right_dual.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e12345) * this.group5.xyz) + (float3(this.e3215) * right_dual.group0.xyz) + (float3(this.e1234) * right_dual.group2.xyz) - (float3(right_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(right_dual.e12345) * this.group6) + (float4(this.e12345) * right_dual.group1), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e12345) * this.group7.xyz) + (float3(this.e12345) * right_dual.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e12345) * this.group8.xyz) + (float3(this.e12345) * right_dual.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
            float4(float2((right_dual.e1234 * this.e5) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_dual.group2.xyz, right_dual.e45)) + (this.group9.yzxz * float4(right_dual.group1.zxy, right_dual.e43)) + float4((this.group4.xyz * right_dual.group3.www), right_dual.e41 * this.e4235) + float4((this.group5.xyz.yzx * right_dual.group3.zxy), right_dual.e42 * this.e4315) - (float4(right_dual.e1234) * this.group3) - (right_dual.group3.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((right_dual.group0.xyz * this.group9.www), right_dual.e4235 * this.e41) - float4((right_dual.group1.yzx * this.group9.zxy), right_dual.e4125 * this.e43), 
            /* e5, 0, 0, 0 */
            float4((right_dual.e4235 * this.e15) + (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(this.e12345) * float4(right_dual.group2.xyz, right_dual.e45)) - float4((this.group8.xyz.zxy * right_dual.group3.yzx), right_dual.e4235 * this.e415), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group6.xyz) + (float3(this.e12345) * right_dual.group0.xyz) + (this.group7.xyz.zxy * right_dual.group3.yzx) - (this.group7.xyz.yzx * right_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e1234) * this.group8.xyz) + (float3(right_dual.e3215) * this.group7.xyz) + (float3(this.e12345) * right_dual.group1.xyz) - (float3(this.e321) * right_dual.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_dual.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e1234 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkContraction<AntiDualNum> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_dual.e5 * this.e1234) + (right_dual.e12345 * this.scalar), right_dual.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group1, 
            /* e5, 0, 0, 0 */
            float4((right_dual.e5 * this.e12345) + (right_dual.e12345 * this.e5), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e12345) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e12345) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkContraction<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_dual.group0, 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<AntiFlector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e15 * this.e1234) + (right_dual.e4125 * this.e31), (right_dual.e25 * this.e1234) + (right_dual.e4235 * this.e12), (right_dual.e35 * this.e1234) + (right_dual.e4315 * this.e23), -(right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) + float4((this.group4.xyz * right_dual.group1.www), right_dual.e45 * this.e1234) - (right_dual.group1.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((right_dual.e4235 * this.e15) + (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(this.e12345) * right_dual.group0) - float4((this.group8.xyz.zxy * right_dual.group1.yzx), right_dual.e4235 * this.e415), 
            /* e41, e42, e43, 0 */
            float4((this.group7.xyz.zxy * right_dual.group1.yzx) - (this.group7.xyz.yzx * right_dual.group1.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_dual.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group1.yzx * this.group9.zxy) - (right_dual.group1.zxy * this.group9.yzx)), right_dual.e3215 * this.e1234), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_dual.group1.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_dual.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<AntiLine> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e415 * this.e321) + (right_dual.e315 * this.e412), (right_dual.e425 * this.e321) + (right_dual.e125 * this.e423), (right_dual.e435 * this.e321) + (right_dual.e235 * this.e431), -(right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) - float4((right_dual.group1.xyz.zxy * this.group7.xyz.yzx), right_dual.e415 * this.e423), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * right_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_dual.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<AntiMotor> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_dual.e12345 * this.scalar) + (right_dual.e5 * this.e1234) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), right_dual.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e415 * this.e321) + (right_dual.e12345 * this.e1), (right_dual.e425 * this.e321) + (right_dual.e12345 * this.e2), (right_dual.e435 * this.e321) + (right_dual.e12345 * this.e3), -(right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) + float4((this.group7.xyz.zxy * right_dual.group1.yzx), right_dual.e12345 * this.e4) - float4((this.group7.xyz.yzx * right_dual.group1.zxy), right_dual.e415 * this.e423), 
            /* e5, 0, 0, 0 */
            float4((right_dual.e12345 * this.e5) + (right_dual.e5 * this.e12345) - (right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e12345 * this.e15) + (right_dual.e315 * this.e4125), (right_dual.e12345 * this.e25) + (right_dual.e125 * this.e4235), (right_dual.e12345 * this.e35) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) + (right_dual.group0 * float4(this.group9.www, this.e45)) - (this.group9.yzxx * float4(right_dual.group1.zxy, right_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e12345) * this.group4.xyz) + (float3(this.e1234) * right_dual.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e12345) * this.group5.xyz) + (float3(this.e1234) * right_dual.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual.e12345) * this.group6.xyz) + (float3(this.e12345) * right_dual.group0.xyz)), right_dual.e12345 * this.e321), 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e12345) * this.group8.xyz) + (float3(this.e12345) * right_dual.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkContraction<AntiPlane> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e42 * right_dual.e4315) - (this.e43 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((this.group7.xyz.zxy * right_dual.group0.yzx) - (this.group7.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_dual.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group9.zxy * right_dual.group0.yzx) - (this.group9.yzx * right_dual.group0.zxy)), this.e1234 * right_dual.e3215), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_dual.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkContraction<Circle> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: Circle) -> Output {
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
            float4(float2(-(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group9.yzxy * float4(right_dual.group1.zxy, right_dual.e42)) + float4((float3(this.e1234) * right_dual.group2.xyz), right_dual.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(right_dual.group2.xyz, right_dual.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<CircleRotor> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: CircleRotor) -> Output {
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
            float4(float2((right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group9.yzxx * float4(right_dual.group1.zxy, right_dual.e41)) + float4((float3(this.e1234) * right_dual.group2.xyz), right_dual.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(right_dual.group2.xyz, right_dual.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<Dipole> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: Dipole) -> Output {
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
            float4(float2(-(right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) - float4((right_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_dual.e423 * this.e415) - float4((right_dual.group2.xyz.zxy * this.group7.xyz.yzx), right_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_dual.group2.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group9.yzx) - (right_dual.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_dual.group0.xyz) + (float3(this.e1234) * right_dual.group2.xyz) - (float3(right_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_dual.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<DipoleInversion> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
            float4(float2((right_dual.e4 * this.e3215) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)) + (float4(this.e12345) * float4(right_dual.group3.xyz, right_dual.e4)) - float4((right_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_dual.e423 * this.e415) - float4((this.group7.xyz.yzx * right_dual.group2.zxy), right_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4((right_dual.e5 * this.e12345) - (right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_dual.group2.zxy, right_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group9.yzx) - (right_dual.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_dual.group0.xyz) + (float3(this.e1234) * right_dual.group2.xyz) - (float3(right_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_dual.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<DualNum> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_dual.e3215 * this.e4) + (right_dual.scalar * this.e12345), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(right_dual.e3215 * this.e45, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e3215) * this.group7.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.e3215 * this.e1234), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e3215) * this.group9.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual.e3215 * this.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<FlatPoint> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual.e321) * this.group6.xyz), 0.0) + float4((this.group7.xyz.zxy * right_dual.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((right_dual.group0.yzx * this.group9.zxy) - (right_dual.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.e321 * this.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<Flector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual.e321) * this.group6.xyz), 0.0) + float4((float3(this.e12345) * right_dual.group1.xyz), 0.0) + float4((this.group7.xyz.zxy * right_dual.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4((right_dual.e5 * this.e12345) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((right_dual.group0.yzx * this.group9.zxy) - (right_dual.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.e321 * this.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<Line> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e1234) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.xyz.zxy * this.group9.yzx), 0.0) - float4((right_dual.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<Motor> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_dual.scalar * this.e12345) + (right_dual.e3215 * this.e4) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual.e3215) * this.group4.xyz), 0.0) + float4((float3(this.e1234) * right_dual.group1.xyz), 0.0) + float4((right_dual.group0.zxy * this.group9.yzx), 0.0) - float4((right_dual.group0.yzx * this.group9.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4((right_dual.e3215 * this.e45) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual.e3215) * this.group6.xyz) + (float3(this.e12345) * right_dual.group1.xyz)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e3215) * this.group7.xyz) + (float3(this.e12345) * right_dual.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual.e3215 * this.e1234), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e3215) * this.group9.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual.e3215 * this.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_dual.scalar * this.e12345) + (right_dual.e12345 * this.scalar) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), right_dual.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e15 * this.e1234) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e4125 * this.e31), (right_dual.e25 * this.e1234) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e4235 * this.e12), (right_dual.e35 * this.e1234) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e4315 * this.e23), -(right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412) - (right_dual.e1234 * this.e45)) + (float4(right_dual.e12345) * this.group1) + (float4(this.e12345) * right_dual.group1) + (this.group9.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)) + float4((this.group4.xyz * right_dual.group9.www), right_dual.e45 * this.e1234) + float4((right_dual.group7.xyz.zxy * this.group8.xyz.yzx), right_dual.e42 * this.e4315) + float4((right_dual.group8.xyz.yzx * this.group7.xyz.zxy), right_dual.e43 * this.e4125) - (right_dual.group9.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((float3(right_dual.e1234) * this.group3.xyz), right_dual.e4125 * this.e43) - float4((right_dual.group4.xyz * this.group9.www), right_dual.e423 * this.e415) - float4((right_dual.group5.xyz.yzx * this.group9.zxy), right_dual.e431 * this.e425) - float4((right_dual.group7.xyz.yzx * this.group8.xyz.zxy), right_dual.e412 * this.e435) - float4((right_dual.group8.xyz.zxy * this.group7.xyz.yzx), right_dual.e4235 * this.e41), 
            /* e5, 0, 0, 0 */
            float4((right_dual.e12345 * this.e5) + (right_dual.e5 * this.e12345) + (right_dual.e4235 * this.e15) + (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215) - (right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125) + (right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235) + (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315) + (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e435 * this.e4125) - (right_dual.e4235 * this.e415) - (right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(right_dual.e12345) * this.group3) + (float4(this.e12345) * right_dual.group3) - (this.group9.yzxx * float4(right_dual.group8.xyz.zxy, right_dual.e415)) - float4((this.group8.xyz.zxy * right_dual.group9.yzx), right_dual.e425 * this.e4315), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e12345) * this.group4.xyz) + (float3(right_dual.e1234) * this.group6.xyz) + (float3(this.e12345) * right_dual.group4.xyz) + (float3(this.e1234) * right_dual.group6.xyz) + (right_dual.group7.xyz.zxy * this.group9.yzx) + (this.group7.xyz.zxy * right_dual.group9.yzx) - (right_dual.group7.xyz.yzx * this.group9.zxy) - (this.group7.xyz.yzx * right_dual.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e12345) * this.group5.xyz) + (float3(right_dual.e3215) * this.group7.xyz) + (float3(right_dual.e1234) * this.group8.xyz) + (float3(this.e12345) * right_dual.group5.xyz) + (float3(this.e3215) * right_dual.group7.xyz) + (float3(this.e1234) * right_dual.group8.xyz) - (float3(right_dual.e321) * this.group9.xyz) - (float3(this.e321) * right_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(right_dual.e12345) * this.group6) + (float4(this.e12345) * right_dual.group6) + (right_dual.group9.yzxw * float4(this.group9.zxy, this.e1234)) - float4((right_dual.group9.zxy * this.group9.yzx), right_dual.e1234 * this.e3215), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e12345) * this.group7.xyz) + (float3(this.e12345) * right_dual.group7.xyz) + (float3(this.e1234) * right_dual.group9.xyz) - (float3(right_dual.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e12345) * this.group8.xyz) + (float3(right_dual.e3215) * this.group9.xyz) + (float3(this.e12345) * right_dual.group8.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual.e12345) * this.group9) + (float4(this.e12345) * right_dual.group9), 
            /* e1234, 0, 0, 0 */
            float4((right_dual.e12345 * this.e1234) + (right_dual.e1234 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkContraction<Plane> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(right_dual.e5 * this.e12345, 0.0, 0.0, 0.0), 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<RoundPoint> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_dual.e3215) + (this.e31 * right_dual.e4125), (this.e42 * right_dual.e3215) + (this.e12 * right_dual.e4235), (this.e43 * right_dual.e3215) + (this.e23 * right_dual.e4315), -(this.e45 * right_dual.e1234) - (this.e43 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((float3(right_dual.e1234) * this.group3.xyz), this.e42 * right_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) - (right_dual.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * right_dual.group0.yzx) - (this.group7.xyz.yzx * right_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e3215) * this.group7.xyz) + (float3(right_dual.e1234) * this.group8.xyz) - (float3(this.e321) * right_dual.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * right_dual.group0.yzx), this.e1234 * right_dual.e3215) - (this.group9.yzxw * float4(right_dual.group0.zxy, right_dual.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_dual.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkContraction<Scalar> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_dual.e12345) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(right_dual.e12345 * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_dual.e12345) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(right_dual.e12345) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e12345) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual.e12345) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e12345) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_dual.e12345) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkContraction<Sphere> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) + (this.e1234 * right_dual.e5), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_dual.e5, 0.0, 0.0, 0.0), 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: BulkContraction<VersorEven> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: VersorEven) -> Output {
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
            float4(float2((this.e12345 * right_dual.scalar) + (this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234) - (this.e415 * right_dual.e23) - (this.e425 * right_dual.e31) - (this.e435 * right_dual.e12) - (this.e321 * right_dual.e45) - (this.e423 * right_dual.e15) - (this.e431 * right_dual.e25) - (this.e412 * right_dual.e35) - (this.e235 * right_dual.e41) - (this.e315 * right_dual.e42) - (this.e125 * right_dual.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_dual.group2.xyz, right_dual.e45)) + (this.group9.yzxz * float4(right_dual.group1.zxy, right_dual.e43)) + float4((this.group4.xyz * right_dual.group3.www), this.e4235 * right_dual.e41) + float4((this.group5.xyz.yzx * right_dual.group3.zxy), this.e4315 * right_dual.e42) - (right_dual.group3.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((this.group9.zxy * right_dual.group1.yzx), this.e43 * right_dual.e4125) - float4((this.group9.www * right_dual.group0.xyz), this.e45 * right_dual.e1234) - float4((right_dual.group2.www * this.group3.xyz), this.e42 * right_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_dual.e4235) + (this.e25 * right_dual.e4315) + (this.e35 * right_dual.e4125) + (this.e45 * right_dual.e3215) - (this.e4235 * right_dual.e15) - (this.e4315 * right_dual.e25) - (this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_dual.e3215) + (this.e315 * right_dual.e4125), (this.e425 * right_dual.e3215) + (this.e125 * right_dual.e4235), (this.e435 * right_dual.e3215) + (this.e235 * right_dual.e4315), -(this.e425 * right_dual.e4315) - (this.e435 * right_dual.e4125)) + (float4(this.e12345) * float4(right_dual.group2.xyz, right_dual.e45)) - (right_dual.group3.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e12345) * right_dual.group0.xyz) + (float3(right_dual.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * right_dual.group3.yzx) - (this.group7.xyz.yzx * right_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e12345) * right_dual.group1.xyz) + (float3(right_dual.e1234) * this.group8.xyz) + (float3(right_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_dual.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * right_dual.group3.yzx), this.e1234 * right_dual.e3215) - (this.group9.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_dual.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: BulkContraction<VersorOdd> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: VersorOdd) -> Output {
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
            float4(float2((this.scalar * right_dual.e12345) + (this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4) + (this.e1234 * right_dual.e5) - (this.e15 * right_dual.e423) - (this.e25 * right_dual.e431) - (this.e35 * right_dual.e412) - (this.e45 * right_dual.e321) - (this.e41 * right_dual.e235) - (this.e42 * right_dual.e315) - (this.e43 * right_dual.e125) - (this.e23 * right_dual.e415) - (this.e31 * right_dual.e425) - (this.e12 * right_dual.e435), this.e12345 * right_dual.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e1 * right_dual.e12345) + (this.e415 * right_dual.e321) + (this.e321 * right_dual.e415) + (this.e315 * right_dual.e412), (this.e2 * right_dual.e12345) + (this.e425 * right_dual.e321) + (this.e321 * right_dual.e425) + (this.e125 * right_dual.e423), (this.e3 * right_dual.e12345) + (this.e435 * right_dual.e321) + (this.e321 * right_dual.e435) + (this.e235 * right_dual.e431), -(this.e415 * right_dual.e423) - (this.e425 * right_dual.e431) - (this.e435 * right_dual.e412) - (this.e412 * right_dual.e435)) + (float4(this.e12345) * right_dual.group3) + float4((this.group7.xyz.zxy * right_dual.group2.yzx), this.e4 * right_dual.e12345) - float4((this.group7.xyz.yzx * right_dual.group2.zxy), this.e423 * right_dual.e415) - float4((this.group8.xyz.zxy * right_dual.group0.yzx), this.e431 * right_dual.e425), 
            /* e5, 0, 0, 0 */
            float4((this.e12345 * right_dual.e5) + (this.e5 * right_dual.e12345) - (this.e415 * right_dual.e235) - (this.e425 * right_dual.e315) - (this.e435 * right_dual.e125) - (this.e235 * right_dual.e415) - (this.e315 * right_dual.e425) - (this.e125 * right_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_dual.e315) + (this.e3215 * right_dual.e415), (this.e4235 * right_dual.e125) + (this.e3215 * right_dual.e425), (this.e4315 * right_dual.e235) + (this.e3215 * right_dual.e435), -(this.e4315 * right_dual.e425) - (this.e4125 * right_dual.e435)) + (float4(right_dual.e12345) * this.group3) - (this.group9.yzxx * float4(right_dual.group2.zxy, right_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group1.xyz) + (float3(right_dual.e12345) * this.group4.xyz) + (this.group9.yzx * right_dual.group0.zxy) - (this.group9.zxy * right_dual.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_dual.group0.xyz) + (float3(this.e1234) * right_dual.group2.xyz) + (float3(right_dual.e12345) * this.group5.xyz) - (float3(right_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * right_dual.group1) + (float4(right_dual.e12345) * this.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e12345) * right_dual.group0.xyz) + (float3(right_dual.e12345) * this.group7.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e12345) * right_dual.group2.xyz) + (float3(right_dual.e12345) * this.group8.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * right_dual.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<Plane> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension Plane: BulkContraction<AntiCircleRotor> {
    associatedtype Output = DipoleInversion;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * this.e3215, right_dual.e431 * this.e3215, right_dual.e412 * this.e3215, -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * right_dual.group1.xyz), 0.0) + float4((right_dual.group2.yzx * this.group0.zxy), 0.0) - float4((right_dual.group2.zxy * this.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0
        );
    }
}
extension Plane: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group3.yzx * this.group0.zxy) - (right_dual.group3.zxy * this.group0.yzx)), right_dual.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(right_dual.e4235 * this.e3215 * -1.0, right_dual.e4315 * this.e3215 * -1.0, right_dual.e4125 * this.e3215 * -1.0, (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * this.e4315, right_dual.e23 * this.e4125, right_dual.e31 * this.e4235, -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (this.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15))
        );
    }
}
extension Plane: BulkContraction<AntiDualNum> {
    associatedtype Output = Plane;
    func bulk_contraction(other: AntiDualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: BulkContraction<AntiFlatPoint> {
    associatedtype Output = DualNum;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(-(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension Plane: BulkContraction<AntiFlector> {
    associatedtype Output = Motor;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((right_dual.group1.yzx * this.group0.zxy) - (right_dual.group1.zxy * this.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e3215 * this.e4235, right_dual.e3215 * this.e4315, right_dual.e3215 * this.e4125, -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215)) - (this.group0.wwwx * float4(right_dual.group1.xyz, right_dual.e15))
        );
    }
}
extension Plane: BulkContraction<AntiLine> {
    associatedtype Output = FlatPoint;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e415))
        );
    }
}
extension Plane: BulkContraction<AntiMotor> {
    associatedtype Output = Flector;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0
        );
    }
}
extension Plane: BulkContraction<AntiPlane> {
    associatedtype Output = Line;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group0.xyz), 0.0)
        );
    }
}
extension Plane: BulkContraction<Circle> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: BulkContraction<CircleRotor> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: BulkContraction<Dipole> {
    associatedtype Output = Dipole;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * this.e3215, right_dual.e431 * this.e3215, right_dual.e412 * this.e3215, -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(this.e3215) * right_dual.group1.xyz) + (right_dual.group2.xyz.yzx * this.group0.zxy) - (right_dual.group2.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Plane: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * this.e3215, right_dual.e431 * this.e3215, right_dual.e412 * this.e3215, -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(right_dual.e125 * this.e4315 * -1.0, right_dual.e235 * this.e4125 * -1.0, right_dual.e315 * this.e4235 * -1.0, (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125)) + (float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (this.group0.zxyx * float4(right_dual.group2.yzx, right_dual.e1))
        );
    }
}
extension Plane: BulkContraction<DualNum> {
    associatedtype Output = AntiFlatPoint;
    func bulk_contraction(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Plane: BulkContraction<FlatPoint> {
    associatedtype Output = AntiLine;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e321) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Plane: BulkContraction<Flector> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((right_dual.group0.www * this.group0.xyz * float3(-1.0)), (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Plane: BulkContraction<Line> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * this.e4315, right_dual.e23 * this.e4125, right_dual.e31 * this.e4235, -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
extension Plane: BulkContraction<Motor> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * this.e4315, right_dual.e23 * this.e4125, right_dual.e31 * this.e4235, -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
extension Plane: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_dual.group8.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((right_dual.group7.xyz.zxy * this.group0.yzx) - (right_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_dual.group7.xyz) - (float3(right_dual.e321) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group9.yzx * this.group0.zxy) - (right_dual.group9.zxy * this.group0.yzx)), right_dual.e1234 * this.e3215 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane: BulkContraction<Plane> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: BulkContraction<RoundPoint> {
    associatedtype Output = Circle;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * right_dual.group0.yzx) - (this.group0.yzx * right_dual.group0.zxy)), this.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group0.xyz), 0.0)
        );
    }
}
extension Plane: BulkContraction<Scalar> {
    associatedtype Output = Plane;
    func bulk_contraction(other: Scalar) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: BulkContraction<Sphere> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e4235 * right_dual.e1) + (this.e4315 * right_dual.e2) + (this.e4125 * right_dual.e3) + (this.e3215 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: BulkContraction<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * right_dual.group3.yzx) - (this.group0.yzx * right_dual.group3.zxy)), this.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * right_dual.e4235 * -1.0, this.e3215 * right_dual.e4315 * -1.0, this.e3215 * right_dual.e4125 * -1.0, (this.e4315 * right_dual.e42) + (this.e4125 * right_dual.e43)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e4315 * right_dual.e12, this.e4125 * right_dual.e23, this.e4235 * right_dual.e31, -(this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45)) - (this.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - (this.group0.wwwy * float4(right_dual.group0.xyz, right_dual.e25))
        );
    }
}
extension Plane: BulkContraction<VersorOdd> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return VersorOdd.from_groups(
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
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<RoundPoint> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension RoundPoint: BulkContraction<AntiCircleRotor> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_dual.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e1234 * this.e5) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkContraction<AntiDualNum> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_dual.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkContraction<AntiFlector> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkContraction<AntiMotor> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_dual.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkContraction<AntiPlane> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkContraction<DualNum> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e5 * this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkContraction<Motor> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(float4(other.group1.xyz, other.e5 * -1.0)[3] * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_dual.e12345 * this.e5, 0.0, 0.0, 0.0), 
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
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension RoundPoint: BulkContraction<RoundPoint> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkContraction<Scalar> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_dual.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkContraction<VersorEven> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_dual.e4235) + (this.e2 * right_dual.e4315) + (this.e3 * right_dual.e4125) + (this.e4 * right_dual.e3215) + (this.e5 * right_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: BulkContraction<VersorOdd> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_dual.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<Scalar> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension Scalar: BulkContraction<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: BulkContraction<AntiDualNum> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiDualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: BulkContraction<AntiMotor> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: BulkContraction<MultiVector> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: BulkContraction<Scalar> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: BulkContraction<VersorOdd> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<Sphere> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension Sphere: BulkContraction<AntiCircleRotor> {
    associatedtype Output = DipoleInversion;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_dual.group1.xyz) + (right_dual.group2.yzx * this.group0.zxy) - (right_dual.group2.zxy * this.group0.yzx)), right_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0
        );
    }
}
extension Sphere: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(right_dual.e4235 * this.e3215 * -1.0, right_dual.e4315 * this.e3215 * -1.0, right_dual.e4125 * this.e3215 * -1.0, (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (this.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15))
        );
    }
}
extension Sphere: BulkContraction<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension Sphere: BulkContraction<AntiFlatPoint> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkContraction<AntiFlector> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group1.yzx * this.group0.zxy) - (right_dual.group1.zxy * this.group0.yzx)), right_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group1.xyz)), right_dual.e45 * this.e1234), 
            /* e1, e2, e3, e5 */
            float4((float3(this.e1234) * right_dual.group0.xyz), -(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215))
        );
    }
}
extension Sphere: BulkContraction<AntiLine> {
    associatedtype Output = Dipole;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_dual.group1.xyz), -(right_dual.e415 * this.e4235) - (right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)), 
            /* e15, e25, e35, 0 */
            float4((float3(this.e3215) * right_dual.group0.xyz) + (right_dual.group1.xyz.yzx * this.group0.zxy) - (right_dual.group1.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Sphere: BulkContraction<AntiMotor> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension Sphere: BulkContraction<AntiPlane> {
    associatedtype Output = Circle;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx)), right_dual.e3215 * this.e1234), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group0.xyz), 0.0)
        );
    }
}
extension Sphere: BulkContraction<Circle> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group0.yzxy * float4(right_dual.group1.zxy, right_dual.e42)) + float4((float3(this.e1234) * right_dual.group2.xyz), right_dual.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkContraction<CircleRotor> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)) + float4((float3(this.e1234) * right_dual.group2.xyz), right_dual.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkContraction<Dipole> {
    associatedtype Output = Dipole;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(this.e3215) * right_dual.group1.xyz) + (right_dual.group2.xyz.yzx * this.group0.zxy) - (right_dual.group2.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Sphere: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group0.yzx) - (right_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(right_dual.e125 * this.e4315 * -1.0, right_dual.e235 * this.e4125 * -1.0, right_dual.e315 * this.e4235 * -1.0, (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)) + (float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (this.group0.zxyx * float4(right_dual.group2.yzx, right_dual.e1))
        );
    }
}
extension Sphere: BulkContraction<DualNum> {
    associatedtype Output = AntiFlatPoint;
    func bulk_contraction(other: DualNum) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e1234)
        );
    }
}
extension Sphere: BulkContraction<FlatPoint> {
    associatedtype Output = AntiLine;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e321) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, 0 */
            float4((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Sphere: BulkContraction<Flector> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.e321 * this.e4235 * -1.0, right_dual.e321 * this.e4315 * -1.0, right_dual.e321 * this.e4125 * -1.0, (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234)) + float4((float3(this.e1234) * right_dual.group0.xyz), right_dual.e1 * this.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group0.zxy) - (right_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Sphere: BulkContraction<Line> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
extension Sphere: BulkContraction<Motor> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e3215) * float4(this.group0.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
extension Sphere: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e45 * this.e1234) + (right_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)) + float4((float3(this.e1234) * right_dual.group3.xyz), right_dual.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_dual.group8.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group6.xyz) + (right_dual.group7.xyz.zxy * this.group0.yzx) - (right_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_dual.group7.xyz) + (float3(this.e1234) * right_dual.group8.xyz) - (float3(right_dual.e321) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group9.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_dual.group9.zxy * this.group0.yzx), right_dual.e1234 * this.e3215), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group9.xyz) - (float3(right_dual.e1234) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkContraction<Plane> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkContraction<RoundPoint> {
    associatedtype Output = Circle;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e1234) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group0.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_dual.group0.zxy * this.group0.yzx), right_dual.e1234 * this.e3215), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_dual.group0.xyz), 0.0)
        );
    }
}
extension Sphere: BulkContraction<Scalar> {
    associatedtype Output = Sphere;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkContraction<Sphere> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: BulkContraction<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group0.zxy * right_dual.group3.yzx), this.e1234 * right_dual.e3215) - (this.group0.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * right_dual.e4235 * -1.0, this.e3215 * right_dual.e4315 * -1.0, this.e3215 * right_dual.e4125 * -1.0, (this.e4315 * right_dual.e42) + (this.e4125 * right_dual.e43) + (this.e1234 * right_dual.e45)) + (this.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4((this.e4315 * right_dual.e12) + (this.e1234 * right_dual.e15), (this.e4125 * right_dual.e23) + (this.e1234 * right_dual.e25), (this.e4235 * right_dual.e31) + (this.e1234 * right_dual.e35), -(this.e4125 * right_dual.e35) - (this.e3215 * right_dual.e45)) - (this.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - (this.group0.wwwy * float4(right_dual.group0.xyz, right_dual.e25))
        );
    }
}
extension Sphere: BulkContraction<VersorOdd> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return VersorOdd.from_groups(
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
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<VersorEven> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension VersorEven: BulkContraction<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return VersorOdd.from_groups(
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
extension VersorEven: BulkContraction<AntiDualNum> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: BulkContraction<AntiFlatPoint> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_dual.e45 * this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group0.xyz), -(right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321))
        );
    }
}
extension VersorEven: BulkContraction<AntiFlector> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorEven: BulkContraction<AntiLine> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group1.xyz), -(right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e415 * this.e321) + (right_dual.e315 * this.e412), (right_dual.e425 * this.e321) + (right_dual.e125 * this.e423), (right_dual.e435 * this.e321) + (right_dual.e235 * this.e431), -(right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - float4((right_dual.group1.xyz.zxy * this.group0.yzx), right_dual.e415 * this.e235)
        );
    }
}
extension VersorEven: BulkContraction<AntiMotor> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: BulkContraction<AntiPlane> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorEven: BulkContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: BulkContraction<Circle> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
extension VersorEven: BulkContraction<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: CircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.scalar * this.e12345) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e45 * this.e321) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412))
        );
    }
}
extension VersorEven: BulkContraction<Dipole> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), -(right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e412 * this.e315) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e423 * this.e125) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e431 * this.e235) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e125 * this.e435)) - float4((right_dual.group0.xyz.yzx * this.group2.zxy), right_dual.e235 * this.e415) - float4((right_dual.group2.xyz.zxy * this.group0.yzx), right_dual.e315 * this.e425)
        );
    }
}
extension VersorEven: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual.group2.xyz), (right_dual.e4 * this.e12345) - (right_dual.e423 * this.e415) - (right_dual.e431 * this.e425) - (right_dual.e412 * this.e435) - (right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412) + (right_dual.e1 * this.e12345), (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423) + (right_dual.e2 * this.e12345), (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431) + (right_dual.e3 * this.e12345), -(right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + float4((right_dual.group0.xyz.zxy * this.group2.yzx), right_dual.e5 * this.e12345) - (this.group2.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e415)) - float4((right_dual.group2.zxy * this.group0.yzx), right_dual.e425 * this.e315)
        );
    }
}
extension VersorEven: BulkContraction<DualNum> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.group0.xy.xx, right_dual.e3215, (right_dual.e3215 * this.e4) + (right_dual.scalar * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(right_dual.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
extension VersorEven: BulkContraction<FlatPoint> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual.e315 * this.e412) + (right_dual.e321 * this.e415), (right_dual.e125 * this.e423) + (right_dual.e321 * this.e425), (right_dual.e235 * this.e431) + (right_dual.e321 * this.e435), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) - (right_dual.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
extension VersorEven: BulkContraction<Flector> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual.e321 * this.e415) + (right_dual.e1 * this.e12345), (right_dual.e321 * this.e425) + (right_dual.e2 * this.e12345), (right_dual.e321 * this.e435) + (right_dual.e3 * this.e12345), -(right_dual.e315 * this.e425) - (right_dual.e125 * this.e435)) + (this.group0.zxyw * float4(right_dual.group0.yzx, right_dual.e5)) - (right_dual.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
extension VersorEven: BulkContraction<Line> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_dual.group0.xyz), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension VersorEven: BulkContraction<Motor> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            (float4(right_dual.e3215) * float4(this.group0.xyz, this.e4)) + (float4(this.e12345) * right_dual.group0) + float4(float3(0.0), -(right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual.e3215) * this.group1.xyz) + (float3(this.e12345) * right_dual.group1.xyz)), right_dual.e3215 * this.e12345)
        );
    }
}
extension VersorEven: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_dual.scalar * this.e12345) + (right_dual.e4235 * this.e1) + (right_dual.e4315 * this.e2) + (right_dual.e4125 * this.e3) + (right_dual.e3215 * this.e4) + (right_dual.e1234 * this.e5) - (right_dual.e15 * this.e423) - (right_dual.e25 * this.e431) - (right_dual.e35 * this.e412) - (right_dual.e45 * this.e321) - (right_dual.e41 * this.e235) - (right_dual.e42 * this.e315) - (right_dual.e43 * this.e125) - (right_dual.e23 * this.e415) - (right_dual.e31 * this.e425) - (right_dual.e12 * this.e435), right_dual.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual.e1 * this.e12345) + (right_dual.e415 * this.e321) + (right_dual.e321 * this.e415) + (right_dual.e315 * this.e412), (right_dual.e2 * this.e12345) + (right_dual.e425 * this.e321) + (right_dual.e321 * this.e425) + (right_dual.e125 * this.e423), (right_dual.e3 * this.e12345) + (right_dual.e435 * this.e321) + (right_dual.e321 * this.e435) + (right_dual.e235 * this.e431), -(right_dual.e415 * this.e423) - (right_dual.e425 * this.e431) - (right_dual.e435 * this.e412) - (right_dual.e412 * this.e435)) + (float4(right_dual.e12345) * this.group3) + float4((right_dual.group7.xyz.zxy * this.group2.yzx), right_dual.e4 * this.e12345) - float4((right_dual.group7.xyz.yzx * this.group2.zxy), right_dual.e423 * this.e415) - float4((right_dual.group8.xyz.zxy * this.group0.yzx), right_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4((right_dual.e12345 * this.e5) + (right_dual.e5 * this.e12345) - (right_dual.e415 * this.e235) - (right_dual.e425 * this.e315) - (right_dual.e435 * this.e125) - (right_dual.e235 * this.e415) - (right_dual.e315 * this.e425) - (right_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e4125 * this.e315) + (right_dual.e3215 * this.e415), (right_dual.e4235 * this.e125) + (right_dual.e3215 * this.e425), (right_dual.e4315 * this.e235) + (right_dual.e3215 * this.e435), -(right_dual.e4315 * this.e425) - (right_dual.e4125 * this.e435)) + (float4(this.e12345) * right_dual.group3) - (right_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e1234) * this.group1.xyz) + (float3(this.e12345) * right_dual.group4.xyz) + (right_dual.group9.yzx * this.group0.zxy) - (right_dual.group9.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e3215) * this.group0.xyz) + (float3(right_dual.e1234) * this.group2.xyz) + (float3(this.e12345) * right_dual.group5.xyz) - (float3(this.e321) * right_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(right_dual.e12345) * this.group1) + (float4(this.e12345) * right_dual.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(right_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_dual.group7.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e12345) * this.group2.xyz) + (float3(this.e12345) * right_dual.group8.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e1234 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: BulkContraction<Plane> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension VersorEven: BulkContraction<RoundPoint> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension VersorEven: BulkContraction<Scalar> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: BulkContraction<Sphere> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(right_dual.e5 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: BulkContraction<VersorEven> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: VersorEven) -> Output {
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
extension VersorEven: BulkContraction<VersorOdd> {
    associatedtype Output = VersorEven;
    func bulk_contraction(other: VersorOdd) -> Output {
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
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: BulkContractionInfix) -> BulkContractionInfixPartial<VersorOdd> {
        return BulkContractionInfixPartial { a: this };
    }
}
extension VersorOdd: BulkContraction<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: BulkContraction<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e41)) + float4(float3(0.0), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234) - (right_dual.e4235 * this.e41) - (right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) - float4((this.group3.www * right_dual.group3.xyz), right_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_dual.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_dual.group2.xyz, right_dual.e4315)) + float4((right_dual.group1.zxy * this.group3.yzx), right_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - float4((right_dual.group3.yzx * this.group1.zxy), right_dual.e35 * this.e4125)
        );
    }
}
extension VersorOdd: BulkContraction<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiDualNum) -> Output {
        DualNum right_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: BulkContraction<AntiFlatPoint> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: BulkContraction<AntiFlector> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiFlector) -> Output {
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group1.yzx * this.group3.zxy) - (right_dual.group1.zxy * this.group3.yzx)), right_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) + float4((right_dual.group1.www * this.group3.xyz), right_dual.e45 * this.e1234) - (right_dual.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (right_dual.group1.zxyy * float4(this.group1.yzx, this.e25)) + (right_dual.group1.wwwz * float4(this.group0.xyz, this.e35)) + (this.group2.wwwx * float4(right_dual.group0.xyz, right_dual.e4235)) + float4(float3(0.0), (right_dual.e3215 * this.e45) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215)) - float4((right_dual.group1.yzx * this.group1.zxy), right_dual.e15 * this.e4235)
        );
    }
}
extension VersorOdd: BulkContraction<AntiLine> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: AntiLine) -> Output {
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * right_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_dual.group1.xyz), -(right_dual.e415 * this.e4235) - (right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((right_dual.group1.xyz.zxy * this.group3.yzx), right_dual.e415 * this.e23)
        );
    }
}
extension VersorOdd: BulkContraction<AntiMotor> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: AntiMotor) -> Output {
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: BulkContraction<AntiPlane> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: AntiPlane) -> Output {
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx)), right_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(right_dual.e3215 * this.e4235, right_dual.e3215 * this.e4315, right_dual.e3215 * this.e4125, -(right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e4315 * this.e12 * -1.0, right_dual.e4125 * this.e23 * -1.0, right_dual.e4235 * this.e31 * -1.0, (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45)) + (right_dual.group0.zxyx * float4(this.group1.yzx, this.e15)) + (right_dual.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
extension VersorOdd: BulkContraction<Circle> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: Circle) -> Output {
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group3.yzxy * float4(right_dual.group1.zxy, right_dual.e42)) + float4((right_dual.group2.xyz * this.group2.www), right_dual.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: BulkContraction<CircleRotor> {
    associatedtype Output = RoundPoint;
    func bulk_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * this.e3215) - (right_dual.e31 * this.e4125), -(right_dual.e42 * this.e3215) - (right_dual.e12 * this.e4235), -(right_dual.e43 * this.e3215) - (right_dual.e23 * this.e4315), (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234)) + (this.group3.yzxx * float4(right_dual.group1.zxy, right_dual.e41)) + float4((this.group2.www * right_dual.group2.xyz), right_dual.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * this.e3215) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: BulkContraction<Dipole> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: Dipole) -> Output {
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group3.yzx) - (right_dual.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((right_dual.group2.xyz.zxy * this.group3.yzx), right_dual.e423 * this.e15)
        );
    }
}
extension VersorOdd: BulkContraction<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func bulk_contraction(other: DipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_dual.group1.xyz) + (right_dual.group0.xyz.zxy * this.group3.yzx) - (right_dual.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_dual.e423 * this.e3215) + (right_dual.e235 * this.e1234), (right_dual.e431 * this.e3215) + (right_dual.e315 * this.e1234), (right_dual.e412 * this.e3215) + (right_dual.e125 * this.e1234), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) - (right_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (this.group3.zxyx * float4(right_dual.group2.yzx, right_dual.e1)) + float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43)) - float4((right_dual.group2.zxy * this.group3.yzx), right_dual.e423 * this.e15)
        );
    }
}
extension VersorOdd: BulkContraction<DualNum> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: DualNum) -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension VersorOdd: BulkContraction<FlatPoint> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_dual.e235 * this.e1234, right_dual.e315 * this.e1234, right_dual.e125 * this.e1234, -(right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension VersorOdd: BulkContraction<Flector> {
    associatedtype Output = AntiMotor;
    func bulk_contraction(other: Flector) -> Output {
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43) - (right_dual.e321 * this.e45)) + float4((this.group2.www * right_dual.group0.xyz), right_dual.e1 * this.e4235) - (right_dual.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * this.group3.zxy) - (right_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension VersorOdd: BulkContraction<Line> {
    associatedtype Output = AntiPlane;
    func bulk_contraction(other: Line) -> Output {
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_dual.e12 * this.e4315) + (right_dual.e15 * this.e1234), (right_dual.e23 * this.e4125) + (right_dual.e25 * this.e1234), (right_dual.e31 * this.e4235) + (right_dual.e35 * this.e1234), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) - (this.group3.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
extension VersorOdd: BulkContraction<Motor> {
    associatedtype Output = AntiFlector;
    func bulk_contraction(other: Motor) -> Output {
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_dual.e15 * this.e1234) + (right_dual.e3215 * this.e41), (right_dual.e25 * this.e1234) + (right_dual.e3215 * this.e42), (right_dual.e35 * this.e1234) + (right_dual.e3215 * this.e43), -(right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125)) + float4((right_dual.group0.zxy * this.group3.yzx), right_dual.e3215 * this.e45) - (this.group3.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
extension VersorOdd: BulkContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func bulk_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_dual.e12345 * this.scalar) + (right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234) - (right_dual.e415 * this.e23) - (right_dual.e425 * this.e31) - (right_dual.e435 * this.e12) - (right_dual.e321 * this.e45) - (right_dual.e423 * this.e15) - (right_dual.e431 * this.e25) - (right_dual.e412 * this.e35) - (right_dual.e235 * this.e41) - (right_dual.e315 * this.e42) - (right_dual.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (this.group3.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)) + float4((right_dual.group9.zxy * this.group1.yzx), right_dual.e43 * this.e4125) + float4((right_dual.group9.www * this.group0.xyz), right_dual.e45 * this.e1234) + float4((this.group2.www * right_dual.group3.xyz), right_dual.e42 * this.e4315) - (float4(right_dual.e1234) * float4(this.group2.xyz, this.e45)) - (right_dual.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((right_dual.group4.xyz * this.group3.www), right_dual.e4235 * this.e41) - float4((right_dual.group5.xyz.yzx * this.group3.zxy), right_dual.e4315 * this.e42), 
            /* e5, 0, 0, 0 */
            float4((right_dual.e4235 * this.e15) + (right_dual.e4315 * this.e25) + (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45) - (right_dual.e15 * this.e4235) - (right_dual.e25 * this.e4315) - (right_dual.e35 * this.e4125) - (right_dual.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * this.e3215) + (right_dual.e315 * this.e4125), (right_dual.e425 * this.e3215) + (right_dual.e125 * this.e4235), (right_dual.e435 * this.e3215) + (right_dual.e235 * this.e4315), -(right_dual.e425 * this.e4315) - (right_dual.e435 * this.e4125)) + (float4(right_dual.e12345) * float4(this.group2.xyz, this.e45)) - (this.group3.yzxx * float4(right_dual.group8.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_dual.e12345) * this.group0.xyz) + (float3(this.e1234) * right_dual.group6.xyz) + (right_dual.group7.xyz.zxy * this.group3.yzx) - (right_dual.group7.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_dual.e12345) * this.group1.xyz) + (float3(this.e1234) * right_dual.group8.xyz) + (float3(this.e3215) * right_dual.group7.xyz) - (float3(right_dual.e321) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group9.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_dual.group9.zxy * this.group3.yzx), right_dual.e1234 * this.e3215), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group9.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * this.group3.xyz) - (float3(this.e3215) * right_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * this.group3, 
            /* e1234, 0, 0, 0 */
            float4(right_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: BulkContraction<Plane> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Plane) -> Output {
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: BulkContraction<RoundPoint> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: RoundPoint) -> Output {
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group0.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group0.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_dual.group0.zxy * this.group3.yzx), right_dual.e1234 * this.e3215), 
            /* e235, e315, e125, e4 */
            float4(right_dual.e3215 * this.e4235, right_dual.e3215 * this.e4315, right_dual.e3215 * this.e4125, -(right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43) - (right_dual.e1234 * this.e45)) - (right_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(right_dual.e4315 * this.e12) - (right_dual.e1234 * this.e15), -(right_dual.e4125 * this.e23) - (right_dual.e1234 * this.e25), -(right_dual.e4235 * this.e31) - (right_dual.e1234 * this.e35), (right_dual.e4125 * this.e35) + (right_dual.e3215 * this.e45)) + (right_dual.group0.zxyx * float4(this.group1.yzx, this.e15)) + (right_dual.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
extension VersorOdd: BulkContraction<Scalar> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: Scalar) -> Output {
        AntiScalar right_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: BulkContraction<Sphere> {
    associatedtype Output = Scalar;
    func bulk_contraction(other: Sphere) -> Output {
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e1 * this.e4235) + (right_dual.e2 * this.e4315) + (right_dual.e3 * this.e4125) + (right_dual.e4 * this.e3215) + (right_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: BulkContraction<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func bulk_contraction(other: VersorEven) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_dual.group3.xyz) - (float3(right_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual.group3.zxy, right_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_dual.group3.www, right_dual.e41)) + float4(float3(0.0), (right_dual.e42 * this.e4315) + (right_dual.e43 * this.e4125) + (right_dual.e45 * this.e1234) - (right_dual.e4235 * this.e41) - (right_dual.e4315 * this.e42) - (right_dual.e4125 * this.e43)) - float4((this.group3.www * right_dual.group3.xyz), right_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_dual.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_dual.group2.xyz, right_dual.e4315)) + float4((right_dual.group1.zxy * this.group3.yzx), right_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (right_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - float4((right_dual.group3.yzx * this.group1.zxy), right_dual.e35 * this.e4125)
        );
    }
}
extension VersorOdd: BulkContraction<VersorOdd> {
    associatedtype Output = VersorOdd;
    func bulk_contraction(other: VersorOdd) -> Output {
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
        return VersorOdd.from_groups(
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

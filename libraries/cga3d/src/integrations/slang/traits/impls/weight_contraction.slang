extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<AntiCircleRotor> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension AntiCircleRotor: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
extension AntiCircleRotor: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((right_anti_dual.group2.www * this.group2.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightContraction<AntiDualNum> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual.e12345) * this.group2
        );
    }
}
extension AntiCircleRotor: WeightContraction<AntiFlector> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightContraction<AntiLine> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightContraction<AntiMotor> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435))
        );
    }
}
extension AntiCircleRotor: WeightContraction<AntiPlane> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightContraction<Dipole> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightContraction<DipoleInversion> {
    associatedtype Output = Scalar;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightContraction<DualNum> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: DualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension AntiCircleRotor: WeightContraction<FlatPoint> {
    associatedtype Output = Scalar;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e45 * right_anti_dual.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightContraction<Flector> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e45 * right_anti_dual.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightContraction<Motor> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Motor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(float4(other.group1.xyz, other.e5 * -1.0)[3]) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension AntiCircleRotor: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group9.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_anti_dual.e1234) * this.group2.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e12345) * this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: WeightContraction<RoundPoint> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_anti_dual.e1234) * this.group2.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightContraction<Scalar> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual.e12345) * this.group2
        );
    }
}
extension AntiCircleRotor: WeightContraction<VersorEven> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: VersorEven) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((right_anti_dual.group2.www * this.group2.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: WeightContraction<VersorOdd> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.scalar * right_anti_dual.e12345) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<AntiDipoleInversion> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension AntiDipoleInversion: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.e4 * right_anti_dual.e12345) - (this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412) + (this.e1 * right_anti_dual.e12345), (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e12345), (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e12345), -(this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + (right_anti_dual.group2.yzxw * float4(this.group0.xyz.zxy, this.e5)) - (right_anti_dual.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), this.e425 * right_anti_dual.e315)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group3.yzx) - (this.group0.xyz.yzx * right_anti_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_anti_dual.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((this.group2.zxy * right_anti_dual.group3.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<AntiDualNum> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
extension AntiDipoleInversion: WeightContraction<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<AntiFlector> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx) - (this.group0.xyz.yzx * right_anti_dual.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_anti_dual.e3215, this.e431 * right_anti_dual.e3215, this.e412 * right_anti_dual.e3215, -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_anti_dual.group1.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) - float4((this.group2.zxy * right_anti_dual.group1.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<AntiLine> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e315) + (this.e321 * right_anti_dual.e415), (this.e423 * right_anti_dual.e125) + (this.e321 * right_anti_dual.e425), (this.e431 * right_anti_dual.e235) + (this.e321 * right_anti_dual.e435), -(this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group1.xyz.zxy), this.e423 * right_anti_dual.e415), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<AntiMotor> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.e4 * right_anti_dual.e12345) - (this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_anti_dual.e415) + (this.e1 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e425) + (this.e2 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e435) + (this.e3 * right_anti_dual.e12345), -(this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx), this.e5 * right_anti_dual.e12345) - (right_anti_dual.group1.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
extension AntiDipoleInversion: WeightContraction<AntiPlane> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_anti_dual.e3215, this.e431 * right_anti_dual.e3215, this.e412 * right_anti_dual.e3215, -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * right_anti_dual.e4315 * -1.0, this.e235 * right_anti_dual.e4125 * -1.0, this.e315 * right_anti_dual.e4235 * -1.0, (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)) + (float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_anti_dual.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
extension AntiDipoleInversion: WeightContraction<Circle> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<CircleRotor> {
    associatedtype Output = Scalar;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<Dipole> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) - float4((this.group0.xyz.yzx * right_anti_dual.group2.xyz.zxy), this.e423 * right_anti_dual.e415) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), this.e431 * right_anti_dual.e425), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<DipoleInversion> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.e423 * this.e415) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), right_anti_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<DualNum> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e4), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<FlatPoint> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321), -(this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125)) - (right_anti_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
extension AntiDipoleInversion: WeightContraction<Flector> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321), -(this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125)) - (right_anti_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
extension AntiDipoleInversion: WeightContraction<Line> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<Motor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz), (this.e4 * right_anti_dual.e3215) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e4 * right_anti_dual.e3215) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) + (float4(right_anti_dual.e12345) * float4(this.group3.xyz, this.e4)) - float4((this.group0.xyz.yzx * right_anti_dual.group8.xyz.zxy), this.e423 * right_anti_dual.e415) - float4((right_anti_dual.group7.xyz.yzx * this.group2.zxy), this.e431 * right_anti_dual.e425), 
            /* e5, 0, 0, 0 */
            float4((this.e5 * right_anti_dual.e12345) - (this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group9.yzx) - (this.group0.xyz.yzx * right_anti_dual.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz) + (float3(right_anti_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e12345) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<RoundPoint> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * right_anti_dual.e4315 * -1.0, this.e235 * right_anti_dual.e4125 * -1.0, this.e315 * right_anti_dual.e4235 * -1.0, (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)) + (float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_anti_dual.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
extension AntiDipoleInversion: WeightContraction<Scalar> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
extension AntiDipoleInversion: WeightContraction<VersorEven> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group3.yzx) - (this.group0.xyz.yzx * right_anti_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e4)) + (right_anti_dual.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((this.group2.zxy * right_anti_dual.group3.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
extension AntiDipoleInversion: WeightContraction<VersorOdd> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), (this.e4 * right_anti_dual.e12345) - (this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412) + (this.e1 * right_anti_dual.e12345), (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e12345), (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e12345), -(this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + float4((this.group0.xyz.zxy * right_anti_dual.group2.yzx), this.e5 * right_anti_dual.e12345) - (right_anti_dual.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((this.group2.zxy * right_anti_dual.group0.yzx), this.e425 * right_anti_dual.e315)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<AntiDualNum> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension AntiDualNum: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.xx, this.e3215, this.scalar) * float4(right_anti_dual.group0.xyz, right_anti_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e12345)
        );
    }
}
extension AntiDualNum: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group3.xyz, right_anti_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45) * float4(-1.0)
        );
    }
}
extension AntiDualNum: WeightContraction<AntiDualNum> {
    associatedtype Output = AntiDualNum;
    func weight_contraction(other: AntiDualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightContraction<AntiFlatPoint> {
    associatedtype Output = DualNum;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e3215, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightContraction<AntiFlector> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e45 * -1.0)
        );
    }
}
extension AntiDualNum: WeightContraction<AntiLine> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: AntiLine) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightContraction<AntiMotor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.scalar * right_anti_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * right_anti_dual.group0
        );
    }
}
extension AntiDualNum: WeightContraction<AntiPlane> {
    associatedtype Output = AntiFlatPoint;
    func weight_contraction(other: AntiPlane) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightContraction<Circle> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45) * float4(-1.0)
        );
    }
}
extension AntiDualNum: WeightContraction<CircleRotor> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45) * float4(-1.0)
        );
    }
}
extension AntiDualNum: WeightContraction<Dipole> {
    associatedtype Output = AntiLine;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e3215) * right_anti_dual.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.e3215) * right_anti_dual.group1.xyz, 0.0)
        );
    }
}
extension AntiDualNum: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e4), 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e3215 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e12345), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group4.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e3215 * right_anti_dual.e45 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e3215) * right_anti_dual.group7.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e3215) * right_anti_dual.group9.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: WeightContraction<RoundPoint> {
    associatedtype Output = AntiFlatPoint;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e1234) * float4(-1.0)
        );
    }
}
extension AntiDualNum: WeightContraction<Scalar> {
    associatedtype Output = AntiDualNum;
    func weight_contraction(other: Scalar) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightContraction<Sphere> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e3215 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: WeightContraction<VersorEven> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group3.xyz, right_anti_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45) * float4(-1.0)
        );
    }
}
extension AntiDualNum: WeightContraction<VersorOdd> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.xx, this.e3215, (this.e3215 * right_anti_dual.e4) + (this.scalar * right_anti_dual.e12345)) * float4(right_anti_dual.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e12345)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<AntiFlatPoint> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension AntiFlatPoint: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_anti_dual.e412) + (this.e321 * right_anti_dual.e415), (this.e125 * right_anti_dual.e423) + (this.e321 * right_anti_dual.e425), (this.e235 * right_anti_dual.e431) + (this.e321 * right_anti_dual.e435), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
extension AntiFlatPoint: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e235 * right_anti_dual.e1234, this.e315 * right_anti_dual.e1234, this.e125 * right_anti_dual.e1234, -(this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - float4((this.group0.www * right_anti_dual.group3.xyz), this.e235 * right_anti_dual.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx)), 0.0)
        );
    }
}
extension AntiFlatPoint: WeightContraction<AntiDualNum> {
    associatedtype Output = AntiFlatPoint;
    func weight_contraction(other: AntiDualNum) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiFlatPoint: WeightContraction<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightContraction<AntiFlector> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(right_anti_dual.group1.xyz, right_anti_dual.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group1.zxy) - (this.group0.zxy * right_anti_dual.group1.yzx)), 0.0)
        );
    }
}
extension AntiFlatPoint: WeightContraction<AntiLine> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * right_anti_dual.group0.xyz), -(this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435))
        );
    }
}
extension AntiFlatPoint: WeightContraction<AntiMotor> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.group0.www * right_anti_dual.group0.xyz), -(this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435))
        );
    }
}
extension AntiFlatPoint: WeightContraction<AntiPlane> {
    associatedtype Output = AntiLine;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * right_anti_dual.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx), 0.0)
        );
    }
}
extension AntiFlatPoint: WeightContraction<Circle> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightContraction<CircleRotor> {
    associatedtype Output = Scalar;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e45 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: WeightContraction<Dipole> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_anti_dual.e412) + (this.e321 * right_anti_dual.e415), (this.e125 * right_anti_dual.e423) + (this.e321 * right_anti_dual.e425), (this.e235 * right_anti_dual.e431) + (this.e321 * right_anti_dual.e435), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
extension AntiFlatPoint: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321), -(right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125)) - (this.group0.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e415))
        );
    }
}
extension AntiFlatPoint: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_anti_dual.group6.xyz), 0.0) + float4((right_anti_dual.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4(-(this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * right_anti_dual.group9.zxy) - (this.group0.zxy * right_anti_dual.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_anti_dual.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiFlatPoint: WeightContraction<RoundPoint> {
    associatedtype Output = AntiLine;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_anti_dual.group0.xyz), 0.0), 
            /* e15, e25, e35, 0 */
            float4((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx), 0.0)
        );
    }
}
extension AntiFlatPoint: WeightContraction<Scalar> {
    associatedtype Output = AntiFlatPoint;
    func weight_contraction(other: Scalar) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiFlatPoint: WeightContraction<VersorEven> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e235 * right_anti_dual.e1234, this.e315 * right_anti_dual.e1234, this.e125 * right_anti_dual.e1234, -(this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx)), 0.0)
        );
    }
}
extension AntiFlatPoint: WeightContraction<VersorOdd> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_anti_dual.e412) + (this.e321 * right_anti_dual.e415), (this.e125 * right_anti_dual.e423) + (this.e321 * right_anti_dual.e425), (this.e235 * right_anti_dual.e431) + (this.e321 * right_anti_dual.e435), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e415))
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<AntiFlector> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension AntiFlector: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_anti_dual.e415) + (this.e1 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e425) + (this.e2 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e435) + (this.e3 * right_anti_dual.e12345), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + float4((right_anti_dual.group0.xyz.zxy * this.group0.yzx), this.e5 * right_anti_dual.e12345) - float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
extension AntiFlector: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) + float4((right_anti_dual.group2.www * this.group0.xyz), this.e1 * right_anti_dual.e4235) - float4((this.group0.www * right_anti_dual.group3.xyz), this.e235 * right_anti_dual.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx)), 0.0)
        );
    }
}
extension AntiFlector: WeightContraction<AntiDualNum> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
extension AntiFlector: WeightContraction<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: WeightContraction<AntiFlector> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * right_anti_dual.group1.xyz * float3(-1.0)), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) - (this.e321 * right_anti_dual.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group1.zxy) - (this.group0.zxy * right_anti_dual.group1.yzx)), 0.0)
        );
    }
}
extension AntiFlector: WeightContraction<AntiLine> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * right_anti_dual.group0.xyz), -(this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435))
        );
    }
}
extension AntiFlector: WeightContraction<AntiMotor> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e1 * right_anti_dual.e12345, this.e2 * right_anti_dual.e12345, this.e3 * right_anti_dual.e12345, -(this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + (right_anti_dual.group0 * float4(this.group0.www, this.e5))
        );
    }
}
extension AntiFlector: WeightContraction<AntiPlane> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * right_anti_dual.group0.xyz * float3(-1.0)), (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: WeightContraction<Circle> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: WeightContraction<CircleRotor> {
    associatedtype Output = Scalar;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e45 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: WeightContraction<Dipole> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e315 * right_anti_dual.e412) + (this.e321 * right_anti_dual.e415), (this.e125 * right_anti_dual.e423) + (this.e321 * right_anti_dual.e425), (this.e235 * right_anti_dual.e431) + (this.e321 * right_anti_dual.e435), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group0.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
extension AntiFlector: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321), -(right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125)) - (this.group0.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e415))
        );
    }
}
extension AntiFlector: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_anti_dual.group6.xyz), 0.0) + float4((float3(right_anti_dual.e12345) * this.group1.xyz), 0.0) + float4((right_anti_dual.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4((this.e5 * right_anti_dual.e12345) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * right_anti_dual.group9.zxy) - (this.group0.zxy * right_anti_dual.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * this.group0.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * right_anti_dual.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiFlector: WeightContraction<RoundPoint> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e321 * right_anti_dual.e4235 * -1.0, this.e321 * right_anti_dual.e4315 * -1.0, this.e321 * right_anti_dual.e4125 * -1.0, (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234)) + float4((float3(right_anti_dual.e1234) * this.group0.xyz), this.e1 * right_anti_dual.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group0.zxy) - (this.group0.zxy * right_anti_dual.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: WeightContraction<Scalar> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
extension AntiFlector: WeightContraction<VersorEven> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43) - (this.e321 * right_anti_dual.e45)) + float4((right_anti_dual.group2.www * this.group0.xyz), this.e1 * right_anti_dual.e4235) - (this.group0.wwwx * float4(right_anti_dual.group3.xyz, right_anti_dual.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * right_anti_dual.group3.zxy) - (this.group0.zxy * right_anti_dual.group3.yzx)), 0.0)
        );
    }
}
extension AntiFlector: WeightContraction<VersorOdd> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * right_anti_dual.e415) + (this.e1 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e425) + (this.e2 * right_anti_dual.e12345), (this.e321 * right_anti_dual.e435) + (this.e3 * right_anti_dual.e12345), -(this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) + (right_anti_dual.group0.zxyw * float4(this.group0.yzx, this.e5)) - (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e415))
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<AntiLine> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension AntiLine: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: WeightContraction<AntiDualNum> {
    associatedtype Output = AntiLine;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(right_anti_dual.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: WeightContraction<AntiFlector> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_anti_dual.e4315 * -1.0, this.e23 * right_anti_dual.e4125 * -1.0, this.e31 * right_anti_dual.e4235 * -1.0, (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group1.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: WeightContraction<AntiLine> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: WeightContraction<AntiMotor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: WeightContraction<AntiPlane> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_anti_dual.e4315 * -1.0, this.e23 * right_anti_dual.e4125 * -1.0, this.e31 * right_anti_dual.e4235 * -1.0, (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: WeightContraction<Dipole> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: WeightContraction<DipoleInversion> {
    associatedtype Output = Scalar;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.xyz.yzx * right_anti_dual.group9.zxy), 0.0) - float4((float3(right_anti_dual.e1234) * this.group1.xyz), 0.0) - float4((this.group0.xyz.zxy * right_anti_dual.group9.yzx), 0.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
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
extension AntiLine: WeightContraction<RoundPoint> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: WeightContraction<Scalar> {
    associatedtype Output = AntiLine;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(right_anti_dual.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: WeightContraction<VersorEven> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: WeightContraction<VersorOdd> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<AntiMotor> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension AntiMotor: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e23 * right_anti_dual.e12345, this.e31 * right_anti_dual.e12345, this.e12 * right_anti_dual.e12345, -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)) + float4((right_anti_dual.group0.xyz * this.group1.www), this.scalar * right_anti_dual.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz)), this.e3215 * right_anti_dual.e12345)
        );
    }
}
extension AntiMotor: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group3.xyz, right_anti_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((right_anti_dual.group0.xyz * this.group1.www), this.e3215 * right_anti_dual.e45)
        );
    }
}
extension AntiMotor: WeightContraction<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
extension AntiMotor: WeightContraction<AntiFlatPoint> {
    associatedtype Output = DualNum;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(float4(other.group0.xyz, other.e321 * -1.0)[3] * this.e3215, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiMotor: WeightContraction<AntiFlector> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            (right_anti_dual.group1.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) - float4((this.group0.zxy * right_anti_dual.group1.yzx), this.e3215 * right_anti_dual.e45)
        );
    }
}
extension AntiMotor: WeightContraction<AntiLine> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: WeightContraction<AntiMotor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz), (this.scalar * right_anti_dual.e12345) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz)), this.e3215 * right_anti_dual.e12345)
        );
    }
}
extension AntiMotor: WeightContraction<AntiPlane> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_anti_dual.e4315 * -1.0, this.e23 * right_anti_dual.e4125 * -1.0, this.e31 * right_anti_dual.e4235 * -1.0, (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
extension AntiMotor: WeightContraction<Circle> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.group0.xyz * this.group1.www * float3(-1.0)), this.e3215 * right_anti_dual.e45 * -1.0)
        );
    }
}
extension AntiMotor: WeightContraction<CircleRotor> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45) * float4(-1.0)
        );
    }
}
extension AntiMotor: WeightContraction<Dipole> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * right_anti_dual.group0.xyz), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * right_anti_dual.group0.xyz), (right_anti_dual.e4 * this.e3215) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.scalar * right_anti_dual.e12345) + (this.e3215 * right_anti_dual.e4) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.yzx * right_anti_dual.group9.zxy), 0.0) - float4((float3(this.e3215) * right_anti_dual.group4.xyz), 0.0) - float4((float3(right_anti_dual.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * right_anti_dual.group9.yzx), 0.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) - (this.e3215 * right_anti_dual.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(this.e3215) * right_anti_dual.group6.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_anti_dual.group7.xyz) + (float3(right_anti_dual.e12345) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e3215) * right_anti_dual.group9.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * right_anti_dual.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: WeightContraction<RoundPoint> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
extension AntiMotor: WeightContraction<Scalar> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
extension AntiMotor: WeightContraction<Sphere> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e3215 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: WeightContraction<VersorEven> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_anti_dual.group3.xyz, right_anti_dual.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e15 * right_anti_dual.e1234) - (this.e3215 * right_anti_dual.e41), -(this.e25 * right_anti_dual.e1234) - (this.e3215 * right_anti_dual.e42), -(this.e35 * right_anti_dual.e1234) - (this.e3215 * right_anti_dual.e43), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (right_anti_dual.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((this.group0.zxy * right_anti_dual.group3.yzx), this.e3215 * right_anti_dual.e45)
        );
    }
}
extension AntiMotor: WeightContraction<VersorOdd> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e4)) + (float4(right_anti_dual.e12345) * this.group0) + float4(float3(0.0), -(this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz)), this.e3215 * right_anti_dual.e12345)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<AntiPlane> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension AntiPlane: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: WeightContraction<AntiDualNum> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: AntiDualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: WeightContraction<AntiFlector> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: WeightContraction<AntiMotor> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: AntiMotor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: WeightContraction<AntiPlane> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_anti_dual.e12345, 0.0, 0.0, 0.0), 
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
extension AntiPlane: WeightContraction<RoundPoint> {
    associatedtype Output = Scalar;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: WeightContraction<Scalar> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Scalar) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: WeightContraction<VersorEven> {
    associatedtype Output = Scalar;
    func weight_contraction(other: VersorEven) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e5 * right_anti_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: WeightContraction<VersorOdd> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: VersorOdd) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<AntiScalar> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension AntiScalar: WeightContraction<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func weight_contraction(other: AntiCircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.e12345) * right_anti_dual.group2
        );
    }
}
extension AntiScalar: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * right_anti_dual.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
extension AntiScalar: WeightContraction<AntiDualNum> {
    associatedtype Output = DualNum;
    func weight_contraction(other: AntiDualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.e12345) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightContraction<AntiFlatPoint> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
extension AntiScalar: WeightContraction<AntiFlector> {
    associatedtype Output = Flector;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
extension AntiScalar: WeightContraction<AntiLine> {
    associatedtype Output = Line;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_anti_dual.group1.xyz, 0.0)
        );
    }
}
extension AntiScalar: WeightContraction<AntiMotor> {
    associatedtype Output = Motor;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
extension AntiScalar: WeightContraction<AntiPlane> {
    associatedtype Output = Plane;
    func weight_contraction(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension AntiScalar: WeightContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightContraction<Circle> {
    associatedtype Output = Dipole;
    func weight_contraction(other: Circle) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * right_anti_dual.group2.xyz, 0.0)
        );
    }
}
extension AntiScalar: WeightContraction<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: CircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * right_anti_dual.group2
        );
    }
}
extension AntiScalar: WeightContraction<Dipole> {
    associatedtype Output = Circle;
    func weight_contraction(other: Dipole) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_anti_dual.group2.xyz, 0.0)
        );
    }
}
extension AntiScalar: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: DipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * right_anti_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
extension AntiScalar: WeightContraction<DualNum> {
    associatedtype Output = AntiDualNum;
    func weight_contraction(other: DualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0), 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightContraction<FlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func weight_contraction(other: FlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
extension AntiScalar: WeightContraction<Flector> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
extension AntiScalar: WeightContraction<Line> {
    associatedtype Output = AntiLine;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * right_anti_dual.group1.xyz, 0.0)
        );
    }
}
extension AntiScalar: WeightContraction<Motor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
extension AntiScalar: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2(this.e12345) * right_anti_dual.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_anti_dual.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_anti_dual.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_anti_dual.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_anti_dual.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_anti_dual.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightContraction<Plane> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension AntiScalar: WeightContraction<RoundPoint> {
    associatedtype Output = Sphere;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightContraction<Scalar> {
    associatedtype Output = AntiScalar;
    func weight_contraction(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightContraction<Sphere> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: WeightContraction<VersorEven> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: VersorEven) -> Output {
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
extension AntiScalar: WeightContraction<VersorOdd> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: VersorOdd) -> Output {
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
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<Circle> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension Circle: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e125 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.e235 * right_anti_dual.e415) - float4((this.group2.xyz.zxy * right_anti_dual.group0.xyz.yzx), this.e315 * right_anti_dual.e425)
        );
    }
}
extension Circle: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group3.yzx) - (this.group0.xyz.yzx * right_anti_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((this.group2.xyz.zxy * right_anti_dual.group3.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
extension Circle: WeightContraction<AntiDualNum> {
    associatedtype Output = Circle;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: WeightContraction<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightContraction<AntiFlector> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * right_anti_dual.group1.yzx) - (this.group0.xyz.yzx * right_anti_dual.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_anti_dual.e3215, this.e431 * right_anti_dual.e3215, this.e412 * right_anti_dual.e3215, -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45)) - float4((this.group2.xyz.zxy * right_anti_dual.group1.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
extension Circle: WeightContraction<AntiLine> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e315) + (this.e321 * right_anti_dual.e415), (this.e423 * right_anti_dual.e125) + (this.e321 * right_anti_dual.e425), (this.e431 * right_anti_dual.e235) + (this.e321 * right_anti_dual.e435), -(this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group1.xyz.zxy), this.e423 * right_anti_dual.e415), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightContraction<AntiMotor> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e321 * right_anti_dual.e415), (this.e423 * right_anti_dual.e125) + (this.e321 * right_anti_dual.e425), (this.e431 * right_anti_dual.e235) + (this.e321 * right_anti_dual.e435), -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group1.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
extension Circle: WeightContraction<AntiPlane> {
    associatedtype Output = Dipole;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_anti_dual.e3215, this.e431 * right_anti_dual.e3215, this.e412 * right_anti_dual.e3215, -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(right_anti_dual.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * right_anti_dual.group0.zxy) - (this.group2.xyz.zxy * right_anti_dual.group0.yzx), 0.0)
        );
    }
}
extension Circle: WeightContraction<Circle> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightContraction<CircleRotor> {
    associatedtype Output = Scalar;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightContraction<Dipole> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.xyz.zxy), right_anti_dual.e423 * this.e415) - float4((right_anti_dual.group2.xyz.zxy * this.group0.xyz.yzx), right_anti_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightContraction<DipoleInversion> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.xyz.zxy), right_anti_dual.e423 * this.e415) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), right_anti_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightContraction<DualNum> {
    associatedtype Output = AntiLine;
    func weight_contraction(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e3215) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(right_anti_dual.e3215) * this.group1.xyz, 0.0)
        );
    }
}
extension Circle: WeightContraction<FlatPoint> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e315 * this.e412) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e125 * this.e423) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e235 * this.e431) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
extension Circle: WeightContraction<Flector> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e315 * this.e412) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e125 * this.e423) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e235 * this.e431) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
extension Circle: WeightContraction<Line> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: WeightContraction<Motor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Circle: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) - float4((this.group0.xyz.yzx * right_anti_dual.group8.xyz.zxy), this.e423 * right_anti_dual.e415) - float4((this.group2.xyz.zxy * right_anti_dual.group7.xyz.yzx), this.e431 * right_anti_dual.e425), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group9.yzxx * float4(this.group2.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group9.yzx) - (this.group0.xyz.yzx * right_anti_dual.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz) + (float3(right_anti_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e12345) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Circle: WeightContraction<RoundPoint> {
    associatedtype Output = Dipole;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(right_anti_dual.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * right_anti_dual.group0.zxy) - (this.group2.xyz.zxy * right_anti_dual.group0.yzx), 0.0)
        );
    }
}
extension Circle: WeightContraction<Scalar> {
    associatedtype Output = Circle;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: WeightContraction<VersorEven> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group3.yzx) - (this.group0.xyz.yzx * right_anti_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((this.group2.xyz.zxy * right_anti_dual.group3.yzx), this.e423 * right_anti_dual.e15)
        );
    }
}
extension Circle: WeightContraction<VersorOdd> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e423 * right_anti_dual.e125) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e431 * right_anti_dual.e235) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e125 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), this.e235 * right_anti_dual.e415) - float4((this.group2.xyz.zxy * right_anti_dual.group0.yzx), this.e315 * right_anti_dual.e425)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<CircleRotor> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension CircleRotor: WeightContraction<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension CircleRotor: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return VersorOdd.from_groups(
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
extension CircleRotor: WeightContraction<AntiDualNum> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension CircleRotor: WeightContraction<AntiFlatPoint> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e12345 * right_anti_dual.e45), 
            /* e15, e25, e35, scalar */
            float4((this.group2.www * right_anti_dual.group0.xyz), -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e321 * right_anti_dual.e45))
        );
    }
}
extension CircleRotor: WeightContraction<AntiFlector> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension CircleRotor: WeightContraction<AntiLine> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group1.xyz), -(this.e423 * right_anti_dual.e415) - (this.e431 * right_anti_dual.e425) - (this.e412 * right_anti_dual.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * right_anti_dual.e315) + (this.e321 * right_anti_dual.e415), (this.e423 * right_anti_dual.e125) + (this.e321 * right_anti_dual.e425), (this.e431 * right_anti_dual.e235) + (this.e321 * right_anti_dual.e435), -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group1.xyz.zxy), this.e235 * right_anti_dual.e415)
        );
    }
}
extension CircleRotor: WeightContraction<AntiMotor> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension CircleRotor: WeightContraction<AntiPlane> {
    associatedtype Output = DipoleInversion;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * right_anti_dual.e3215, this.e431 * right_anti_dual.e3215, this.e412 * right_anti_dual.e3215, -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * right_anti_dual.group0.zxy), 0.0) - float4((this.group2.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0
        );
    }
}
extension CircleRotor: WeightContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightContraction<Circle> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43))
        );
    }
}
extension CircleRotor: WeightContraction<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: CircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
extension CircleRotor: WeightContraction<Dipole> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e125 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.e235 * this.e415) - float4((right_anti_dual.group2.xyz.zxy * this.group0.xyz.yzx), right_anti_dual.e315 * this.e425)
        );
    }
}
extension CircleRotor: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: DipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.e4 * this.e12345) - (right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412) + (right_anti_dual.e1 * this.e12345), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423) + (right_anti_dual.e2 * this.e12345), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431) + (right_anti_dual.e3 * this.e12345), -(right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + (this.group2.yzxw * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e5)) - (this.group2.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e415)) - float4((this.group0.xyz.yzx * right_anti_dual.group2.zxy), right_anti_dual.e425 * this.e315)
        );
    }
}
extension CircleRotor: WeightContraction<DualNum> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, right_anti_dual.scalar) * float4(this.group0.xyz, this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
extension CircleRotor: WeightContraction<FlatPoint> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e315 * this.e412) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e125 * this.e423) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e235 * this.e431) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
extension CircleRotor: WeightContraction<Flector> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e321 * this.e415) + (right_anti_dual.e1 * this.e12345), (right_anti_dual.e321 * this.e425) + (right_anti_dual.e2 * this.e12345), (right_anti_dual.e321 * this.e435) + (right_anti_dual.e3 * this.e12345), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + float4((this.group0.xyz.zxy * right_anti_dual.group0.yzx), right_anti_dual.e5 * this.e12345) - float4((this.group0.xyz.yzx * right_anti_dual.group0.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
extension CircleRotor: WeightContraction<Line> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension CircleRotor: WeightContraction<Motor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e23 * this.e12345, right_anti_dual.e31 * this.e12345, right_anti_dual.e12 * this.e12345, -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)) + float4((this.group0.xyz * right_anti_dual.group1.www), right_anti_dual.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual.e3215) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz)), right_anti_dual.e3215 * this.e12345)
        );
    }
}
extension CircleRotor: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e12345 * right_anti_dual.scalar) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), this.e12345 * right_anti_dual.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412) + (this.e12345 * right_anti_dual.e1), (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423) + (this.e12345 * right_anti_dual.e2), (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431) + (this.e12345 * right_anti_dual.e3), -(this.e412 * right_anti_dual.e435) - (this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) + float4((this.group0.xyz.zxy * right_anti_dual.group8.xyz.yzx), this.e12345 * right_anti_dual.e4) - float4((this.group0.xyz.yzx * right_anti_dual.group8.xyz.zxy), this.e423 * right_anti_dual.e415) - float4((right_anti_dual.group7.xyz.yzx * this.group2.zxy), this.e431 * right_anti_dual.e425), 
            /* e5, 0, 0, 0 */
            float4((this.e12345 * right_anti_dual.e5) - (this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e315 * right_anti_dual.e4125) + (this.e12345 * right_anti_dual.e15), (this.e125 * right_anti_dual.e4235) + (this.e12345 * right_anti_dual.e25), (this.e235 * right_anti_dual.e4315) + (this.e12345 * right_anti_dual.e35), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) + float4((right_anti_dual.group9.www * this.group1.xyz), this.e12345 * right_anti_dual.e45) - (right_anti_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e12345) * right_anti_dual.group4.xyz) + (float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group9.yzx) - (this.group0.xyz.yzx * right_anti_dual.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e12345) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.e3215) * this.group0.xyz) + (float3(right_anti_dual.e1234) * this.group2.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * right_anti_dual.group6) + (float4(right_anti_dual.e12345) * this.group1), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e12345) * right_anti_dual.group7.xyz) + (float3(right_anti_dual.e12345) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e12345) * right_anti_dual.group8.xyz) + (float3(right_anti_dual.e12345) * this.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightContraction<Plane> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension CircleRotor: WeightContraction<RoundPoint> {
    associatedtype Output = DipoleInversion;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * right_anti_dual.group0.yzx) - (this.group0.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e1234), (this.e431 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e1234), (this.e412 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e1234), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (this.group1.wwwx * right_anti_dual.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e3215) * this.group1.xyz) + (this.group2.yzx * right_anti_dual.group0.zxy) - (this.group2.zxy * right_anti_dual.group0.yzx)), this.e12345 * right_anti_dual.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0
        );
    }
}
extension CircleRotor: WeightContraction<Scalar> {
    associatedtype Output = CircleRotor;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.e12345) * this.group2
        );
    }
}
extension CircleRotor: WeightContraction<Sphere> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: WeightContraction<VersorEven> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: VersorEven) -> Output {
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
extension CircleRotor: WeightContraction<VersorOdd> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: VersorOdd) -> Output {
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
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<Dipole> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension Dipole: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43))
        );
    }
}
extension Dipole: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_anti_dual.group2.www), this.e41 * right_anti_dual.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightContraction<AntiDualNum> {
    associatedtype Output = Dipole;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(right_anti_dual.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: WeightContraction<AntiFlector> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightContraction<AntiLine> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightContraction<AntiMotor> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435))
        );
    }
}
extension Dipole: WeightContraction<AntiPlane> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightContraction<Dipole> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightContraction<DipoleInversion> {
    associatedtype Output = Scalar;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightContraction<DualNum> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: DualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension Dipole: WeightContraction<FlatPoint> {
    associatedtype Output = Scalar;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightContraction<Flector> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightContraction<Motor> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.group0.xyz * right_anti_dual.group1.www), right_anti_dual.e3215 * this.e45)
        );
    }
}
extension Dipole: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group9.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_anti_dual.e1234) * this.group2.xyz), this.e41 * right_anti_dual.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e12345) * this.group1.xyz, 0.0), 
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
extension Dipole: WeightContraction<RoundPoint> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group0.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_anti_dual.e1234) * this.group2.xyz), this.e41 * right_anti_dual.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightContraction<Scalar> {
    associatedtype Output = Dipole;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(right_anti_dual.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: WeightContraction<VersorEven> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: VersorEven) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_anti_dual.group2.www), this.e41 * right_anti_dual.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: WeightContraction<VersorOdd> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz), -(this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412))
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<DipoleInversion> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension DipoleInversion: WeightContraction<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension DipoleInversion: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)) + float4(float3(0.0), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234) - (right_anti_dual.e1234 * this.e45) - (right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) - float4((this.group3.www * right_anti_dual.group3.xyz), right_anti_dual.e4235 * this.e41), 
            /* e1, e2, e3, e5 */
            (right_anti_dual.group3.zxyw * this.group1.yzxw) + (this.group2.wwwz * float4(right_anti_dual.group2.xyz, right_anti_dual.e4125)) + float4((this.group0.xyz * right_anti_dual.group3.www), right_anti_dual.e4235 * this.e15) + float4((right_anti_dual.group1.zxy * this.group3.yzx), right_anti_dual.e4315 * this.e25) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15)) - float4((right_anti_dual.group3.yzx * this.group1.zxy), right_anti_dual.e35 * this.e4125)
        );
    }
}
extension DipoleInversion: WeightContraction<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension DipoleInversion: WeightContraction<AntiFlatPoint> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(-(this.e4235 * right_anti_dual.e15) - (this.e4315 * right_anti_dual.e25) - (this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightContraction<AntiFlector> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_anti_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * right_anti_dual.group1.yzx) - (this.group3.yzx * right_anti_dual.group1.zxy)), this.e1234 * right_anti_dual.e3215), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) + float4((right_anti_dual.group1.www * this.group3.xyz), this.e1234 * right_anti_dual.e45) - (right_anti_dual.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e4315)) + (right_anti_dual.group1.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (this.e35 * right_anti_dual.e4125) - (this.e4315 * right_anti_dual.e25) - (this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45)) - float4((this.group1.zxy * right_anti_dual.group1.yzx), this.e4235 * right_anti_dual.e15)
        );
    }
}
extension DipoleInversion: WeightContraction<AntiLine> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz), -(this.e4235 * right_anti_dual.e415) - (this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)), 
            /* e15, e25, e35, scalar */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435)) - float4((right_anti_dual.group1.xyz.zxy * this.group3.yzx), this.e41 * right_anti_dual.e235)
        );
    }
}
extension DipoleInversion: WeightContraction<AntiMotor> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension DipoleInversion: WeightContraction<AntiPlane> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * right_anti_dual.group0.yzx) - (this.group3.yzx * right_anti_dual.group0.zxy)), this.e1234 * right_anti_dual.e3215), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * right_anti_dual.e3215, this.e4315 * right_anti_dual.e3215, this.e4125 * right_anti_dual.e3215, -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * right_anti_dual.e4315 * -1.0, this.e23 * right_anti_dual.e4125 * -1.0, this.e31 * right_anti_dual.e4235 * -1.0, (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
extension DipoleInversion: WeightContraction<Circle> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group3.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e42)) + float4((right_anti_dual.group2.xyz * this.group2.www), right_anti_dual.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightContraction<CircleRotor> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)) + float4((this.group2.www * right_anti_dual.group2.xyz), right_anti_dual.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightContraction<Dipole> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group3.yzx) - (right_anti_dual.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((right_anti_dual.group2.xyz.zxy * this.group3.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
extension DipoleInversion: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group3.yzx) - (right_anti_dual.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4)) + (this.group3.zxyx * float4(right_anti_dual.group2.yzx, right_anti_dual.e1)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((right_anti_dual.group2.zxy * this.group3.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
extension DipoleInversion: WeightContraction<DualNum> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension DipoleInversion: WeightContraction<FlatPoint> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e235 * this.e1234, right_anti_dual.e315 * this.e1234, right_anti_dual.e125 * this.e1234, -(right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - float4((right_anti_dual.group0.www * this.group3.xyz), right_anti_dual.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension DipoleInversion: WeightContraction<Flector> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) + float4((this.group2.www * right_anti_dual.group0.xyz), right_anti_dual.e1 * this.e4235) - float4((right_anti_dual.group0.www * this.group3.xyz), right_anti_dual.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension DipoleInversion: WeightContraction<Line> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group3.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e15))
        );
    }
}
extension DipoleInversion: WeightContraction<Motor> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) + float4((this.group0.xyz * right_anti_dual.group1.www), right_anti_dual.e3215 * this.e45) - (this.group3.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e15))
        );
    }
}
extension DipoleInversion: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e1234 * right_anti_dual.e5) + (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435) - (this.e45 * right_anti_dual.e321) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * right_anti_dual.group3) + (this.group3.yzxy * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e42)) + float4((this.group0.xyz * right_anti_dual.group9.www), this.e4235 * right_anti_dual.e41) + float4((this.group1.yzx * right_anti_dual.group9.zxy), this.e4125 * right_anti_dual.e43) - (float4(right_anti_dual.e1234) * float4(this.group2.xyz, this.e45)) - (right_anti_dual.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((right_anti_dual.group4.xyz * this.group3.www), this.e41 * right_anti_dual.e4235) - float4((right_anti_dual.group5.xyz.yzx * this.group3.zxy), this.e42 * right_anti_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * right_anti_dual.e3215) + (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) - (this.e4235 * right_anti_dual.e15) - (this.e4315 * right_anti_dual.e25) - (this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) + (float4(right_anti_dual.e12345) * float4(this.group2.xyz, this.e45)) - float4((right_anti_dual.group8.xyz.zxy * this.group3.yzx), this.e4235 * right_anti_dual.e415), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group6.xyz) + (float3(right_anti_dual.e12345) * this.group0.xyz) + (right_anti_dual.group7.xyz.zxy * this.group3.yzx) - (right_anti_dual.group7.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * right_anti_dual.group8.xyz) + (float3(this.e3215) * right_anti_dual.group7.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz) - (float3(right_anti_dual.e321) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group9.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group9.zxy, right_anti_dual.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group9.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group3.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * right_anti_dual.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightContraction<Plane> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightContraction<RoundPoint> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group0.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group0.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * right_anti_dual.e3215, this.e4315 * right_anti_dual.e3215, this.e4125 * right_anti_dual.e3215, -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * right_anti_dual.e4315) - (this.e15 * right_anti_dual.e1234), -(this.e23 * right_anti_dual.e4125) - (this.e25 * right_anti_dual.e1234), -(this.e31 * right_anti_dual.e4235) - (this.e35 * right_anti_dual.e1234), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125)) + (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
extension DipoleInversion: WeightContraction<Scalar> {
    associatedtype Output = DipoleInversion;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3
        );
    }
}
extension DipoleInversion: WeightContraction<Sphere> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1234 * right_anti_dual.e5) + (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: WeightContraction<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (this.e4235 * right_anti_dual.e41) + (this.e4315 * right_anti_dual.e42) + (this.e4125 * right_anti_dual.e43) - (this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125) - (this.e45 * right_anti_dual.e1234)) + float4((right_anti_dual.group3.www * this.group3.xyz), this.e1234 * right_anti_dual.e45) - (right_anti_dual.group3.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(right_anti_dual.group2.xyz, right_anti_dual.e4315)) + (right_anti_dual.group3.zxyx * float4(this.group1.yzx, this.e15)) + float4((this.group3.yzx * right_anti_dual.group1.zxy), this.e35 * right_anti_dual.e4125) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (this.group3.zxyz * float4(right_anti_dual.group1.yzx, right_anti_dual.e35)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - float4((this.group1.zxy * right_anti_dual.group3.yzx), this.e4235 * right_anti_dual.e15)
        );
    }
}
extension DipoleInversion: WeightContraction<VersorOdd> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return VersorOdd.from_groups(
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
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<DualNum> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension DualNum: WeightContraction<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension DualNum: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return VersorOdd.from_groups(
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
extension DualNum: WeightContraction<AntiDualNum> {
    associatedtype Output = DualNum;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((right_anti_dual.e5 * this.e12345) + (right_anti_dual.e12345 * this.e5), right_anti_dual.e12345 * this.e12345), 0.0, 0.0)
        );
    }
}
extension DualNum: WeightContraction<AntiFlatPoint> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
extension DualNum: WeightContraction<AntiFlector> {
    associatedtype Output = Flector;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
extension DualNum: WeightContraction<AntiLine> {
    associatedtype Output = Line;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_anti_dual.group1.xyz, 0.0)
        );
    }
}
extension DualNum: WeightContraction<AntiMotor> {
    associatedtype Output = Motor;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.yy, this.e12345, (this.e5 * right_anti_dual.e12345) + (this.e12345 * right_anti_dual.e5)) * float4(right_anti_dual.group1.xyz, 1.0)
        );
    }
}
extension DualNum: WeightContraction<AntiPlane> {
    associatedtype Output = Plane;
    func weight_contraction(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
extension DualNum: WeightContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: WeightContraction<Circle> {
    associatedtype Output = Dipole;
    func weight_contraction(other: Circle) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * right_anti_dual.group2.xyz, 0.0)
        );
    }
}
extension DualNum: WeightContraction<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: CircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * right_anti_dual.group2
        );
    }
}
extension DualNum: WeightContraction<Dipole> {
    associatedtype Output = Circle;
    func weight_contraction(other: Dipole) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_anti_dual.group2.xyz, 0.0)
        );
    }
}
extension DualNum: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: DipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * right_anti_dual.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_anti_dual.group3
        );
    }
}
extension DualNum: WeightContraction<DualNum> {
    associatedtype Output = AntiDualNum;
    func weight_contraction(other: DualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0), 0.0, 0.0)
        );
    }
}
extension DualNum: WeightContraction<FlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func weight_contraction(other: FlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
extension DualNum: WeightContraction<Flector> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
extension DualNum: WeightContraction<Line> {
    associatedtype Output = AntiLine;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * right_anti_dual.group1.xyz, 0.0)
        );
    }
}
extension DualNum: WeightContraction<Motor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
extension DualNum: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e5 * right_anti_dual.e1234) + (this.e12345 * right_anti_dual.scalar), this.e12345 * right_anti_dual.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e5, 0, 0, 0 */
            float4((this.e5 * right_anti_dual.e12345) + (this.e12345 * right_anti_dual.e5), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_anti_dual.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_anti_dual.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_anti_dual.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_anti_dual.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_anti_dual.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: WeightContraction<Plane> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension DualNum: WeightContraction<RoundPoint> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension DualNum: WeightContraction<Scalar> {
    associatedtype Output = DualNum;
    func weight_contraction(other: Scalar) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: WeightContraction<Sphere> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: WeightContraction<VersorEven> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: VersorEven) -> Output {
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
extension DualNum: WeightContraction<VersorOdd> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: VersorOdd) -> Output {
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
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<FlatPoint> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension FlatPoint: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
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
            float4(float3(0.0), right_anti_dual.e12345 * this.e45), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.group2.www * this.group0.xyz), -(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45))
        );
    }
}
extension FlatPoint: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e4235 * this.e15) + (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightContraction<AntiDualNum> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: AntiDualNum) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
extension FlatPoint: WeightContraction<AntiFlector> {
    associatedtype Output = DualNum;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightContraction<AntiMotor> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: AntiMotor) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
extension FlatPoint: WeightContraction<AntiPlane> {
    associatedtype Output = DualNum;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightContraction<Dipole> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightContraction<DipoleInversion> {
    associatedtype Output = Scalar;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightContraction<DualNum> {
    associatedtype Output = DualNum;
    func weight_contraction(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.e5 * this.e45 * -1.0, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightContraction<FlatPoint> {
    associatedtype Output = Scalar;
    func weight_contraction(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e45 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightContraction<Flector> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e45 * other.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightContraction<Motor> {
    associatedtype Output = DualNum;
    func weight_contraction(other: Motor) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(float4(other.group1.xyz, other.e5 * -1.0)[3] * this.e45, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * this.group0, 
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
extension FlatPoint: WeightContraction<RoundPoint> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightContraction<Scalar> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: Scalar) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
extension FlatPoint: WeightContraction<VersorEven> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: VersorEven) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: WeightContraction<VersorOdd> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45 * right_anti_dual.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz), -(this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321))
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<Flector> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension Flector: WeightContraction<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension Flector: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group3.yzx * this.group1.zxy) - (right_anti_dual.group3.zxy * this.group1.yzx)), right_anti_dual.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)) + float4(float3(0.0), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) - float4((this.group1.www * right_anti_dual.group3.xyz), right_anti_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e35 * this.e4125)) + float4((right_anti_dual.group1.zxy * this.group1.yzx), right_anti_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (right_anti_dual.group2.wwwy * float4(this.group0.xyz, this.e4315)) - (this.group1.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15))
        );
    }
}
extension Flector: WeightContraction<AntiDualNum> {
    associatedtype Output = Flector;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
extension Flector: WeightContraction<AntiFlatPoint> {
    associatedtype Output = DualNum;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(-(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension Flector: WeightContraction<AntiFlector> {
    associatedtype Output = Motor;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual.group1.yzx * this.group1.zxy) - (right_anti_dual.group1.zxy * this.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (right_anti_dual.group1.wwwx * float4(this.group1.xyz, this.e15)) + float4(float3(0.0), (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215)) - (this.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e15))
        );
    }
}
extension Flector: WeightContraction<AntiLine> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) - (this.group1.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e415))
        );
    }
}
extension Flector: WeightContraction<AntiMotor> {
    associatedtype Output = Flector;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) + (float4(right_anti_dual.e12345) * this.group0) - (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
extension Flector: WeightContraction<AntiPlane> {
    associatedtype Output = Motor;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((this.group1.zxy * right_anti_dual.group0.yzx) - (this.group1.yzx * right_anti_dual.group0.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(this.e3215 * right_anti_dual.e4235 * -1.0, this.e3215 * right_anti_dual.e4315 * -1.0, this.e3215 * right_anti_dual.e4125 * -1.0, (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215)) + (right_anti_dual.group0.wwwx * float4(this.group1.xyz, this.e15))
        );
    }
}
extension Flector: WeightContraction<Circle> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: WeightContraction<CircleRotor> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: WeightContraction<Dipole> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx) - (right_anti_dual.group0.xyz.yzx * this.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e423 * this.e3215, right_anti_dual.e431 * this.e3215, right_anti_dual.e412 * this.e3215, -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)) - float4((right_anti_dual.group2.xyz.zxy * this.group1.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
extension Flector: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx) - (right_anti_dual.group0.xyz.yzx * this.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e423 * this.e3215, right_anti_dual.e431 * this.e3215, right_anti_dual.e412 * this.e3215, -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4)) + (this.group1.zxyx * float4(right_anti_dual.group2.yzx, right_anti_dual.e1)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e321 * this.e45)) - float4((right_anti_dual.group2.zxy * this.group1.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
extension Flector: WeightContraction<DualNum> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e45)
        );
    }
}
extension Flector: WeightContraction<FlatPoint> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e321) * float4(this.group1.xyz, this.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group1.zxy) - (right_anti_dual.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
extension Flector: WeightContraction<Flector> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual.group0.www * this.group1.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) - (right_anti_dual.e321 * this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group1.zxy) - (right_anti_dual.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
extension Flector: WeightContraction<Line> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12 * this.e4315, right_anti_dual.e23 * this.e4125, right_anti_dual.e31 * this.e4235, -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group1.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e15))
        );
    }
}
extension Flector: WeightContraction<Motor> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) + float4((right_anti_dual.group0.zxy * this.group1.yzx), right_anti_dual.e3215 * this.e45) - (this.group1.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e15))
        );
    }
}
extension Flector: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(this.e15 * right_anti_dual.e1234) - (this.e4125 * right_anti_dual.e31), -(this.e25 * right_anti_dual.e1234) - (this.e4235 * right_anti_dual.e12), -(this.e35 * right_anti_dual.e1234) - (this.e4315 * right_anti_dual.e23), (this.e4315 * right_anti_dual.e42) + (this.e4125 * right_anti_dual.e43)) + (this.group1.yzxx * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e41)) - float4((right_anti_dual.group4.xyz * this.group1.www), this.e45 * right_anti_dual.e1234), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215) - (this.e4235 * right_anti_dual.e15) - (this.e4315 * right_anti_dual.e25) - (this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) + (float4(right_anti_dual.e12345) * this.group0) - float4((right_anti_dual.group8.xyz.zxy * this.group1.yzx), this.e4235 * right_anti_dual.e415), 
            /* e41, e42, e43, 0 */
            float4((right_anti_dual.group7.xyz.zxy * this.group1.yzx) - (right_anti_dual.group7.xyz.yzx * this.group1.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_anti_dual.group7.xyz) - (float3(right_anti_dual.e321) * this.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_anti_dual.group9.yzx) - (this.group1.yzx * right_anti_dual.group9.zxy)), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group1.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: WeightContraction<Plane> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: WeightContraction<RoundPoint> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_anti_dual.group0.yzx) - (this.group1.yzx * right_anti_dual.group0.zxy)), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual.e3215) * this.group1.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz)), this.e45 * right_anti_dual.e1234 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(right_anti_dual.e1234) * this.group0.xyz * float3(-1.0)), (this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215))
        );
    }
}
extension Flector: WeightContraction<Scalar> {
    associatedtype Output = Flector;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
extension Flector: WeightContraction<Sphere> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: WeightContraction<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * right_anti_dual.group3.yzx) - (this.group1.yzx * right_anti_dual.group3.zxy)), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)) + float4(float3(0.0), (this.e4315 * right_anti_dual.e42) + (this.e4125 * right_anti_dual.e43)) - float4((this.group1.www * right_anti_dual.group3.xyz), this.e45 * right_anti_dual.e1234), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215) - (this.e3215 * right_anti_dual.e45)) + float4((this.group1.yzx * right_anti_dual.group1.zxy), this.e15 * right_anti_dual.e4235) - (this.group1.zxyy * float4(right_anti_dual.group1.yzx, right_anti_dual.e25)) - (this.group1.wwwz * float4(right_anti_dual.group0.xyz, right_anti_dual.e35)) - (right_anti_dual.group2.wwwx * float4(this.group0.xyz, this.e4235))
        );
    }
}
extension Flector: WeightContraction<VersorOdd> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return VersorOdd.from_groups(
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
extension Line {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<Line> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension Line: WeightContraction<AntiCircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
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
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group1.xyz), -(right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
extension Line: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz), -(right_anti_dual.e4235 * this.e415) - (right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)) - float4((this.group1.xyz.zxy * right_anti_dual.group3.yzx), right_anti_dual.e41 * this.e235)
        );
    }
}
extension Line: WeightContraction<AntiDualNum> {
    associatedtype Output = Line;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: WeightContraction<AntiFlector> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) - (right_anti_dual.group1.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
extension Line: WeightContraction<AntiLine> {
    associatedtype Output = DualNum;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(-(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0), 0.0, 0.0)
        );
    }
}
extension Line: WeightContraction<AntiMotor> {
    associatedtype Output = Motor;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(right_anti_dual.e12345) * this.group1.xyz), -(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435))
        );
    }
}
extension Line: WeightContraction<AntiPlane> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
extension Line: WeightContraction<Circle> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: WeightContraction<CircleRotor> {
    associatedtype Output = Scalar;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: WeightContraction<Dipole> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_anti_dual.e423 * this.e415), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: WeightContraction<DipoleInversion> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group1.xyz.zxy), right_anti_dual.e423 * this.e415), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: WeightContraction<DualNum> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: WeightContraction<FlatPoint> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(right_anti_dual.e321) * this.group0.xyz), -(right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435))
        );
    }
}
extension Line: WeightContraction<Flector> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(right_anti_dual.e321) * this.group0.xyz), -(right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435))
        );
    }
}
extension Line: WeightContraction<Line> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: WeightContraction<Motor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2(-(this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_anti_dual.e321) + (this.e315 * right_anti_dual.e412), (this.e425 * right_anti_dual.e321) + (this.e125 * right_anti_dual.e423), (this.e435 * right_anti_dual.e321) + (this.e235 * right_anti_dual.e431), -(this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) - float4((this.group1.xyz.zxy * right_anti_dual.group7.xyz.yzx), this.e415 * right_anti_dual.e423), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group9.yzxx * float4(this.group1.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e1234) * this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e12345) * this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Line: WeightContraction<RoundPoint> {
    associatedtype Output = Dipole;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz), -(this.e415 * right_anti_dual.e4235) - (this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)), 
            /* e15, e25, e35, 0 */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz) + (this.group1.xyz.yzx * right_anti_dual.group0.zxy) - (this.group1.xyz.zxy * right_anti_dual.group0.yzx), 0.0)
        );
    }
}
extension Line: WeightContraction<Scalar> {
    associatedtype Output = Line;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(right_anti_dual.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: WeightContraction<VersorEven> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz), -(this.e415 * right_anti_dual.e4235) - (this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43)) - float4((this.group1.xyz.zxy * right_anti_dual.group3.yzx), this.e415 * right_anti_dual.e23)
        );
    }
}
extension Line: WeightContraction<VersorOdd> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual.e12345) * this.group1.xyz), -(this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * right_anti_dual.e321) + (this.e315 * right_anti_dual.e412), (this.e425 * right_anti_dual.e321) + (this.e125 * right_anti_dual.e423), (this.e435 * right_anti_dual.e321) + (this.e235 * right_anti_dual.e431), -(this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435)) - float4((this.group1.xyz.zxy * right_anti_dual.group0.yzx), this.e415 * right_anti_dual.e235)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<Motor> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension Motor: WeightContraction<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension Motor: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return VersorOdd.from_groups(
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
extension Motor: WeightContraction<AntiDualNum> {
    associatedtype Output = Motor;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.group0.xy.yy, right_anti_dual.e12345, (right_anti_dual.e5 * this.e12345) + (right_anti_dual.e12345 * this.e5)) * float4(this.group1.xyz, 1.0)
        );
    }
}
extension Motor: WeightContraction<AntiFlatPoint> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
extension Motor: WeightContraction<AntiFlector> {
    associatedtype Output = Flector;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(this.e12345) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group1
        );
    }
}
extension Motor: WeightContraction<AntiLine> {
    associatedtype Output = Motor;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e12345) * right_anti_dual.group1.xyz), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435))
        );
    }
}
extension Motor: WeightContraction<AntiMotor> {
    associatedtype Output = Motor;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((float3(right_anti_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz)), right_anti_dual.e12345 * this.e12345), 
            /* e235, e315, e125, e5 */
            (float4(right_anti_dual.e12345) * this.group1) + (float4(this.e12345) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435))
        );
    }
}
extension Motor: WeightContraction<AntiPlane> {
    associatedtype Output = Flector;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0
        );
    }
}
extension Motor: WeightContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: WeightContraction<Circle> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435))
        );
    }
}
extension Motor: WeightContraction<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: CircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435))
        );
    }
}
extension Motor: WeightContraction<Dipole> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group1.zxy), right_anti_dual.e235 * this.e415)
        );
    }
}
extension Motor: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: DipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.e4 * this.e12345) - (right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e321 * this.e415) + (right_anti_dual.e1 * this.e12345), (right_anti_dual.e321 * this.e425) + (right_anti_dual.e2 * this.e12345), (right_anti_dual.e321 * this.e435) + (right_anti_dual.e3 * this.e12345), -(right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + float4((right_anti_dual.group0.xyz.zxy * this.group1.yzx), right_anti_dual.e5 * this.e12345) - (this.group1.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e415))
        );
    }
}
extension Motor: WeightContraction<DualNum> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_anti_dual.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e3215) * this.group0
        );
    }
}
extension Motor: WeightContraction<FlatPoint> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.group0.www * this.group0.xyz), -(right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435))
        );
    }
}
extension Motor: WeightContraction<Flector> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e1 * this.e12345, right_anti_dual.e2 * this.e12345, right_anti_dual.e3 * this.e12345, -(right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + (this.group0 * float4(right_anti_dual.group0.www, right_anti_dual.e5))
        );
    }
}
extension Motor: WeightContraction<Line> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: WeightContraction<Motor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual.e3215) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz)), right_anti_dual.e3215 * this.e12345)
        );
    }
}
extension Motor: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((this.e12345 * right_anti_dual.scalar) + (this.e5 * right_anti_dual.e1234) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), this.e12345 * right_anti_dual.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * right_anti_dual.e321) + (this.e12345 * right_anti_dual.e1), (this.e425 * right_anti_dual.e321) + (this.e12345 * right_anti_dual.e2), (this.e435 * right_anti_dual.e321) + (this.e12345 * right_anti_dual.e3), -(this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412)) + float4((right_anti_dual.group7.xyz.zxy * this.group1.yzx), this.e12345 * right_anti_dual.e4) - float4((right_anti_dual.group7.xyz.yzx * this.group1.zxy), this.e415 * right_anti_dual.e423), 
            /* e5, 0, 0, 0 */
            float4((this.e12345 * right_anti_dual.e5) + (this.e5 * right_anti_dual.e12345) - (this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e12345 * right_anti_dual.e15) + (this.e315 * right_anti_dual.e4125), (this.e12345 * right_anti_dual.e25) + (this.e125 * right_anti_dual.e4235), (this.e12345 * right_anti_dual.e35) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) + (this.group0 * float4(right_anti_dual.group9.www, right_anti_dual.e45)) - (right_anti_dual.group9.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e12345) * right_anti_dual.group4.xyz) + (float3(right_anti_dual.e1234) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e12345) * right_anti_dual.group5.xyz) + (float3(right_anti_dual.e1234) * this.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * right_anti_dual.group6.xyz) + (float3(right_anti_dual.e12345) * this.group0.xyz)), this.e12345 * right_anti_dual.e321), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_anti_dual.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e12345) * right_anti_dual.group8.xyz) + (float3(right_anti_dual.e12345) * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: WeightContraction<Plane> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension Motor: WeightContraction<RoundPoint> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension Motor: WeightContraction<Scalar> {
    associatedtype Output = Motor;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e12345) * this.group1
        );
    }
}
extension Motor: WeightContraction<Sphere> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: WeightContraction<VersorEven> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: VersorEven) -> Output {
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
extension Motor: WeightContraction<VersorOdd> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: VersorOdd) -> Output {
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
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<MultiVector> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension MultiVector: WeightContraction<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: AntiCircleRotor) -> Output {
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
            float4(float2((right_anti_dual.e12345 * this.scalar) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), right_anti_dual.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412) + (right_anti_dual.e12345 * this.e1), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423) + (right_anti_dual.e12345 * this.e2), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431) + (right_anti_dual.e12345 * this.e3), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) + float4((right_anti_dual.group0.xyz.zxy * this.group8.xyz.yzx), right_anti_dual.e12345 * this.e4) - float4((right_anti_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_anti_dual.e423 * this.e415) - float4((this.group7.xyz.yzx * right_anti_dual.group2.zxy), right_anti_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e12345 * this.e5) - (right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e315 * this.e4125) + (right_anti_dual.e12345 * this.e15), (right_anti_dual.e125 * this.e4235) + (right_anti_dual.e12345 * this.e25), (right_anti_dual.e235 * this.e4315) + (right_anti_dual.e12345 * this.e35), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) + float4((this.group9.www * right_anti_dual.group1.xyz), right_anti_dual.e12345 * this.e45) - (this.group9.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e12345) * this.group4.xyz) + (float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group9.yzx) - (right_anti_dual.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e12345) * this.group5.xyz) + (float3(this.e3215) * right_anti_dual.group0.xyz) + (float3(this.e1234) * right_anti_dual.group2.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(right_anti_dual.e12345) * this.group6) + (float4(this.e12345) * right_anti_dual.group1), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.e12345) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e12345) * this.group8.xyz) + (float3(this.e12345) * right_anti_dual.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
            float4(float2((right_anti_dual.e1234 * this.e5) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)) + (this.group9.yzxz * float4(right_anti_dual.group1.zxy, right_anti_dual.e43)) + float4((this.group4.xyz * right_anti_dual.group3.www), right_anti_dual.e41 * this.e4235) + float4((this.group5.xyz.yzx * right_anti_dual.group3.zxy), right_anti_dual.e42 * this.e4315) - (float4(right_anti_dual.e1234) * this.group3) - (right_anti_dual.group3.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((right_anti_dual.group0.xyz * this.group9.www), right_anti_dual.e4235 * this.e41) - float4((right_anti_dual.group1.yzx * this.group9.zxy), right_anti_dual.e4125 * this.e43), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e4235 * this.e15) + (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(this.e12345) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)) - float4((this.group8.xyz.zxy * right_anti_dual.group3.yzx), right_anti_dual.e4235 * this.e415), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group6.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz) + (this.group7.xyz.zxy * right_anti_dual.group3.yzx) - (this.group7.xyz.yzx * right_anti_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e1234) * this.group8.xyz) + (float3(right_anti_dual.e3215) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz) - (float3(this.e321) * right_anti_dual.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e1234 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightContraction<AntiDualNum> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_anti_dual.e5 * this.e1234) + (right_anti_dual.e12345 * this.scalar), right_anti_dual.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e5 * this.e12345) + (right_anti_dual.e12345 * this.e5), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e12345) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e12345) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightContraction<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * right_anti_dual.group0, 
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
extension MultiVector: WeightContraction<AntiFlector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e15 * this.e1234) + (right_anti_dual.e4125 * this.e31), (right_anti_dual.e25 * this.e1234) + (right_anti_dual.e4235 * this.e12), (right_anti_dual.e35 * this.e1234) + (right_anti_dual.e4315 * this.e23), -(right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) + float4((this.group4.xyz * right_anti_dual.group1.www), right_anti_dual.e45 * this.e1234) - (right_anti_dual.group1.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e4235 * this.e15) + (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(this.e12345) * right_anti_dual.group0) - float4((this.group8.xyz.zxy * right_anti_dual.group1.yzx), right_anti_dual.e4235 * this.e415), 
            /* e41, e42, e43, 0 */
            float4((this.group7.xyz.zxy * right_anti_dual.group1.yzx) - (this.group7.xyz.yzx * right_anti_dual.group1.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_anti_dual.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group1.yzx * this.group9.zxy) - (right_anti_dual.group1.zxy * this.group9.yzx)), right_anti_dual.e3215 * this.e1234), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_anti_dual.group1.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightContraction<AntiLine> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) - float4((right_anti_dual.group1.xyz.zxy * this.group7.xyz.yzx), right_anti_dual.e415 * this.e423), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * right_anti_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_anti_dual.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightContraction<AntiMotor> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), right_anti_dual.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e12345 * this.e1), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e12345 * this.e2), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e12345 * this.e3), -(right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) + float4((this.group7.xyz.zxy * right_anti_dual.group1.yzx), right_anti_dual.e12345 * this.e4) - float4((this.group7.xyz.yzx * right_anti_dual.group1.zxy), right_anti_dual.e415 * this.e423), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e12345 * this.e5) + (right_anti_dual.e5 * this.e12345) - (right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e12345 * this.e15) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e12345 * this.e25) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e12345 * this.e35) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) + (right_anti_dual.group0 * float4(this.group9.www, this.e45)) - (this.group9.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e12345) * this.group4.xyz) + (float3(this.e1234) * right_anti_dual.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e12345) * this.group5.xyz) + (float3(this.e1234) * right_anti_dual.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual.e12345) * this.group6.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz)), right_anti_dual.e12345 * this.e321), 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e12345) * this.group8.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightContraction<AntiPlane> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e42 * right_anti_dual.e4315) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((this.group7.xyz.zxy * right_anti_dual.group0.yzx) - (this.group7.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_anti_dual.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group9.zxy * right_anti_dual.group0.yzx) - (this.group9.yzx * right_anti_dual.group0.zxy)), this.e1234 * right_anti_dual.e3215), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_anti_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightContraction<Circle> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: Circle) -> Output {
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
            float4(float2(-(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group9.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e42)) + float4((float3(this.e1234) * right_anti_dual.group2.xyz), right_anti_dual.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_anti_dual.group1.xyz, 0.0), 
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
extension MultiVector: WeightContraction<CircleRotor> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: CircleRotor) -> Output {
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
            float4(float2((right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group9.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)) + float4((float3(this.e1234) * right_anti_dual.group2.xyz), right_anti_dual.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_anti_dual.group1.xyz, 0.0), 
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
extension MultiVector: WeightContraction<Dipole> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: Dipole) -> Output {
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
            float4(float2(-(right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) - float4((right_anti_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_anti_dual.e423 * this.e415) - float4((right_anti_dual.group2.xyz.zxy * this.group7.xyz.yzx), right_anti_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_anti_dual.group2.xyz.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group9.yzx) - (right_anti_dual.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_anti_dual.group0.xyz) + (float3(this.e1234) * right_anti_dual.group2.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_anti_dual.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightContraction<DipoleInversion> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: DipoleInversion) -> Output {
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
            float4(float2((right_anti_dual.e4 * this.e3215) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)) + (float4(this.e12345) * float4(right_anti_dual.group3.xyz, right_anti_dual.e4)) - float4((right_anti_dual.group0.xyz.yzx * this.group8.xyz.zxy), right_anti_dual.e423 * this.e415) - float4((this.group7.xyz.yzx * right_anti_dual.group2.zxy), right_anti_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e5 * this.e12345) - (right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group9.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group9.yzx) - (right_anti_dual.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_anti_dual.group0.xyz) + (float3(this.e1234) * right_anti_dual.group2.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_anti_dual.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightContraction<DualNum> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_anti_dual.e3215 * this.e4) + (right_anti_dual.scalar * this.e12345), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e3215 * this.e45, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e3215) * this.group7.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e1234), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e3215) * this.group9.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightContraction<FlatPoint> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual.e321) * this.group6.xyz), 0.0) + float4((this.group7.xyz.zxy * right_anti_dual.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual.group0.yzx * this.group9.zxy) - (right_anti_dual.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e321 * this.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightContraction<Flector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual.e321) * this.group6.xyz), 0.0) + float4((float3(this.e12345) * right_anti_dual.group1.xyz), 0.0) + float4((this.group7.xyz.zxy * right_anti_dual.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e5 * this.e12345) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual.group0.yzx * this.group9.zxy) - (right_anti_dual.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e321 * this.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightContraction<Line> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.xyz.zxy * this.group9.yzx), 0.0) - float4((right_anti_dual.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
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
extension MultiVector: WeightContraction<Motor> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_anti_dual.scalar * this.e12345) + (right_anti_dual.e3215 * this.e4) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual.e3215) * this.group4.xyz), 0.0) + float4((float3(this.e1234) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group0.zxy * this.group9.yzx), 0.0) - float4((right_anti_dual.group0.yzx * this.group9.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e3215 * this.e45) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual.e3215) * this.group6.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e3215) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e1234), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e3215) * this.group9.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual.e3215 * this.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_anti_dual.scalar * this.e12345) + (right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), right_anti_dual.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e15 * this.e1234) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e4125 * this.e31), (right_anti_dual.e25 * this.e1234) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e4235 * this.e12), (right_anti_dual.e35 * this.e1234) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e4315 * this.e23), -(right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412) - (right_anti_dual.e1234 * this.e45)) + (float4(right_anti_dual.e12345) * this.group1) + (float4(this.e12345) * right_anti_dual.group1) + (this.group9.yzxx * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e41)) + float4((this.group4.xyz * right_anti_dual.group9.www), right_anti_dual.e45 * this.e1234) + float4((right_anti_dual.group7.xyz.zxy * this.group8.xyz.yzx), right_anti_dual.e42 * this.e4315) + float4((right_anti_dual.group8.xyz.yzx * this.group7.xyz.zxy), right_anti_dual.e43 * this.e4125) - (right_anti_dual.group9.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((float3(right_anti_dual.e1234) * this.group3.xyz), right_anti_dual.e4125 * this.e43) - float4((right_anti_dual.group4.xyz * this.group9.www), right_anti_dual.e423 * this.e415) - float4((right_anti_dual.group5.xyz.yzx * this.group9.zxy), right_anti_dual.e431 * this.e425) - float4((right_anti_dual.group7.xyz.yzx * this.group8.xyz.zxy), right_anti_dual.e412 * this.e435) - float4((right_anti_dual.group8.xyz.zxy * this.group7.xyz.yzx), right_anti_dual.e4235 * this.e41), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e12345 * this.e5) + (right_anti_dual.e5 * this.e12345) + (right_anti_dual.e4235 * this.e15) + (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215) - (right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125) + (right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235) + (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315) + (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e435 * this.e4125) - (right_anti_dual.e4235 * this.e415) - (right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(right_anti_dual.e12345) * this.group3) + (float4(this.e12345) * right_anti_dual.group3) - (this.group9.yzxx * float4(right_anti_dual.group8.xyz.zxy, right_anti_dual.e415)) - float4((this.group8.xyz.zxy * right_anti_dual.group9.yzx), right_anti_dual.e425 * this.e4315), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e12345) * this.group4.xyz) + (float3(right_anti_dual.e1234) * this.group6.xyz) + (float3(this.e12345) * right_anti_dual.group4.xyz) + (float3(this.e1234) * right_anti_dual.group6.xyz) + (right_anti_dual.group7.xyz.zxy * this.group9.yzx) + (this.group7.xyz.zxy * right_anti_dual.group9.yzx) - (right_anti_dual.group7.xyz.yzx * this.group9.zxy) - (this.group7.xyz.yzx * right_anti_dual.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e12345) * this.group5.xyz) + (float3(right_anti_dual.e3215) * this.group7.xyz) + (float3(right_anti_dual.e1234) * this.group8.xyz) + (float3(this.e12345) * right_anti_dual.group5.xyz) + (float3(this.e3215) * right_anti_dual.group7.xyz) + (float3(this.e1234) * right_anti_dual.group8.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(right_anti_dual.e12345) * this.group6) + (float4(this.e12345) * right_anti_dual.group6) + (right_anti_dual.group9.yzxw * float4(this.group9.zxy, this.e1234)) - float4((right_anti_dual.group9.zxy * this.group9.yzx), right_anti_dual.e1234 * this.e3215), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.e12345) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual.group7.xyz) + (float3(this.e1234) * right_anti_dual.group9.xyz) - (float3(right_anti_dual.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e12345) * this.group8.xyz) + (float3(right_anti_dual.e3215) * this.group9.xyz) + (float3(this.e12345) * right_anti_dual.group8.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual.e12345) * this.group9) + (float4(this.e12345) * right_anti_dual.group9), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e12345 * this.e1234) + (right_anti_dual.e1234 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightContraction<Plane> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e5 * this.e12345, 0.0, 0.0, 0.0), 
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
extension MultiVector: WeightContraction<RoundPoint> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e3215) + (this.e31 * right_anti_dual.e4125), (this.e42 * right_anti_dual.e3215) + (this.e12 * right_anti_dual.e4235), (this.e43 * right_anti_dual.e3215) + (this.e23 * right_anti_dual.e4315), -(this.e45 * right_anti_dual.e1234) - (this.e43 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((float3(right_anti_dual.e1234) * this.group3.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) - (right_anti_dual.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * right_anti_dual.group0.yzx) - (this.group7.xyz.yzx * right_anti_dual.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e3215) * this.group7.xyz) + (float3(right_anti_dual.e1234) * this.group8.xyz) - (float3(this.e321) * right_anti_dual.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * right_anti_dual.group0.yzx), this.e1234 * right_anti_dual.e3215) - (this.group9.yzxw * float4(right_anti_dual.group0.zxy, right_anti_dual.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightContraction<Scalar> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(right_anti_dual.e12345) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e12345) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(right_anti_dual.e12345) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e12345) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual.e12345) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e12345) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(right_anti_dual.e12345) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightContraction<Sphere> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e5, 0.0, 0.0, 0.0), 
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
extension MultiVector: WeightContraction<VersorEven> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: VersorEven) -> Output {
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
            float4(float2((this.e12345 * right_anti_dual.scalar) + (this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234) - (this.e415 * right_anti_dual.e23) - (this.e425 * right_anti_dual.e31) - (this.e435 * right_anti_dual.e12) - (this.e321 * right_anti_dual.e45) - (this.e423 * right_anti_dual.e15) - (this.e431 * right_anti_dual.e25) - (this.e412 * right_anti_dual.e35) - (this.e235 * right_anti_dual.e41) - (this.e315 * right_anti_dual.e42) - (this.e125 * right_anti_dual.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)) + (this.group9.yzxz * float4(right_anti_dual.group1.zxy, right_anti_dual.e43)) + float4((this.group4.xyz * right_anti_dual.group3.www), this.e4235 * right_anti_dual.e41) + float4((this.group5.xyz.yzx * right_anti_dual.group3.zxy), this.e4315 * right_anti_dual.e42) - (right_anti_dual.group3.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((this.group9.zxy * right_anti_dual.group1.yzx), this.e43 * right_anti_dual.e4125) - float4((this.group9.www * right_anti_dual.group0.xyz), this.e45 * right_anti_dual.e1234) - float4((right_anti_dual.group2.www * this.group3.xyz), this.e42 * right_anti_dual.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * right_anti_dual.e4235) + (this.e25 * right_anti_dual.e4315) + (this.e35 * right_anti_dual.e4125) + (this.e45 * right_anti_dual.e3215) - (this.e4235 * right_anti_dual.e15) - (this.e4315 * right_anti_dual.e25) - (this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * right_anti_dual.e3215) + (this.e315 * right_anti_dual.e4125), (this.e425 * right_anti_dual.e3215) + (this.e125 * right_anti_dual.e4235), (this.e435 * right_anti_dual.e3215) + (this.e235 * right_anti_dual.e4315), -(this.e425 * right_anti_dual.e4315) - (this.e435 * right_anti_dual.e4125)) + (float4(this.e12345) * float4(right_anti_dual.group2.xyz, right_anti_dual.e45)) - (right_anti_dual.group3.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * right_anti_dual.group3.yzx) - (this.group7.xyz.yzx * right_anti_dual.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e12345) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e1234) * this.group8.xyz) + (float3(right_anti_dual.e3215) * this.group7.xyz) - (float3(this.e321) * right_anti_dual.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * right_anti_dual.group3.yzx), this.e1234 * right_anti_dual.e3215) - (this.group9.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * right_anti_dual.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: WeightContraction<VersorOdd> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: VersorOdd) -> Output {
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
            float4(float2((this.scalar * right_anti_dual.e12345) + (this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4) + (this.e1234 * right_anti_dual.e5) - (this.e15 * right_anti_dual.e423) - (this.e25 * right_anti_dual.e431) - (this.e35 * right_anti_dual.e412) - (this.e45 * right_anti_dual.e321) - (this.e41 * right_anti_dual.e235) - (this.e42 * right_anti_dual.e315) - (this.e43 * right_anti_dual.e125) - (this.e23 * right_anti_dual.e415) - (this.e31 * right_anti_dual.e425) - (this.e12 * right_anti_dual.e435), this.e12345 * right_anti_dual.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e1 * right_anti_dual.e12345) + (this.e415 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e415) + (this.e315 * right_anti_dual.e412), (this.e2 * right_anti_dual.e12345) + (this.e425 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e425) + (this.e125 * right_anti_dual.e423), (this.e3 * right_anti_dual.e12345) + (this.e435 * right_anti_dual.e321) + (this.e321 * right_anti_dual.e435) + (this.e235 * right_anti_dual.e431), -(this.e415 * right_anti_dual.e423) - (this.e425 * right_anti_dual.e431) - (this.e435 * right_anti_dual.e412) - (this.e412 * right_anti_dual.e435)) + (float4(this.e12345) * right_anti_dual.group3) + float4((this.group7.xyz.zxy * right_anti_dual.group2.yzx), this.e4 * right_anti_dual.e12345) - float4((this.group7.xyz.yzx * right_anti_dual.group2.zxy), this.e423 * right_anti_dual.e415) - float4((this.group8.xyz.zxy * right_anti_dual.group0.yzx), this.e431 * right_anti_dual.e425), 
            /* e5, 0, 0, 0 */
            float4((this.e12345 * right_anti_dual.e5) + (this.e5 * right_anti_dual.e12345) - (this.e415 * right_anti_dual.e235) - (this.e425 * right_anti_dual.e315) - (this.e435 * right_anti_dual.e125) - (this.e235 * right_anti_dual.e415) - (this.e315 * right_anti_dual.e425) - (this.e125 * right_anti_dual.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * right_anti_dual.e315) + (this.e3215 * right_anti_dual.e415), (this.e4235 * right_anti_dual.e125) + (this.e3215 * right_anti_dual.e425), (this.e4315 * right_anti_dual.e235) + (this.e3215 * right_anti_dual.e435), -(this.e4315 * right_anti_dual.e425) - (this.e4125 * right_anti_dual.e435)) + (float4(right_anti_dual.e12345) * this.group3) - (this.group9.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz) + (float3(right_anti_dual.e12345) * this.group4.xyz) + (this.group9.yzx * right_anti_dual.group0.zxy) - (this.group9.zxy * right_anti_dual.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_anti_dual.group0.xyz) + (float3(this.e1234) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e12345) * this.group5.xyz) - (float3(right_anti_dual.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * right_anti_dual.group1) + (float4(right_anti_dual.e12345) * this.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz) + (float3(right_anti_dual.e12345) * this.group7.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz) + (float3(right_anti_dual.e12345) * this.group8.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * right_anti_dual.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<Plane> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension Plane: WeightContraction<AntiCircleRotor> {
    associatedtype Output = DipoleInversion;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e423 * this.e3215, right_anti_dual.e431 * this.e3215, right_anti_dual.e412 * this.e3215, -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * right_anti_dual.group1.xyz), 0.0) + float4((right_anti_dual.group2.yzx * this.group0.zxy), 0.0) - float4((right_anti_dual.group2.zxy * this.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0
        );
    }
}
extension Plane: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group3.yzx * this.group0.zxy) - (right_anti_dual.group3.zxy * this.group0.yzx)), right_anti_dual.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e4235 * this.e3215 * -1.0, right_anti_dual.e4315 * this.e3215 * -1.0, right_anti_dual.e4125 * this.e3215 * -1.0, (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12 * this.e4315, right_anti_dual.e23 * this.e4125, right_anti_dual.e31 * this.e4235, -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15))
        );
    }
}
extension Plane: WeightContraction<AntiDualNum> {
    associatedtype Output = Plane;
    func weight_contraction(other: AntiDualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: WeightContraction<AntiFlatPoint> {
    associatedtype Output = DualNum;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(-(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension Plane: WeightContraction<AntiFlector> {
    associatedtype Output = Motor;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual.e3215 * this.e4235, right_anti_dual.e3215 * this.e4315, right_anti_dual.e3215 * this.e4125, -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215)) - (this.group0.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e15))
        );
    }
}
extension Plane: WeightContraction<AntiLine> {
    associatedtype Output = FlatPoint;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual.group1.xyz.zxy, right_anti_dual.e415))
        );
    }
}
extension Plane: WeightContraction<AntiMotor> {
    associatedtype Output = Flector;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0
        );
    }
}
extension Plane: WeightContraction<AntiPlane> {
    associatedtype Output = Line;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz), 0.0)
        );
    }
}
extension Plane: WeightContraction<Circle> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: WeightContraction<CircleRotor> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: WeightContraction<Dipole> {
    associatedtype Output = Dipole;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e423 * this.e3215, right_anti_dual.e431 * this.e3215, right_anti_dual.e412 * this.e3215, -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(this.e3215) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.yzx * this.group0.zxy) - (right_anti_dual.group2.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Plane: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual.e423 * this.e3215, right_anti_dual.e431 * this.e3215, right_anti_dual.e412 * this.e3215, -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual.e125 * this.e4315 * -1.0, right_anti_dual.e235 * this.e4125 * -1.0, right_anti_dual.e315 * this.e4235 * -1.0, (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)) + (float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4)) + (this.group0.zxyx * float4(right_anti_dual.group2.yzx, right_anti_dual.e1))
        );
    }
}
extension Plane: WeightContraction<DualNum> {
    associatedtype Output = AntiFlatPoint;
    func weight_contraction(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Plane: WeightContraction<FlatPoint> {
    associatedtype Output = AntiLine;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_anti_dual.e321) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Plane: WeightContraction<Flector> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual.group0.www * this.group0.xyz * float3(-1.0)), (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Plane: WeightContraction<Line> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12 * this.e4315, right_anti_dual.e23 * this.e4125, right_anti_dual.e31 * this.e4235, -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e15))
        );
    }
}
extension Plane: WeightContraction<Motor> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e12 * this.e4315, right_anti_dual.e23 * this.e4125, right_anti_dual.e31 * this.e4235, -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e15))
        );
    }
}
extension Plane: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual.group8.xyz.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((right_anti_dual.group7.xyz.zxy * this.group0.yzx) - (right_anti_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_anti_dual.group7.xyz) - (float3(right_anti_dual.e321) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group9.yzx * this.group0.zxy) - (right_anti_dual.group9.zxy * this.group0.yzx)), right_anti_dual.e1234 * this.e3215 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane: WeightContraction<Plane> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: WeightContraction<RoundPoint> {
    associatedtype Output = Circle;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * right_anti_dual.group0.yzx) - (this.group0.yzx * right_anti_dual.group0.zxy)), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz), 0.0)
        );
    }
}
extension Plane: WeightContraction<Scalar> {
    associatedtype Output = Plane;
    func weight_contraction(other: Scalar) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: WeightContraction<Sphere> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e4235 * right_anti_dual.e1) + (this.e4315 * right_anti_dual.e2) + (this.e4125 * right_anti_dual.e3) + (this.e3215 * right_anti_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: WeightContraction<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_anti_dual.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * right_anti_dual.group3.yzx) - (this.group0.yzx * right_anti_dual.group3.zxy)), this.e3215 * right_anti_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * right_anti_dual.e4235 * -1.0, this.e3215 * right_anti_dual.e4315 * -1.0, this.e3215 * right_anti_dual.e4125 * -1.0, (this.e4315 * right_anti_dual.e42) + (this.e4125 * right_anti_dual.e43)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e4315 * right_anti_dual.e12, this.e4125 * right_anti_dual.e23, this.e4235 * right_anti_dual.e31, -(this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15)) - (this.group0.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e25))
        );
    }
}
extension Plane: WeightContraction<VersorOdd> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return VersorOdd.from_groups(
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
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<RoundPoint> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension RoundPoint: WeightContraction<AntiCircleRotor> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e1234 * this.e5) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightContraction<AntiDualNum> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightContraction<AntiFlector> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightContraction<AntiMotor> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightContraction<AntiPlane> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightContraction<DualNum> {
    associatedtype Output = Scalar;
    func weight_contraction(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e5 * this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightContraction<Motor> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(float4(other.group1.xyz, other.e5 * -1.0)[3] * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e5, 0.0, 0.0, 0.0), 
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
extension RoundPoint: WeightContraction<RoundPoint> {
    associatedtype Output = Scalar;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightContraction<Scalar> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightContraction<VersorEven> {
    associatedtype Output = Scalar;
    func weight_contraction(other: VersorEven) -> Output {
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
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * right_anti_dual.e4235) + (this.e2 * right_anti_dual.e4315) + (this.e3 * right_anti_dual.e4125) + (this.e4 * right_anti_dual.e3215) + (this.e5 * right_anti_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: WeightContraction<VersorOdd> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * right_anti_dual.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<Scalar> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension Scalar: WeightContraction<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: WeightContraction<AntiDualNum> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiDualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: WeightContraction<AntiMotor> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: WeightContraction<MultiVector> {
    associatedtype Output = Scalar;
    func weight_contraction(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: WeightContraction<Scalar> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: WeightContraction<VersorOdd> {
    associatedtype Output = Scalar;
    func weight_contraction(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<Sphere> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension Sphere: WeightContraction<AntiCircleRotor> {
    associatedtype Output = DipoleInversion;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.yzx * this.group0.zxy) - (right_anti_dual.group2.zxy * this.group0.yzx)), right_anti_dual.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0
        );
    }
}
extension Sphere: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e4235 * this.e3215 * -1.0, right_anti_dual.e4315 * this.e3215 * -1.0, right_anti_dual.e4125 * this.e3215 * -1.0, (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15))
        );
    }
}
extension Sphere: WeightContraction<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension Sphere: WeightContraction<AntiFlatPoint> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightContraction<AntiFlector> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_anti_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), right_anti_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group1.xyz)), right_anti_dual.e45 * this.e1234), 
            /* e1, e2, e3, e5 */
            float4((float3(this.e1234) * right_anti_dual.group0.xyz), -(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215))
        );
    }
}
extension Sphere: WeightContraction<AntiLine> {
    associatedtype Output = Dipole;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz), -(right_anti_dual.e415 * this.e4235) - (right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)), 
            /* e15, e25, e35, 0 */
            float4((float3(this.e3215) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.xyz.yzx * this.group0.zxy) - (right_anti_dual.group1.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Sphere: WeightContraction<AntiMotor> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension Sphere: WeightContraction<AntiPlane> {
    associatedtype Output = Circle;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)), right_anti_dual.e3215 * this.e1234), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz), 0.0)
        );
    }
}
extension Sphere: WeightContraction<Circle> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group0.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e42)) + float4((float3(this.e1234) * right_anti_dual.group2.xyz), right_anti_dual.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightContraction<CircleRotor> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)) + float4((float3(this.e1234) * right_anti_dual.group2.xyz), right_anti_dual.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightContraction<Dipole> {
    associatedtype Output = Dipole;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(this.e3215) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.yzx * this.group0.zxy) - (right_anti_dual.group2.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Sphere: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group0.yzx) - (right_anti_dual.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual.e125 * this.e4315 * -1.0, right_anti_dual.e235 * this.e4125 * -1.0, right_anti_dual.e315 * this.e4235 * -1.0, (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)) + (float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4)) + (this.group0.zxyx * float4(right_anti_dual.group2.yzx, right_anti_dual.e1))
        );
    }
}
extension Sphere: WeightContraction<DualNum> {
    associatedtype Output = AntiFlatPoint;
    func weight_contraction(other: DualNum) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e1234)
        );
    }
}
extension Sphere: WeightContraction<FlatPoint> {
    associatedtype Output = AntiLine;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e321) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, 0 */
            float4((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Sphere: WeightContraction<Flector> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e321 * this.e4235 * -1.0, right_anti_dual.e321 * this.e4315 * -1.0, right_anti_dual.e321 * this.e4125 * -1.0, (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234)) + float4((float3(this.e1234) * right_anti_dual.group0.xyz), right_anti_dual.e1 * this.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Sphere: WeightContraction<Line> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e15))
        );
    }
}
extension Sphere: WeightContraction<Motor> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e15))
        );
    }
}
extension Sphere: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e45 * this.e1234) + (right_anti_dual.e43 * this.e4125)) + (this.group0.yzxx * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e41)) + float4((float3(this.e1234) * right_anti_dual.group3.xyz), right_anti_dual.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual.group8.xyz.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group6.xyz) + (right_anti_dual.group7.xyz.zxy * this.group0.yzx) - (right_anti_dual.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * right_anti_dual.group7.xyz) + (float3(this.e1234) * right_anti_dual.group8.xyz) - (float3(right_anti_dual.e321) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group9.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_anti_dual.group9.zxy * this.group0.yzx), right_anti_dual.e1234 * this.e3215), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group9.xyz) - (float3(right_anti_dual.e1234) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightContraction<Plane> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightContraction<RoundPoint> {
    associatedtype Output = Circle;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e1234) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group0.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_anti_dual.group0.zxy * this.group0.yzx), right_anti_dual.e1234 * this.e3215), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual.group0.xyz), 0.0)
        );
    }
}
extension Sphere: WeightContraction<Scalar> {
    associatedtype Output = Sphere;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightContraction<Sphere> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: WeightContraction<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group0.zxy * right_anti_dual.group3.yzx), this.e1234 * right_anti_dual.e3215) - (this.group0.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * right_anti_dual.e4235 * -1.0, this.e3215 * right_anti_dual.e4315 * -1.0, this.e3215 * right_anti_dual.e4125 * -1.0, (this.e4315 * right_anti_dual.e42) + (this.e4125 * right_anti_dual.e43) + (this.e1234 * right_anti_dual.e45)) + (this.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4((this.e4315 * right_anti_dual.e12) + (this.e1234 * right_anti_dual.e15), (this.e4125 * right_anti_dual.e23) + (this.e1234 * right_anti_dual.e25), (this.e4235 * right_anti_dual.e31) + (this.e1234 * right_anti_dual.e35), -(this.e4125 * right_anti_dual.e35) - (this.e3215 * right_anti_dual.e45)) - (this.group0.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15)) - (this.group0.wwwy * float4(right_anti_dual.group0.xyz, right_anti_dual.e25))
        );
    }
}
extension Sphere: WeightContraction<VersorOdd> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return VersorOdd.from_groups(
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
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<VersorEven> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension VersorEven: WeightContraction<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return VersorOdd.from_groups(
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
extension VersorEven: WeightContraction<AntiDualNum> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: WeightContraction<AntiFlatPoint> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_anti_dual.e45 * this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz), -(right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321))
        );
    }
}
extension VersorEven: WeightContraction<AntiFlector> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorEven: WeightContraction<AntiLine> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group1.xyz), -(right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - float4((right_anti_dual.group1.xyz.zxy * this.group0.yzx), right_anti_dual.e415 * this.e235)
        );
    }
}
extension VersorEven: WeightContraction<AntiMotor> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: WeightContraction<AntiPlane> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorEven: WeightContraction<AntiScalar> {
    associatedtype Output = Scalar;
    func weight_contraction(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: WeightContraction<Circle> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
extension VersorEven: WeightContraction<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: CircleRotor) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.scalar * this.e12345) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412))
        );
    }
}
extension VersorEven: WeightContraction<Dipole> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), -(right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e412 * this.e315) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e423 * this.e125) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e431 * this.e235) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e125 * this.e435)) - float4((right_anti_dual.group0.xyz.yzx * this.group2.zxy), right_anti_dual.e235 * this.e415) - float4((right_anti_dual.group2.xyz.zxy * this.group0.yzx), right_anti_dual.e315 * this.e425)
        );
    }
}
extension VersorEven: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: DipoleInversion) -> Output {
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
            float4(float3(this.e12345) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * right_anti_dual.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_anti_dual.group2.xyz), (right_anti_dual.e4 * this.e12345) - (right_anti_dual.e423 * this.e415) - (right_anti_dual.e431 * this.e425) - (right_anti_dual.e412 * this.e435) - (right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412) + (right_anti_dual.e1 * this.e12345), (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423) + (right_anti_dual.e2 * this.e12345), (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431) + (right_anti_dual.e3 * this.e12345), -(right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + float4((right_anti_dual.group0.xyz.zxy * this.group2.yzx), right_anti_dual.e5 * this.e12345) - (this.group2.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e415)) - float4((right_anti_dual.group2.zxy * this.group0.yzx), right_anti_dual.e425 * this.e315)
        );
    }
}
extension VersorEven: WeightContraction<DualNum> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.group0.xy.xx, right_anti_dual.e3215, (right_anti_dual.e3215 * this.e4) + (right_anti_dual.scalar * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
extension VersorEven: WeightContraction<FlatPoint> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e315 * this.e412) + (right_anti_dual.e321 * this.e415), (right_anti_dual.e125 * this.e423) + (right_anti_dual.e321 * this.e425), (right_anti_dual.e235 * this.e431) + (right_anti_dual.e321 * this.e435), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) - (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
extension VersorEven: WeightContraction<Flector> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e321 * this.e415) + (right_anti_dual.e1 * this.e12345), (right_anti_dual.e321 * this.e425) + (right_anti_dual.e2 * this.e12345), (right_anti_dual.e321 * this.e435) + (right_anti_dual.e3 * this.e12345), -(right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435)) + (this.group0.zxyw * float4(right_anti_dual.group0.yzx, right_anti_dual.e5)) - (right_anti_dual.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
extension VersorEven: WeightContraction<Line> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_anti_dual.group0.xyz), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension VersorEven: WeightContraction<Motor> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e4)) + (float4(this.e12345) * right_anti_dual.group0) + float4(float3(0.0), -(right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual.e3215) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual.group1.xyz)), right_anti_dual.e3215 * this.e12345)
        );
    }
}
extension VersorEven: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_anti_dual.scalar * this.e12345) + (right_anti_dual.e4235 * this.e1) + (right_anti_dual.e4315 * this.e2) + (right_anti_dual.e4125 * this.e3) + (right_anti_dual.e3215 * this.e4) + (right_anti_dual.e1234 * this.e5) - (right_anti_dual.e15 * this.e423) - (right_anti_dual.e25 * this.e431) - (right_anti_dual.e35 * this.e412) - (right_anti_dual.e45 * this.e321) - (right_anti_dual.e41 * this.e235) - (right_anti_dual.e42 * this.e315) - (right_anti_dual.e43 * this.e125) - (right_anti_dual.e23 * this.e415) - (right_anti_dual.e31 * this.e425) - (right_anti_dual.e12 * this.e435), right_anti_dual.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e1 * this.e12345) + (right_anti_dual.e415 * this.e321) + (right_anti_dual.e321 * this.e415) + (right_anti_dual.e315 * this.e412), (right_anti_dual.e2 * this.e12345) + (right_anti_dual.e425 * this.e321) + (right_anti_dual.e321 * this.e425) + (right_anti_dual.e125 * this.e423), (right_anti_dual.e3 * this.e12345) + (right_anti_dual.e435 * this.e321) + (right_anti_dual.e321 * this.e435) + (right_anti_dual.e235 * this.e431), -(right_anti_dual.e415 * this.e423) - (right_anti_dual.e425 * this.e431) - (right_anti_dual.e435 * this.e412) - (right_anti_dual.e412 * this.e435)) + (float4(right_anti_dual.e12345) * this.group3) + float4((right_anti_dual.group7.xyz.zxy * this.group2.yzx), right_anti_dual.e4 * this.e12345) - float4((right_anti_dual.group7.xyz.yzx * this.group2.zxy), right_anti_dual.e423 * this.e415) - float4((right_anti_dual.group8.xyz.zxy * this.group0.yzx), right_anti_dual.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e12345 * this.e5) + (right_anti_dual.e5 * this.e12345) - (right_anti_dual.e415 * this.e235) - (right_anti_dual.e425 * this.e315) - (right_anti_dual.e435 * this.e125) - (right_anti_dual.e235 * this.e415) - (right_anti_dual.e315 * this.e425) - (right_anti_dual.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e4125 * this.e315) + (right_anti_dual.e3215 * this.e415), (right_anti_dual.e4235 * this.e125) + (right_anti_dual.e3215 * this.e425), (right_anti_dual.e4315 * this.e235) + (right_anti_dual.e3215 * this.e435), -(right_anti_dual.e4315 * this.e425) - (right_anti_dual.e4125 * this.e435)) + (float4(this.e12345) * right_anti_dual.group3) - (right_anti_dual.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e1234) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual.group4.xyz) + (right_anti_dual.group9.yzx * this.group0.zxy) - (right_anti_dual.group9.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e3215) * this.group0.xyz) + (float3(right_anti_dual.e1234) * this.group2.xyz) + (float3(this.e12345) * right_anti_dual.group5.xyz) - (float3(this.e321) * right_anti_dual.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(right_anti_dual.e12345) * this.group1) + (float4(this.e12345) * right_anti_dual.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual.group7.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e12345) * this.group2.xyz) + (float3(this.e12345) * right_anti_dual.group8.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * right_anti_dual.group9, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e1234 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: WeightContraction<Plane> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
extension VersorEven: WeightContraction<RoundPoint> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension VersorEven: WeightContraction<Scalar> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
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
extension VersorEven: WeightContraction<Sphere> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(right_anti_dual.e5 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: WeightContraction<VersorEven> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: VersorEven) -> Output {
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
extension VersorEven: WeightContraction<VersorOdd> {
    associatedtype Output = VersorEven;
    func weight_contraction(other: VersorOdd) -> Output {
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
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: WeightContractionInfix) -> WeightContractionInfixPartial<VersorOdd> {
        return WeightContractionInfixPartial { a: this };
    }
}
extension VersorOdd: WeightContraction<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiCircleRotor) -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: WeightContraction<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiDipoleInversion) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)) + float4(float3(0.0), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234) - (right_anti_dual.e4235 * this.e41) - (right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) - float4((this.group3.www * right_anti_dual.group3.xyz), right_anti_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_anti_dual.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_anti_dual.group2.xyz, right_anti_dual.e4315)) + float4((right_anti_dual.group1.zxy * this.group3.yzx), right_anti_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15)) - float4((right_anti_dual.group3.yzx * this.group1.zxy), right_anti_dual.e35 * this.e4125)
        );
    }
}
extension VersorOdd: WeightContraction<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiDualNum) -> Output {
        DualNum right_anti_dual = DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: WeightContraction<AntiFlatPoint> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: AntiFlatPoint) -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: WeightContraction<AntiFlector> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiFlector) -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_anti_dual.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group1.yzx * this.group3.zxy) - (right_anti_dual.group1.zxy * this.group3.yzx)), right_anti_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) + float4((right_anti_dual.group1.www * this.group3.xyz), right_anti_dual.e45 * this.e1234) - (right_anti_dual.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (right_anti_dual.group1.zxyy * float4(this.group1.yzx, this.e25)) + (right_anti_dual.group1.wwwz * float4(this.group0.xyz, this.e35)) + (this.group2.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e4235)) + float4(float3(0.0), (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215)) - float4((right_anti_dual.group1.yzx * this.group1.zxy), right_anti_dual.e15 * this.e4235)
        );
    }
}
extension VersorOdd: WeightContraction<AntiLine> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: AntiLine) -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz), -(right_anti_dual.e415 * this.e4235) - (right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((right_anti_dual.group1.xyz.zxy * this.group3.yzx), right_anti_dual.e415 * this.e23)
        );
    }
}
extension VersorOdd: WeightContraction<AntiMotor> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: AntiMotor) -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: WeightContraction<AntiPlane> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: AntiPlane) -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx)), right_anti_dual.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e3215 * this.e4235, right_anti_dual.e3215 * this.e4315, right_anti_dual.e3215 * this.e4125, -(right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e4315 * this.e12 * -1.0, right_anti_dual.e4125 * this.e23 * -1.0, right_anti_dual.e4235 * this.e31 * -1.0, (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45)) + (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e15)) + (right_anti_dual.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
extension VersorOdd: WeightContraction<Circle> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: Circle) -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(other.group2.xyz, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group3.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e42)) + float4((right_anti_dual.group2.xyz * this.group2.www), right_anti_dual.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: WeightContraction<CircleRotor> {
    associatedtype Output = RoundPoint;
    func weight_contraction(other: CircleRotor) -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual.e41 * this.e3215) - (right_anti_dual.e31 * this.e4125), -(right_anti_dual.e42 * this.e3215) - (right_anti_dual.e12 * this.e4235), -(right_anti_dual.e43 * this.e3215) - (right_anti_dual.e23 * this.e4315), (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234)) + (this.group3.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)) + float4((this.group2.www * right_anti_dual.group2.xyz), right_anti_dual.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(right_anti_dual.e45 * this.e3215) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: WeightContraction<Dipole> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: Dipole) -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group3.yzx) - (right_anti_dual.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((right_anti_dual.group2.xyz.zxy * this.group3.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
extension VersorOdd: WeightContraction<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func weight_contraction(other: DipoleInversion) -> Output {
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
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.zxy * this.group3.yzx) - (right_anti_dual.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.e423 * this.e3215) + (right_anti_dual.e235 * this.e1234), (right_anti_dual.e431 * this.e3215) + (right_anti_dual.e315 * this.e1234), (right_anti_dual.e412 * this.e3215) + (right_anti_dual.e125 * this.e1234), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) - (right_anti_dual.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4)) + (this.group3.zxyx * float4(right_anti_dual.group2.yzx, right_anti_dual.e1)) + float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43)) - float4((right_anti_dual.group2.zxy * this.group3.yzx), right_anti_dual.e423 * this.e15)
        );
    }
}
extension VersorOdd: WeightContraction<DualNum> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: DualNum) -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension VersorOdd: WeightContraction<FlatPoint> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: FlatPoint) -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e235 * this.e1234, right_anti_dual.e315 * this.e1234, right_anti_dual.e125 * this.e1234, -(right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension VersorOdd: WeightContraction<Flector> {
    associatedtype Output = AntiMotor;
    func weight_contraction(other: Flector) -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43) - (right_anti_dual.e321 * this.e45)) + float4((this.group2.www * right_anti_dual.group0.xyz), right_anti_dual.e1 * this.e4235) - (right_anti_dual.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual.group0.yzx * this.group3.zxy) - (right_anti_dual.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension VersorOdd: WeightContraction<Line> {
    associatedtype Output = AntiPlane;
    func weight_contraction(other: Line) -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(other.group1.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e12 * this.e4315) + (right_anti_dual.e15 * this.e1234), (right_anti_dual.e23 * this.e4125) + (right_anti_dual.e25 * this.e1234), (right_anti_dual.e31 * this.e4235) + (right_anti_dual.e35 * this.e1234), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) - (this.group3.zxyx * float4(right_anti_dual.group0.xyz.yzx, right_anti_dual.e15))
        );
    }
}
extension VersorOdd: WeightContraction<Motor> {
    associatedtype Output = AntiFlector;
    func weight_contraction(other: Motor) -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual.e15 * this.e1234) + (right_anti_dual.e3215 * this.e41), (right_anti_dual.e25 * this.e1234) + (right_anti_dual.e3215 * this.e42), (right_anti_dual.e35 * this.e1234) + (right_anti_dual.e3215 * this.e43), -(right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125)) + float4((right_anti_dual.group0.zxy * this.group3.yzx), right_anti_dual.e3215 * this.e45) - (this.group3.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e15))
        );
    }
}
extension VersorOdd: WeightContraction<MultiVector> {
    associatedtype Output = MultiVector;
    func weight_contraction(other: MultiVector) -> Output {
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
            float4(float2((right_anti_dual.e12345 * this.scalar) + (right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234) - (right_anti_dual.e415 * this.e23) - (right_anti_dual.e425 * this.e31) - (right_anti_dual.e435 * this.e12) - (right_anti_dual.e321 * this.e45) - (right_anti_dual.e423 * this.e15) - (right_anti_dual.e431 * this.e25) - (right_anti_dual.e412 * this.e35) - (right_anti_dual.e235 * this.e41) - (right_anti_dual.e315 * this.e42) - (right_anti_dual.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (this.group3.yzxx * float4(right_anti_dual.group5.xyz.zxy, right_anti_dual.e41)) + float4((right_anti_dual.group9.zxy * this.group1.yzx), right_anti_dual.e43 * this.e4125) + float4((right_anti_dual.group9.www * this.group0.xyz), right_anti_dual.e45 * this.e1234) + float4((this.group2.www * right_anti_dual.group3.xyz), right_anti_dual.e42 * this.e4315) - (float4(right_anti_dual.e1234) * float4(this.group2.xyz, this.e45)) - (right_anti_dual.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((right_anti_dual.group4.xyz * this.group3.www), right_anti_dual.e4235 * this.e41) - float4((right_anti_dual.group5.xyz.yzx * this.group3.zxy), right_anti_dual.e4315 * this.e42), 
            /* e5, 0, 0, 0 */
            float4((right_anti_dual.e4235 * this.e15) + (right_anti_dual.e4315 * this.e25) + (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45) - (right_anti_dual.e15 * this.e4235) - (right_anti_dual.e25 * this.e4315) - (right_anti_dual.e35 * this.e4125) - (right_anti_dual.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual.e415 * this.e3215) + (right_anti_dual.e315 * this.e4125), (right_anti_dual.e425 * this.e3215) + (right_anti_dual.e125 * this.e4235), (right_anti_dual.e435 * this.e3215) + (right_anti_dual.e235 * this.e4315), -(right_anti_dual.e425 * this.e4315) - (right_anti_dual.e435 * this.e4125)) + (float4(right_anti_dual.e12345) * float4(this.group2.xyz, this.e45)) - (this.group3.yzxx * float4(right_anti_dual.group8.xyz.zxy, right_anti_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(right_anti_dual.e12345) * this.group0.xyz) + (float3(this.e1234) * right_anti_dual.group6.xyz) + (right_anti_dual.group7.xyz.zxy * this.group3.yzx) - (right_anti_dual.group7.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_anti_dual.e12345) * this.group1.xyz) + (float3(this.e1234) * right_anti_dual.group8.xyz) + (float3(this.e3215) * right_anti_dual.group7.xyz) - (float3(right_anti_dual.e321) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group9.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_anti_dual.group9.zxy * this.group3.yzx), right_anti_dual.e1234 * this.e3215), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group9.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_anti_dual.e3215) * this.group3.xyz) - (float3(this.e3215) * right_anti_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual.e12345) * this.group3, 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: WeightContraction<Plane> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Plane) -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: WeightContraction<RoundPoint> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: RoundPoint) -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group0.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_anti_dual.group0.zxy * this.group3.yzx), right_anti_dual.e1234 * this.e3215), 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual.e3215 * this.e4235, right_anti_dual.e3215 * this.e4315, right_anti_dual.e3215 * this.e4125, -(right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43) - (right_anti_dual.e1234 * this.e45)) - (right_anti_dual.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(right_anti_dual.e4315 * this.e12) - (right_anti_dual.e1234 * this.e15), -(right_anti_dual.e4125 * this.e23) - (right_anti_dual.e1234 * this.e25), -(right_anti_dual.e4235 * this.e31) - (right_anti_dual.e1234 * this.e35), (right_anti_dual.e4125 * this.e35) + (right_anti_dual.e3215 * this.e45)) + (right_anti_dual.group0.zxyx * float4(this.group1.yzx, this.e15)) + (right_anti_dual.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
extension VersorOdd: WeightContraction<Scalar> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: Scalar) -> Output {
        AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar, 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
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
extension VersorOdd: WeightContraction<Sphere> {
    associatedtype Output = Scalar;
    func weight_contraction(other: Sphere) -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_anti_dual.e1 * this.e4235) + (right_anti_dual.e2 * this.e4315) + (right_anti_dual.e3 * this.e4125) + (right_anti_dual.e4 * this.e3215) + (right_anti_dual.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: WeightContraction<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func weight_contraction(other: VersorEven) -> Output {
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
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e41)) + float4(float3(0.0), (right_anti_dual.e42 * this.e4315) + (right_anti_dual.e43 * this.e4125) + (right_anti_dual.e45 * this.e1234) - (right_anti_dual.e4235 * this.e41) - (right_anti_dual.e4315 * this.e42) - (right_anti_dual.e4125 * this.e43)) - float4((this.group3.www * right_anti_dual.group3.xyz), right_anti_dual.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual.e3215) * float4(this.group0.xyz, this.e45)) + (right_anti_dual.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_anti_dual.group2.xyz, right_anti_dual.e4315)) + float4((right_anti_dual.group1.zxy * this.group3.yzx), right_anti_dual.e4235 * this.e15) - (float4(this.e3215) * float4(right_anti_dual.group0.xyz, right_anti_dual.e45)) - (right_anti_dual.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_anti_dual.group1.yzx, right_anti_dual.e15)) - float4((right_anti_dual.group3.yzx * this.group1.zxy), right_anti_dual.e35 * this.e4125)
        );
    }
}
extension VersorOdd: WeightContraction<VersorOdd> {
    associatedtype Output = VersorOdd;
    func weight_contraction(other: VersorOdd) -> Output {
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
        return VersorOdd.from_groups(
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

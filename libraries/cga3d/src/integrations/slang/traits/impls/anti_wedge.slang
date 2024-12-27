extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<AntiCircleRotor> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension AntiCircleRotor: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension AntiCircleRotor: AntiWedge<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiWedge<AntiFlector> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiWedge<AntiMotor> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension AntiCircleRotor: AntiWedge<AntiScalar> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: AntiScalar) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e12345) * this.group2
        );
    }
}
extension AntiCircleRotor: AntiWedge<Circle> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiWedge<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
extension AntiCircleRotor: AntiWedge<DipoleInversion> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: DipoleInversion) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((other.group2.www * this.group2.xyz), this.e42 * other.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiWedge<DualNum> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: DualNum) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e12345) * this.group2
        );
    }
}
extension AntiCircleRotor: AntiWedge<Flector> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Flector) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiWedge<Line> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiWedge<Motor> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Motor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435))
        );
    }
}
extension AntiCircleRotor: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group9.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(other.e1234) * this.group2.xyz), this.e42 * other.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e12345) * this.group1.xyz, 0.0), 
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
extension AntiCircleRotor: AntiWedge<Plane> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Plane) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiWedge<Sphere> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Sphere) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(other.e1234) * this.group2.xyz), this.e42 * other.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiWedge<VersorEven> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
extension AntiCircleRotor: AntiWedge<VersorOdd> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: VersorOdd) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((other.group2.www * this.group2.xyz), this.e42 * other.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<AntiDipoleInversion> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension AntiDipoleInversion: AntiWedge<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.xyz.yzx * this.group2.zxy), other.e423 * this.e415) - float4((this.group0.xyz.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.e3215) * float4(this.group0.xyz, this.e4), 
            /* e15, e25, e35, e3215 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321), (this.e423 * other.e125) + (this.e425 * other.e321), (this.e431 * other.e235) + (this.e435 * other.e321), -(this.e425 * other.e315) - (this.e435 * other.e125)) - (other.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
extension AntiDipoleInversion: AntiWedge<AntiFlector> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321), (this.e423 * other.e125) + (this.e425 * other.e321), (this.e431 * other.e235) + (this.e435 * other.e321), -(this.e425 * other.e315) - (this.e435 * other.e125)) - (other.group0.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
extension AntiDipoleInversion: AntiWedge<AntiLine> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiLine) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(other.e3215) * this.group0.xyz), (this.e4 * other.e3215) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<AntiScalar> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: AntiScalar) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group3
        );
    }
}
extension AntiDipoleInversion: AntiWedge<Circle> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Circle) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) - float4((this.group0.xyz.yzx * other.group2.xyz.zxy), this.e423 * other.e415) - float4((other.group0.xyz.yzx * this.group2.zxy), this.e431 * other.e425), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<CircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412) + (this.e1 * other.e12345), (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423) + (this.e2 * other.e12345), (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431) + (this.e3 * other.e12345), -(this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + (other.group2.yzxw * float4(this.group0.xyz.zxy, this.e5)) - (other.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((other.group0.xyz.yzx * this.group2.zxy), this.e425 * other.e315)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<Dipole> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group3.yzx) - (this.group0.xyz.yzx * other.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.zxy * other.group3.yzx), this.e423 * other.e15)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<DualNum> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: DualNum) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group3
        );
    }
}
extension AntiDipoleInversion: AntiWedge<FlatPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<Flector> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Flector) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * other.group1.yzx) - (this.group0.xyz.yzx * other.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * other.e3215, this.e431 * other.e3215, this.e412 * other.e3215, -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - float4((this.group2.zxy * other.group1.yzx), this.e423 * other.e15)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<Line> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Line) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e431 * other.e425) - (this.e412 * other.e435)) - float4((this.group0.xyz.yzx * other.group1.xyz.zxy), this.e423 * other.e415), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<Motor> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Motor) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e321 * other.e415) + (this.e1 * other.e12345), (this.e321 * other.e425) + (this.e2 * other.e12345), (this.e321 * other.e435) + (this.e3 * other.e12345), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((this.group0.xyz.zxy * other.group1.yzx), this.e5 * other.e12345) - (other.group1.zxyx * float4(this.group0.xyz.yzx, this.e415))
        );
    }
}
extension AntiDipoleInversion: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + (float4(other.e12345) * float4(this.group3.xyz, this.e4)) - float4((this.group0.xyz.yzx * other.group8.xyz.zxy), this.e423 * other.e415) - float4((other.group7.xyz.yzx * this.group2.zxy), this.e431 * other.e425), 
            /* e5, 0, 0, 0 */
            float4((this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group9.yzx) - (this.group0.xyz.yzx * other.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<Plane> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Plane) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * other.e3215, this.e431 * other.e3215, this.e412 * other.e3215, -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * other.e4315 * -1.0, this.e235 * other.e4125 * -1.0, this.e315 * other.e4235 * -1.0, (this.e2 * other.e4315) + (this.e3 * other.e4125)) + (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
extension AntiDipoleInversion: AntiWedge<Sphere> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Sphere) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(this.e125 * other.e4315 * -1.0, this.e235 * other.e4125 * -1.0, this.e315 * other.e4235 * -1.0, (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) + (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group0.zxyx * float4(this.group2.yzx, this.e1))
        );
    }
}
extension AntiDipoleInversion: AntiWedge<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412) + (this.e1 * other.e12345), (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423) + (this.e2 * other.e12345), (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431) + (this.e3 * other.e12345), -(this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((this.group0.xyz.zxy * other.group2.yzx), this.e5 * other.e12345) - (other.group2.zxyx * float4(this.group0.xyz.yzx, this.e415)) - float4((this.group2.zxy * other.group0.yzx), this.e425 * other.e315)
        );
    }
}
extension AntiDipoleInversion: AntiWedge<VersorOdd> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group3.yzx) - (this.group0.xyz.yzx * other.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.zxy * other.group3.yzx), this.e423 * other.e15)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<AntiDualNum> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension AntiDualNum: AntiWedge<AntiCircleRotor> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0)
        );
    }
}
extension AntiDualNum: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e3215) * float4(other.group0.xyz, other.e4), 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiWedge<AntiScalar> {
    associatedtype Output = AntiDualNum;
    func anti_wedge(other: AntiScalar) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.e12345) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiWedge<Circle> {
    associatedtype Output = AntiLine;
    func anti_wedge(other: Circle) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e3215) * other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.e3215) * other.group1.xyz, 0.0)
        );
    }
}
extension AntiDualNum: AntiWedge<CircleRotor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.xx, this.e3215, this.scalar) * float4(other.group0.xyz, other.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(other.group1.xyz, other.e12345)
        );
    }
}
extension AntiDualNum: AntiWedge<Dipole> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: Dipole) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0)
        );
    }
}
extension AntiDualNum: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0)
        );
    }
}
extension AntiDualNum: AntiWedge<DualNum> {
    associatedtype Output = AntiDualNum;
    func anti_wedge(other: DualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.e12345) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiWedge<FlatPoint> {
    associatedtype Output = DualNum;
    func anti_wedge(other: FlatPoint) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.e3215 * other.e45, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiWedge<Flector> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: Flector) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e3215 * other.e45 * -1.0)
        );
    }
}
extension AntiDualNum: AntiWedge<Line> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: Line) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiWedge<Motor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: Motor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.scalar * other.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * other.group0
        );
    }
}
extension AntiDualNum: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e3215 * other.e4) + (this.scalar * other.e12345), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group4.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e3215 * other.e45 * -1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e3215) * other.group7.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * other.e1234 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e3215) * other.group9.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * other.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: AntiWedge<Plane> {
    associatedtype Output = AntiFlatPoint;
    func anti_wedge(other: Plane) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiWedge<RoundPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e3215 * other.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiWedge<Sphere> {
    associatedtype Output = AntiFlatPoint;
    func anti_wedge(other: Sphere) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group0.xyz, other.e1234) * float4(-1.0)
        );
    }
}
extension AntiDualNum: AntiWedge<VersorEven> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.group0.xy.xx, this.e3215, (this.e3215 * other.e4) + (this.scalar * other.e12345)) * float4(other.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(other.group1.xyz, other.e12345)
        );
    }
}
extension AntiDualNum: AntiWedge<VersorOdd> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<AntiFlatPoint> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension AntiFlatPoint: AntiWedge<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321), (other.e423 * this.e125) + (other.e425 * this.e321), (other.e431 * this.e235) + (other.e435 * this.e321), -(other.e425 * this.e315) - (other.e435 * this.e125)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e415))
        );
    }
}
extension AntiFlatPoint: AntiWedge<AntiScalar> {
    associatedtype Output = AntiFlatPoint;
    func anti_wedge(other: AntiScalar) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0
        );
    }
}
extension AntiFlatPoint: AntiWedge<Circle> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: Circle) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e235 * other.e415)
        );
    }
}
extension AntiFlatPoint: AntiWedge<CircleRotor> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e235 * other.e415)
        );
    }
}
extension AntiFlatPoint: AntiWedge<Dipole> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e235 * other.e1234, this.e315 * other.e1234, this.e125 * other.e1234, -(this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - float4((this.group0.www * other.group3.xyz), this.e235 * other.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
extension AntiFlatPoint: AntiWedge<DualNum> {
    associatedtype Output = AntiFlatPoint;
    func anti_wedge(other: DualNum) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0
        );
    }
}
extension AntiFlatPoint: AntiWedge<FlatPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: AntiWedge<Flector> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: Flector) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
extension AntiFlatPoint: AntiWedge<Line> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: Line) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * other.group0.xyz), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
extension AntiFlatPoint: AntiWedge<Motor> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: Motor) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.group0.www * other.group0.xyz), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
extension AntiFlatPoint: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((other.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((other.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4(-(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * other.group9.zxy) - (this.group0.zxy * other.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * other.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiFlatPoint: AntiWedge<Plane> {
    associatedtype Output = AntiLine;
    func anti_wedge(other: Plane) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e321) * other.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 0.0)
        );
    }
}
extension AntiFlatPoint: AntiWedge<Sphere> {
    associatedtype Output = AntiLine;
    func anti_wedge(other: Sphere) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4((float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, 0 */
            float4((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 0.0)
        );
    }
}
extension AntiFlatPoint: AntiWedge<VersorEven> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - (this.group0.zxyx * float4(other.group0.yzx, other.e415))
        );
    }
}
extension AntiFlatPoint: AntiWedge<VersorOdd> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e235 * other.e1234, this.e315 * other.e1234, this.e125 * other.e1234, -(this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<AntiFlector> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension AntiFlector: AntiWedge<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321), (other.e423 * this.e125) + (other.e425 * this.e321), (other.e431 * this.e235) + (other.e435 * this.e321), -(other.e425 * this.e315) - (other.e435 * this.e125)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e415))
        );
    }
}
extension AntiFlector: AntiWedge<AntiScalar> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiScalar) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group1
        );
    }
}
extension AntiFlector: AntiWedge<Circle> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: Circle) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e235 * other.e415)
        );
    }
}
extension AntiFlector: AntiWedge<CircleRotor> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * other.e415) + (this.e1 * other.e12345), (this.e321 * other.e425) + (this.e2 * other.e12345), (this.e321 * other.e435) + (this.e3 * other.e12345), -(this.e315 * other.e425) - (this.e125 * other.e435)) + float4((other.group0.xyz.zxy * this.group0.yzx), this.e5 * other.e12345) - float4((other.group0.xyz.yzx * this.group0.zxy), this.e235 * other.e415)
        );
    }
}
extension AntiFlector: AntiWedge<Dipole> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) + float4((other.group2.www * this.group0.xyz), this.e1 * other.e4235) - float4((this.group0.www * other.group3.xyz), this.e235 * other.e41), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
extension AntiFlector: AntiWedge<DualNum> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: DualNum) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group1
        );
    }
}
extension AntiFlector: AntiWedge<FlatPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: AntiWedge<Flector> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: Flector) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * other.group1.xyz * float3(-1.0)), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e321 * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
extension AntiFlector: AntiWedge<Line> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: Line) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * other.group0.xyz), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
extension AntiFlector: AntiWedge<Motor> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: Motor) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e1 * other.e12345, this.e2 * other.e12345, this.e3 * other.e12345, -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + (other.group0 * float4(this.group0.www, this.e5))
        );
    }
}
extension AntiFlector: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((float3(other.e12345) * this.group1.xyz), 0.0) + float4((other.group7.xyz.zxy * this.group0.yzx), 0.0) - float4((other.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4((this.e5 * other.e12345) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * other.group9.zxy) - (this.group0.zxy * other.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * other.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiFlector: AntiWedge<Plane> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: Plane) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((this.group0.www * other.group0.xyz * float3(-1.0)), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: AntiWedge<Sphere> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: Sphere) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e321 * other.e4235 * -1.0, this.e321 * other.e4315 * -1.0, this.e321 * other.e4125 * -1.0, (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) + float4((float3(other.e1234) * this.group0.xyz), this.e1 * other.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: AntiWedge<VersorEven> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((this.e321 * other.e415) + (this.e1 * other.e12345), (this.e321 * other.e425) + (this.e2 * other.e12345), (this.e321 * other.e435) + (this.e3 * other.e12345), -(this.e315 * other.e425) - (this.e125 * other.e435)) + (other.group0.zxyw * float4(this.group0.yzx, this.e5)) - (this.group0.zxyx * float4(other.group0.yzx, other.e415))
        );
    }
}
extension AntiFlector: AntiWedge<VersorOdd> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) + float4((other.group2.www * this.group0.xyz), this.e1 * other.e4235) - (this.group0.wwwx * float4(other.group3.xyz, other.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<AntiLine> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension AntiLine: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: AntiWedge<AntiScalar> {
    associatedtype Output = AntiLine;
    func anti_wedge(other: AntiScalar) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(other.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: AntiWedge<Circle> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: AntiWedge<CircleRotor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: AntiWedge<DualNum> {
    associatedtype Output = AntiLine;
    func anti_wedge(other: DualNum) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(other.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: AntiWedge<Flector> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: Flector) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12 * other.e4315 * -1.0, this.e23 * other.e4125 * -1.0, this.e31 * other.e4235 * -1.0, (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group1.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: AntiWedge<Line> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: AntiWedge<Motor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: Motor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.xyz.yzx * other.group9.zxy), 0.0) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.xyz.zxy * other.group9.yzx), 0.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.yy, other.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
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
extension AntiLine: AntiWedge<Plane> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: Plane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12 * other.e4315 * -1.0, this.e23 * other.e4125 * -1.0, this.e31 * other.e4235 * -1.0, (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: AntiWedge<Sphere> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: Sphere) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group0.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiLine: AntiWedge<VersorEven> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: AntiWedge<VersorOdd> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.xyz.yzx, this.e15))
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<AntiMotor> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension AntiMotor: AntiWedge<AntiCircleRotor> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0)
        );
    }
}
extension AntiMotor: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * other.group0.xyz), (other.e4 * this.e3215) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: AntiWedge<AntiScalar> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiScalar) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
extension AntiMotor: AntiWedge<Circle> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: Circle) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * other.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: AntiWedge<CircleRotor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e23 * other.e12345, this.e31 * other.e12345, this.e12 * other.e12345, -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((other.group0.xyz * this.group1.www), this.scalar * other.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e3215 * other.e12345)
        );
    }
}
extension AntiMotor: AntiWedge<Dipole> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: Dipole) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((other.group0.xyz * this.group1.www * float3(-1.0)), this.e3215 * other.e45 * -1.0)
        );
    }
}
extension AntiMotor: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((other.group0.xyz * this.group1.www), this.e3215 * other.e45)
        );
    }
}
extension AntiMotor: AntiWedge<DualNum> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: DualNum) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
extension AntiMotor: AntiWedge<FlatPoint> {
    associatedtype Output = DualNum;
    func anti_wedge(other: FlatPoint) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.e3215 * other.e45, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiMotor: AntiWedge<Flector> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: Flector) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            (other.group1.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((this.group0.zxy * other.group1.yzx), this.e3215 * other.e45)
        );
    }
}
extension AntiMotor: AntiWedge<Line> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: Line) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: AntiWedge<Motor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: Motor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e3215 * other.e12345)
        );
    }
}
extension AntiMotor: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.scalar * other.e12345) + (this.e3215 * other.e4) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.yzx * other.group9.zxy), 0.0) - float4((float3(this.e3215) * other.group4.xyz), 0.0) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * other.group9.yzx), 0.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) - (this.e3215 * other.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(this.e3215) * other.group6.xyz) + (float3(other.e12345) * this.group1.xyz)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * other.group7.xyz) + (float3(other.e12345) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * other.e1234 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e3215) * other.group9.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * other.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: AntiWedge<Plane> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: Plane) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * other.e4315 * -1.0, this.e23 * other.e4125 * -1.0, this.e31 * other.e4235 * -1.0, (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
extension AntiMotor: AntiWedge<RoundPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e3215 * other.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: AntiWedge<Sphere> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: Sphere) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group0.xyz, other.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
extension AntiMotor: AntiWedge<VersorEven> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            (float4(this.e3215) * float4(other.group0.xyz, other.e4)) + (float4(other.e12345) * this.group0) + float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e3215 * other.e12345)
        );
    }
}
extension AntiMotor: AntiWedge<VersorOdd> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(this.e15 * other.e1234) - (this.e3215 * other.e41), -(this.e25 * other.e1234) - (this.e3215 * other.e42), -(this.e35 * other.e1234) - (this.e3215 * other.e43), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((this.group0.zxy * other.group3.yzx), this.e3215 * other.e45)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<AntiPlane> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension AntiPlane: AntiWedge<AntiScalar> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiScalar) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
extension AntiPlane: AntiWedge<CircleRotor> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
extension AntiPlane: AntiWedge<DipoleInversion> {
    associatedtype Output = Scalar;
    func anti_wedge(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: AntiWedge<DualNum> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: DualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
extension AntiPlane: AntiWedge<Flector> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: AntiWedge<Motor> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: Motor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
extension AntiPlane: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.yy, other.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * other.e12345, 0.0, 0.0, 0.0), 
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
extension AntiPlane: AntiWedge<Plane> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: AntiWedge<Sphere> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: AntiWedge<VersorEven> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
extension AntiPlane: AntiWedge<VersorOdd> {
    associatedtype Output = Scalar;
    func anti_wedge(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<AntiScalar> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension AntiScalar: AntiWedge<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * other.group2
        );
    }
}
extension AntiScalar: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group3
        );
    }
}
extension AntiScalar: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiDualNum;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e12345) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0
        );
    }
}
extension AntiScalar: AntiWedge<AntiFlector> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group1
        );
    }
}
extension AntiScalar: AntiWedge<AntiLine> {
    associatedtype Output = AntiLine;
    func anti_wedge(other: AntiLine) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * other.group1.xyz, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
extension AntiScalar: AntiWedge<AntiPlane> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiPlane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
extension AntiScalar: AntiWedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<Circle> {
    associatedtype Output = Circle;
    func anti_wedge(other: Circle) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * other.group2.xyz, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<CircleRotor> {
    associatedtype Output = CircleRotor;
    func anti_wedge(other: CircleRotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.e12345) * other.group2
        );
    }
}
extension AntiScalar: AntiWedge<Dipole> {
    associatedtype Output = Dipole;
    func anti_wedge(other: Dipole) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * other.group2.xyz, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func anti_wedge(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
extension AntiScalar: AntiWedge<DualNum> {
    associatedtype Output = DualNum;
    func anti_wedge(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.e12345) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<FlatPoint> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: FlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0
        );
    }
}
extension AntiScalar: AntiWedge<Flector> {
    associatedtype Output = Flector;
    func anti_wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
extension AntiScalar: AntiWedge<Line> {
    associatedtype Output = Line;
    func anti_wedge(other: Line) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * other.group1.xyz, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<Motor> {
    associatedtype Output = Motor;
    func anti_wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * other.group1
        );
    }
}
extension AntiScalar: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.e12345) * other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * other.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * other.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<Plane> {
    associatedtype Output = Plane;
    func anti_wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
extension AntiScalar: AntiWedge<RoundPoint> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: RoundPoint) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * other.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<Scalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<Sphere> {
    associatedtype Output = Sphere;
    func anti_wedge(other: Sphere) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiWedge<VersorEven> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: VersorEven) -> Output {
        return VersorEven.from_groups(
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
extension AntiScalar: AntiWedge<VersorOdd> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
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
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Circle> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Circle: AntiWedge<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.xyz.yzx * this.group2.xyz.zxy), other.e423 * this.e415) - float4((this.group0.xyz.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiLine;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(other.e3215) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(other.e3215) * this.group1.xyz, 0.0)
        );
    }
}
extension Circle: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - float4((this.group0.xyz.yzx * other.group0.zxy), other.e235 * this.e415)
        );
    }
}
extension Circle: AntiWedge<AntiFlector> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - float4((this.group0.xyz.yzx * other.group0.zxy), other.e235 * this.e415)
        );
    }
}
extension Circle: AntiWedge<AntiLine> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiLine) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiWedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(other.e3215) * this.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Circle: AntiWedge<AntiScalar> {
    associatedtype Output = Circle;
    func anti_wedge(other: AntiScalar) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: AntiWedge<Circle> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Circle) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.xyz.yzx * this.group2.xyz.zxy), other.e423 * this.e415) - float4((other.group2.xyz.zxy * this.group0.xyz.yzx), other.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiWedge<CircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e125 * other.e435)) - float4((this.group0.xyz.yzx * other.group2.zxy), this.e235 * other.e415) - float4((this.group2.xyz.zxy * other.group0.xyz.yzx), this.e315 * other.e425)
        );
    }
}
extension Circle: AntiWedge<Dipole> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group3.yzx) - (this.group0.xyz.yzx * other.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.xyz.zxy * other.group3.yzx), this.e423 * other.e15)
        );
    }
}
extension Circle: AntiWedge<DualNum> {
    associatedtype Output = Circle;
    func anti_wedge(other: DualNum) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: AntiWedge<FlatPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiWedge<Flector> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Flector) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * other.group1.yzx) - (this.group0.xyz.yzx * other.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * other.e3215, this.e431 * other.e3215, this.e412 * other.e3215, -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - float4((this.group2.xyz.zxy * other.group1.yzx), this.e423 * other.e15)
        );
    }
}
extension Circle: AntiWedge<Line> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Line) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e431 * other.e425) - (this.e412 * other.e435)) - float4((this.group0.xyz.yzx * other.group1.xyz.zxy), this.e423 * other.e415), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiWedge<Motor> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Motor) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e315 * other.e425) - (this.e125 * other.e435)) - float4((this.group0.xyz.yzx * other.group1.zxy), this.e235 * other.e415)
        );
    }
}
extension Circle: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) - float4((this.group0.xyz.yzx * other.group8.xyz.zxy), this.e423 * other.e415) - float4((this.group2.xyz.zxy * other.group7.xyz.yzx), this.e431 * other.e425), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group9.yzxx * float4(this.group2.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group9.yzx) - (this.group0.xyz.yzx * other.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Circle: AntiWedge<Plane> {
    associatedtype Output = Dipole;
    func anti_wedge(other: Plane) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * other.e3215, this.e431 * other.e3215, this.e412 * other.e3215, -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(other.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * other.group0.zxy) - (this.group2.xyz.zxy * other.group0.yzx), 0.0)
        );
    }
}
extension Circle: AntiWedge<Sphere> {
    associatedtype Output = Dipole;
    func anti_wedge(other: Sphere) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(other.e3215) * this.group1.xyz) + (this.group2.xyz.yzx * other.group0.zxy) - (this.group2.xyz.zxy * other.group0.yzx), 0.0)
        );
    }
}
extension Circle: AntiWedge<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e125 * other.e435)) - float4((this.group0.xyz.yzx * other.group2.zxy), this.e235 * other.e415) - float4((this.group2.xyz.zxy * other.group0.yzx), this.e315 * other.e425)
        );
    }
}
extension Circle: AntiWedge<VersorOdd> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group3.yzx) - (this.group0.xyz.yzx * other.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.xyz.zxy * other.group3.yzx), this.e423 * other.e15)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<CircleRotor> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension CircleRotor: AntiWedge<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
extension CircleRotor: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e1 * this.e12345), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e2 * this.e12345), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e3 * this.e12345), -(other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + (this.group2.yzxw * float4(other.group0.xyz.zxy, other.e5)) - (this.group2.zxyx * float4(other.group0.xyz.yzx, other.e415)) - float4((this.group0.xyz.yzx * other.group2.zxy), other.e425 * this.e315)
        );
    }
}
extension CircleRotor: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.group0.xy.xx, other.e3215, other.scalar) * float4(this.group0.xyz, this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
extension CircleRotor: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - float4((this.group0.xyz.yzx * other.group0.zxy), other.e235 * this.e415)
        );
    }
}
extension CircleRotor: AntiWedge<AntiFlector> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.e321 * this.e415) + (other.e1 * this.e12345), (other.e321 * this.e425) + (other.e2 * this.e12345), (other.e321 * this.e435) + (other.e3 * this.e12345), -(other.e315 * this.e425) - (other.e125 * this.e435)) + float4((this.group0.xyz.zxy * other.group0.yzx), other.e5 * this.e12345) - float4((this.group0.xyz.yzx * other.group0.zxy), other.e235 * this.e415)
        );
    }
}
extension CircleRotor: AntiWedge<AntiLine> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiLine) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension CircleRotor: AntiWedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.e23 * this.e12345, other.e31 * this.e12345, other.e12 * this.e12345, -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((this.group0.xyz * other.group1.www), other.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group1.xyz)), other.e3215 * this.e12345)
        );
    }
}
extension CircleRotor: AntiWedge<AntiPlane> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiPlane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
extension CircleRotor: AntiWedge<AntiScalar> {
    associatedtype Output = CircleRotor;
    func anti_wedge(other: AntiScalar) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.e12345) * this.group2
        );
    }
}
extension CircleRotor: AntiWedge<Circle> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Circle) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e125 * this.e435)) - float4((other.group0.xyz.yzx * this.group2.zxy), other.e235 * this.e415) - float4((other.group2.xyz.zxy * this.group0.xyz.yzx), other.e315 * this.e425)
        );
    }
}
extension CircleRotor: AntiWedge<CircleRotor> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: CircleRotor) -> Output {
        return VersorEven.from_groups(
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
extension CircleRotor: AntiWedge<Dipole> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Dipole) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43))
        );
    }
}
extension CircleRotor: AntiWedge<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
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
extension CircleRotor: AntiWedge<DualNum> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: DualNum) -> Output {
        return VersorEven.from_groups(
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
extension CircleRotor: AntiWedge<FlatPoint> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: FlatPoint) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e12345 * other.e45), 
            /* e15, e25, e35, scalar */
            float4((this.group2.www * other.group0.xyz), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45))
        );
    }
}
extension CircleRotor: AntiWedge<Flector> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: Flector) -> Output {
        return VersorOdd.from_groups(
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
extension CircleRotor: AntiWedge<Line> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Line) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group1.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435)), 
            /* e1, e2, e3, e5 */
            float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e315 * other.e425) - (this.e125 * other.e435)) - float4((this.group0.xyz.yzx * other.group1.xyz.zxy), this.e235 * other.e415)
        );
    }
}
extension CircleRotor: AntiWedge<Motor> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: Motor) -> Output {
        return VersorEven.from_groups(
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
extension CircleRotor: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), this.e12345 * other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412) + (this.e12345 * other.e1), (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423) + (this.e12345 * other.e2), (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431) + (this.e12345 * other.e3), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + float4((this.group0.xyz.zxy * other.group8.xyz.yzx), this.e12345 * other.e4) - float4((this.group0.xyz.yzx * other.group8.xyz.zxy), this.e423 * other.e415) - float4((other.group7.xyz.yzx * this.group2.zxy), this.e431 * other.e425), 
            /* e5, 0, 0, 0 */
            float4((this.e12345 * other.e5) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e315 * other.e4125) + (this.e12345 * other.e15), (this.e125 * other.e4235) + (this.e12345 * other.e25), (this.e235 * other.e4315) + (this.e12345 * other.e35), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((other.group9.www * this.group1.xyz), this.e12345 * other.e45) - (other.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group9.yzx) - (this.group0.xyz.yzx * other.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e12345) * other.group5.xyz) + (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * other.group6) + (float4(other.e12345) * this.group1), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e12345) * other.group7.xyz) + (float3(other.e12345) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e12345) * other.group8.xyz) + (float3(other.e12345) * this.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiWedge<Plane> {
    associatedtype Output = DipoleInversion;
    func anti_wedge(other: Plane) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e423 * other.e3215, this.e431 * other.e3215, this.e412 * other.e3215, -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - float4((this.group2.zxy * other.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
extension CircleRotor: AntiWedge<RoundPoint> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: RoundPoint) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * other.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiWedge<Scalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiWedge<Sphere> {
    associatedtype Output = DipoleInversion;
    func anti_wedge(other: Sphere) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group1.xyz) + (this.group0.xyz.zxy * other.group0.yzx) - (this.group0.xyz.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
extension CircleRotor: AntiWedge<VersorEven> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: VersorEven) -> Output {
        return VersorEven.from_groups(
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
extension CircleRotor: AntiWedge<VersorOdd> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
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
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Dipole> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Dipole: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension Dipole: AntiWedge<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiWedge<AntiFlector> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiWedge<AntiMotor> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((this.group0.xyz * other.group1.www), other.e3215 * this.e45)
        );
    }
}
extension Dipole: AntiWedge<AntiScalar> {
    associatedtype Output = Dipole;
    func anti_wedge(other: AntiScalar) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(other.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: AntiWedge<Circle> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiWedge<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43))
        );
    }
}
extension Dipole: AntiWedge<DipoleInversion> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: DipoleInversion) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * other.group2.www), this.e41 * other.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiWedge<DualNum> {
    associatedtype Output = Dipole;
    func anti_wedge(other: DualNum) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(other.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: AntiWedge<Flector> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Flector) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiWedge<Line> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiWedge<Motor> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Motor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435))
        );
    }
}
extension Dipole: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group9.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(other.e1234) * this.group2.xyz), this.e41 * other.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e12345) * this.group1.xyz, 0.0), 
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
extension Dipole: AntiWedge<Plane> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Plane) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiWedge<Sphere> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Sphere) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group0.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(other.e1234) * this.group2.xyz), this.e41 * other.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiWedge<VersorEven> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
extension Dipole: AntiWedge<VersorOdd> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: VersorOdd) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * other.group2.www), this.e41 * other.e4235), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<DipoleInversion> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension DipoleInversion: AntiWedge<AntiCircleRotor> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxx * float4(other.group1.zxy, other.e41)) + float4((this.group2.www * other.group2.xyz), other.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group3.yzx) - (other.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.zxy * this.group3.yzx), other.e423 * this.e15)
        );
    }
}
extension DipoleInversion: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension DipoleInversion: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.e235 * this.e1234, other.e315 * this.e1234, other.e125 * this.e1234, -(other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - float4((other.group0.www * this.group3.xyz), other.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension DipoleInversion: AntiWedge<AntiFlector> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) + float4((this.group2.www * other.group0.xyz), other.e1 * this.e4235) - float4((other.group0.www * this.group3.xyz), other.e235 * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension DipoleInversion: AntiWedge<AntiLine> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiLine) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group3.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
extension DipoleInversion: AntiWedge<AntiMotor> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((this.group0.xyz * other.group1.www), other.e3215 * this.e45) - (this.group3.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
extension DipoleInversion: AntiWedge<AntiPlane> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiPlane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiWedge<AntiScalar> {
    associatedtype Output = DipoleInversion;
    func anti_wedge(other: AntiScalar) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
extension DipoleInversion: AntiWedge<Circle> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Circle) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group3.yzx) - (other.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.xyz.zxy * this.group3.yzx), other.e423 * this.e15)
        );
    }
}
extension DipoleInversion: AntiWedge<CircleRotor> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: CircleRotor) -> Output {
        return VersorOdd.from_groups(
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
extension DipoleInversion: AntiWedge<Dipole> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Dipole) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxy * float4(other.group1.zxy, other.e42)) + float4((other.group2.xyz * this.group2.www), other.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e1234 * this.e45) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - float4((this.group3.www * other.group3.xyz), other.e4235 * this.e41), 
            /* e1, e2, e3, e5 */
            (other.group3.zxyw * this.group1.yzxw) + (this.group2.wwwz * float4(other.group2.xyz, other.e4125)) + float4((this.group0.xyz * other.group3.www), other.e4235 * this.e15) + float4((other.group1.zxy * this.group3.yzx), other.e4315 * this.e25) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), other.e35 * this.e4125)
        );
    }
}
extension DipoleInversion: AntiWedge<DualNum> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: DualNum) -> Output {
        return VersorOdd.from_groups(
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
extension DipoleInversion: AntiWedge<FlatPoint> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: FlatPoint) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(-(this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiWedge<Flector> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Flector) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * other.group1.yzx) - (this.group3.yzx * other.group1.zxy)), this.e1234 * other.e3215), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((other.group1.www * this.group3.xyz), this.e1234 * other.e45) - (other.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(other.group0.xyz, other.e4315)) + (other.group1.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (this.e35 * other.e4125) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45)) - float4((this.group1.zxy * other.group1.yzx), this.e4235 * other.e15)
        );
    }
}
extension DipoleInversion: AntiWedge<Line> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Line) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * other.group1.xyz), -(this.e4235 * other.e415) - (this.e4315 * other.e425) - (this.e4125 * other.e435)), 
            /* e15, e25, e35, scalar */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) - float4((other.group1.xyz.zxy * this.group3.yzx), this.e41 * other.e235)
        );
    }
}
extension DipoleInversion: AntiWedge<Motor> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: Motor) -> Output {
        return VersorOdd.from_groups(
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
extension DipoleInversion: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * other.group3) + (this.group3.yzxy * float4(other.group5.xyz.zxy, other.e42)) + float4((this.group0.xyz * other.group9.www), this.e4235 * other.e41) + float4((this.group1.yzx * other.group9.zxy), this.e4125 * other.e43) - (float4(other.e1234) * float4(this.group2.xyz, this.e45)) - (other.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((other.group4.xyz * this.group3.www), this.e41 * other.e4235) - float4((other.group5.xyz.yzx * this.group3.zxy), this.e42 * other.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * float4(this.group2.xyz, this.e45)) - float4((other.group8.xyz.zxy * this.group3.yzx), this.e4235 * other.e415), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group6.xyz) + (float3(other.e12345) * this.group0.xyz) + (other.group7.xyz.zxy * this.group3.yzx) - (other.group7.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * other.group8.xyz) + (float3(this.e3215) * other.group7.xyz) + (float3(other.e12345) * this.group1.xyz) - (float3(other.e321) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group9.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group9.zxy, other.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group3.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiWedge<Plane> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Plane) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * other.group0.yzx) - (this.group3.yzx * other.group0.zxy)), this.e1234 * other.e3215), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * other.e3215, this.e4315 * other.e3215, this.e4125 * other.e3215, -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e12 * other.e4315 * -1.0, this.e23 * other.e4125 * -1.0, this.e31 * other.e4235 * -1.0, (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
extension DipoleInversion: AntiWedge<RoundPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiWedge<Sphere> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Sphere) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group0.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group0.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e4235 * other.e3215, this.e4315 * other.e3215, this.e4125 * other.e3215, -(this.e42 * other.e4315) - (this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
extension DipoleInversion: AntiWedge<VersorEven> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: VersorEven) -> Output {
        return VersorOdd.from_groups(
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
extension DipoleInversion: AntiWedge<VersorOdd> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (this.e4235 * other.e41) + (this.e4315 * other.e42) + (this.e4125 * other.e43) - (this.e42 * other.e4315) - (this.e43 * other.e4125) - (this.e45 * other.e1234)) + float4((other.group3.www * this.group3.xyz), this.e1234 * other.e45) - (other.group3.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + (other.group3.zxyx * float4(this.group1.yzx, this.e15)) + float4((this.group3.yzx * other.group1.zxy), this.e35 * other.e4125) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group3.zxyz * float4(other.group1.yzx, other.e35)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - float4((this.group1.zxy * other.group3.yzx), this.e4235 * other.e15)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<DualNum> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension DualNum: AntiWedge<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * other.group2
        );
    }
}
extension DualNum: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group3
        );
    }
}
extension DualNum: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiDualNum;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e12345) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0
        );
    }
}
extension DualNum: AntiWedge<AntiFlector> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group1
        );
    }
}
extension DualNum: AntiWedge<AntiLine> {
    associatedtype Output = AntiLine;
    func anti_wedge(other: AntiLine) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * other.group1.xyz, 0.0)
        );
    }
}
extension DualNum: AntiWedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
extension DualNum: AntiWedge<AntiPlane> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiPlane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
extension DualNum: AntiWedge<AntiScalar> {
    associatedtype Output = DualNum;
    func anti_wedge(other: AntiScalar) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.e12345) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiWedge<Circle> {
    associatedtype Output = Circle;
    func anti_wedge(other: Circle) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * other.group2.xyz, 0.0)
        );
    }
}
extension DualNum: AntiWedge<CircleRotor> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: CircleRotor) -> Output {
        return VersorEven.from_groups(
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
extension DualNum: AntiWedge<Dipole> {
    associatedtype Output = Dipole;
    func anti_wedge(other: Dipole) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.e12345) * other.group2.xyz, 0.0)
        );
    }
}
extension DualNum: AntiWedge<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
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
extension DualNum: AntiWedge<DualNum> {
    associatedtype Output = DualNum;
    func anti_wedge(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((other.e5 * this.e12345) + (other.e12345 * this.e5), other.e12345 * this.e12345), 0.0, 0.0)
        );
    }
}
extension DualNum: AntiWedge<FlatPoint> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: FlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0
        );
    }
}
extension DualNum: AntiWedge<Flector> {
    associatedtype Output = Flector;
    func anti_wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
extension DualNum: AntiWedge<Line> {
    associatedtype Output = Line;
    func anti_wedge(other: Line) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * other.group1.xyz, 0.0)
        );
    }
}
extension DualNum: AntiWedge<Motor> {
    associatedtype Output = Motor;
    func anti_wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.yy, this.e12345, (this.e5 * other.e12345) + (this.e12345 * other.e5)) * float4(other.group1.xyz, 1.0)
        );
    }
}
extension DualNum: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e5 * other.e1234) + (this.e12345 * other.scalar), this.e12345 * other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group1, 
            /* e5, 0, 0, 0 */
            float4((this.e5 * other.e12345) + (this.e12345 * other.e5), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * other.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * other.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiWedge<Plane> {
    associatedtype Output = Plane;
    func anti_wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
extension DualNum: AntiWedge<RoundPoint> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: RoundPoint) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * other.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiWedge<Scalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiWedge<Sphere> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: Sphere) -> Output {
        return VersorOdd.from_groups(
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
extension DualNum: AntiWedge<VersorEven> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: VersorEven) -> Output {
        return VersorEven.from_groups(
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
extension DualNum: AntiWedge<VersorOdd> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
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
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<FlatPoint> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension FlatPoint: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiWedge<AntiDualNum> {
    associatedtype Output = DualNum;
    func anti_wedge(other: AntiDualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.e3215 * this.e45, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiWedge<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiWedge<AntiFlector> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiWedge<AntiMotor> {
    associatedtype Output = DualNum;
    func anti_wedge(other: AntiMotor) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.e3215 * this.e45, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiWedge<AntiScalar> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: AntiScalar) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0
        );
    }
}
extension FlatPoint: AntiWedge<Circle> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiWedge<CircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e12345 * this.e45), 
            /* e15, e25, e35, scalar */
            float4((other.group2.www * this.group0.xyz), -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45))
        );
    }
}
extension FlatPoint: AntiWedge<DipoleInversion> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: DipoleInversion) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiWedge<DualNum> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: DualNum) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0
        );
    }
}
extension FlatPoint: AntiWedge<Flector> {
    associatedtype Output = DualNum;
    func anti_wedge(other: Flector) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiWedge<Motor> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: Motor) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0
        );
    }
}
extension FlatPoint: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0, 
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
extension FlatPoint: AntiWedge<Plane> {
    associatedtype Output = DualNum;
    func anti_wedge(other: Plane) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiWedge<Sphere> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Sphere) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiWedge<VersorEven> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45 * other.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321))
        );
    }
}
extension FlatPoint: AntiWedge<VersorOdd> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: VersorOdd) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0 * float4(-1.0), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Flector> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Flector: AntiWedge<AntiCircleRotor> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((other.group0.xyz.zxy * this.group1.yzx) - (other.group0.xyz.yzx * this.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * this.e3215, other.e431 * this.e3215, other.e412 * this.e3215, -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group1.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - float4((other.group2.zxy * this.group1.yzx), other.e423 * this.e15)
        );
    }
}
extension Flector: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), other.e3215 * this.e45)
        );
    }
}
extension Flector: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
extension Flector: AntiWedge<AntiFlector> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * this.group1.xyz * float3(-1.0)), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e321 * this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
extension Flector: AntiWedge<AntiLine> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiLine) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group1.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
extension Flector: AntiWedge<AntiMotor> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((other.group0.zxy * this.group1.yzx), other.e3215 * this.e45) - (this.group1.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
extension Flector: AntiWedge<AntiPlane> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiPlane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiWedge<AntiScalar> {
    associatedtype Output = Flector;
    func anti_wedge(other: AntiScalar) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
extension Flector: AntiWedge<Circle> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Circle) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((other.group0.xyz.zxy * this.group1.yzx) - (other.group0.xyz.yzx * this.group1.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * this.e3215, other.e431 * this.e3215, other.e412 * this.e3215, -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - float4((other.group2.xyz.zxy * this.group1.yzx), other.e423 * this.e15)
        );
    }
}
extension Flector: AntiWedge<CircleRotor> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: CircleRotor) -> Output {
        return VersorOdd.from_groups(
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
extension Flector: AntiWedge<Dipole> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Dipole) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group3.yzx * this.group1.zxy) - (other.group3.zxy * this.group1.yzx)), other.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((this.group1.www * other.group3.xyz), other.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e35 * this.e4125)) + float4((other.group1.zxy * this.group1.yzx), other.e4235 * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group0.xyz, this.e4315)) - (this.group1.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
extension Flector: AntiWedge<DualNum> {
    associatedtype Output = Flector;
    func anti_wedge(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
extension Flector: AntiWedge<FlatPoint> {
    associatedtype Output = DualNum;
    func anti_wedge(other: FlatPoint) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension Flector: AntiWedge<Flector> {
    associatedtype Output = Motor;
    func anti_wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (other.group1.wwwx * float4(this.group1.xyz, this.e15)) + float4(float3(0.0), (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)) - (this.group1.wwwx * float4(other.group1.xyz, other.e15))
        );
    }
}
extension Flector: AntiWedge<Line> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: Line) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) - (this.group1.yzxx * float4(other.group1.xyz.zxy, other.e415))
        );
    }
}
extension Flector: AntiWedge<Motor> {
    associatedtype Output = Flector;
    func anti_wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
extension Flector: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(this.e15 * other.e1234) - (this.e4125 * other.e31), -(this.e25 * other.e1234) - (this.e4235 * other.e12), -(this.e35 * other.e1234) - (this.e4315 * other.e23), (this.e4315 * other.e42) + (this.e4125 * other.e43)) + (this.group1.yzxx * float4(other.group5.xyz.zxy, other.e41)) - float4((other.group4.xyz * this.group1.www), this.e45 * other.e1234), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group0) - float4((other.group8.xyz.zxy * this.group1.yzx), this.e4235 * other.e415), 
            /* e41, e42, e43, 0 */
            float4((other.group7.xyz.zxy * this.group1.yzx) - (other.group7.xyz.yzx * this.group1.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * other.group9.yzx) - (this.group1.yzx * other.group9.zxy)), this.e3215 * other.e1234 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(float3(other.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group1.xyz) - (float3(this.e3215) * other.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: AntiWedge<Plane> {
    associatedtype Output = Motor;
    func anti_wedge(other: Plane) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(this.e3215 * other.e4235 * -1.0, this.e3215 * other.e4315 * -1.0, this.e3215 * other.e4125 * -1.0, (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215)) + (other.group0.wwwx * float4(this.group1.xyz, this.e15))
        );
    }
}
extension Flector: AntiWedge<RoundPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiWedge<Sphere> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Sphere) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e3215) * this.group1.xyz) - (float3(this.e3215) * other.group0.xyz)), this.e45 * other.e1234 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(other.e1234) * this.group0.xyz * float3(-1.0)), (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215))
        );
    }
}
extension Flector: AntiWedge<VersorEven> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: VersorEven) -> Output {
        return VersorOdd.from_groups(
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
extension Flector: AntiWedge<VersorOdd> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e1234) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * other.group3.yzx) - (this.group1.yzx * other.group3.zxy)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (this.e4315 * other.e42) + (this.e4125 * other.e43)) - float4((this.group1.www * other.group3.xyz), this.e45 * other.e1234), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e3215 * other.e45)) + float4((this.group1.yzx * other.group1.zxy), this.e15 * other.e4235) - (this.group1.zxyy * float4(other.group1.yzx, other.e25)) - (this.group1.wwwz * float4(other.group0.xyz, other.e35)) - (other.group2.wwwx * float4(this.group0.xyz, this.e4235))
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Line> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Line: AntiWedge<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e431 * this.e425) - (other.e412 * this.e435)) - float4((other.group0.xyz.yzx * this.group1.xyz.zxy), other.e423 * this.e415), 
            /* e5, 0, 0, 0 */
            float4(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiWedge<AntiDualNum> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: AntiDualNum) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * this.group0.xyz), -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
extension Line: AntiWedge<AntiFlector> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * this.group0.xyz), -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
extension Line: AntiWedge<AntiLine> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiLine) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiWedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: AntiWedge<AntiScalar> {
    associatedtype Output = Line;
    func anti_wedge(other: AntiScalar) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: AntiWedge<Circle> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Circle) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e431 * this.e425) - (other.e412 * this.e435)) - float4((other.group0.xyz.yzx * this.group1.xyz.zxy), other.e423 * this.e415), 
            /* e5, 0, 0, 0 */
            float4(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiWedge<CircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: CircleRotor) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group1.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e315 * this.e425) - (other.e125 * this.e435)) - float4((other.group0.xyz.yzx * this.group1.xyz.zxy), other.e235 * this.e415)
        );
    }
}
extension Line: AntiWedge<Dipole> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(other.e1234) * this.group1.xyz), -(other.e4235 * this.e415) - (other.e4315 * this.e425) - (other.e4125 * this.e435)), 
            /* e15, e25, e35, scalar */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) - float4((this.group1.xyz.zxy * other.group3.yzx), other.e41 * this.e235)
        );
    }
}
extension Line: AntiWedge<DualNum> {
    associatedtype Output = Line;
    func anti_wedge(other: DualNum) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(other.e12345) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: AntiWedge<Flector> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: Flector) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) - (other.group1.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
extension Line: AntiWedge<Line> {
    associatedtype Output = DualNum;
    func anti_wedge(other: Line) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0), 0.0, 0.0)
        );
    }
}
extension Line: AntiWedge<Motor> {
    associatedtype Output = Motor;
    func anti_wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e12345) * this.group1.xyz), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
extension Line: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * other.e321) + (this.e315 * other.e412), (this.e425 * other.e321) + (this.e125 * other.e423), (this.e435 * other.e321) + (this.e235 * other.e431), -(this.e425 * other.e431) - (this.e435 * other.e412)) - float4((this.group1.xyz.zxy * other.group7.xyz.yzx), this.e415 * other.e423), 
            /* e5, 0, 0, 0 */
            float4(-(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group9.yzxx * float4(this.group1.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e1234) * this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xy.yy, other.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Line: AntiWedge<Plane> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: Plane) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group1.xyz.zxy, this.e415))
        );
    }
}
extension Line: AntiWedge<Sphere> {
    associatedtype Output = Dipole;
    func anti_wedge(other: Sphere) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(other.e1234) * this.group1.xyz), -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)), 
            /* e15, e25, e35, 0 */
            float4((float3(other.e3215) * this.group0.xyz) + (this.group1.xyz.yzx * other.group0.zxy) - (this.group1.xyz.zxy * other.group0.yzx), 0.0)
        );
    }
}
extension Line: AntiWedge<VersorEven> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: VersorEven) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group1.xyz), -(this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((this.e415 * other.e321) + (this.e315 * other.e412), (this.e425 * other.e321) + (this.e125 * other.e423), (this.e435 * other.e321) + (this.e235 * other.e431), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) - float4((this.group1.xyz.zxy * other.group0.yzx), this.e415 * other.e235)
        );
    }
}
extension Line: AntiWedge<VersorOdd> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(other.e1234) * this.group1.xyz), -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)), 
            /* e15, e25, e35, scalar */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group1.xyz.zxy * other.group3.yzx), this.e415 * other.e23)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Motor> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Motor: AntiWedge<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435))
        );
    }
}
extension Motor: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((other.e321 * this.e415) + (other.e1 * this.e12345), (other.e321 * this.e425) + (other.e2 * this.e12345), (other.e321 * this.e435) + (other.e3 * this.e12345), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((other.group0.xyz.zxy * this.group1.yzx), other.e5 * this.e12345) - (this.group1.zxyx * float4(other.group0.xyz.yzx, other.e415))
        );
    }
}
extension Motor: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * this.group0
        );
    }
}
extension Motor: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.group0.www * this.group0.xyz), -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
extension Motor: AntiWedge<AntiFlector> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e1 * this.e12345, other.e2 * this.e12345, other.e3 * this.e12345, -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + (this.group0 * float4(other.group0.www, other.e5))
        );
    }
}
extension Motor: AntiWedge<AntiLine> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiLine) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: AntiWedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(this.e12345) * other.group1.xyz)), other.e3215 * this.e12345)
        );
    }
}
extension Motor: AntiWedge<AntiPlane> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiPlane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
extension Motor: AntiWedge<AntiScalar> {
    associatedtype Output = Motor;
    func anti_wedge(other: AntiScalar) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.e12345) * this.group1
        );
    }
}
extension Motor: AntiWedge<Circle> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Circle) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e315 * this.e425) - (other.e125 * this.e435)) - float4((other.group0.xyz.yzx * this.group1.zxy), other.e235 * this.e415)
        );
    }
}
extension Motor: AntiWedge<CircleRotor> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: CircleRotor) -> Output {
        return VersorEven.from_groups(
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
extension Motor: AntiWedge<Dipole> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Dipole) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435))
        );
    }
}
extension Motor: AntiWedge<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
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
extension Motor: AntiWedge<DualNum> {
    associatedtype Output = Motor;
    func anti_wedge(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.group0.xy.yy, other.e12345, (other.e5 * this.e12345) + (other.e12345 * this.e5)) * float4(this.group1.xyz, 1.0)
        );
    }
}
extension Motor: AntiWedge<FlatPoint> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: FlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0
        );
    }
}
extension Motor: AntiWedge<Flector> {
    associatedtype Output = Flector;
    func anti_wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
extension Motor: AntiWedge<Line> {
    associatedtype Output = Motor;
    func anti_wedge(other: Line) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e12345) * other.group1.xyz), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
extension Motor: AntiWedge<Motor> {
    associatedtype Output = Motor;
    func anti_wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e12345), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
extension Motor: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e12345 * other.scalar) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), this.e12345 * other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e415 * other.e321) + (this.e12345 * other.e1), (this.e425 * other.e321) + (this.e12345 * other.e2), (this.e435 * other.e321) + (this.e12345 * other.e3), -(this.e425 * other.e431) - (this.e435 * other.e412)) + float4((other.group7.xyz.zxy * this.group1.yzx), this.e12345 * other.e4) - float4((other.group7.xyz.yzx * this.group1.zxy), this.e415 * other.e423), 
            /* e5, 0, 0, 0 */
            float4((this.e12345 * other.e5) + (this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e12345 * other.e15) + (this.e315 * other.e4125), (this.e12345 * other.e25) + (this.e125 * other.e4235), (this.e12345 * other.e35) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + (this.group0 * float4(other.group9.www, other.e45)) - (other.group9.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e12345) * other.group5.xyz) + (float3(other.e1234) * this.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * other.group6.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e12345 * other.e321), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e12345) * other.group8.xyz) + (float3(other.e12345) * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiWedge<Plane> {
    associatedtype Output = Flector;
    func anti_wedge(other: Plane) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
extension Motor: AntiWedge<RoundPoint> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: RoundPoint) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * other.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiWedge<Scalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiWedge<Sphere> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: Sphere) -> Output {
        return VersorOdd.from_groups(
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
extension Motor: AntiWedge<VersorEven> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: VersorEven) -> Output {
        return VersorEven.from_groups(
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
extension Motor: AntiWedge<VersorOdd> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
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
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<MultiVector> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension MultiVector: AntiWedge<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group9.yzxx * float4(other.group1.zxy, other.e41)) + float4((float3(this.e1234) * other.group2.xyz), other.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * other.group1.xyz, 0.0), 
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
extension MultiVector: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + (float4(this.e12345) * float4(other.group3.xyz, other.e4)) - float4((other.group0.xyz.yzx * this.group8.xyz.zxy), other.e423 * this.e415) - float4((this.group7.xyz.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4((other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group9.yzx) - (other.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<AntiDualNum> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: AntiDualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e3215 * this.e4) + (other.scalar * this.e12345), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e3215 * this.e45, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group6.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e3215) * this.group7.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e3215 * this.e1234), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e3215) * this.group9.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * this.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((this.group7.xyz.zxy * other.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * other.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4(-(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((other.group0.yzx * this.group9.zxy) - (other.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * this.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<AntiFlector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: AntiFlector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((float3(this.e12345) * other.group1.xyz), 0.0) + float4((this.group7.xyz.zxy * other.group0.yzx), 0.0) - float4((this.group7.xyz.yzx * other.group0.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4((other.e5 * this.e12345) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((other.group0.yzx * this.group9.zxy) - (other.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * this.e12345), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<AntiLine> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: AntiLine) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.xyz.zxy * this.group9.yzx), 0.0) - float4((other.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
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
extension MultiVector: AntiWedge<AntiMotor> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: AntiMotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.scalar * this.e12345) + (other.e3215 * this.e4) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e3215) * this.group4.xyz), 0.0) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group9.yzx), 0.0) - float4((other.group0.yzx * this.group9.zxy), 0.0), 
            /* e5, 0, 0, 0 */
            float4((other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * this.group6.xyz) + (float3(this.e12345) * other.group1.xyz)), 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e3215) * this.group7.xyz) + (float3(this.e12345) * other.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e3215 * this.e1234), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e3215) * this.group9.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * this.e12345), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<AntiPlane> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: AntiPlane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * this.e12345, 0.0, 0.0, 0.0), 
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
extension MultiVector: AntiWedge<AntiScalar> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.e12345) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e12345 * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e12345) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiWedge<Circle> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Circle) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.xyz.yzx * this.group8.xyz.zxy), other.e423 * this.e415) - float4((other.group2.xyz.zxy * this.group7.xyz.yzx), other.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group9.yzxx * float4(other.group2.xyz.zxy, other.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group9.yzx) - (other.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<CircleRotor> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: CircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), other.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e12345 * this.e1), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e12345 * this.e2), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e12345 * this.e3), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((other.group0.xyz.zxy * this.group8.xyz.yzx), other.e12345 * this.e4) - float4((other.group0.xyz.yzx * this.group8.xyz.zxy), other.e423 * this.e415) - float4((this.group7.xyz.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4((other.e12345 * this.e5) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e315 * this.e4125) + (other.e12345 * this.e15), (other.e125 * this.e4235) + (other.e12345 * this.e25), (other.e235 * this.e4315) + (other.e12345 * this.e35), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((this.group9.www * other.group1.xyz), other.e12345 * this.e45) - (this.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e12345) * this.group4.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group9.yzx) - (other.group0.xyz.yzx * this.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e12345) * this.group5.xyz) + (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group6) + (float4(this.e12345) * other.group1), 
            /* e423, e431, e412, 0 */
            float4((float3(other.e12345) * this.group7.xyz) + (float3(this.e12345) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e12345) * this.group8.xyz) + (float3(this.e12345) * other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiWedge<Dipole> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Dipole) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group9.yzxy * float4(other.group1.zxy, other.e42)) + float4((float3(this.e1234) * other.group2.xyz), other.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e12345) * other.group1.xyz, 0.0), 
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
extension MultiVector: AntiWedge<DipoleInversion> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: DipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.group2.xyz, other.e45)) + (this.group9.yzxz * float4(other.group1.zxy, other.e43)) + float4((this.group4.xyz * other.group3.www), other.e41 * this.e4235) + float4((this.group5.xyz.yzx * other.group3.zxy), other.e42 * this.e4315) - (float4(other.e1234) * this.group3) - (other.group3.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((other.group0.xyz * this.group9.www), other.e4235 * this.e41) - float4((other.group1.yzx * this.group9.zxy), other.e4125 * this.e43), 
            /* e5, 0, 0, 0 */
            float4((other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * float4(other.group2.xyz, other.e45)) - float4((this.group8.xyz.zxy * other.group3.yzx), other.e4235 * this.e415), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz) + (this.group7.xyz.zxy * other.group3.yzx) - (this.group7.xyz.yzx * other.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e1234) * this.group8.xyz) + (float3(other.e3215) * this.group7.xyz) + (float3(this.e12345) * other.group1.xyz) - (float3(this.e321) * other.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiWedge<DualNum> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e5 * this.e1234) + (other.e12345 * this.scalar), other.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group1, 
            /* e5, 0, 0, 0 */
            float4((other.e5 * this.e12345) + (other.e12345 * this.e5), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(other.e12345) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.e12345) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e12345) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiWedge<FlatPoint> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: FlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0, 
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
extension MultiVector: AntiWedge<Flector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e15 * this.e1234) + (other.e4125 * this.e31), (other.e25 * this.e1234) + (other.e4235 * this.e12), (other.e35 * this.e1234) + (other.e4315 * this.e23), -(other.e4315 * this.e42) - (other.e4125 * this.e43)) + float4((this.group4.xyz * other.group1.www), other.e45 * this.e1234) - (other.group1.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group0) - float4((this.group8.xyz.zxy * other.group1.yzx), other.e4235 * this.e415), 
            /* e41, e42, e43, 0 */
            float4((this.group7.xyz.zxy * other.group1.yzx) - (this.group7.xyz.yzx * other.group1.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group1.yzx * this.group9.zxy) - (other.group1.zxy * this.group9.yzx)), other.e3215 * this.e1234), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<Line> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(-(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * this.e321) + (other.e315 * this.e412), (other.e425 * this.e321) + (other.e125 * this.e423), (other.e435 * this.e321) + (other.e235 * this.e431), -(other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group1.xyz.zxy * this.group7.xyz.yzx), other.e415 * this.e423), 
            /* e5, 0, 0, 0 */
            float4(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group9.yzxx * float4(other.group1.xyz.zxy, other.e415)), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.yy, this.e12345, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e12345) * other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<Motor> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e12345 * this.scalar) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), other.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e415 * this.e321) + (other.e12345 * this.e1), (other.e425 * this.e321) + (other.e12345 * this.e2), (other.e435 * this.e321) + (other.e12345 * this.e3), -(other.e425 * this.e431) - (other.e435 * this.e412)) + float4((this.group7.xyz.zxy * other.group1.yzx), other.e12345 * this.e4) - float4((this.group7.xyz.yzx * other.group1.zxy), other.e415 * this.e423), 
            /* e5, 0, 0, 0 */
            float4((other.e12345 * this.e5) + (other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e12345 * this.e15) + (other.e315 * this.e4125), (other.e12345 * this.e25) + (other.e125 * this.e4235), (other.e12345 * this.e35) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + (other.group0 * float4(this.group9.www, this.e45)) - (this.group9.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e12345) * this.group4.xyz) + (float3(this.e1234) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e12345) * this.group5.xyz) + (float3(this.e1234) * other.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e321), 
            /* e423, e431, e412, 0 */
            float4(float3(other.e12345) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e12345) * this.group8.xyz) + (float3(this.e12345) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.scalar * this.e12345) + (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), other.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e15 * this.e1234) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e4125 * this.e31), (other.e25 * this.e1234) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e4235 * this.e12), (other.e35 * this.e1234) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e4315 * this.e23), -(other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412) - (other.e1234 * this.e45)) + (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1) + (this.group9.yzxx * float4(other.group5.xyz.zxy, other.e41)) + float4((this.group4.xyz * other.group9.www), other.e45 * this.e1234) + float4((other.group7.xyz.zxy * this.group8.xyz.yzx), other.e42 * this.e4315) + float4((other.group8.xyz.yzx * this.group7.xyz.zxy), other.e43 * this.e4125) - (other.group9.yzxy * float4(this.group5.xyz.zxy, this.e42)) - float4((float3(other.e1234) * this.group3.xyz), other.e4125 * this.e43) - float4((other.group4.xyz * this.group9.www), other.e423 * this.e415) - float4((other.group5.xyz.yzx * this.group9.zxy), other.e431 * this.e425) - float4((other.group7.xyz.yzx * this.group8.xyz.zxy), other.e412 * this.e435) - float4((other.group8.xyz.zxy * this.group7.xyz.yzx), other.e4235 * this.e41), 
            /* e5, 0, 0, 0 */
            float4((other.e12345 * this.e5) + (other.e5 * this.e12345) + (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125) + (other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e425 * this.e3215) + (other.e125 * this.e4235) + (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e435 * this.e3215) + (other.e235 * this.e4315) + (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e435 * this.e4125) - (other.e4235 * this.e415) - (other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(other.e12345) * this.group3) + (float4(this.e12345) * other.group3) - (this.group9.yzxx * float4(other.group8.xyz.zxy, other.e415)) - float4((this.group8.xyz.zxy * other.group9.yzx), other.e425 * this.e4315), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e12345) * this.group4.xyz) + (float3(other.e1234) * this.group6.xyz) + (float3(this.e12345) * other.group4.xyz) + (float3(this.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * this.group9.yzx) + (this.group7.xyz.zxy * other.group9.yzx) - (other.group7.xyz.yzx * this.group9.zxy) - (this.group7.xyz.yzx * other.group9.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e12345) * this.group5.xyz) + (float3(other.e3215) * this.group7.xyz) + (float3(other.e1234) * this.group8.xyz) + (float3(this.e12345) * other.group5.xyz) + (float3(this.e3215) * other.group7.xyz) + (float3(this.e1234) * other.group8.xyz) - (float3(other.e321) * this.group9.xyz) - (float3(this.e321) * other.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group6) + (float4(this.e12345) * other.group6) + (other.group9.yzxw * float4(this.group9.zxy, this.e1234)) - float4((other.group9.zxy * this.group9.yzx), other.e1234 * this.e3215), 
            /* e423, e431, e412, 0 */
            float4((float3(other.e12345) * this.group7.xyz) + (float3(this.e12345) * other.group7.xyz) + (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e12345) * this.group8.xyz) + (float3(other.e3215) * this.group9.xyz) + (float3(this.e12345) * other.group8.xyz) - (float3(this.e3215) * other.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * this.group9) + (float4(this.e12345) * other.group9), 
            /* e1234, 0, 0, 0 */
            float4((other.e12345 * this.e1234) + (other.e1234 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiWedge<Plane> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((this.group7.xyz.zxy * other.group0.yzx) - (this.group7.xyz.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group9.zxy * other.group0.yzx) - (this.group9.yzx * other.group0.zxy)), this.e1234 * other.e3215), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<RoundPoint> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: RoundPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e12345 * other.e5, 0.0, 0.0, 0.0), 
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
extension MultiVector: AntiWedge<Scalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiWedge<Sphere> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: Sphere) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e45 * other.e1234) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((float3(other.e1234) * this.group3.xyz), this.e42 * other.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * other.group0.yzx) - (this.group7.xyz.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e3215) * this.group7.xyz) + (float3(other.e1234) * this.group8.xyz) - (float3(this.e321) * other.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * other.group0.yzx), this.e1234 * other.e3215) - (this.group9.yzxw * float4(other.group0.zxy, other.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiWedge<VersorEven> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: VersorEven) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.scalar * other.e12345) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435), this.e12345 * other.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e1 * other.e12345) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e2 * other.e12345) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e3 * other.e12345) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412) - (this.e412 * other.e435)) + (float4(this.e12345) * other.group3) + float4((this.group7.xyz.zxy * other.group2.yzx), this.e4 * other.e12345) - float4((this.group7.xyz.yzx * other.group2.zxy), this.e423 * other.e415) - float4((this.group8.xyz.zxy * other.group0.yzx), this.e431 * other.e425), 
            /* e5, 0, 0, 0 */
            float4((this.e12345 * other.e5) + (this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group3) - (this.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group1.xyz) + (float3(other.e12345) * this.group4.xyz) + (this.group9.yzx * other.group0.zxy) - (this.group9.zxy * other.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) + (float3(other.e12345) * this.group5.xyz) - (float3(other.e321) * this.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * other.group1) + (float4(other.e12345) * this.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group7.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e12345) * other.group2.xyz) + (float3(other.e12345) * this.group8.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiWedge<VersorOdd> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: VersorOdd) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((this.e12345 * other.scalar) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.group2.xyz, other.e45)) + (this.group9.yzxz * float4(other.group1.zxy, other.e43)) + float4((this.group4.xyz * other.group3.www), this.e4235 * other.e41) + float4((this.group5.xyz.yzx * other.group3.zxy), this.e4315 * other.e42) - (other.group3.yzxx * float4(this.group5.xyz.zxy, this.e41)) - float4((this.group9.zxy * other.group1.yzx), this.e43 * other.e4125) - float4((this.group9.www * other.group0.xyz), this.e45 * other.e1234) - float4((other.group2.www * this.group3.xyz), this.e42 * other.e4315), 
            /* e5, 0, 0, 0 */
            float4((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + (float4(this.e12345) * float4(other.group2.xyz, other.e45)) - (other.group3.yzxx * float4(this.group8.xyz.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e12345) * other.group0.xyz) + (float3(other.e1234) * this.group6.xyz) + (this.group7.xyz.zxy * other.group3.yzx) - (this.group7.xyz.yzx * other.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e12345) * other.group1.xyz) + (float3(other.e1234) * this.group8.xyz) + (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group9.zxy * other.group3.yzx), this.e1234 * other.e3215) - (this.group9.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group9.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3, 
            /* e1234, 0, 0, 0 */
            float4(this.e12345 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Plane> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Plane: AntiWedge<AntiCircleRotor> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * this.e3215, other.e431 * this.e3215, other.e412 * this.e3215, -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(other.e125 * this.e4315 * -1.0, other.e235 * this.e4125 * -1.0, other.e315 * this.e4235 * -1.0, (other.e2 * this.e4315) + (other.e3 * this.e4125)) + (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group0.zxyx * float4(other.group2.yzx, other.e1))
        );
    }
}
extension Plane: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiFlatPoint;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.group0.xy.xx, other.e3215, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Plane: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiLine;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(other.e321) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Plane: AntiWedge<AntiFlector> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((other.group0.www * this.group0.xyz * float3(-1.0)), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Plane: AntiWedge<AntiLine> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiLine) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
extension Plane: AntiWedge<AntiMotor> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
extension Plane: AntiWedge<AntiPlane> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiPlane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: AntiWedge<AntiScalar> {
    associatedtype Output = Plane;
    func anti_wedge(other: AntiScalar) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
extension Plane: AntiWedge<Circle> {
    associatedtype Output = Dipole;
    func anti_wedge(other: Circle) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * this.e3215, other.e431 * this.e3215, other.e412 * this.e3215, -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(this.e3215) * other.group1.xyz) + (other.group2.xyz.yzx * this.group0.zxy) - (other.group2.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Plane: AntiWedge<CircleRotor> {
    associatedtype Output = DipoleInversion;
    func anti_wedge(other: CircleRotor) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.e423 * this.e3215, other.e431 * this.e3215, other.e412 * this.e3215, -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - float4((other.group2.zxy * this.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
extension Plane: AntiWedge<Dipole> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Dipole) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), other.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(other.e4235 * this.e3215 * -1.0, other.e4315 * this.e3215 * -1.0, other.e4125 * this.e3215 * -1.0, (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
extension Plane: AntiWedge<DualNum> {
    associatedtype Output = Plane;
    func anti_wedge(other: DualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
extension Plane: AntiWedge<FlatPoint> {
    associatedtype Output = DualNum;
    func anti_wedge(other: FlatPoint) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension Plane: AntiWedge<Flector> {
    associatedtype Output = Motor;
    func anti_wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(other.e3215 * this.e4235, other.e3215 * this.e4315, other.e3215 * this.e4125, -(other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)) - (this.group0.wwwx * float4(other.group1.xyz, other.e15))
        );
    }
}
extension Plane: AntiWedge<Line> {
    associatedtype Output = FlatPoint;
    func anti_wedge(other: Line) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group1.xyz.zxy, other.e415))
        );
    }
}
extension Plane: AntiWedge<Motor> {
    associatedtype Output = Flector;
    func anti_wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
extension Plane: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group5.xyz.zxy, other.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group8.xyz.zxy, other.e415)), 
            /* e41, e42, e43, 0 */
            float4((other.group7.xyz.zxy * this.group0.yzx) - (other.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group9.yzx * this.group0.zxy) - (other.group9.zxy * this.group0.yzx)), other.e1234 * this.e3215 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(float3(other.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane: AntiWedge<Plane> {
    associatedtype Output = Line;
    func anti_wedge(other: Plane) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz), 0.0)
        );
    }
}
extension Plane: AntiWedge<RoundPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: AntiWedge<Sphere> {
    associatedtype Output = Circle;
    func anti_wedge(other: Sphere) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz), 0.0)
        );
    }
}
extension Plane: AntiWedge<VersorEven> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: VersorEven) -> Output {
        return VersorOdd.from_groups(
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
extension Plane: AntiWedge<VersorOdd> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e1234) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * other.e4235 * -1.0, this.e3215 * other.e4315 * -1.0, this.e3215 * other.e4125 * -1.0, (this.e4315 * other.e42) + (this.e4125 * other.e43)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(this.e4315 * other.e12, this.e4125 * other.e23, this.e4235 * other.e31, -(this.e4125 * other.e35) - (this.e3215 * other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15)) - (this.group0.wwwy * float4(other.group0.xyz, other.e25))
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<RoundPoint> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension RoundPoint: AntiWedge<AntiDualNum> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiDualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e3215 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiWedge<AntiMotor> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e3215 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiWedge<AntiScalar> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: AntiScalar) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(other.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiWedge<CircleRotor> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: CircleRotor) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(other.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiWedge<DipoleInversion> {
    associatedtype Output = Scalar;
    func anti_wedge(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiWedge<DualNum> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: DualNum) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(other.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiWedge<Flector> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiWedge<Motor> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Motor) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(other.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(other.e12345 * this.e5, 0.0, 0.0, 0.0), 
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
extension RoundPoint: AntiWedge<Plane> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiWedge<Sphere> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiWedge<VersorEven> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: VersorEven) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiWedge<VersorOdd> {
    associatedtype Output = Scalar;
    func anti_wedge(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Scalar> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Scalar: AntiWedge<AntiScalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiWedge<CircleRotor> {
    associatedtype Output = Scalar;
    func anti_wedge(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiWedge<DualNum> {
    associatedtype Output = Scalar;
    func anti_wedge(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiWedge<Motor> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiWedge<MultiVector> {
    associatedtype Output = Scalar;
    func anti_wedge(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiWedge<VersorEven> {
    associatedtype Output = Scalar;
    func anti_wedge(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<Sphere> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension Sphere: AntiWedge<AntiCircleRotor> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group0.yzxx * float4(other.group1.zxy, other.e41)) + float4((float3(this.e1234) * other.group2.xyz), other.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(other.e125 * this.e4315 * -1.0, other.e235 * this.e4125 * -1.0, other.e315 * this.e4235 * -1.0, (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) + (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group0.zxyx * float4(other.group2.yzx, other.e1))
        );
    }
}
extension Sphere: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiFlatPoint;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group0.xyz, this.e1234)
        );
    }
}
extension Sphere: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiLine;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4((float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, 0 */
            float4((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Sphere: AntiWedge<AntiFlector> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.e321 * this.e4235 * -1.0, other.e321 * this.e4315 * -1.0, other.e321 * this.e4125 * -1.0, (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) + float4((float3(this.e1234) * other.group0.xyz), other.e1 * this.e4235), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Sphere: AntiWedge<AntiLine> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiLine) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
extension Sphere: AntiWedge<AntiMotor> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group0.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
extension Sphere: AntiWedge<AntiPlane> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiPlane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiWedge<AntiScalar> {
    associatedtype Output = Sphere;
    func anti_wedge(other: AntiScalar) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(other.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiWedge<Circle> {
    associatedtype Output = Dipole;
    func anti_wedge(other: Circle) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(this.e3215) * other.group1.xyz) + (other.group2.xyz.yzx * this.group0.zxy) - (other.group2.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Sphere: AntiWedge<CircleRotor> {
    associatedtype Output = DipoleInversion;
    func anti_wedge(other: CircleRotor) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group0.yzx) - (other.group0.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
extension Sphere: AntiWedge<Dipole> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Dipole) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group0.yzxy * float4(other.group1.zxy, other.e42)) + float4((float3(this.e1234) * other.group2.xyz), other.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            float4(other.e4235 * this.e3215 * -1.0, other.e4315 * this.e3215 * -1.0, other.e4125 * this.e3215 * -1.0, (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
extension Sphere: AntiWedge<DualNum> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: DualNum) -> Output {
        return VersorOdd.from_groups(
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
extension Sphere: AntiWedge<FlatPoint> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: FlatPoint) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiWedge<Flector> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Flector) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), other.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group1.xyz)), other.e45 * this.e1234), 
            /* e1, e2, e3, e5 */
            float4((float3(this.e1234) * other.group0.xyz), -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215))
        );
    }
}
extension Sphere: AntiWedge<Line> {
    associatedtype Output = Dipole;
    func anti_wedge(other: Line) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125)), 
            /* e15, e25, e35, 0 */
            float4((float3(this.e3215) * other.group0.xyz) + (other.group1.xyz.yzx * this.group0.zxy) - (other.group1.xyz.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Sphere: AntiWedge<Motor> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: Motor) -> Output {
        return VersorOdd.from_groups(
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
extension Sphere: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e45 * this.e1234) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group5.xyz.zxy, other.e41)) + float4((float3(this.e1234) * other.group3.xyz), other.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group8.xyz.zxy, other.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * this.group0.yzx) - (other.group7.xyz.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.e3215) * other.group7.xyz) + (float3(this.e1234) * other.group8.xyz) - (float3(other.e321) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group9.yzxw * float4(this.group0.zxy, this.e1234)) - float4((other.group9.zxy * this.group0.yzx), other.e1234 * this.e3215), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(other.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiWedge<Plane> {
    associatedtype Output = Circle;
    func anti_wedge(other: Plane) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), other.e3215 * this.e1234), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz), 0.0)
        );
    }
}
extension Sphere: AntiWedge<RoundPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiWedge<Sphere> {
    associatedtype Output = Circle;
    func anti_wedge(other: Sphere) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group0.yzxw * float4(this.group0.zxy, this.e1234)) - float4((other.group0.zxy * this.group0.yzx), other.e1234 * this.e3215), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz), 0.0)
        );
    }
}
extension Sphere: AntiWedge<VersorEven> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: VersorEven) -> Output {
        return VersorOdd.from_groups(
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
extension Sphere: AntiWedge<VersorOdd> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group0.zxy * other.group3.yzx), this.e1234 * other.e3215) - (this.group0.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            float4(this.e3215 * other.e4235 * -1.0, this.e3215 * other.e4315 * -1.0, this.e3215 * other.e4125 * -1.0, (this.e4315 * other.e42) + (this.e4125 * other.e43) + (this.e1234 * other.e45)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4((this.e4315 * other.e12) + (this.e1234 * other.e15), (this.e4125 * other.e23) + (this.e1234 * other.e25), (this.e4235 * other.e31) + (this.e1234 * other.e35), -(this.e4125 * other.e35) - (this.e3215 * other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15)) - (this.group0.wwwy * float4(other.group0.xyz, other.e25))
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<VersorEven> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension VersorEven: AntiWedge<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
extension VersorEven: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e1 * this.e12345), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e2 * this.e12345), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e3 * this.e12345), -(other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((other.group0.xyz.zxy * this.group2.yzx), other.e5 * this.e12345) - (this.group2.zxyx * float4(other.group0.xyz.yzx, other.e415)) - float4((other.group2.zxy * this.group0.yzx), other.e425 * this.e315)
        );
    }
}
extension VersorEven: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.group0.xy.xx, other.e3215, (other.e3215 * this.e4) + (other.scalar * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
extension VersorEven: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - (other.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
extension VersorEven: AntiWedge<AntiFlector> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((other.e321 * this.e415) + (other.e1 * this.e12345), (other.e321 * this.e425) + (other.e2 * this.e12345), (other.e321 * this.e435) + (other.e3 * this.e12345), -(other.e315 * this.e425) - (other.e125 * this.e435)) + (this.group0.zxyw * float4(other.group0.yzx, other.e5)) - (other.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
extension VersorEven: AntiWedge<AntiLine> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiLine) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension VersorEven: AntiWedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            (float4(other.e3215) * float4(this.group0.xyz, this.e4)) + (float4(this.e12345) * other.group0) + float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group1.xyz)), other.e3215 * this.e12345)
        );
    }
}
extension VersorEven: AntiWedge<AntiPlane> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiPlane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
extension VersorEven: AntiWedge<AntiScalar> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: AntiScalar) -> Output {
        return VersorEven.from_groups(
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
extension VersorEven: AntiWedge<Circle> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Circle) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e125 * this.e435)) - float4((other.group0.xyz.yzx * this.group2.zxy), other.e235 * this.e415) - float4((other.group2.xyz.zxy * this.group0.yzx), other.e315 * this.e425)
        );
    }
}
extension VersorEven: AntiWedge<CircleRotor> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: CircleRotor) -> Output {
        return VersorEven.from_groups(
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
extension VersorEven: AntiWedge<Dipole> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Dipole) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e12345) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
extension VersorEven: AntiWedge<DipoleInversion> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: DipoleInversion) -> Output {
        return VersorOdd.from_groups(
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
extension VersorEven: AntiWedge<DualNum> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: DualNum) -> Output {
        return VersorEven.from_groups(
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
extension VersorEven: AntiWedge<FlatPoint> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: FlatPoint) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45 * this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321))
        );
    }
}
extension VersorEven: AntiWedge<Flector> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: Flector) -> Output {
        return VersorOdd.from_groups(
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
extension VersorEven: AntiWedge<Line> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Line) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group1.xyz), -(other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((other.e415 * this.e321) + (other.e315 * this.e412), (other.e425 * this.e321) + (other.e125 * this.e423), (other.e435 * this.e321) + (other.e235 * this.e431), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) - float4((other.group1.xyz.zxy * this.group0.yzx), other.e415 * this.e235)
        );
    }
}
extension VersorEven: AntiWedge<Motor> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: Motor) -> Output {
        return VersorEven.from_groups(
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
extension VersorEven: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.scalar * this.e12345) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435), other.e12345 * this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e12345) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e2 * this.e12345) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e3 * this.e12345) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412) - (other.e412 * this.e435)) + (float4(other.e12345) * this.group3) + float4((other.group7.xyz.zxy * this.group2.yzx), other.e4 * this.e12345) - float4((other.group7.xyz.yzx * this.group2.zxy), other.e423 * this.e415) - float4((other.group8.xyz.zxy * this.group0.yzx), other.e431 * this.e425), 
            /* e5, 0, 0, 0 */
            float4((other.e12345 * this.e5) + (other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group3) - (other.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e1234) * this.group1.xyz) + (float3(this.e12345) * other.group4.xyz) + (other.group9.yzx * this.group0.zxy) - (other.group9.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) + (float3(this.e12345) * other.group5.xyz) - (float3(this.e321) * other.group9.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group7.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e12345) * this.group2.xyz) + (float3(this.e12345) * other.group8.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiWedge<Plane> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: Plane) -> Output {
        return VersorOdd.from_groups(
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
extension VersorEven: AntiWedge<RoundPoint> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: RoundPoint) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(other.e5 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiWedge<Scalar> {
    associatedtype Output = Scalar;
    func anti_wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiWedge<Sphere> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: Sphere) -> Output {
        return VersorOdd.from_groups(
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
extension VersorEven: AntiWedge<VersorEven> {
    associatedtype Output = VersorEven;
    func anti_wedge(other: VersorEven) -> Output {
        return VersorEven.from_groups(
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
extension VersorEven: AntiWedge<VersorOdd> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
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
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: AntiWedgeInfix) -> AntiWedgeInfixPartial<VersorOdd> {
        return AntiWedgeInfixPartial { a: this };
    }
}
extension VersorOdd: AntiWedge<AntiCircleRotor> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: AntiCircleRotor) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxx * float4(other.group1.zxy, other.e41)) + float4((this.group2.www * other.group2.xyz), other.e42 * this.e4315), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiWedge<AntiDipoleInversion> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: AntiDipoleInversion) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group3.yzx) - (other.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.zxy * this.group3.yzx), other.e423 * this.e15)
        );
    }
}
extension VersorOdd: AntiWedge<AntiDualNum> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiDualNum) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
extension VersorOdd: AntiWedge<AntiFlatPoint> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiFlatPoint) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.e235 * this.e1234, other.e315 * this.e1234, other.e125 * this.e1234, -(other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension VersorOdd: AntiWedge<AntiFlector> {
    associatedtype Output = AntiMotor;
    func anti_wedge(other: AntiFlector) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) + float4((this.group2.www * other.group0.xyz), other.e1 * this.e4235) - (other.group0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
extension VersorOdd: AntiWedge<AntiLine> {
    associatedtype Output = AntiPlane;
    func anti_wedge(other: AntiLine) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group3.zxyx * float4(other.group0.xyz.yzx, other.e15))
        );
    }
}
extension VersorOdd: AntiWedge<AntiMotor> {
    associatedtype Output = AntiFlector;
    func anti_wedge(other: AntiMotor) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((other.e15 * this.e1234) + (other.e3215 * this.e41), (other.e25 * this.e1234) + (other.e3215 * this.e42), (other.e35 * this.e1234) + (other.e3215 * this.e43), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((other.group0.zxy * this.group3.yzx), other.e3215 * this.e45) - (this.group3.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
extension VersorOdd: AntiWedge<AntiPlane> {
    associatedtype Output = Scalar;
    func anti_wedge(other: AntiPlane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiWedge<AntiScalar> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: AntiScalar) -> Output {
        return VersorOdd.from_groups(
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
extension VersorOdd: AntiWedge<Circle> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Circle) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e1234) * other.group1.xyz) + (other.group0.xyz.zxy * this.group3.yzx) - (other.group0.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.xyz.zxy * this.group3.yzx), other.e423 * this.e15)
        );
    }
}
extension VersorOdd: AntiWedge<CircleRotor> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: CircleRotor) -> Output {
        return VersorOdd.from_groups(
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
extension VersorOdd: AntiWedge<Dipole> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: Dipole) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxy * float4(other.group1.zxy, other.e42)) + float4((other.group2.xyz * this.group2.www), other.e41 * this.e4235), 
            /* e5, 0, 0, 0 */
            float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiWedge<DipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: DipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e4235 * this.e41) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - float4((this.group3.www * other.group3.xyz), other.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + float4((other.group1.zxy * this.group3.yzx), other.e4235 * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), other.e35 * this.e4125)
        );
    }
}
extension VersorOdd: AntiWedge<DualNum> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: DualNum) -> Output {
        return VersorOdd.from_groups(
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
extension VersorOdd: AntiWedge<FlatPoint> {
    associatedtype Output = RoundPoint;
    func anti_wedge(other: FlatPoint) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiWedge<Flector> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Flector) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx)), other.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(other.e4315 * this.e42) - (other.e4125 * this.e43)) + float4((other.group1.www * this.group3.xyz), other.e45 * this.e1234) - (other.group1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (other.group1.zxyy * float4(this.group1.yzx, this.e25)) + (other.group1.wwwz * float4(this.group0.xyz, this.e35)) + (this.group2.wwwx * float4(other.group0.xyz, other.e4235)) + float4(float3(0.0), (other.e3215 * this.e45) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)) - float4((other.group1.yzx * this.group1.zxy), other.e15 * this.e4235)
        );
    }
}
extension VersorOdd: AntiWedge<Line> {
    associatedtype Output = AntiCircleRotor;
    func anti_wedge(other: Line) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group1.xyz.zxy * this.group3.yzx), other.e415 * this.e23)
        );
    }
}
extension VersorOdd: AntiWedge<Motor> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: Motor) -> Output {
        return VersorOdd.from_groups(
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
extension VersorOdd: AntiWedge<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (this.group3.yzxx * float4(other.group5.xyz.zxy, other.e41)) + float4((other.group9.zxy * this.group1.yzx), other.e43 * this.e4125) + float4((other.group9.www * this.group0.xyz), other.e45 * this.e1234) + float4((this.group2.www * other.group3.xyz), other.e42 * this.e4315) - (float4(other.e1234) * float4(this.group2.xyz, this.e45)) - (other.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((other.group4.xyz * this.group3.www), other.e4235 * this.e41) - float4((other.group5.xyz.yzx * this.group3.zxy), other.e4315 * this.e42), 
            /* e5, 0, 0, 0 */
            float4((other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + (float4(other.e12345) * float4(this.group2.xyz, this.e45)) - (this.group3.yzxx * float4(other.group8.xyz.zxy, other.e415)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.e12345) * this.group0.xyz) + (float3(this.e1234) * other.group6.xyz) + (other.group7.xyz.zxy * this.group3.yzx) - (other.group7.xyz.yzx * this.group3.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.e12345) * this.group1.xyz) + (float3(this.e1234) * other.group8.xyz) + (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group9.yzxw * float4(this.group3.zxy, this.e1234)) - float4((other.group9.zxy * this.group3.yzx), other.e1234 * this.e3215), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group3.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3, 
            /* e1234, 0, 0, 0 */
            float4(other.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiWedge<Plane> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Plane) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e1234) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), other.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(other.e3215 * this.e4235, other.e3215 * this.e4315, other.e3215 * this.e4125, -(other.e4315 * this.e42) - (other.e4125 * this.e43)) - (other.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(other.e4315 * this.e12 * -1.0, other.e4125 * this.e23 * -1.0, other.e4235 * this.e31 * -1.0, (other.e4125 * this.e35) + (other.e3215 * this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + (other.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
extension VersorOdd: AntiWedge<RoundPoint> {
    associatedtype Output = Scalar;
    func anti_wedge(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiWedge<Sphere> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: Sphere) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group0.yzxw * float4(this.group3.zxy, this.e1234)) - float4((other.group0.zxy * this.group3.yzx), other.e1234 * this.e3215), 
            /* e235, e315, e125, e4 */
            float4(other.e3215 * this.e4235, other.e3215 * this.e4315, other.e3215 * this.e4125, -(other.e4315 * this.e42) - (other.e4125 * this.e43) - (other.e1234 * this.e45)) - (other.group0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(other.e4315 * this.e12) - (other.e1234 * this.e15), -(other.e4125 * this.e23) - (other.e1234 * this.e25), -(other.e4235 * this.e31) - (other.e1234 * this.e35), (other.e4125 * this.e35) + (other.e3215 * this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + (other.group0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
extension VersorOdd: AntiWedge<VersorEven> {
    associatedtype Output = VersorOdd;
    func anti_wedge(other: VersorEven) -> Output {
        return VersorOdd.from_groups(
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
extension VersorOdd: AntiWedge<VersorOdd> {
    associatedtype Output = AntiDipoleInversion;
    func anti_wedge(other: VersorOdd) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e4235 * this.e41) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - float4((this.group3.www * other.group3.xyz), other.e1234 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + float4((other.group1.zxy * this.group3.yzx), other.e4235 * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), other.e35 * this.e4125)
        );
    }
}

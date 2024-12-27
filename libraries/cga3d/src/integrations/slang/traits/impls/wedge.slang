extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<AntiCircleRotor> {
        return WedgeInfixPartial { a: this };
    }
}
extension AntiCircleRotor: Wedge<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func wedge(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
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
extension AntiCircleRotor: Wedge<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiDipoleInversion) -> Output {
        return VersorEven.from_groups(
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
extension AntiCircleRotor: Wedge<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func wedge(other: AntiDualNum) -> Output {
        return VersorOdd.from_groups(
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
extension AntiCircleRotor: Wedge<AntiFlatPoint> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiFlatPoint) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.scalar * other.e321), 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * other.group0.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321))
        );
    }
}
extension AntiCircleRotor: Wedge<AntiFlector> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiFlector) -> Output {
        return VersorEven.from_groups(
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
extension AntiCircleRotor: Wedge<AntiLine> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiLine) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group1.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e45 * other.e23), (this.e43 * other.e15) + (this.e45 * other.e31), (this.e41 * other.e25) + (this.e45 * other.e12), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group0.xyz.zxy * other.group1.xyz.yzx), this.e15 * other.e23)
        );
    }
}
extension AntiCircleRotor: Wedge<AntiMotor> {
    associatedtype Output = VersorOdd;
    func wedge(other: AntiMotor) -> Output {
        return VersorOdd.from_groups(
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
extension AntiCircleRotor: Wedge<AntiPlane> {
    associatedtype Output = AntiDipoleInversion;
    func wedge(other: AntiPlane) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * other.e5, this.e42 * other.e5, this.e43 * other.e5, -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * other.group0.yzx), 0.0) - float4((this.group2.yzx * other.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
extension AntiCircleRotor: Wedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: Wedge<Circle> {
    associatedtype Output = CircleRotor;
    func wedge(other: Circle) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
extension AntiCircleRotor: Wedge<CircleRotor> {
    associatedtype Output = CircleRotor;
    func wedge(other: CircleRotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group2.www * other.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
extension AntiCircleRotor: Wedge<Dipole> {
    associatedtype Output = DipoleInversion;
    func wedge(other: Dipole) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group0.xyz.zxy * other.group2.xyz.yzx), this.e23 * other.e15) - float4((other.group0.xyz.zxy * this.group2.yzx), this.e31 * other.e25)
        );
    }
}
extension AntiCircleRotor: Wedge<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func wedge(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.www * other.group2.xyz), (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.scalar * other.e4125), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.xyz.yzx * other.group2.zxy), this.scalar * other.e3215) - (other.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((other.group0.xyz.zxy * this.group2.yzx), this.e31 * other.e25)
        );
    }
}
extension AntiCircleRotor: Wedge<DualNum> {
    associatedtype Output = Motor;
    func wedge(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.group0.xy.xx, other.e5, other.e12345) * float4(this.group0.xyz, this.scalar), 
            /* e235, e315, e125, e5 */
            float4(other.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
extension AntiCircleRotor: Wedge<FlatPoint> {
    associatedtype Output = Flector;
    func wedge(other: FlatPoint) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension AntiCircleRotor: Wedge<Flector> {
    associatedtype Output = Flector;
    func wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.scalar * other.e4125), -(this.e31 * other.e25) - (this.e12 * other.e35)) + float4((this.group0.xyz.yzx * other.group0.zxy), this.scalar * other.e3215) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension AntiCircleRotor: Wedge<Line> {
    associatedtype Output = Motor;
    func wedge(other: Line) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiCircleRotor: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.scalar * other.e415, this.scalar * other.e425, this.scalar * other.e435, -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) + float4((this.group0.xyz * other.group1.www), this.scalar * other.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(other.e5) * this.group1.xyz)), this.scalar * other.e5)
        );
    }
}
extension AntiCircleRotor: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar * other.scalar, (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * other.group3) + (float4(other.scalar) * float4(this.group2.xyz, this.e45)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * other.group4.xyz) + (float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * other.group5.xyz) + (float3(other.scalar) * this.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * other.e4) + (this.scalar * other.e415), (this.e25 * other.e4) + (this.scalar * other.e425), (this.e35 * other.e4) + (this.scalar * other.e435), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e5) * this.group0.xyz), this.scalar * other.e321) - (this.group1.wwwx * other.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.scalar) * other.group7.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group1.zxy) - (this.group0.xyz.zxy * other.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.scalar) * other.group8.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.scalar * other.e4125), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) + float4((this.group0.xyz.yzx * other.group3.zxy), this.scalar * other.e3215) - (this.group2.yzxy * float4(other.group4.xyz.zxy, other.e31)) - float4((this.group0.xyz.zxy * other.group3.yzx), this.e15 * other.e23), 
            /* e1234, 0, 0, 0 */
            float4((this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: Wedge<Plane> {
    associatedtype Output = Plane;
    func wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
extension AntiCircleRotor: Wedge<RoundPoint> {
    associatedtype Output = AntiDipoleInversion;
    func wedge(other: RoundPoint) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group0.yzx) - (this.group2.yzx * other.group0.zxy)), this.scalar * other.e4), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5)
        );
    }
}
extension AntiCircleRotor: Wedge<Scalar> {
    associatedtype Output = AntiCircleRotor;
    func wedge(other: Scalar) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.scalar) * this.group2
        );
    }
}
extension AntiCircleRotor: Wedge<Sphere> {
    associatedtype Output = Sphere;
    func wedge(other: Sphere) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: Wedge<VersorEven> {
    associatedtype Output = VersorEven;
    func wedge(other: VersorEven) -> Output {
        return VersorEven.from_groups(
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
extension AntiCircleRotor: Wedge<VersorOdd> {
    associatedtype Output = VersorOdd;
    func wedge(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
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
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<AntiDipoleInversion> {
        return WedgeInfixPartial { a: this };
    }
}
extension AntiDipoleInversion: Wedge<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiCircleRotor) -> Output {
        return VersorEven.from_groups(
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
extension AntiDipoleInversion: Wedge<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiDipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (other.group3.zxyw * float4(this.group3.yzx, this.e4)) - (this.group3.zxyw * float4(other.group3.yzx, other.e4)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e4 * this.e321) - (other.e2 * this.e431) - (other.e3 * this.e412)) - float4((this.group3.www * other.group3.xyz), other.e1 * this.e423), 
            /* e4235, e4315, e4125, e3215 */
            (other.group3.yzxw * this.group1.zxyw) + (this.group2.wwwz * float4(other.group2.xyz, other.e3)) + float4((this.group0.xyz * other.group3.www), other.e1 * this.e235) + float4((other.group1.yzx * this.group3.zxy), other.e2 * this.e315) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e125 * this.e3)
        );
    }
}
extension AntiDipoleInversion: Wedge<AntiDualNum> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiDualNum) -> Output {
        return VersorEven.from_groups(
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
extension AntiDipoleInversion: Wedge<AntiFlatPoint> {
    associatedtype Output = Sphere;
    func wedge(other: AntiFlatPoint) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group2.www * other.group0.xyz), -(this.e1 * other.e235) - (this.e2 * other.e315) - (this.e3 * other.e125) - (this.e5 * other.e321)), 
            /* e1234, 0, 0, 0 */
            float4(this.e4 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: Wedge<AntiFlector> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiFlector) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group1.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx)), this.e4 * other.e5), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(this.e431 * other.e2) - (this.e412 * other.e3)) + float4((other.group1.www * this.group3.xyz), this.e4 * other.e321) - (other.group1.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group0.xyz, other.e2)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * other.e3) - (this.e2 * other.e315) - (this.e3 * other.e125) - (this.e5 * other.e321)) - float4((this.group1.yzx * other.group1.zxy), this.e1 * other.e235)
        );
    }
}
extension AntiDipoleInversion: Wedge<AntiLine> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiLine) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(this.e1 * other.e23) - (this.e2 * other.e31) - (this.e3 * other.e12)), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)) - float4((other.group1.xyz.yzx * this.group3.zxy), this.e423 * other.e15)
        );
    }
}
extension AntiDipoleInversion: Wedge<AntiMotor> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiMotor) -> Output {
        return VersorEven.from_groups(
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
extension AntiDipoleInversion: Wedge<AntiPlane> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiPlane) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), this.e4 * other.e5), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * other.e5, this.e2 * other.e5, this.e3 * other.e5, -(this.e431 * other.e2) - (this.e412 * other.e3)) - (other.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * other.e3 * -1.0, this.e435 * other.e1 * -1.0, this.e415 * other.e2 * -1.0, (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
extension AntiDipoleInversion: Wedge<Circle> {
    associatedtype Output = Sphere;
    func wedge(other: Circle) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group3.wwwx * float4(other.group0.xyz, other.e235)) - float4((this.group3.yzx * other.group1.zxy), this.e2 * other.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: Wedge<CircleRotor> {
    associatedtype Output = Sphere;
    func wedge(other: CircleRotor) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group3.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group3.www), this.e1 * other.e235), 
            /* e1234, 0, 0, 0 */
            float4((this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: Wedge<Dipole> {
    associatedtype Output = CircleRotor;
    func wedge(other: Dipole) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group3.zxy) - (other.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((other.group2.xyz.yzx * this.group3.zxy), this.e423 * other.e15)
        );
    }
}
extension AntiDipoleInversion: Wedge<DipoleInversion> {
    associatedtype Output = CircleRotor;
    func wedge(other: DipoleInversion) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group3.zxy) - (other.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group3.yzx * other.group2.zxy), this.e4 * other.e3215) - (other.group2.yzxx * float4(this.group3.zxy, this.e423))
        );
    }
}
extension AntiDipoleInversion: Wedge<DualNum> {
    associatedtype Output = Flector;
    func wedge(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension AntiDipoleInversion: Wedge<FlatPoint> {
    associatedtype Output = Motor;
    func wedge(other: FlatPoint) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e4 * other.e15, this.e4 * other.e25, this.e4 * other.e35, -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), 0.0)
        );
    }
}
extension AntiDipoleInversion: Wedge<Flector> {
    associatedtype Output = Motor;
    func wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (float4(this.e4) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), 0.0)
        );
    }
}
extension AntiDipoleInversion: Wedge<Line> {
    associatedtype Output = Plane;
    func wedge(other: Line) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group3.yzxx * float4(other.group0.xyz.zxy, other.e235))
        );
    }
}
extension AntiDipoleInversion: Wedge<Motor> {
    associatedtype Output = Flector;
    func wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e2 * other.e315) - (this.e3 * other.e125)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) - (this.group3.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
extension AntiDipoleInversion: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.group3.xyz, this.e4), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * other.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * float4(this.group3.xyz, this.e4)) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - float4((other.group3.www * this.group3.xyz), this.e1 * other.e23), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz) + (other.group4.xyz.yzx * this.group3.zxy) - (other.group4.xyz.zxy * this.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group3.zxy) - (this.group3.zxy * other.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((other.group8.xyz * this.group2.www), this.e235 * other.e1) + float4((this.group3.zxy * other.group6.yzx), this.e125 * other.e3) - (this.group3.yzxw * other.group6.zxyw) - (this.group3.wwwx * float4(other.group7.xyz, other.e235)) - float4((this.group1.yzx * other.group1.zxy), this.e2 * other.e315) - float4((other.group1.www * this.group2.xyz), this.e3 * other.e125), 
            /* e1234, 0, 0, 0 */
            float4((this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: Wedge<Plane> {
    associatedtype Output = AntiScalar;
    func wedge(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: Wedge<RoundPoint> {
    associatedtype Output = DipoleInversion;
    func wedge(other: RoundPoint) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * other.group0.zxy), this.e4 * other.e5) - (this.group3.zxyw * other.group0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * other.e5, this.e2 * other.e5, this.e3 * other.e5, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
extension AntiDipoleInversion: Wedge<Scalar> {
    associatedtype Output = AntiDipoleInversion;
    func wedge(other: Scalar) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.scalar) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group3
        );
    }
}
extension AntiDipoleInversion: Wedge<Sphere> {
    associatedtype Output = AntiScalar;
    func wedge(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: Wedge<VersorEven> {
    associatedtype Output = DipoleInversion;
    func wedge(other: VersorEven) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * other.group3.zxy), this.e4 * other.e5) - (this.group3.zxyw * other.group3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) + float4((other.group2.www * this.group3.xyz), this.e4 * other.e321) - (other.group3.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4((this.group3.zxy * other.group1.yzx), this.e125 * other.e3) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxz * float4(other.group1.zxy, other.e125)) - float4((this.group1.yzx * other.group3.zxy), this.e1 * other.e235) - float4((other.group3.www * this.group2.xyz), this.e2 * other.e315)
        );
    }
}
extension AntiDipoleInversion: Wedge<VersorOdd> {
    associatedtype Output = VersorEven;
    func wedge(other: VersorOdd) -> Output {
        return VersorEven.from_groups(
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
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<AntiDualNum> {
        return WedgeInfixPartial { a: this };
    }
}
extension AntiDualNum: Wedge<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func wedge(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
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
extension AntiDualNum: Wedge<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiDipoleInversion) -> Output {
        return VersorEven.from_groups(
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
extension AntiDualNum: Wedge<AntiDualNum> {
    associatedtype Output = AntiDualNum;
    func wedge(other: AntiDualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2((other.e3215 * this.scalar) + (other.scalar * this.e3215), other.scalar * this.scalar), 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Wedge<AntiFlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: AntiFlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension AntiDualNum: Wedge<AntiFlector> {
    associatedtype Output = AntiFlector;
    func wedge(other: AntiFlector) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
extension AntiDualNum: Wedge<AntiLine> {
    associatedtype Output = AntiLine;
    func wedge(other: AntiLine) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0)
        );
    }
}
extension AntiDualNum: Wedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * other.scalar) + (this.scalar * other.e3215)) * float4(other.group1.xyz, 1.0)
        );
    }
}
extension AntiDualNum: Wedge<AntiPlane> {
    associatedtype Output = AntiPlane;
    func wedge(other: AntiPlane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
extension AntiDualNum: Wedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Wedge<Circle> {
    associatedtype Output = Circle;
    func wedge(other: Circle) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * other.group2.xyz, 0.0)
        );
    }
}
extension AntiDualNum: Wedge<CircleRotor> {
    associatedtype Output = CircleRotor;
    func wedge(other: CircleRotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * other.group2
        );
    }
}
extension AntiDualNum: Wedge<Dipole> {
    associatedtype Output = Dipole;
    func wedge(other: Dipole) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * other.group2.xyz, 0.0)
        );
    }
}
extension AntiDualNum: Wedge<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func wedge(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group3
        );
    }
}
extension AntiDualNum: Wedge<DualNum> {
    associatedtype Output = DualNum;
    func wedge(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.scalar) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Wedge<FlatPoint> {
    associatedtype Output = FlatPoint;
    func wedge(other: FlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0
        );
    }
}
extension AntiDualNum: Wedge<Flector> {
    associatedtype Output = Flector;
    func wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group1
        );
    }
}
extension AntiDualNum: Wedge<Line> {
    associatedtype Output = Line;
    func wedge(other: Line) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0)
        );
    }
}
extension AntiDualNum: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
extension AntiDualNum: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar * other.scalar, (this.e3215 * other.e4) + (this.scalar * other.e12345)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * other.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xy.yy, this.scalar, (this.e3215 * other.scalar) + (this.scalar * other.e3215)) * float4(other.group9.xyz, 1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Wedge<Plane> {
    associatedtype Output = Plane;
    func wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
extension AntiDualNum: Wedge<RoundPoint> {
    associatedtype Output = VersorEven;
    func wedge(other: RoundPoint) -> Output {
        return VersorEven.from_groups(
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
extension AntiDualNum: Wedge<Scalar> {
    associatedtype Output = AntiDualNum;
    func wedge(other: Scalar) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Wedge<Sphere> {
    associatedtype Output = Sphere;
    func wedge(other: Sphere) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: Wedge<VersorEven> {
    associatedtype Output = VersorEven;
    func wedge(other: VersorEven) -> Output {
        return VersorEven.from_groups(
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
extension AntiDualNum: Wedge<VersorOdd> {
    associatedtype Output = VersorOdd;
    func wedge(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
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
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<AntiFlatPoint> {
        return WedgeInfixPartial { a: this };
    }
}
extension AntiFlatPoint: Wedge<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiCircleRotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.scalar * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((other.group2.www * this.group0.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321))
        );
    }
}
extension AntiFlatPoint: Wedge<AntiDipoleInversion> {
    associatedtype Output = Sphere;
    func wedge(other: AntiDipoleInversion) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.group2.www * this.group0.xyz * float3(-1.0)), (other.e1 * this.e235) + (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321)), 
            /* e1234, 0, 0, 0 */
            float4(other.e4 * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<AntiDualNum> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: AntiDualNum) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiFlatPoint: Wedge<AntiFlector> {
    associatedtype Output = AntiDualNum;
    func wedge(other: AntiFlector) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2((this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<AntiMotor> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: AntiMotor) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiFlatPoint: Wedge<AntiPlane> {
    associatedtype Output = AntiDualNum;
    func wedge(other: AntiPlane) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2((this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<Dipole> {
    associatedtype Output = AntiScalar;
    func wedge(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func wedge(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<DualNum> {
    associatedtype Output = AntiDualNum;
    func wedge(other: DualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e321 * other.e5, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<FlatPoint> {
    associatedtype Output = AntiScalar;
    func wedge(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e321 * other.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<Flector> {
    associatedtype Output = AntiScalar;
    func wedge(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e321 * other.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<Motor> {
    associatedtype Output = AntiDualNum;
    func wedge(other: Motor) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e321 * other.e5, 1.0) * float2(1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)), 0.0, 0.0), 
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
            float4(float3(0.0), this.e321 * other.scalar), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)), 
            /* e1234, 0, 0, 0 */
            float4(this.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<RoundPoint> {
    associatedtype Output = Sphere;
    func wedge(other: RoundPoint) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)), 
            /* e1234, 0, 0, 0 */
            float4(this.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<Scalar> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: Scalar) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiFlatPoint: Wedge<VersorEven> {
    associatedtype Output = Sphere;
    func wedge(other: VersorEven) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)), 
            /* e1234, 0, 0, 0 */
            float4(this.e321 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Wedge<VersorOdd> {
    associatedtype Output = CircleRotor;
    func wedge(other: VersorOdd) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * other.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45))
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<AntiFlector> {
        return WedgeInfixPartial { a: this };
    }
}
extension AntiFlector: Wedge<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiCircleRotor) -> Output {
        return VersorEven.from_groups(
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
extension AntiFlector: Wedge<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiDipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((other.group3.zxy * this.group1.yzx) - (other.group3.yzx * this.group1.zxy)), other.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3)) - float4((this.group1.www * other.group3.xyz), other.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321) - (other.e125 * this.e3)) + float4((other.group1.yzx * this.group1.zxy), other.e1 * this.e235) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group0.xyz, this.e2)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
extension AntiFlector: Wedge<AntiDualNum> {
    associatedtype Output = AntiFlector;
    func wedge(other: AntiDualNum) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
extension AntiFlector: Wedge<AntiFlatPoint> {
    associatedtype Output = AntiDualNum;
    func wedge(other: AntiFlatPoint) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(-(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlector: Wedge<AntiFlector> {
    associatedtype Output = AntiMotor;
    func wedge(other: AntiFlector) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (other.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) - (this.group1.wwwx * float4(other.group1.xyz, other.e235))
        );
    }
}
extension AntiFlector: Wedge<AntiLine> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: AntiLine) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.zxyx * float4(other.group1.xyz.yzx, other.e23))
        );
    }
}
extension AntiFlector: Wedge<AntiMotor> {
    associatedtype Output = AntiFlector;
    func wedge(other: AntiMotor) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group0) - (this.group1.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
extension AntiFlector: Wedge<AntiPlane> {
    associatedtype Output = AntiMotor;
    func wedge(other: AntiPlane) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e5 * other.e1 * -1.0, this.e5 * other.e2 * -1.0, this.e5 * other.e3 * -1.0, (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)) + (other.group0.wwwx * float4(this.group1.xyz, this.e235))
        );
    }
}
extension AntiFlector: Wedge<Circle> {
    associatedtype Output = Sphere;
    func wedge(other: Circle) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group1.wwwx * float4(other.group0.xyz, other.e235)) - float4((this.group1.yzx * other.group1.zxy), this.e2 * other.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: Wedge<CircleRotor> {
    associatedtype Output = Sphere;
    func wedge(other: CircleRotor) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group1.www), this.e1 * other.e235), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: Wedge<Dipole> {
    associatedtype Output = CircleRotor;
    func wedge(other: Dipole) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((other.group0.xyz.yzx * this.group1.zxy) - (other.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - float4((other.group2.xyz.yzx * this.group1.zxy), this.e235 * other.e41)
        );
    }
}
extension AntiFlector: Wedge<DipoleInversion> {
    associatedtype Output = CircleRotor;
    func wedge(other: DipoleInversion) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((other.group0.xyz.yzx * this.group1.zxy) - (other.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (this.group1.yzxx * float4(other.group2.zxy, other.e4235)) + (this.group1.wwwy * float4(other.group1.xyz, other.e4315)) + float4(float3(0.0), (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - float4((this.group1.zxy * other.group2.yzx), this.e235 * other.e41)
        );
    }
}
extension AntiFlector: Wedge<DualNum> {
    associatedtype Output = Flector;
    func wedge(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e321 * other.e5)
        );
    }
}
extension AntiFlector: Wedge<FlatPoint> {
    associatedtype Output = Motor;
    func wedge(other: FlatPoint) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.e45) * float4(this.group1.xyz, this.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: Wedge<Flector> {
    associatedtype Output = Motor;
    func wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45) * this.group1.xyz * float3(-1.0)), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e321 * other.e45)), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
extension AntiFlector: Wedge<Line> {
    associatedtype Output = Plane;
    func wedge(other: Line) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e235))
        );
    }
}
extension AntiFlector: Wedge<Motor> {
    associatedtype Output = Flector;
    func wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group1.zxy * other.group0.yzx), this.e321 * other.e5) - (this.group1.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
extension AntiFlector: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * other.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group1.xyz)), this.e5 * other.e4 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * other.group1.zxy) - (this.group1.zxy * other.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((other.group4.xyz * this.group1.www), this.e321 * other.scalar) - float4((other.group3.www * this.group1.xyz), this.e1 * other.e23), 
            /* e423, e431, e412, 0 */
            float4((other.group4.xyz.yzx * this.group1.zxy) - (other.group4.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz) + (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5) - (this.e5 * other.e321)) + float4((this.group1.zxy * other.group6.yzx), this.e235 * other.e1) - (this.group1.wwwx * float4(other.group7.xyz, other.e235)) - float4((this.group1.yzx * other.group6.zxy), this.e3 * other.e125) - float4((other.group1.www * this.group0.xyz), this.e2 * other.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: Wedge<Plane> {
    associatedtype Output = AntiScalar;
    func wedge(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: Wedge<RoundPoint> {
    associatedtype Output = DipoleInversion;
    func wedge(other: RoundPoint) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group0.xyz)), this.e321 * other.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5))
        );
    }
}
extension AntiFlector: Wedge<Scalar> {
    associatedtype Output = AntiFlector;
    func wedge(other: Scalar) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
extension AntiFlector: Wedge<Sphere> {
    associatedtype Output = AntiScalar;
    func wedge(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: Wedge<VersorEven> {
    associatedtype Output = DipoleInversion;
    func wedge(other: VersorEven) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(other.group2.www, other.e423)) + float4(float3(0.0), (this.e2 * other.e431) + (this.e3 * other.e412)) - (other.group3 * float4(this.group1.www, this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5) - (this.e5 * other.e321)) + float4((this.group1.zxy * other.group1.yzx), this.e235 * other.e1) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - (this.group1.wwwz * float4(other.group0.xyz, other.e125)) - float4((other.group3.www * this.group0.xyz), this.e1 * other.e235)
        );
    }
}
extension AntiFlector: Wedge<VersorOdd> {
    associatedtype Output = VersorEven;
    func wedge(other: VersorOdd) -> Output {
        return VersorEven.from_groups(
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
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<AntiLine> {
        return WedgeInfixPartial { a: this };
    }
}
extension AntiLine: Wedge<AntiCircleRotor> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiCircleRotor) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group1.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e45 * this.e23), (other.e43 * this.e15) + (other.e45 * this.e31), (other.e41 * this.e25) + (other.e45 * this.e12), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group1.xyz.yzx), other.e15 * this.e23)
        );
    }
}
extension AntiLine: Wedge<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiDipoleInversion) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e4) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12)), 
            /* e235, e315, e125, e12345 */
            float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)) - float4((this.group1.xyz.yzx * other.group3.zxy), other.e423 * this.e15)
        );
    }
}
extension AntiLine: Wedge<AntiDualNum> {
    associatedtype Output = AntiLine;
    func wedge(other: AntiDualNum) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: Wedge<AntiFlector> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: AntiFlector) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
extension AntiLine: Wedge<AntiLine> {
    associatedtype Output = AntiDualNum;
    func wedge(other: AntiLine) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(-(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiLine: Wedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.scalar) * this.group1.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12))
        );
    }
}
extension AntiLine: Wedge<AntiPlane> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: AntiPlane) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.zxyx * float4(this.group1.xyz.yzx, this.e23))
        );
    }
}
extension AntiLine: Wedge<Circle> {
    associatedtype Output = AntiScalar;
    func wedge(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: Wedge<CircleRotor> {
    associatedtype Output = AntiScalar;
    func wedge(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: Wedge<Dipole> {
    associatedtype Output = Sphere;
    func wedge(other: Dipole) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.xyz.yzx * other.group0.xyz.zxy), this.e23 * other.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: Wedge<DipoleInversion> {
    associatedtype Output = Sphere;
    func wedge(other: DipoleInversion) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.xyz.yzx * other.group0.xyz.zxy), this.e23 * other.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: Wedge<DualNum> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: DualNum) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: Wedge<FlatPoint> {
    associatedtype Output = Plane;
    func wedge(other: FlatPoint) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * this.group0.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35))
        );
    }
}
extension AntiLine: Wedge<Flector> {
    associatedtype Output = Plane;
    func wedge(other: Flector) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * this.group0.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35))
        );
    }
}
extension AntiLine: Wedge<Line> {
    associatedtype Output = AntiScalar;
    func wedge(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiLine: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)), 
            /* e423, e431, e412, 0 */
            float4(float3(other.e4) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group0.xyz) + (this.group1.xyz.zxy * other.group1.yzx) - (this.group1.xyz.yzx * other.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.xyz.yzx * other.group4.xyz.zxy), this.e23 * other.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: Wedge<RoundPoint> {
    associatedtype Output = Circle;
    func wedge(other: RoundPoint) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e4) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group0.xyz) + (this.group1.xyz.zxy * other.group0.yzx) - (this.group1.xyz.yzx * other.group0.zxy), 0.0)
        );
    }
}
extension AntiLine: Wedge<Scalar> {
    associatedtype Output = AntiLine;
    func wedge(other: Scalar) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine: Wedge<VersorEven> {
    associatedtype Output = CircleRotor;
    func wedge(other: VersorEven) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.e4) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group1.xyz.yzx * other.group3.zxy), this.e23 * other.e415)
        );
    }
}
extension AntiLine: Wedge<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func wedge(other: VersorOdd) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group1.xyz), -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.xyz.yzx * other.group0.zxy), this.e23 * other.e15)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<AntiMotor> {
        return WedgeInfixPartial { a: this };
    }
}
extension AntiMotor: Wedge<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func wedge(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
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
extension AntiMotor: Wedge<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiDipoleInversion) -> Output {
        return VersorEven.from_groups(
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
extension AntiMotor: Wedge<AntiDualNum> {
    associatedtype Output = AntiMotor;
    func wedge(other: AntiDualNum) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.group0.xy.yy, other.scalar, (other.e3215 * this.scalar) + (other.scalar * this.e3215)) * float4(this.group1.xyz, 1.0)
        );
    }
}
extension AntiMotor: Wedge<AntiFlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: AntiFlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension AntiMotor: Wedge<AntiFlector> {
    associatedtype Output = AntiFlector;
    func wedge(other: AntiFlector) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group0) - (other.group1.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
extension AntiMotor: Wedge<AntiLine> {
    associatedtype Output = AntiMotor;
    func wedge(other: AntiLine) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.scalar) * other.group1.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12))
        );
    }
}
extension AntiMotor: Wedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.scalar), 
            /* e15, e25, e35, e3215 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12))
        );
    }
}
extension AntiMotor: Wedge<AntiPlane> {
    associatedtype Output = AntiFlector;
    func wedge(other: AntiPlane) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
extension AntiMotor: Wedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: Wedge<Circle> {
    associatedtype Output = CircleRotor;
    func wedge(other: Circle) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
extension AntiMotor: Wedge<CircleRotor> {
    associatedtype Output = CircleRotor;
    func wedge(other: CircleRotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((this.group0.www * other.group2.xyz), (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
extension AntiMotor: Wedge<Dipole> {
    associatedtype Output = DipoleInversion;
    func wedge(other: Dipole) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((other.group0.xyz.zxy * this.group1.yzx), this.e23 * other.e15)
        );
    }
}
extension AntiMotor: Wedge<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func wedge(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group0.www * other.group2.xyz), (this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.scalar * other.e4125), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((other.group0.xyz.yzx * this.group1.zxy), this.scalar * other.e3215) - float4((other.group0.xyz.zxy * this.group1.yzx), this.e23 * other.e15)
        );
    }
}
extension AntiMotor: Wedge<DualNum> {
    associatedtype Output = Motor;
    func wedge(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), this.scalar * other.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.e5) * this.group0
        );
    }
}
extension AntiMotor: Wedge<FlatPoint> {
    associatedtype Output = Flector;
    func wedge(other: FlatPoint) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * this.group0.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35))
        );
    }
}
extension AntiMotor: Wedge<Flector> {
    associatedtype Output = Flector;
    func wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * other.e4235, this.scalar * other.e4315, this.scalar * other.e4125, -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35)) + (this.group0 * float4(other.group0.www, other.e3215))
        );
    }
}
extension AntiMotor: Wedge<Line> {
    associatedtype Output = Motor;
    func wedge(other: Line) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiMotor: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((this.group0.www * other.group0.xyz), (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(other.e5) * this.group0.xyz)), this.scalar * other.e5)
        );
    }
}
extension AntiMotor: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar * other.scalar, (this.scalar * other.e12345) + (this.e3215 * other.e4) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(this.scalar) * other.group3.xyz) + (float3(other.scalar) * this.group1.xyz)), this.scalar * other.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e15 * other.e4, this.e25 * other.e4, this.e35 * other.e4, -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)) + (float4(this.scalar) * other.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(this.scalar) * other.group7.xyz) + (float3(other.e4) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.scalar) * other.group8.xyz) + (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group1.yzx) - (this.group1.yzx * other.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar * other.e4235, this.scalar * other.e4315, this.scalar * other.e4125, -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + (this.group0 * float4(other.group3.www, other.e3215)) + (this.group1.zxyw * float4(other.group4.xyz.yzx, other.scalar)) - (this.group1.yzxx * float4(other.group4.xyz.zxy, other.e23)), 
            /* e1234, 0, 0, 0 */
            float4((this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: Wedge<Plane> {
    associatedtype Output = Plane;
    func wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
extension AntiMotor: Wedge<RoundPoint> {
    associatedtype Output = VersorEven;
    func wedge(other: RoundPoint) -> Output {
        return VersorEven.from_groups(
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
extension AntiMotor: Wedge<Scalar> {
    associatedtype Output = AntiMotor;
    func wedge(other: Scalar) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
extension AntiMotor: Wedge<Sphere> {
    associatedtype Output = Sphere;
    func wedge(other: Sphere) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: Wedge<VersorEven> {
    associatedtype Output = VersorEven;
    func wedge(other: VersorEven) -> Output {
        return VersorEven.from_groups(
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
extension AntiMotor: Wedge<VersorOdd> {
    associatedtype Output = VersorOdd;
    func wedge(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
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
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<AntiPlane> {
        return WedgeInfixPartial { a: this };
    }
}
extension AntiPlane: Wedge<AntiCircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func wedge(other: AntiCircleRotor) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e41 * this.e5, other.e42 * this.e5, other.e43 * this.e5, -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e5) * other.group1.xyz), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - float4((other.group2.yzx * this.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: Wedge<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiDipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((other.group3.zxy * this.group0.yzx) - (other.group3.yzx * this.group0.zxy)), other.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(other.e1 * this.e5 * -1.0, other.e2 * this.e5 * -1.0, other.e3 * this.e5 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3)) + (this.group0.xyzx * float4(other.group3.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e425 * this.e3, other.e435 * this.e1, other.e415 * this.e2, -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
extension AntiPlane: Wedge<AntiDualNum> {
    associatedtype Output = AntiPlane;
    func wedge(other: AntiDualNum) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: Wedge<AntiFlatPoint> {
    associatedtype Output = AntiDualNum;
    func wedge(other: AntiFlatPoint) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(-(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiPlane: Wedge<AntiFlector> {
    associatedtype Output = AntiMotor;
    func wedge(other: AntiFlector) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(((other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(other.e5 * this.e1, other.e5 * this.e2, other.e5 * this.e3, -(other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) - (this.group0.wwwx * float4(other.group1.xyz, other.e235))
        );
    }
}
extension AntiPlane: Wedge<AntiLine> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: AntiLine) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.zxyx * float4(other.group1.xyz.yzx, other.e23))
        );
    }
}
extension AntiPlane: Wedge<AntiMotor> {
    associatedtype Output = AntiFlector;
    func wedge(other: AntiMotor) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: Wedge<AntiPlane> {
    associatedtype Output = AntiLine;
    func wedge(other: AntiPlane) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy), 0.0), 
            /* e15, e25, e35, 0 */
            float4((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz), 0.0)
        );
    }
}
extension AntiPlane: Wedge<Circle> {
    associatedtype Output = Sphere;
    func wedge(other: Circle) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.wwwx * float4(other.group0.xyz, other.e235)) - float4((this.group0.yzx * other.group1.zxy), this.e2 * other.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: Wedge<CircleRotor> {
    associatedtype Output = Sphere;
    func wedge(other: CircleRotor) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group0.www), this.e1 * other.e235), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: Wedge<Dipole> {
    associatedtype Output = Circle;
    func wedge(other: Dipole) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group1.xyz) + (other.group2.xyz.zxy * this.group0.yzx) - (other.group2.xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension AntiPlane: Wedge<DipoleInversion> {
    associatedtype Output = CircleRotor;
    func wedge(other: DipoleInversion) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(this.e3 * other.e25 * -1.0, this.e1 * other.e35 * -1.0, this.e2 * other.e15 * -1.0, (this.e3 * other.e4125) + (this.e5 * other.e1234)) + (this.group0.yzxx * float4(other.group2.zxy, other.e4235)) + (this.group0.wwwy * float4(other.group1.xyz, other.e4315))
        );
    }
}
extension AntiPlane: Wedge<DualNum> {
    associatedtype Output = FlatPoint;
    func wedge(other: DualNum) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension AntiPlane: Wedge<FlatPoint> {
    associatedtype Output = Line;
    func wedge(other: FlatPoint) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(other.e45) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 0.0)
        );
    }
}
extension AntiPlane: Wedge<Flector> {
    associatedtype Output = Motor;
    func wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45) * this.group0.xyz * float3(-1.0)), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
extension AntiPlane: Wedge<Line> {
    associatedtype Output = Plane;
    func wedge(other: Line) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e235))
        );
    }
}
extension AntiPlane: Wedge<Motor> {
    associatedtype Output = Flector;
    func wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
extension AntiPlane: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * other.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group1.xyz)), this.e5 * other.e4 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - float4((other.group3.www * this.group0.xyz), this.e1 * other.e23), 
            /* e423, e431, e412, 0 */
            float4((other.group4.xyz.yzx * this.group0.zxy) - (other.group4.xyz.zxy * this.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group5.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.wwwx * float4(other.group7.xyz, other.e235)) - float4((this.group0.yzx * other.group6.zxy), this.e2 * other.e315), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: Wedge<Plane> {
    associatedtype Output = AntiScalar;
    func wedge(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: Wedge<RoundPoint> {
    associatedtype Output = Dipole;
    func wedge(other: RoundPoint) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, 0 */
            float4((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz), 0.0)
        );
    }
}
extension AntiPlane: Wedge<Scalar> {
    associatedtype Output = AntiPlane;
    func wedge(other: Scalar) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
extension AntiPlane: Wedge<Sphere> {
    associatedtype Output = AntiScalar;
    func wedge(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: Wedge<VersorEven> {
    associatedtype Output = DipoleInversion;
    func wedge(other: VersorEven) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.e4) * this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * other.e1 * -1.0, this.e5 * other.e2 * -1.0, this.e5 * other.e3 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412)) + (this.group0.xyzx * float4(other.group2.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)) - (this.group0.wwwy * float4(other.group0.xyz, other.e315))
        );
    }
}
extension AntiPlane: Wedge<VersorOdd> {
    associatedtype Output = VersorEven;
    func wedge(other: VersorOdd) -> Output {
        return VersorEven.from_groups(
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
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<AntiScalar> {
        return WedgeInfixPartial { a: this };
    }
}
extension AntiScalar: Wedge<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: Wedge<AntiDualNum> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiDualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: Wedge<AntiMotor> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: Wedge<MultiVector> {
    associatedtype Output = AntiScalar;
    func wedge(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: Wedge<Scalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: Wedge<VersorOdd> {
    associatedtype Output = AntiScalar;
    func wedge(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Circle> {
        return WedgeInfixPartial { a: this };
    }
}
extension Circle: Wedge<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiCircleRotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
extension Circle: Wedge<AntiDipoleInversion> {
    associatedtype Output = Sphere;
    func wedge(other: AntiDipoleInversion) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group3.wwwx * float4(this.group0.xyz, this.e235)) + float4((other.group3.yzx * this.group1.zxy), other.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Wedge<AntiDualNum> {
    associatedtype Output = Circle;
    func wedge(other: AntiDualNum) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: Wedge<AntiFlector> {
    associatedtype Output = Sphere;
    func wedge(other: AntiFlector) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group1.wwwx * float4(this.group0.xyz, this.e235)) + float4((other.group1.yzx * this.group1.zxy), other.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Wedge<AntiLine> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiLine) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Wedge<AntiMotor> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiMotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
extension Circle: Wedge<AntiPlane> {
    associatedtype Output = Sphere;
    func wedge(other: AntiPlane) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.wwwx * float4(this.group0.xyz, this.e235)) + float4((other.group0.yzx * this.group1.zxy), other.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Wedge<Dipole> {
    associatedtype Output = AntiScalar;
    func wedge(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Wedge<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func wedge(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Wedge<DualNum> {
    associatedtype Output = Plane;
    func wedge(other: DualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension Circle: Wedge<FlatPoint> {
    associatedtype Output = AntiScalar;
    func wedge(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Wedge<Flector> {
    associatedtype Output = AntiScalar;
    func wedge(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Wedge<Motor> {
    associatedtype Output = Plane;
    func wedge(other: Motor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension Circle: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 0.0, 0.0), 
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
            float4(other.scalar) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(other.e5) * this.group0.xyz), this.e235 * other.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Wedge<RoundPoint> {
    associatedtype Output = Sphere;
    func wedge(other: RoundPoint) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(other.e5) * this.group0.xyz), this.e235 * other.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Wedge<Scalar> {
    associatedtype Output = Circle;
    func wedge(other: Scalar) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Circle: Wedge<VersorEven> {
    associatedtype Output = Sphere;
    func wedge(other: VersorEven) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group2.www), this.e235 * other.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Wedge<VersorOdd> {
    associatedtype Output = CircleRotor;
    func wedge(other: VersorOdd) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43))
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<CircleRotor> {
        return WedgeInfixPartial { a: this };
    }
}
extension CircleRotor: Wedge<AntiCircleRotor> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiCircleRotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((other.group2.www * this.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
extension CircleRotor: Wedge<AntiDipoleInversion> {
    associatedtype Output = Sphere;
    func wedge(other: AntiDipoleInversion) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group3.www), other.e1 * this.e235), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: Wedge<AntiDualNum> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiDualNum) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.scalar) * this.group2
        );
    }
}
extension CircleRotor: Wedge<AntiFlector> {
    associatedtype Output = Sphere;
    func wedge(other: AntiFlector) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group1.www), other.e1 * this.e235), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: Wedge<AntiLine> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiLine) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: Wedge<AntiMotor> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiMotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((other.group0.www * this.group2.xyz), (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
extension CircleRotor: Wedge<AntiPlane> {
    associatedtype Output = Sphere;
    func wedge(other: AntiPlane) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group0.www), other.e1 * this.e235), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: Wedge<Dipole> {
    associatedtype Output = AntiScalar;
    func wedge(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: Wedge<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func wedge(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: Wedge<DualNum> {
    associatedtype Output = Plane;
    func wedge(other: DualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension CircleRotor: Wedge<FlatPoint> {
    associatedtype Output = AntiScalar;
    func wedge(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: Wedge<Flector> {
    associatedtype Output = AntiScalar;
    func wedge(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: Wedge<Motor> {
    associatedtype Output = Plane;
    func wedge(other: Motor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension CircleRotor: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 0.0, 0.0), 
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
            float4(other.scalar) * this.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: Wedge<RoundPoint> {
    associatedtype Output = Sphere;
    func wedge(other: RoundPoint) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: Wedge<Scalar> {
    associatedtype Output = CircleRotor;
    func wedge(other: Scalar) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.scalar) * this.group2
        );
    }
}
extension CircleRotor: Wedge<VersorEven> {
    associatedtype Output = Sphere;
    func wedge(other: VersorEven) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: Wedge<VersorOdd> {
    associatedtype Output = CircleRotor;
    func wedge(other: VersorOdd) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), (this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43))
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Dipole> {
        return WedgeInfixPartial { a: this };
    }
}
extension Dipole: Wedge<AntiCircleRotor> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiCircleRotor) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group2.xyz.yzx), other.e23 * this.e15) - float4((this.group0.xyz.zxy * other.group2.yzx), other.e31 * this.e25)
        );
    }
}
extension Dipole: Wedge<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiDipoleInversion) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group3.zxy) - (this.group0.xyz.zxy * other.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((this.group2.xyz.yzx * other.group3.zxy), other.e423 * this.e15)
        );
    }
}
extension Dipole: Wedge<AntiDualNum> {
    associatedtype Output = Dipole;
    func wedge(other: AntiDualNum) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(other.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: Wedge<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: Wedge<AntiFlector> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiFlector) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * other.group1.zxy) - (this.group0.xyz.zxy * other.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - float4((this.group2.xyz.yzx * other.group1.zxy), other.e235 * this.e41)
        );
    }
}
extension Dipole: Wedge<AntiLine> {
    associatedtype Output = Sphere;
    func wedge(other: AntiLine) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.xyz.yzx * this.group0.xyz.zxy), other.e23 * this.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: Wedge<AntiMotor> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiMotor) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((this.group0.xyz.zxy * other.group1.yzx), other.e23 * this.e15)
        );
    }
}
extension Dipole: Wedge<AntiPlane> {
    associatedtype Output = Circle;
    func wedge(other: AntiPlane) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group1.xyz) + (this.group2.xyz.zxy * other.group0.yzx) - (this.group2.xyz.yzx * other.group0.zxy), 0.0)
        );
    }
}
extension Dipole: Wedge<Circle> {
    associatedtype Output = AntiScalar;
    func wedge(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: Wedge<CircleRotor> {
    associatedtype Output = AntiScalar;
    func wedge(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: Wedge<Dipole> {
    associatedtype Output = Sphere;
    func wedge(other: Dipole) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group2.xyz.yzx), other.e15 * this.e23) - float4((other.group2.xyz.yzx * this.group0.xyz.zxy), other.e25 * this.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: Wedge<DipoleInversion> {
    associatedtype Output = Sphere;
    func wedge(other: DipoleInversion) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) - float4((this.group0.xyz.zxy * other.group2.yzx), this.e15 * other.e23) - float4((this.group2.xyz.yzx * other.group0.xyz.zxy), this.e25 * other.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: Wedge<DualNum> {
    associatedtype Output = Line;
    func wedge(other: DualNum) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(other.e5) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e5) * this.group1.xyz, 0.0)
        );
    }
}
extension Dipole: Wedge<FlatPoint> {
    associatedtype Output = Plane;
    func wedge(other: FlatPoint) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension Dipole: Wedge<Flector> {
    associatedtype Output = Plane;
    func wedge(other: Flector) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension Dipole: Wedge<Line> {
    associatedtype Output = AntiScalar;
    func wedge(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e5) * this.group0.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Dipole: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group1.zxy) - (this.group0.xyz.zxy * other.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group1.xyz) + (this.group2.xyz.zxy * other.group1.yzx) - (this.group2.xyz.yzx * other.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) - float4((this.group0.xyz.zxy * other.group3.yzx), this.e15 * other.e23) - float4((this.group2.xyz.yzx * other.group4.xyz.zxy), this.e25 * other.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: Wedge<RoundPoint> {
    associatedtype Output = Circle;
    func wedge(other: RoundPoint) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group1.xyz) + (this.group2.xyz.zxy * other.group0.yzx) - (this.group2.xyz.yzx * other.group0.zxy), 0.0)
        );
    }
}
extension Dipole: Wedge<Scalar> {
    associatedtype Output = Dipole;
    func wedge(other: Scalar) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(other.scalar) * this.group2.xyz, 0.0)
        );
    }
}
extension Dipole: Wedge<VersorEven> {
    associatedtype Output = CircleRotor;
    func wedge(other: VersorEven) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group3.zxy) - (this.group0.xyz.zxy * other.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group2.xyz.yzx * other.group3.zxy), this.e41 * other.e235)
        );
    }
}
extension Dipole: Wedge<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func wedge(other: VersorOdd) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) - float4((this.group0.xyz.zxy * other.group2.yzx), this.e15 * other.e23) - float4((this.group2.xyz.yzx * other.group0.zxy), this.e25 * other.e31)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<DipoleInversion> {
        return WedgeInfixPartial { a: this };
    }
}
extension DipoleInversion: Wedge<AntiCircleRotor> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiCircleRotor) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.www * this.group2.xyz), (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.scalar * this.e4125), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.xyz.yzx * this.group2.zxy), other.scalar * this.e3215) - (this.group2.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((this.group0.xyz.zxy * other.group2.yzx), other.e31 * this.e25)
        );
    }
}
extension DipoleInversion: Wedge<AntiDipoleInversion> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiDipoleInversion) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group3.zxy) - (this.group0.xyz.zxy * other.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (other.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group3.yzx * this.group2.zxy), other.e4 * this.e3215) - (this.group2.yzxx * float4(other.group3.zxy, other.e423))
        );
    }
}
extension DipoleInversion: Wedge<AntiDualNum> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiDualNum) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group3
        );
    }
}
extension DipoleInversion: Wedge<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Wedge<AntiFlector> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiFlector) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * other.group1.zxy) - (this.group0.xyz.zxy * other.group1.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (other.group1.yzxx * float4(this.group2.zxy, this.e4235)) + (other.group1.wwwy * float4(this.group1.xyz, this.e4315)) + float4(float3(0.0), (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - float4((other.group1.zxy * this.group2.yzx), other.e235 * this.e41)
        );
    }
}
extension DipoleInversion: Wedge<AntiLine> {
    associatedtype Output = Sphere;
    func wedge(other: AntiLine) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.xyz.yzx * this.group0.xyz.zxy), other.e23 * this.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Wedge<AntiMotor> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiMotor) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group0.www * this.group2.xyz), (other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.scalar * this.e4125), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((this.group0.xyz.yzx * other.group1.zxy), other.scalar * this.e3215) - float4((this.group0.xyz.zxy * other.group1.yzx), other.e23 * this.e15)
        );
    }
}
extension DipoleInversion: Wedge<AntiPlane> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiPlane) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(other.e3 * this.e25 * -1.0, other.e1 * this.e35 * -1.0, other.e2 * this.e15 * -1.0, (other.e3 * this.e4125) + (other.e5 * this.e1234)) + (other.group0.yzxx * float4(this.group2.zxy, this.e4235)) + (other.group0.wwwy * float4(this.group1.xyz, this.e4315))
        );
    }
}
extension DipoleInversion: Wedge<Circle> {
    associatedtype Output = AntiScalar;
    func wedge(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Wedge<CircleRotor> {
    associatedtype Output = AntiScalar;
    func wedge(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Wedge<Dipole> {
    associatedtype Output = Sphere;
    func wedge(other: Dipole) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group2.yzx), other.e15 * this.e23) - float4((other.group2.xyz.yzx * this.group0.xyz.zxy), other.e25 * this.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Wedge<DipoleInversion> {
    associatedtype Output = Sphere;
    func wedge(other: DipoleInversion) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (this.group2.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((this.group0.xyz.zxy * other.group2.yzx), other.e31 * this.e25), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Wedge<DualNum> {
    associatedtype Output = Motor;
    func wedge(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.e5) * float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DipoleInversion: Wedge<FlatPoint> {
    associatedtype Output = Plane;
    func wedge(other: FlatPoint) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension DipoleInversion: Wedge<Flector> {
    associatedtype Output = Plane;
    func wedge(other: Flector) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e23))
        );
    }
}
extension DipoleInversion: Wedge<Line> {
    associatedtype Output = AntiScalar;
    func wedge(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e5) * this.group0.xyz), (this.e1234 * other.e5) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group1.www, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DipoleInversion: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group1.zxy) - (this.group0.xyz.zxy * other.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) + (float4(other.scalar) * this.group3) - (this.group2.yzxy * float4(other.group4.xyz.zxy, other.e31)) - float4((this.group0.xyz.zxy * other.group3.yzx), this.e15 * other.e23), 
            /* e1234, 0, 0, 0 */
            float4((this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: Wedge<RoundPoint> {
    associatedtype Output = CircleRotor;
    func wedge(other: RoundPoint) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group0.zxy) - (this.group0.xyz.zxy * other.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(this.e25 * other.e3 * -1.0, this.e35 * other.e1 * -1.0, this.e15 * other.e2 * -1.0, (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) + (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group0.yzxx * float4(this.group2.zxy, this.e4235))
        );
    }
}
extension DipoleInversion: Wedge<Scalar> {
    associatedtype Output = DipoleInversion;
    func wedge(other: Scalar) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group3
        );
    }
}
extension DipoleInversion: Wedge<VersorEven> {
    associatedtype Output = CircleRotor;
    func wedge(other: VersorEven) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group1.xyz) + (this.group0.xyz.yzx * other.group3.zxy) - (this.group0.xyz.zxy * other.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group2.yzx * other.group3.zxy), this.e41 * other.e235)
        );
    }
}
extension DipoleInversion: Wedge<VersorOdd> {
    associatedtype Output = DipoleInversion;
    func wedge(other: VersorOdd) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.e4235 * other.scalar), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.e4315 * other.scalar), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.e4125 * other.scalar), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.xyz.yzx * other.group2.zxy), this.e3215 * other.scalar) - (other.group2.yzxx * float4(this.group0.xyz.zxy, this.e23)) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e25)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<DualNum> {
        return WedgeInfixPartial { a: this };
    }
}
extension DualNum: Wedge<AntiCircleRotor> {
    associatedtype Output = Motor;
    func wedge(other: AntiCircleRotor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, this.e12345) * float4(other.group0.xyz, other.scalar), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(other.group1.xyz, other.scalar)
        );
    }
}
extension DualNum: Wedge<AntiDipoleInversion> {
    associatedtype Output = Flector;
    func wedge(other: AntiDipoleInversion) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group3.xyz, other.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
extension DualNum: Wedge<AntiDualNum> {
    associatedtype Output = DualNum;
    func wedge(other: AntiDualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: Wedge<AntiFlatPoint> {
    associatedtype Output = AntiDualNum;
    func wedge(other: AntiFlatPoint) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.e321 * this.e5, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension DualNum: Wedge<AntiFlector> {
    associatedtype Output = Flector;
    func wedge(other: AntiFlector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e321 * this.e5 * -1.0)
        );
    }
}
extension DualNum: Wedge<AntiLine> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: AntiLine) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DualNum: Wedge<AntiMotor> {
    associatedtype Output = Motor;
    func wedge(other: AntiMotor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), other.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * other.group0
        );
    }
}
extension DualNum: Wedge<AntiPlane> {
    associatedtype Output = FlatPoint;
    func wedge(other: AntiPlane) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension DualNum: Wedge<Circle> {
    associatedtype Output = Plane;
    func wedge(other: Circle) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
extension DualNum: Wedge<CircleRotor> {
    associatedtype Output = Plane;
    func wedge(other: CircleRotor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
extension DualNum: Wedge<Dipole> {
    associatedtype Output = Line;
    func wedge(other: Dipole) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e5) * other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e5) * other.group1.xyz, 0.0)
        );
    }
}
extension DualNum: Wedge<DipoleInversion> {
    associatedtype Output = Motor;
    func wedge(other: DipoleInversion) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e5) * float4(other.group0.xyz, other.e1234), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension DualNum: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e5 * other.e1234) + (this.e12345 * other.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * other.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group0.xy.xx, this.e5, 0.0) * float4(float3(1.0), 0.0) * float4(other.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.e5) * other.group5.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group7.xyz, other.e321) * float4(-1.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension DualNum: Wedge<RoundPoint> {
    associatedtype Output = FlatPoint;
    func wedge(other: RoundPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group0 * float4(-1.0)
        );
    }
}
extension DualNum: Wedge<Scalar> {
    associatedtype Output = DualNum;
    func wedge(other: Scalar) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension DualNum: Wedge<Sphere> {
    associatedtype Output = AntiScalar;
    func wedge(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e5 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: Wedge<VersorEven> {
    associatedtype Output = Flector;
    func wedge(other: VersorEven) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
extension DualNum: Wedge<VersorOdd> {
    associatedtype Output = Motor;
    func wedge(other: VersorOdd) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xy.xx, this.e5, (this.e5 * other.e1234) + (this.e12345 * other.scalar)) * float4(other.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(other.group1.xyz, other.scalar)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<FlatPoint> {
        return WedgeInfixPartial { a: this };
    }
}
extension FlatPoint: Wedge<AntiCircleRotor> {
    associatedtype Output = Flector;
    func wedge(other: AntiCircleRotor) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
extension FlatPoint: Wedge<AntiDipoleInversion> {
    associatedtype Output = Motor;
    func wedge(other: AntiDipoleInversion) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.e4 * this.e15, other.e4 * this.e25, other.e4 * this.e35, -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423)), 
            /* e235, e315, e125, e5 */
            float4(((other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension FlatPoint: Wedge<AntiDualNum> {
    associatedtype Output = FlatPoint;
    func wedge(other: AntiDualNum) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
extension FlatPoint: Wedge<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e321 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: Wedge<AntiFlector> {
    associatedtype Output = Motor;
    func wedge(other: AntiFlector) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e45) * float4(other.group1.xyz, other.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension FlatPoint: Wedge<AntiLine> {
    associatedtype Output = Plane;
    func wedge(other: AntiLine) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * other.group0.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35))
        );
    }
}
extension FlatPoint: Wedge<AntiMotor> {
    associatedtype Output = Flector;
    func wedge(other: AntiMotor) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * other.group0.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35))
        );
    }
}
extension FlatPoint: Wedge<AntiPlane> {
    associatedtype Output = Line;
    func wedge(other: AntiPlane) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.e45) * other.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension FlatPoint: Wedge<Circle> {
    associatedtype Output = AntiScalar;
    func wedge(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: Wedge<CircleRotor> {
    associatedtype Output = AntiScalar;
    func wedge(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: Wedge<Dipole> {
    associatedtype Output = Plane;
    func wedge(other: Dipole) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
extension FlatPoint: Wedge<DipoleInversion> {
    associatedtype Output = Plane;
    func wedge(other: DipoleInversion) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
extension FlatPoint: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e42) + (this.e45 * other.e23), (this.e15 * other.e43) + (this.e45 * other.e31), (this.e25 * other.e41) + (this.e45 * other.e12), -(this.e25 * other.e31) - (this.e35 * other.e12)) - (this.group0.yzxx * float4(other.group4.xyz.zxy, other.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension FlatPoint: Wedge<RoundPoint> {
    associatedtype Output = Line;
    func wedge(other: RoundPoint) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4((float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy), 0.0)
        );
    }
}
extension FlatPoint: Wedge<Scalar> {
    associatedtype Output = FlatPoint;
    func wedge(other: Scalar) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
extension FlatPoint: Wedge<VersorEven> {
    associatedtype Output = Motor;
    func wedge(other: VersorEven) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e15 * other.e4, this.e25 * other.e4, this.e35 * other.e4, -(this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy)), 0.0)
        );
    }
}
extension FlatPoint: Wedge<VersorOdd> {
    associatedtype Output = Flector;
    func wedge(other: VersorOdd) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e42) + (this.e45 * other.e23), (this.e15 * other.e43) + (this.e45 * other.e31), (this.e25 * other.e41) + (this.e45 * other.e12), -(this.e25 * other.e31) - (this.e35 * other.e12)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Flector> {
        return WedgeInfixPartial { a: this };
    }
}
extension Flector: Wedge<AntiCircleRotor> {
    associatedtype Output = Flector;
    func wedge(other: AntiCircleRotor) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.scalar * this.e4125), -(other.e31 * this.e25) - (other.e12 * this.e35)) + float4((other.group0.xyz.yzx * this.group0.zxy), other.scalar * this.e3215) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
extension Flector: Wedge<AntiDipoleInversion> {
    associatedtype Output = Motor;
    func wedge(other: AntiDipoleInversion) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (float4(other.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423)), 
            /* e235, e315, e125, e5 */
            float4(((other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Flector: Wedge<AntiDualNum> {
    associatedtype Output = Flector;
    func wedge(other: AntiDualNum) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
extension Flector: Wedge<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e321 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: Wedge<AntiFlector> {
    associatedtype Output = Motor;
    func wedge(other: AntiFlector) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * other.group1.xyz * float3(-1.0)), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e321 * this.e45)), 
            /* e235, e315, e125, e5 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Flector: Wedge<AntiLine> {
    associatedtype Output = Plane;
    func wedge(other: AntiLine) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * other.group0.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35))
        );
    }
}
extension Flector: Wedge<AntiMotor> {
    associatedtype Output = Flector;
    func wedge(other: AntiMotor) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * this.e4235, other.scalar * this.e4315, other.scalar * this.e4125, -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35)) + (other.group0 * float4(this.group0.www, this.e3215))
        );
    }
}
extension Flector: Wedge<AntiPlane> {
    associatedtype Output = Motor;
    func wedge(other: AntiPlane) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45) * other.group0.xyz * float3(-1.0)), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
extension Flector: Wedge<Circle> {
    associatedtype Output = AntiScalar;
    func wedge(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: Wedge<CircleRotor> {
    associatedtype Output = AntiScalar;
    func wedge(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: Wedge<Dipole> {
    associatedtype Output = Plane;
    func wedge(other: Dipole) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
extension Flector: Wedge<DipoleInversion> {
    associatedtype Output = Plane;
    func wedge(other: DipoleInversion) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e23))
        );
    }
}
extension Flector: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e42) + (this.e45 * other.e23), (this.e15 * other.e43) + (this.e45 * other.e31), (this.e25 * other.e41) + (this.e45 * other.e12), -(this.e25 * other.e31) - (this.e35 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group4.xyz.zxy, other.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Flector: Wedge<RoundPoint> {
    associatedtype Output = Motor;
    func wedge(other: RoundPoint) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e45 * other.e1 * -1.0, this.e45 * other.e2 * -1.0, this.e45 * other.e3 * -1.0, (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) + (other.group0.wwwx * float4(this.group0.xyz, this.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
extension Flector: Wedge<Scalar> {
    associatedtype Output = Flector;
    func wedge(other: Scalar) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
extension Flector: Wedge<VersorEven> {
    associatedtype Output = Motor;
    func wedge(other: VersorEven) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (other.group3.wwwx * float4(this.group0.xyz, this.e4235)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy)), 0.0)
        );
    }
}
extension Flector: Wedge<VersorOdd> {
    associatedtype Output = Flector;
    func wedge(other: VersorOdd) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e45 * other.e23) + (this.e4235 * other.scalar), (this.e45 * other.e31) + (this.e4315 * other.scalar), (this.e45 * other.e12) + (this.e4125 * other.scalar), -(this.e25 * other.e31) - (this.e35 * other.e12)) + (other.group0.yzxw * float4(this.group0.zxy, this.e3215)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Line> {
        return WedgeInfixPartial { a: this };
    }
}
extension Line: Wedge<AntiCircleRotor> {
    associatedtype Output = Motor;
    func wedge(other: AntiCircleRotor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group2.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: Wedge<AntiDipoleInversion> {
    associatedtype Output = Plane;
    func wedge(other: AntiDipoleInversion) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: Wedge<AntiDualNum> {
    associatedtype Output = Line;
    func wedge(other: AntiDualNum) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: Wedge<AntiFlector> {
    associatedtype Output = Plane;
    func wedge(other: AntiFlector) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: Wedge<AntiLine> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiLine) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: Wedge<AntiMotor> {
    associatedtype Output = Motor;
    func wedge(other: AntiMotor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: Wedge<AntiPlane> {
    associatedtype Output = Plane;
    func wedge(other: AntiPlane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: Wedge<Dipole> {
    associatedtype Output = AntiScalar;
    func wedge(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: Wedge<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func wedge(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 0.0, 0.0), 
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
            float4(other.group0.xy.xx, other.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group1.yzxx * float4(this.group0.xyz.zxy, this.e235)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Line: Wedge<RoundPoint> {
    associatedtype Output = Plane;
    func wedge(other: RoundPoint) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group0.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: Wedge<Scalar> {
    associatedtype Output = Line;
    func wedge(other: Scalar) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(other.scalar) * this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * this.group1.xyz, 0.0)
        );
    }
}
extension Line: Wedge<VersorEven> {
    associatedtype Output = Plane;
    func wedge(other: VersorEven) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group3.yzxx * float4(this.group0.xyz.zxy, this.e235))
        );
    }
}
extension Line: Wedge<VersorOdd> {
    associatedtype Output = Motor;
    func wedge(other: VersorOdd) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.xyz, 0.0) * float4(other.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Motor> {
        return WedgeInfixPartial { a: this };
    }
}
extension Motor: Wedge<AntiCircleRotor> {
    associatedtype Output = Motor;
    func wedge(other: AntiCircleRotor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.scalar * this.e415, other.scalar * this.e425, other.scalar * this.e435, -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) + float4((other.group0.xyz * this.group1.www), other.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.e5) * other.group1.xyz)), other.scalar * this.e5)
        );
    }
}
extension Motor: Wedge<AntiDipoleInversion> {
    associatedtype Output = Flector;
    func wedge(other: AntiDipoleInversion) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group3.xyz, other.e4) * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235)) - (float4(this.e5) * float4(other.group0.xyz, other.e321))
        );
    }
}
extension Motor: Wedge<AntiDualNum> {
    associatedtype Output = Motor;
    func wedge(other: AntiDualNum) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
extension Motor: Wedge<AntiFlatPoint> {
    associatedtype Output = AntiDualNum;
    func wedge(other: AntiFlatPoint) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(other.e321 * this.e5, 1.0) * float2(-1.0, 0.0), 0.0, 0.0)
        );
    }
}
extension Motor: Wedge<AntiFlector> {
    associatedtype Output = Flector;
    func wedge(other: AntiFlector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((other.group1.zxy * this.group0.yzx), other.e321 * this.e5)
        );
    }
}
extension Motor: Wedge<AntiLine> {
    associatedtype Output = Motor;
    func wedge(other: AntiLine) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: Wedge<AntiMotor> {
    associatedtype Output = Motor;
    func wedge(other: AntiMotor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((other.group0.www * this.group0.xyz), (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.e5) * other.group0.xyz)), other.scalar * this.e5)
        );
    }
}
extension Motor: Wedge<AntiPlane> {
    associatedtype Output = Flector;
    func wedge(other: AntiPlane) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
extension Motor: Wedge<Circle> {
    associatedtype Output = Plane;
    func wedge(other: Circle) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
extension Motor: Wedge<CircleRotor> {
    associatedtype Output = Plane;
    func wedge(other: CircleRotor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
extension Motor: Wedge<Dipole> {
    associatedtype Output = Motor;
    func wedge(other: Dipole) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * other.group0.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: Wedge<DipoleInversion> {
    associatedtype Output = Motor;
    func wedge(other: DipoleInversion) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * other.group0.xyz), (other.e1234 * this.e5) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(this.group1.www, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e12345 * other.scalar) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5 * other.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * other.group4.xyz) + (float3(other.scalar) * this.group0.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group1.yzxx * float4(this.group0.zxy, this.e235)) - float4((other.group7.xyz * this.group1.www), this.e5 * other.e321), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Motor: Wedge<RoundPoint> {
    associatedtype Output = Flector;
    func wedge(other: RoundPoint) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
extension Motor: Wedge<Scalar> {
    associatedtype Output = Motor;
    func wedge(other: Scalar) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
extension Motor: Wedge<Sphere> {
    associatedtype Output = AntiScalar;
    func wedge(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e5 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: Wedge<VersorEven> {
    associatedtype Output = Flector;
    func wedge(other: VersorEven) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e5) * other.group3 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e235 * other.e4) - (this.e5 * other.e423), -(this.e315 * other.e4) - (this.e5 * other.e431), -(this.e125 * other.e4) - (this.e5 * other.e412), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235)) - float4((this.group0.yzx * other.group3.zxy), this.e5 * other.e321)
        );
    }
}
extension Motor: Wedge<VersorOdd> {
    associatedtype Output = Motor;
    func wedge(other: VersorOdd) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (float4(this.e5) * float4(other.group0.xyz, other.e1234)) + (float4(other.scalar) * this.group0) + float4(float3(0.0), -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group1.xyz)), this.e5 * other.scalar)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<MultiVector> {
        return WedgeInfixPartial { a: this };
    }
}
extension MultiVector: Wedge<AntiCircleRotor> {
    associatedtype Output = MultiVector;
    func wedge(other: AntiCircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * this.scalar, (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(other.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(other.scalar) * this.group3) + (float4(this.scalar) * float4(other.group2.xyz, other.e45)), 
            /* e41, e42, e43, 0 */
            float4((float3(other.scalar) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e15 * this.e4) + (other.scalar * this.e415), (other.e25 * this.e4) + (other.scalar * this.e425), (other.e35 * this.e4) + (other.scalar * this.e435), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e5) * other.group0.xyz), other.scalar * this.e321) - (other.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(other.scalar) * this.group7.xyz) + (float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group1.zxy) - (other.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.scalar) * this.group8.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.scalar * this.e4125), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) + float4((other.group0.xyz.yzx * this.group3.zxy), other.scalar * this.e3215) - (other.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((other.group0.xyz.zxy * this.group3.yzx), other.e15 * this.e23), 
            /* e1234, 0, 0, 0 */
            float4((other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<AntiDipoleInversion> {
    associatedtype Output = MultiVector;
    func wedge(other: AntiDipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e4), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group1) - (float4(this.e5) * float4(other.group3.xyz, other.e4)), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((other.group3.zxy * this.group1.yzx) - (other.group3.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - float4((this.group3.www * other.group3.xyz), other.e1 * this.e23), 
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + (this.group4.xyz.yzx * other.group3.zxy) - (this.group4.xyz.zxy * other.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group3.zxy) - (other.group3.zxy * this.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group3.yzxw * this.group6.zxyw) + (other.group3.wwwx * float4(this.group7.xyz, this.e235)) + float4((other.group1.yzx * this.group1.zxy), other.e2 * this.e315) + float4((this.group1.www * other.group2.xyz), other.e3 * this.e125) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((this.group8.xyz * other.group2.www), other.e235 * this.e1) - float4((other.group3.zxy * this.group6.yzx), other.e125 * this.e3), 
            /* e1234, 0, 0, 0 */
            float4((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<AntiDualNum> {
    associatedtype Output = MultiVector;
    func wedge(other: AntiDualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * this.scalar, (other.e3215 * this.e4) + (other.scalar * this.e12345)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(other.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.yy, other.scalar, (other.e3215 * this.scalar) + (other.scalar * this.e3215)) * float4(this.group9.xyz, 1.0), 
            /* e1234, 0, 0, 0 */
            float4(other.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<AntiFlatPoint> {
    associatedtype Output = MultiVector;
    func wedge(other: AntiFlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)), 0.0, 0.0), 
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
            float4(float3(0.0), other.e321 * this.scalar), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)), 
            /* e1234, 0, 0, 0 */
            float4(other.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<AntiFlector> {
    associatedtype Output = MultiVector;
    func wedge(other: AntiFlector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group1.xyz)), other.e5 * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group1.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((this.group4.xyz * other.group1.www), other.e321 * this.scalar) - float4((this.group3.www * other.group1.xyz), other.e1 * this.e23), 
            /* e423, e431, e412, 0 */
            float4((this.group4.xyz.yzx * other.group1.zxy) - (this.group4.xyz.zxy * other.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + (other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.wwwx * float4(this.group7.xyz, this.e235)) + float4(float3(0.0), (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) + float4((other.group1.yzx * this.group6.zxy), other.e3 * this.e125) + float4((this.group1.www * other.group0.xyz), other.e2 * this.e315) - float4((other.group1.zxy * this.group6.yzx), other.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<AntiLine> {
    associatedtype Output = MultiVector;
    func wedge(other: AntiLine) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)), 
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group0.xyz) + (other.group1.xyz.zxy * this.group1.yzx) - (other.group1.xyz.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.xyz.yzx * this.group4.xyz.zxy), other.e23 * this.e15), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<AntiMotor> {
    associatedtype Output = MultiVector;
    func wedge(other: AntiMotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * this.scalar, (other.scalar * this.e12345) + (other.e3215 * this.e4) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(other.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e15 * this.e4, other.e25 * this.e4, other.e35 * this.e4, -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group6), 
            /* e423, e431, e412, 0 */
            float4((float3(other.scalar) * this.group7.xyz) + (float3(this.e4) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.scalar) * this.group8.xyz) + (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * this.e4235, other.scalar * this.e4315, other.scalar * this.e4125, -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (other.group0 * float4(this.group3.www, this.e3215)) + (other.group1.zxyw * float4(this.group4.xyz.yzx, this.scalar)) - (other.group1.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234, 0, 0, 0 */
            float4((other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<AntiPlane> {
    associatedtype Output = MultiVector;
    func wedge(other: AntiPlane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group0.xyz)), other.e5 * this.e4), 
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4((other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - float4((this.group3.www * other.group0.xyz), other.e1 * this.e23), 
            /* e423, e431, e412, 0 */
            float4((this.group4.xyz.yzx * other.group0.zxy) - (this.group4.xyz.zxy * other.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group5.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.wwwx * float4(this.group7.xyz, this.e235)) + float4((other.group0.yzx * this.group6.zxy), other.e2 * this.e315), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<Circle> {
    associatedtype Output = MultiVector;
    func wedge(other: Circle) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 0.0, 0.0), 
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
            float4(this.scalar) * other.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((float3(this.e5) * other.group0.xyz), other.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<CircleRotor> {
    associatedtype Output = MultiVector;
    func wedge(other: CircleRotor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 0.0, 0.0), 
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
            float4(this.scalar) * other.group1, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234, 0, 0, 0 */
            float4((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<Dipole> {
    associatedtype Output = MultiVector;
    func wedge(other: Dipole) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group1.zxy) - (other.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group1.xyz) + (other.group2.xyz.zxy * this.group1.yzx) - (other.group2.xyz.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group3.yzx), other.e15 * this.e23) - float4((other.group2.xyz.yzx * this.group4.xyz.zxy), other.e25 * this.e31), 
            /* e1234, 0, 0, 0 */
            float4(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<DipoleInversion> {
    associatedtype Output = MultiVector;
    func wedge(other: DipoleInversion) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group1.zxy) - (other.group0.xyz.zxy * this.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) + (float4(this.scalar) * other.group3) - (other.group2.yzxy * float4(this.group4.xyz.zxy, this.e31)) - float4((other.group0.xyz.zxy * this.group3.yzx), other.e15 * this.e23), 
            /* e1234, 0, 0, 0 */
            float4((other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<DualNum> {
    associatedtype Output = MultiVector;
    func wedge(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.e5 * this.e1234) + (other.e12345 * this.scalar)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.e5) * this.group5.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group7.xyz, this.e321), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: Wedge<FlatPoint> {
    associatedtype Output = MultiVector;
    func wedge(other: FlatPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e35 * this.e42) + (other.e45 * this.e23), (other.e15 * this.e43) + (other.e45 * this.e31), (other.e25 * this.e41) + (other.e45 * this.e12), -(other.e25 * this.e31) - (other.e35 * this.e12)) - (other.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: Wedge<Flector> {
    associatedtype Output = MultiVector;
    func wedge(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e35 * this.e42) + (other.e45 * this.e23), (other.e15 * this.e43) + (other.e45 * this.e31), (other.e25 * this.e41) + (other.e45 * this.e12), -(other.e25 * this.e31) - (other.e35 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group0.yzxx * float4(this.group4.xyz.zxy, this.e23)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: Wedge<Line> {
    associatedtype Output = MultiVector;
    func wedge(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 0.0, 0.0), 
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
            float4(this.group0.xy.xx, this.scalar, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group1.yzxx * float4(other.group0.xyz.zxy, other.e235)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: Wedge<Motor> {
    associatedtype Output = MultiVector;
    func wedge(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.e12345 * this.scalar) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(other.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e5) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz)), 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((this.group7.xyz * other.group1.www), other.e5 * this.e321) - (this.group1.yzxx * float4(other.group0.zxy, other.e235)), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * this.scalar, (other.scalar * this.e12345) + (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e5, 0, 0, 0 */
            float4((other.scalar * this.e5) + (other.e5 * this.scalar), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(other.scalar) * this.group3) + (float4(other.e5) * this.group1) + (float4(this.scalar) * other.group3) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(other.scalar) * this.group4.xyz) + (float3(this.scalar) * other.group4.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group5.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e4 * this.e15) + (other.e5 * this.e41) + (other.e15 * this.e4) + (other.e41 * this.e5), (other.e4 * this.e25) + (other.e5 * this.e42) + (other.e25 * this.e4) + (other.e42 * this.e5), (other.e4 * this.e35) + (other.e5 * this.e43) + (other.e35 * this.e4) + (other.e43 * this.e5), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group6) + (float4(this.scalar) * other.group6) - (this.group1.xyzy * float4(other.group3.www, other.e31)) - float4((this.group3.www * other.group1.xyz), other.e23 * this.e1), 
            /* e423, e431, e412, 0 */
            float4((float3(other.scalar) * this.group7.xyz) + (float3(other.e4) * this.group5.xyz) + (float3(this.scalar) * other.group7.xyz) + (float3(this.e4) * other.group5.xyz) + (other.group4.xyz.yzx * this.group1.zxy) + (this.group4.xyz.yzx * other.group1.zxy) - (other.group4.xyz.zxy * this.group1.yzx) - (this.group4.xyz.zxy * other.group1.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.scalar) * this.group8.xyz) + (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group8.xyz) + (float3(this.e5) * other.group5.xyz) + (other.group1.yzx * this.group3.zxy) + (other.group3.zxy * this.group1.yzx) - (other.group1.zxy * this.group3.yzx) - (other.group3.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e2 * this.e435) + (other.e5 * this.e423) + (other.e45 * this.e23) + (other.e425 * this.e3), (other.e3 * this.e415) + (other.e5 * this.e431) + (other.e45 * this.e31) + (other.e435 * this.e1), (other.e1 * this.e425) + (other.e5 * this.e412) + (other.e45 * this.e12) + (other.e415 * this.e2), -(other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12) - (other.e321 * this.e5)) + (float4(other.scalar) * this.group9) + (float4(this.scalar) * other.group9) + float4((other.group5.xyz * this.group3.www), other.e2 * this.e315) + float4((other.group8.xyz * this.group1.www), other.e3 * this.e125) + float4((other.group4.xyz.yzx * this.group3.zxy), other.e1 * this.e235) + float4((this.group4.xyz.yzx * other.group3.zxy), other.e5 * this.e321) - (this.group1.yzxz * float4(other.group6.zxy, other.e125)) - (this.group3.yzxx * float4(other.group4.xyz.zxy, other.e23)) - float4((float3(this.e5) * other.group7.xyz), other.e31 * this.e25) - float4((this.group8.xyz * other.group1.www), other.e235 * this.e1) - float4((this.group4.xyz.zxy * other.group3.yzx), other.e12 * this.e35) - float4((other.group1.zxy * this.group6.yzx), other.e315 * this.e2), 
            /* e1234, 0, 0, 0 */
            float4((other.scalar * this.e1234) + (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e1234 * this.scalar) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<Plane> {
    associatedtype Output = MultiVector;
    func wedge(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215)), 0.0, 0.0), 
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
            float4(this.scalar) * other.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector: Wedge<RoundPoint> {
    associatedtype Output = MultiVector;
    func wedge(other: RoundPoint) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group1) - (float4(this.e5) * other.group0), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * other.e4) + (this.e41 * other.e5), (this.e25 * other.e4) + (this.e42 * other.e5), (this.e35 * other.e4) + (this.e43 * other.e5), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group5.xyz) + (this.group4.xyz.yzx * other.group0.zxy) - (this.group4.xyz.zxy * other.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group5.xyz) + (this.group3.zxy * other.group0.yzx) - (this.group3.yzx * other.group0.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group0.yzxy * float4(this.group6.zxy, this.e315)) + float4((float3(other.e5) * this.group7.xyz), this.e235 * other.e1), 
            /* e1234, 0, 0, 0 */
            float4(-(this.e321 * other.e4) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<Scalar> {
    associatedtype Output = MultiVector;
    func wedge(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * other.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(other.scalar) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(other.scalar) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(other.scalar) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(other.scalar) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<Sphere> {
    associatedtype Output = MultiVector;
    func wedge(other: Sphere) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)), 0.0, 0.0), 
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
            float4(this.scalar) * other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<VersorEven> {
    associatedtype Output = MultiVector;
    func wedge(other: VersorEven) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (this.scalar * other.e12345) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3, 
            /* e5, 0, 0, 0 */
            float4(this.scalar * other.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group1) - (float4(this.e5) * other.group3), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group1.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e15 * other.e4) + (this.e41 * other.e5), (this.e25 * other.e4) + (this.e42 * other.e5), (this.e35 * other.e4) + (this.e43 * other.e5), -(this.e31 * other.e2) - (this.e12 * other.e3)) + (float4(this.scalar) * other.group1) - (other.group3.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.scalar) * other.group0.xyz) + (float3(other.e4) * this.group5.xyz) + (this.group4.xyz.yzx * other.group3.zxy) - (this.group4.xyz.zxy * other.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group5.xyz) + (this.group3.zxy * other.group3.yzx) - (this.group3.yzx * other.group3.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (this.group6.zxyw * float4(other.group3.yzx, other.e5)) + float4((this.group7.xyz * other.group2.www), this.e235 * other.e1) + float4((this.group1.zxy * other.group1.yzx), this.e315 * other.e2) + float4((this.group1.www * other.group2.xyz), this.e125 * other.e3) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((this.group8.xyz * other.group3.www), this.e1 * other.e235) - float4((this.group6.yzx * other.group3.zxy), this.e3 * other.e125), 
            /* e1234, 0, 0, 0 */
            float4((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e321 * other.e4) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<VersorOdd> {
    associatedtype Output = MultiVector;
    func wedge(other: VersorOdd) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar * other.scalar, (this.e12345 * other.scalar) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * other.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * float4(other.group2.xyz, other.e45)) + (float4(other.scalar) * this.group3), 
            /* e41, e42, e43, 0 */
            float4((float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group4.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(this.scalar) * other.group1.xyz) + (float3(other.scalar) * this.group5.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((this.e5 * other.e41) + (this.e415 * other.scalar), (this.e5 * other.e42) + (this.e425 * other.scalar), (this.e5 * other.e43) + (this.e435 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((this.group1.www * other.group2.xyz), this.e321 * other.scalar) - (this.group1.xyzx * other.group1.wwwx), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (float3(other.scalar) * this.group7.xyz) + (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group8.xyz) + (this.group1.yzx * other.group2.zxy) - (this.group1.zxy * other.group2.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e42) + (this.e45 * other.e23) + (this.e23 * other.e45) + (this.e4235 * other.scalar), (this.e15 * other.e43) + (this.e45 * other.e31) + (this.e31 * other.e45) + (this.e4315 * other.scalar), (this.e25 * other.e41) + (this.e45 * other.e12) + (this.e12 * other.e45) + (this.e4125 * other.scalar), -(this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12) - (this.e12 * other.e35)) + (float4(this.scalar) * other.group3) + float4((this.group4.xyz.yzx * other.group2.zxy), this.e3215 * other.scalar) - (other.group2.yzxx * float4(this.group4.xyz.zxy, this.e23)) - float4((this.group3.yzx * other.group0.zxy), this.e31 * other.e25), 
            /* e1234, 0, 0, 0 */
            float4((this.scalar * other.e1234) + (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Plane> {
        return WedgeInfixPartial { a: this };
    }
}
extension Plane: Wedge<AntiCircleRotor> {
    associatedtype Output = Plane;
    func wedge(other: AntiCircleRotor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: Wedge<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: Wedge<AntiDualNum> {
    associatedtype Output = Plane;
    func wedge(other: AntiDualNum) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: Wedge<AntiFlector> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: Wedge<AntiMotor> {
    associatedtype Output = Plane;
    func wedge(other: AntiMotor) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: Wedge<AntiPlane> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiPlane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215)), 0.0, 0.0), 
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
            float4(other.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane: Wedge<RoundPoint> {
    associatedtype Output = AntiScalar;
    func wedge(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: Wedge<Scalar> {
    associatedtype Output = Plane;
    func wedge(other: Scalar) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: Wedge<VersorEven> {
    associatedtype Output = AntiScalar;
    func wedge(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: Wedge<VersorOdd> {
    associatedtype Output = Plane;
    func wedge(other: VersorOdd) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<RoundPoint> {
        return WedgeInfixPartial { a: this };
    }
}
extension RoundPoint: Wedge<AntiCircleRotor> {
    associatedtype Output = AntiDipoleInversion;
    func wedge(other: AntiCircleRotor) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy)), other.scalar * this.e4), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * float4(this.group0.xyz, this.e5)
        );
    }
}
extension RoundPoint: Wedge<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiDipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (other.group3.zxyw * this.group0.yzxw) - float4((other.group3.yzx * this.group0.zxy), other.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            float4(other.e1 * this.e5 * -1.0, other.e2 * this.e5 * -1.0, other.e3 * this.e5 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (this.group0.xyzx * float4(other.group3.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
extension RoundPoint: Wedge<AntiDualNum> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiDualNum) -> Output {
        return VersorEven.from_groups(
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
extension RoundPoint: Wedge<AntiFlatPoint> {
    associatedtype Output = Sphere;
    func wedge(other: AntiFlatPoint) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)), 
            /* e1234, 0, 0, 0 */
            float4(other.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: Wedge<AntiFlector> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiFlector) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group1.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), other.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group1.xyz)), other.e321 * this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5))
        );
    }
}
extension RoundPoint: Wedge<AntiLine> {
    associatedtype Output = Circle;
    func wedge(other: AntiLine) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group0.xyz) + (other.group1.xyz.zxy * this.group0.yzx) - (other.group1.xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension RoundPoint: Wedge<AntiMotor> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiMotor) -> Output {
        return VersorEven.from_groups(
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
extension RoundPoint: Wedge<AntiPlane> {
    associatedtype Output = Dipole;
    func wedge(other: AntiPlane) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), other.e5 * this.e4), 
            /* e15, e25, e35, 0 */
            float4((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz), 0.0)
        );
    }
}
extension RoundPoint: Wedge<Circle> {
    associatedtype Output = Sphere;
    func wedge(other: Circle) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group0.yzxy * float4(other.group1.zxy, other.e315)) - float4((float3(this.e5) * other.group0.xyz), other.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: Wedge<CircleRotor> {
    associatedtype Output = Sphere;
    func wedge(other: CircleRotor) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234, 0, 0, 0 */
            float4((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: Wedge<Dipole> {
    associatedtype Output = Circle;
    func wedge(other: Dipole) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group1.xyz) + (other.group2.xyz.zxy * this.group0.yzx) - (other.group2.xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension RoundPoint: Wedge<DipoleInversion> {
    associatedtype Output = CircleRotor;
    func wedge(other: DipoleInversion) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group0.zxy) - (other.group0.xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(other.e25 * this.e3 * -1.0, other.e35 * this.e1 * -1.0, other.e15 * this.e2 * -1.0, (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) + (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group0.yzxx * float4(other.group2.zxy, other.e4235))
        );
    }
}
extension RoundPoint: Wedge<DualNum> {
    associatedtype Output = FlatPoint;
    func wedge(other: DualNum) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group0
        );
    }
}
extension RoundPoint: Wedge<FlatPoint> {
    associatedtype Output = Line;
    func wedge(other: FlatPoint) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4((float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy), 0.0)
        );
    }
}
extension RoundPoint: Wedge<Flector> {
    associatedtype Output = Motor;
    func wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.e45 * this.e1 * -1.0, other.e45 * this.e2 * -1.0, other.e45 * this.e3 * -1.0, (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) + (this.group0.wwwx * float4(other.group0.xyz, other.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
extension RoundPoint: Wedge<Line> {
    associatedtype Output = Plane;
    func wedge(other: Line) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group0.yzxx * float4(other.group0.xyz.zxy, other.e235))
        );
    }
}
extension RoundPoint: Wedge<Motor> {
    associatedtype Output = Flector;
    func wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
extension RoundPoint: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(other.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group0) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e15 * this.e4) + (other.e41 * this.e5), (other.e25 * this.e4) + (other.e42 * this.e5), (other.e35 * this.e4) + (other.e43 * this.e5), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.xyzx * float4(other.group3.www, other.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group5.xyz) + (other.group4.xyz.yzx * this.group0.zxy) - (other.group4.xyz.zxy * this.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(this.e5) * other.group5.xyz) + (other.group3.zxy * this.group0.yzx) - (other.group3.yzx * this.group0.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group0.yzxy * float4(other.group6.zxy, other.e315)) - float4((float3(this.e5) * other.group7.xyz), other.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: Wedge<Plane> {
    associatedtype Output = AntiScalar;
    func wedge(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: Wedge<RoundPoint> {
    associatedtype Output = Dipole;
    func wedge(other: RoundPoint) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.zxy * this.group0.yzx), other.e5 * this.e4) - (other.group0.yzxw * float4(this.group0.zxy, this.e5)), 
            /* e15, e25, e35, 0 */
            float4((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz), 0.0)
        );
    }
}
extension RoundPoint: Wedge<Scalar> {
    associatedtype Output = RoundPoint;
    func wedge(other: Scalar) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: Wedge<Sphere> {
    associatedtype Output = AntiScalar;
    func wedge(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: Wedge<VersorEven> {
    associatedtype Output = DipoleInversion;
    func wedge(other: VersorEven) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(other.group3.zxy, other.e5)) - float4((this.group0.zxy * other.group3.yzx), this.e5 * other.e4), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * other.e1 * -1.0, this.e5 * other.e2 * -1.0, this.e5 * other.e3 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.xyzx * float4(other.group2.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e3 * other.e425) + (this.e4 * other.e235), (this.e1 * other.e435) + (this.e4 * other.e315), (this.e2 * other.e415) + (this.e4 * other.e125), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)) - float4((float3(this.e5) * other.group0.xyz), this.e2 * other.e315)
        );
    }
}
extension RoundPoint: Wedge<VersorOdd> {
    associatedtype Output = VersorEven;
    func wedge(other: VersorOdd) -> Output {
        return VersorEven.from_groups(
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
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Scalar> {
        return WedgeInfixPartial { a: this };
    }
}
extension Scalar: Wedge<AntiCircleRotor> {
    associatedtype Output = AntiCircleRotor;
    func wedge(other: AntiCircleRotor) -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(this.scalar) * other.group2
        );
    }
}
extension Scalar: Wedge<AntiDipoleInversion> {
    associatedtype Output = AntiDipoleInversion;
    func wedge(other: AntiDipoleInversion) -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(this.scalar) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group3
        );
    }
}
extension Scalar: Wedge<AntiDualNum> {
    associatedtype Output = AntiDualNum;
    func wedge(other: AntiDualNum) -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.scalar) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension Scalar: Wedge<AntiFlatPoint> {
    associatedtype Output = AntiFlatPoint;
    func wedge(other: AntiFlatPoint) -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Scalar: Wedge<AntiFlector> {
    associatedtype Output = AntiFlector;
    func wedge(other: AntiFlector) -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
extension Scalar: Wedge<AntiLine> {
    associatedtype Output = AntiLine;
    func wedge(other: AntiLine) -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0)
        );
    }
}
extension Scalar: Wedge<AntiMotor> {
    associatedtype Output = AntiMotor;
    func wedge(other: AntiMotor) -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.scalar) * other.group1
        );
    }
}
extension Scalar: Wedge<AntiPlane> {
    associatedtype Output = AntiPlane;
    func wedge(other: AntiPlane) -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Scalar: Wedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: Wedge<Circle> {
    associatedtype Output = Circle;
    func wedge(other: Circle) -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * other.group2.xyz, 0.0)
        );
    }
}
extension Scalar: Wedge<CircleRotor> {
    associatedtype Output = CircleRotor;
    func wedge(other: CircleRotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * other.group2
        );
    }
}
extension Scalar: Wedge<Dipole> {
    associatedtype Output = Dipole;
    func wedge(other: Dipole) -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(this.scalar) * other.group2.xyz, 0.0)
        );
    }
}
extension Scalar: Wedge<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func wedge(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group3
        );
    }
}
extension Scalar: Wedge<DualNum> {
    associatedtype Output = DualNum;
    func wedge(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.scalar) * other.group0.xy, 0.0, 0.0)
        );
    }
}
extension Scalar: Wedge<FlatPoint> {
    associatedtype Output = FlatPoint;
    func wedge(other: FlatPoint) -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Scalar: Wedge<Flector> {
    associatedtype Output = Flector;
    func wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group1
        );
    }
}
extension Scalar: Wedge<Line> {
    associatedtype Output = Line;
    func wedge(other: Line) -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * other.group1.xyz, 0.0)
        );
    }
}
extension Scalar: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
extension Scalar: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar) * other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(this.scalar) * other.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(this.scalar) * other.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group9, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: Wedge<Plane> {
    associatedtype Output = Plane;
    func wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Scalar: Wedge<RoundPoint> {
    associatedtype Output = RoundPoint;
    func wedge(other: RoundPoint) -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e5, 0, 0, 0 */
            float4(other.e5 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: Wedge<Scalar> {
    associatedtype Output = Scalar;
    func wedge(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: Wedge<Sphere> {
    associatedtype Output = Sphere;
    func wedge(other: Sphere) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.scalar * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: Wedge<VersorEven> {
    associatedtype Output = VersorEven;
    func wedge(other: VersorEven) -> Output {
        return VersorEven.from_groups(
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
extension Scalar: Wedge<VersorOdd> {
    associatedtype Output = VersorOdd;
    func wedge(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
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
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<Sphere> {
        return WedgeInfixPartial { a: this };
    }
}
extension Sphere: Wedge<AntiCircleRotor> {
    associatedtype Output = Sphere;
    func wedge(other: AntiCircleRotor) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(other.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<AntiDualNum> {
    associatedtype Output = Sphere;
    func wedge(other: AntiDualNum) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(other.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<AntiFlector> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<AntiMotor> {
    associatedtype Output = Sphere;
    func wedge(other: AntiMotor) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(other.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<AntiPlane> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiPlane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<DualNum> {
    associatedtype Output = AntiScalar;
    func wedge(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e5 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<Motor> {
    associatedtype Output = AntiScalar;
    func wedge(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e5 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)), 0.0, 0.0), 
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
            float4(other.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(other.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<RoundPoint> {
    associatedtype Output = AntiScalar;
    func wedge(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<Scalar> {
    associatedtype Output = Sphere;
    func wedge(other: Scalar) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(other.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<VersorEven> {
    associatedtype Output = AntiScalar;
    func wedge(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Wedge<VersorOdd> {
    associatedtype Output = Sphere;
    func wedge(other: VersorOdd) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<VersorEven> {
        return WedgeInfixPartial { a: this };
    }
}
extension VersorEven: Wedge<AntiCircleRotor> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiCircleRotor) -> Output {
        return VersorEven.from_groups(
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
extension VersorEven: Wedge<AntiDipoleInversion> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiDipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (other.group3.zxyw * this.group3.yzxw) - float4((other.group3.yzx * this.group3.zxy), other.e4 * this.e5), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)) - float4((this.group2.www * other.group3.xyz), other.e4 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((other.group1.yzx * this.group3.zxy), other.e1 * this.e235) + float4((this.group3.www * other.group2.xyz), other.e2 * this.e315) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e125 * this.e3)
        );
    }
}
extension VersorEven: Wedge<AntiDualNum> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiDualNum) -> Output {
        return VersorEven.from_groups(
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
extension VersorEven: Wedge<AntiFlatPoint> {
    associatedtype Output = Sphere;
    func wedge(other: AntiFlatPoint) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)), 
            /* e1234, 0, 0, 0 */
            float4(other.e321 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Wedge<AntiFlector> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiFlector) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group1.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy)), other.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            (this.group3 * float4(other.group1.www, other.e321)) + float4(float3(0.0), -(other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group1.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + (other.group1.wwwz * float4(this.group0.xyz, this.e125)) + float4(float3(0.0), (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) + float4((this.group3.www * other.group0.xyz), other.e1 * this.e235) - float4((other.group1.zxy * this.group1.yzx), other.e235 * this.e1)
        );
    }
}
extension VersorEven: Wedge<AntiLine> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiLine) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.e4) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)), 
            /* e235, e315, e125, e12345 */
            float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group1.xyz.yzx * this.group3.zxy), other.e23 * this.e415)
        );
    }
}
extension VersorEven: Wedge<AntiMotor> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiMotor) -> Output {
        return VersorEven.from_groups(
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
extension VersorEven: Wedge<AntiPlane> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiPlane) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.e4) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), other.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(other.e5 * this.e1, other.e5 * this.e2, other.e5 * this.e3, -(other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + (other.group0.wwwy * float4(this.group0.xyz, this.e315))
        );
    }
}
extension VersorEven: Wedge<Circle> {
    associatedtype Output = Sphere;
    func wedge(other: Circle) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group3.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group2.www), other.e235 * this.e1), 
            /* e1234, 0, 0, 0 */
            float4((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Wedge<CircleRotor> {
    associatedtype Output = Sphere;
    func wedge(other: CircleRotor) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234, 0, 0, 0 */
            float4((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Wedge<Dipole> {
    associatedtype Output = CircleRotor;
    func wedge(other: Dipole) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group3.zxy) - (other.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group2.xyz.yzx * this.group3.zxy), other.e41 * this.e235)
        );
    }
}
extension VersorEven: Wedge<DipoleInversion> {
    associatedtype Output = CircleRotor;
    func wedge(other: DipoleInversion) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(this.e4) * other.group1.xyz) + (other.group0.xyz.yzx * this.group3.zxy) - (other.group0.xyz.zxy * this.group3.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group3.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group2.yzx * this.group3.zxy), other.e41 * this.e235)
        );
    }
}
extension VersorEven: Wedge<DualNum> {
    associatedtype Output = Flector;
    func wedge(other: DualNum) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
extension VersorEven: Wedge<FlatPoint> {
    associatedtype Output = Motor;
    func wedge(other: FlatPoint) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.e15 * this.e4, other.e25 * this.e4, other.e35 * this.e4, -(other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
extension VersorEven: Wedge<Flector> {
    associatedtype Output = Motor;
    func wedge(other: Flector) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (this.group3.wwwx * float4(other.group0.xyz, other.e4235)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
extension VersorEven: Wedge<Line> {
    associatedtype Output = Plane;
    func wedge(other: Line) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group3.yzxx * float4(other.group0.xyz.zxy, other.e235))
        );
    }
}
extension VersorEven: Wedge<Motor> {
    associatedtype Output = Flector;
    func wedge(other: Motor) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e235 * this.e4) + (other.e5 * this.e423), (other.e315 * this.e4) + (other.e5 * this.e431), (other.e125 * this.e4) + (other.e5 * this.e412), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((other.group0.yzx * this.group3.zxy), other.e5 * this.e321) - (this.group3.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
extension VersorEven: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (other.scalar * this.e12345) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3, 
            /* e5, 0, 0, 0 */
            float4(other.scalar * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group3) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e15 * this.e4) + (other.e41 * this.e5), (other.e25 * this.e4) + (other.e42 * this.e5), (other.e35 * this.e4) + (other.e43 * this.e5), -(other.e31 * this.e2) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group1) - (this.group3.xyzx * float4(other.group3.www, other.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(other.scalar) * this.group0.xyz) + (float3(this.e4) * other.group5.xyz) + (other.group4.xyz.yzx * this.group3.zxy) - (other.group4.xyz.zxy * this.group3.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group5.xyz) + (other.group3.zxy * this.group3.yzx) - (other.group3.yzx * this.group3.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((other.group8.xyz * this.group3.www), other.e1 * this.e235) + float4((other.group6.yzx * this.group3.zxy), other.e3 * this.e125) - (other.group6.zxyw * float4(this.group3.yzx, this.e5)) - float4((other.group7.xyz * this.group2.www), other.e235 * this.e1) - float4((other.group1.zxy * this.group1.yzx), other.e315 * this.e2) - float4((other.group1.www * this.group2.xyz), other.e125 * this.e3), 
            /* e1234, 0, 0, 0 */
            float4((other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Wedge<Plane> {
    associatedtype Output = AntiScalar;
    func wedge(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Wedge<RoundPoint> {
    associatedtype Output = DipoleInversion;
    func wedge(other: RoundPoint) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.zxy * this.group3.yzx), other.e5 * this.e4) - (other.group0.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            float4(other.e5 * this.e1, other.e5 * this.e2, other.e5 * this.e3, -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(other.e3 * this.e425) - (other.e4 * this.e235), -(other.e1 * this.e435) - (other.e4 * this.e315), -(other.e2 * this.e415) - (other.e4 * this.e125), (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4((float3(other.e5) * this.group0.xyz), other.e2 * this.e315)
        );
    }
}
extension VersorEven: Wedge<Scalar> {
    associatedtype Output = VersorEven;
    func wedge(other: Scalar) -> Output {
        return VersorEven.from_groups(
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
extension VersorEven: Wedge<Sphere> {
    associatedtype Output = AntiScalar;
    func wedge(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Wedge<VersorEven> {
    associatedtype Output = DipoleInversion;
    func wedge(other: VersorEven) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (this.group3.yzxw * float4(other.group3.zxy, other.e5)) - (other.group3.yzxw * float4(this.group3.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(other.group2.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group3.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((other.group1.yzx * this.group3.zxy), other.e5 * this.e321) + float4((other.group2.www * this.group0.xyz), other.e2 * this.e315) + float4((this.group3.www * other.group2.xyz), other.e1 * this.e235) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e315 * this.e2) - float4((other.group3.www * this.group2.xyz), other.e125 * this.e3)
        );
    }
}
extension VersorEven: Wedge<VersorOdd> {
    associatedtype Output = VersorEven;
    func wedge(other: VersorOdd) -> Output {
        return VersorEven.from_groups(
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
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: WedgeInfix) -> WedgeInfixPartial<VersorOdd> {
        return WedgeInfixPartial { a: this };
    }
}
extension VersorOdd: Wedge<AntiCircleRotor> {
    associatedtype Output = VersorOdd;
    func wedge(other: AntiCircleRotor) -> Output {
        return VersorOdd.from_groups(
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
extension VersorOdd: Wedge<AntiDipoleInversion> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiDipoleInversion) -> Output {
        return VersorEven.from_groups(
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
extension VersorOdd: Wedge<AntiDualNum> {
    associatedtype Output = VersorOdd;
    func wedge(other: AntiDualNum) -> Output {
        return VersorOdd.from_groups(
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
extension VersorOdd: Wedge<AntiFlatPoint> {
    associatedtype Output = CircleRotor;
    func wedge(other: AntiFlatPoint) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45))
        );
    }
}
extension VersorOdd: Wedge<AntiFlector> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiFlector) -> Output {
        return VersorEven.from_groups(
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
extension VersorOdd: Wedge<AntiLine> {
    associatedtype Output = DipoleInversion;
    func wedge(other: AntiLine) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group1.xyz), -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.xyz.yzx * this.group0.zxy), other.e23 * this.e15)
        );
    }
}
extension VersorOdd: Wedge<AntiMotor> {
    associatedtype Output = VersorOdd;
    func wedge(other: AntiMotor) -> Output {
        return VersorOdd.from_groups(
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
extension VersorOdd: Wedge<AntiPlane> {
    associatedtype Output = VersorEven;
    func wedge(other: AntiPlane) -> Output {
        return VersorEven.from_groups(
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
extension VersorOdd: Wedge<AntiScalar> {
    associatedtype Output = AntiScalar;
    func wedge(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Wedge<Circle> {
    associatedtype Output = CircleRotor;
    func wedge(other: Circle) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43))
        );
    }
}
extension VersorOdd: Wedge<CircleRotor> {
    associatedtype Output = CircleRotor;
    func wedge(other: CircleRotor) -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), (other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43))
        );
    }
}
extension VersorOdd: Wedge<Dipole> {
    associatedtype Output = DipoleInversion;
    func wedge(other: Dipole) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.xyz.zxy * this.group2.yzx), other.e15 * this.e23) - float4((other.group2.xyz.yzx * this.group0.zxy), other.e25 * this.e31)
        );
    }
}
extension VersorOdd: Wedge<DipoleInversion> {
    associatedtype Output = DipoleInversion;
    func wedge(other: DipoleInversion) -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(this.scalar) * other.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.e4235 * this.scalar), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.e4315 * this.scalar), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.e4125 * this.scalar), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.xyz.yzx * this.group2.zxy), other.e3215 * this.scalar) - (this.group2.yzxx * float4(other.group0.xyz.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25)
        );
    }
}
extension VersorOdd: Wedge<DualNum> {
    associatedtype Output = Motor;
    func wedge(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(other.group0.xy.xx, other.e5, (other.e5 * this.e1234) + (other.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
extension VersorOdd: Wedge<FlatPoint> {
    associatedtype Output = Flector;
    func wedge(other: FlatPoint) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e35 * this.e42) + (other.e45 * this.e23), (other.e15 * this.e43) + (other.e45 * this.e31), (other.e25 * this.e41) + (other.e45 * this.e12), -(other.e25 * this.e31) - (other.e35 * this.e12)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension VersorOdd: Wedge<Flector> {
    associatedtype Output = Flector;
    func wedge(other: Flector) -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e45 * this.e23) + (other.e4235 * this.scalar), (other.e45 * this.e31) + (other.e4315 * this.scalar), (other.e45 * this.e12) + (other.e4125 * this.scalar), -(other.e25 * this.e31) - (other.e35 * this.e12)) + (this.group0.yzxw * float4(other.group0.zxy, other.e3215)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension VersorOdd: Wedge<Line> {
    associatedtype Output = Motor;
    func wedge(other: Line) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(float3(1.0), 0.0) * float4(other.group1.xyz, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension VersorOdd: Wedge<Motor> {
    associatedtype Output = Motor;
    func wedge(other: Motor) -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            (float4(other.e5) * float4(this.group0.xyz, this.e1234)) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.e5 * this.scalar)
        );
    }
}
extension VersorOdd: Wedge<MultiVector> {
    associatedtype Output = MultiVector;
    func wedge(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(other.scalar * this.scalar, (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5, 0, 0, 0 */
            float4(other.e5 * this.scalar, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            (float4(other.scalar) * float4(this.group2.xyz, this.e45)) + (float4(this.scalar) * other.group3), 
            /* e41, e42, e43, 0 */
            float4((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group4.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group5.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4((other.e5 * this.e41) + (other.e415 * this.scalar), (other.e5 * this.e42) + (other.e425 * this.scalar), (other.e5 * this.e43) + (other.e435 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((other.group1.www * this.group2.xyz), other.e321 * this.scalar) - (other.group1.xyzx * this.group1.wwwx), 
            /* e423, e431, e412, 0 */
            float4((float3(other.e4) * this.group1.xyz) + (float3(this.scalar) * other.group7.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group8.xyz) + (other.group1.yzx * this.group2.zxy) - (other.group1.zxy * this.group2.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e35 * this.e42) + (other.e45 * this.e23) + (other.e23 * this.e45) + (other.e4235 * this.scalar), (other.e15 * this.e43) + (other.e45 * this.e31) + (other.e31 * this.e45) + (other.e4315 * this.scalar), (other.e25 * this.e41) + (other.e45 * this.e12) + (other.e12 * this.e45) + (other.e4125 * this.scalar), -(other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12) - (other.e12 * this.e35)) + (float4(other.scalar) * this.group3) + float4((other.group4.xyz.yzx * this.group2.zxy), other.e3215 * this.scalar) - (this.group2.yzxx * float4(other.group4.xyz.zxy, other.e23)) - float4((other.group3.yzx * this.group0.zxy), other.e31 * this.e25), 
            /* e1234, 0, 0, 0 */
            float4((other.scalar * this.e1234) + (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Wedge<Plane> {
    associatedtype Output = Plane;
    func wedge(other: Plane) -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
extension VersorOdd: Wedge<RoundPoint> {
    associatedtype Output = VersorEven;
    func wedge(other: RoundPoint) -> Output {
        return VersorEven.from_groups(
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
extension VersorOdd: Wedge<Scalar> {
    associatedtype Output = VersorOdd;
    func wedge(other: Scalar) -> Output {
        return VersorOdd.from_groups(
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
extension VersorOdd: Wedge<Sphere> {
    associatedtype Output = Sphere;
    func wedge(other: Sphere) -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234, 0, 0, 0 */
            float4(other.e1234 * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: Wedge<VersorEven> {
    associatedtype Output = VersorEven;
    func wedge(other: VersorEven) -> Output {
        return VersorEven.from_groups(
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
extension VersorOdd: Wedge<VersorOdd> {
    associatedtype Output = VersorOdd;
    func wedge(other: VersorOdd) -> Output {
        return VersorOdd.from_groups(
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

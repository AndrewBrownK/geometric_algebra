extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<AntiCircleRotor> {
        return DotProductInfixPartial { a: this };
    }
}
extension AntiCircleRotor: DotProduct<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: DotProduct<AntiDualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: DotProduct<AntiLine> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiLine) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: DotProduct<AntiMotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: DotProduct<Dipole> {
    associatedtype Output = Scalar;
    func dot_product(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: DotProduct<FlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.scalar * other.scalar) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.scalar * other.scalar) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<AntiDipoleInversion> {
        return DotProductInfixPartial { a: this };
    }
}
extension AntiDipoleInversion: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4 * this.e5) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: DotProduct<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: DotProduct<AntiFlector> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e4 * other.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: DotProduct<AntiPlane> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiPlane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: DotProduct<Circle> {
    associatedtype Output = Scalar;
    func dot_product(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: DotProduct<CircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e4 * other.e5 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e4 * other.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e4 * other.e5) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: DotProduct<RoundPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e4 * other.e5) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<AntiDualNum> {
        return DotProductInfixPartial { a: this };
    }
}
extension AntiDualNum: DotProduct<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: DotProduct<AntiDualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: DotProduct<AntiMotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e3215 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e3215 * other.e1234) + (this.scalar * other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: DotProduct<Sphere> {
    associatedtype Output = Scalar;
    func dot_product(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e3215 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e3215 * other.e1234) + (this.scalar * other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<AntiFlatPoint> {
        return DotProductInfixPartial { a: this };
    }
}
extension AntiFlatPoint: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: DotProduct<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: DotProduct<AntiFlector> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: DotProduct<Circle> {
    associatedtype Output = Scalar;
    func dot_product(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: DotProduct<CircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<AntiFlector> {
        return DotProductInfixPartial { a: this };
    }
}
extension AntiFlector: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e4 * this.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: DotProduct<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: DotProduct<AntiFlector> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: DotProduct<AntiPlane> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiPlane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: DotProduct<Circle> {
    associatedtype Output = Scalar;
    func dot_product(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: DotProduct<CircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: DotProduct<RoundPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<AntiLine> {
        return DotProductInfixPartial { a: this };
    }
}
extension AntiLine: DotProduct<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: DotProduct<AntiLine> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiLine) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: DotProduct<AntiMotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: DotProduct<Dipole> {
    associatedtype Output = Scalar;
    func dot_product(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<AntiMotor> {
        return DotProductInfixPartial { a: this };
    }
}
extension AntiMotor: DotProduct<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: DotProduct<AntiDualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: DotProduct<AntiLine> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiLine) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: DotProduct<AntiMotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: DotProduct<Dipole> {
    associatedtype Output = Scalar;
    func dot_product(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: DotProduct<Sphere> {
    associatedtype Output = Scalar;
    func dot_product(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e3215 * other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<AntiPlane> {
        return DotProductInfixPartial { a: this };
    }
}
extension AntiPlane: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: DotProduct<AntiFlector> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: DotProduct<AntiPlane> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiPlane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: DotProduct<RoundPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<AntiScalar> {
        return DotProductInfixPartial { a: this };
    }
}
extension AntiScalar: DotProduct<AntiScalar> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: DotProduct<CircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Circle> {
        return DotProductInfixPartial { a: this };
    }
}
extension Circle: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: DotProduct<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: DotProduct<AntiFlector> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: DotProduct<Circle> {
    associatedtype Output = Scalar;
    func dot_product(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: DotProduct<CircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<CircleRotor> {
        return DotProductInfixPartial { a: this };
    }
}
extension CircleRotor: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: DotProduct<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: DotProduct<AntiFlector> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: DotProduct<AntiScalar> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: DotProduct<Circle> {
    associatedtype Output = Scalar;
    func dot_product(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: DotProduct<CircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e12345 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e12345 * other.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e12345 * other.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e12345 * other.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Dipole> {
        return DotProductInfixPartial { a: this };
    }
}
extension Dipole: DotProduct<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: DotProduct<AntiLine> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiLine) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: DotProduct<AntiMotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: DotProduct<Dipole> {
    associatedtype Output = Scalar;
    func dot_product(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: DotProduct<FlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<DipoleInversion> {
        return DotProductInfixPartial { a: this };
    }
}
extension DipoleInversion: DotProduct<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: DotProduct<AntiDualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e3215 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: DotProduct<AntiLine> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiLine) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: DotProduct<AntiMotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: DotProduct<Dipole> {
    associatedtype Output = Scalar;
    func dot_product(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: DotProduct<FlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e1234 * other.e3215) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: DotProduct<Sphere> {
    associatedtype Output = Scalar;
    func dot_product(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<DualNum> {
        return DotProductInfixPartial { a: this };
    }
}
extension DualNum: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e4 * this.e5 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: DotProduct<AntiScalar> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: DotProduct<CircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e12345 * other.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e5 * other.e4) - (this.e12345 * other.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: DotProduct<RoundPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e5 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e5 * other.e4) - (this.e12345 * other.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<FlatPoint> {
        return DotProductInfixPartial { a: this };
    }
}
extension FlatPoint: DotProduct<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: DotProduct<Dipole> {
    associatedtype Output = Scalar;
    func dot_product(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: DotProduct<FlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e45 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e45 * other.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Flector> {
        return DotProductInfixPartial { a: this };
    }
}
extension Flector: DotProduct<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<Dipole> {
    associatedtype Output = Scalar;
    func dot_product(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e1234 * this.e3215) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<FlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e45 * this.e45 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<Sphere> {
    associatedtype Output = Scalar;
    func dot_product(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Line> {
        return DotProductInfixPartial { a: this };
    }
}
extension Line: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: DotProduct<Circle> {
    associatedtype Output = Scalar;
    func dot_product(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: DotProduct<CircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Motor> {
        return DotProductInfixPartial { a: this };
    }
}
extension Motor: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e4 * this.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<AntiScalar> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<Circle> {
    associatedtype Output = Scalar;
    func dot_product(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<CircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e12345 * other.e12345) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<RoundPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e5 * other.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e12345 * other.e12345) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<MultiVector> {
        return DotProductInfixPartial { a: this };
    }
}
extension MultiVector: DotProduct<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4 * this.e5) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<AntiDualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e3215 * this.e1234) + (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<AntiFlector> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<AntiLine> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiLine) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<AntiMotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<AntiPlane> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiPlane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<AntiScalar> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Circle> {
    associatedtype Output = Scalar;
    func dot_product(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<CircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e12345 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Dipole> {
    associatedtype Output = Scalar;
    func dot_product(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e5 * this.e4) - (other.e12345 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<FlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.scalar * this.scalar) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e321 * this.e321) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e12345 * this.e12345) - (other.e4 * this.e5) - (other.e5 * this.e4) - (other.e45 * this.e45) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<RoundPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Sphere> {
    associatedtype Output = Scalar;
    func dot_product(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e3215 * other.e1234) + (this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) + (this.e321 * other.e321) - (this.e12345 * other.e12345) - (this.e4 * other.e5) - (this.e5 * other.e4) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e3215 * other.e1234) + (this.e1234 * other.e3215) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Plane> {
        return DotProductInfixPartial { a: this };
    }
}
extension Plane: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: DotProduct<Sphere> {
    associatedtype Output = Scalar;
    func dot_product(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<RoundPoint> {
        return DotProductInfixPartial { a: this };
    }
}
extension RoundPoint: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: DotProduct<AntiFlector> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: DotProduct<AntiPlane> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiPlane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e5 * this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e5 * this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: DotProduct<RoundPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5) - (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Scalar> {
        return DotProductInfixPartial { a: this };
    }
}
extension Scalar: DotProduct<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: DotProduct<AntiDualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: DotProduct<AntiMotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Sphere> {
        return DotProductInfixPartial { a: this };
    }
}
extension Sphere: DotProduct<AntiDualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e3215 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: DotProduct<AntiMotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e3215 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: DotProduct<Sphere> {
    associatedtype Output = Scalar;
    func dot_product(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e3215 * other.e1234) + (this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<VersorEven> {
        return DotProductInfixPartial { a: this };
    }
}
extension VersorEven: DotProduct<AntiDipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4 * this.e5) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<AntiFlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<AntiFlector> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiFlector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<AntiPlane> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiPlane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<AntiScalar> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiScalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e12345 * this.e12345 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<Circle> {
    associatedtype Output = Scalar;
    func dot_product(other: Circle) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<CircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: CircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e12345 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e5 * this.e4) - (other.e12345 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321) - (other.e12345 * this.e12345) - (other.e4 * this.e5) - (other.e5 * this.e4) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<RoundPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: RoundPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: DotProduct<VersorEven> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorEven) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e12345 * this.e12345) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4) - (other.e4 * this.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<VersorOdd> {
        return DotProductInfixPartial { a: this };
    }
}
extension VersorOdd: DotProduct<AntiCircleRotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiCircleRotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<AntiDualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiDualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e3215 * this.e1234) + (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<AntiLine> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiLine) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<AntiMotor> {
    associatedtype Output = Scalar;
    func dot_product(other: AntiMotor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<Dipole> {
    associatedtype Output = Scalar;
    func dot_product(other: Dipole) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<DipoleInversion> {
    associatedtype Output = Scalar;
    func dot_product(other: DipoleInversion) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<FlatPoint> {
    associatedtype Output = Scalar;
    func dot_product(other: FlatPoint) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.scalar * this.scalar) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<Sphere> {
    associatedtype Output = Scalar;
    func dot_product(other: Sphere) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: DotProduct<VersorOdd> {
    associatedtype Output = Scalar;
    func dot_product(other: VersorOdd) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.scalar * this.scalar) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}

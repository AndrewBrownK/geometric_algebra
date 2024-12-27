extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<AntiCircleRotor> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension AntiCircleRotor: AntiDotProduct<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiDotProduct<AntiDualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiDotProduct<AntiLine> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiLine) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiDotProduct<AntiMotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.scalar * other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiDotProduct<Dipole> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiDotProduct<FlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.scalar * other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiDotProduct<Scalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiCircleRotor: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.scalar * other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<AntiDipoleInversion> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension AntiDipoleInversion: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiDotProduct<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiDotProduct<AntiFlector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e4 * other.e5) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiDotProduct<AntiPlane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiPlane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4 * other.e5) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiDotProduct<Circle> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiDotProduct<CircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e4 * other.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiDotProduct<Line> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e4 * other.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiDotProduct<RoundPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4 * other.e5) + (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<AntiDualNum> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension AntiDualNum: AntiDotProduct<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiDotProduct<AntiDualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiDotProduct<AntiMotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e3215 * other.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e3215 * other.e1234) - (this.scalar * other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiDotProduct<Scalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiDotProduct<Sphere> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e3215 * other.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDualNum: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e3215 * other.e1234) - (this.scalar * other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<AntiFlatPoint> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension AntiFlatPoint: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: AntiDotProduct<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e321 * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: AntiDotProduct<AntiFlector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e321 * other.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: AntiDotProduct<Circle> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: AntiDotProduct<CircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<AntiFlector> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension AntiFlector: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e4 * this.e5) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: AntiDotProduct<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e321 * this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: AntiDotProduct<AntiFlector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: AntiDotProduct<AntiPlane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiPlane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: AntiDotProduct<Circle> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: AntiDotProduct<CircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: AntiDotProduct<RoundPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlector: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<AntiLine> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension AntiLine: AntiDotProduct<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: AntiDotProduct<AntiLine> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiLine) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: AntiDotProduct<AntiMotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: AntiDotProduct<Dipole> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiLine: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<AntiMotor> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension AntiMotor: AntiDotProduct<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: AntiDotProduct<AntiDualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: AntiDotProduct<AntiLine> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiLine) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: AntiDotProduct<AntiMotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: AntiDotProduct<Dipole> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.scalar * other.scalar) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: AntiDotProduct<Scalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: AntiDotProduct<Sphere> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e3215 * other.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiMotor: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.scalar * other.scalar) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<AntiPlane> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension AntiPlane: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4 * this.e5) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: AntiDotProduct<AntiFlector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: AntiDotProduct<AntiPlane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiPlane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: AntiDotProduct<RoundPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiPlane: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<AntiScalar> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension AntiScalar: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiDotProduct<CircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Circle> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Circle: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiDotProduct<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiDotProduct<AntiFlector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiDotProduct<Circle> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiDotProduct<CircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiDotProduct<Line> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<CircleRotor> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension CircleRotor: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiDotProduct<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiDotProduct<AntiFlector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiDotProduct<Circle> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiDotProduct<CircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e12345 * this.e12345) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiDotProduct<Line> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e12345 * other.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e12345 * other.e12345) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e12345 * other.e12345) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Dipole> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Dipole: AntiDotProduct<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiDotProduct<AntiLine> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiLine) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiDotProduct<AntiMotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiDotProduct<Dipole> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiDotProduct<FlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<DipoleInversion> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension DipoleInversion: AntiDotProduct<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiDotProduct<AntiDualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e3215 * this.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiDotProduct<AntiLine> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiLine) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiDotProduct<AntiMotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiDotProduct<Dipole> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiDotProduct<FlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e1234 * other.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e1234 * other.e3215) - (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiDotProduct<Plane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e1234 * other.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiDotProduct<Sphere> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e1234 * other.e3215) - (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e1234 * other.e3215) - (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<DualNum> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension DualNum: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e4 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiDotProduct<CircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345 * other.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e5 * other.e4) + (this.e12345 * other.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiDotProduct<RoundPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e5 * other.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e5 * other.e4) + (this.e12345 * other.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<FlatPoint> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension FlatPoint: AntiDotProduct<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiDotProduct<Dipole> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiDotProduct<FlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e45 * this.e45, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e45 * other.e45, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Flector> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Flector: AntiDotProduct<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiDotProduct<Dipole> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e1234 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiDotProduct<FlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e45 * this.e45, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiDotProduct<Plane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiDotProduct<Sphere> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Line> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Line: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiDotProduct<Circle> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiDotProduct<CircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiDotProduct<Line> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Motor> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Motor: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e4 * this.e5), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiDotProduct<Circle> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiDotProduct<CircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiDotProduct<Line> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e12345 * other.e12345) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiDotProduct<RoundPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e5 * other.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e12345 * other.e12345) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<MultiVector> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension MultiVector: AntiDotProduct<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<AntiDualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e3215 * this.e1234) - (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<AntiFlector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<AntiLine> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiLine) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<AntiMotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<AntiPlane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiPlane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<Circle> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<CircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e12345 * this.e12345) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<Dipole> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e5 * this.e4) + (other.e12345 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<FlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<Line> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e12345 * this.e12345) + (other.e4 * this.e5) + (other.e5 * this.e4) + (other.e45 * this.e45) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.scalar * this.scalar) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e321 * this.e321) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<Plane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e1234 * other.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<RoundPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4 * other.e5) + (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<Scalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<Sphere> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234) - (this.e1234 * other.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e12345 * other.e12345) + (this.e4 * other.e5) + (this.e5 * other.e4) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3) - (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.scalar * other.scalar) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e3215 * other.e1234) - (this.e1234 * other.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Plane> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Plane: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e1234 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e1234 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: AntiDotProduct<Plane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: AntiDotProduct<Sphere> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<RoundPoint> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension RoundPoint: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiDotProduct<AntiFlector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiDotProduct<AntiPlane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiPlane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e5 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e5 * this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiDotProduct<RoundPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4 * other.e5) + (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Scalar> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Scalar: AntiDotProduct<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiDotProduct<AntiDualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiDotProduct<AntiMotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiDotProduct<Scalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.scalar * other.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Sphere> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Sphere: AntiDotProduct<AntiDualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e3215 * this.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiDotProduct<AntiMotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e3215 * this.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiDotProduct<Plane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiDotProduct<Sphere> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234) - (this.e1234 * other.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<VersorEven> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension VersorEven: AntiDotProduct<AntiDipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<AntiFlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<AntiFlector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiFlector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<AntiPlane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiPlane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.e12345 * this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<Circle> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Circle) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<CircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: CircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e12345 * this.e12345) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e5 * this.e4) + (other.e12345 * this.e12345), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<Line> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e12345 * this.e12345) + (other.e4 * this.e5) + (other.e5 * this.e4) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3) - (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<RoundPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: RoundPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: AntiDotProduct<VersorEven> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorEven) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e12345 * this.e12345) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4) + (other.e4 * this.e5) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<VersorOdd> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension VersorOdd: AntiDotProduct<AntiCircleRotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiCircleRotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<AntiDualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiDualNum) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e3215 * this.e1234) - (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<AntiLine> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiLine) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<AntiMotor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiMotor) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<Dipole> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Dipole) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<DipoleInversion> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DipoleInversion) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<FlatPoint> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: FlatPoint) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.scalar * this.scalar) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<Plane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<Scalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(other.scalar * this.scalar * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<Sphere> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Sphere) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: AntiDotProduct<VersorOdd> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: VersorOdd) -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.scalar * this.scalar) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
    }
}

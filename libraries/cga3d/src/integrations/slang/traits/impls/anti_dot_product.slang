implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<AntiCircleRotor> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<AntiCircleRotor>(this);
    }
}
public extension AntiDotProductInfixPartial<AntiCircleRotor> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.scalar * this.scalar)
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12)
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.scalar * other.scalar)
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35)
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35)
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.scalar * other.scalar)
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.scalar * other.scalar)
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<AntiDipoleInversion> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<AntiDipoleInversion>(this);
    }
}
public extension AntiDotProductInfixPartial<AntiDipoleInversion> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) - (this.e321 * other.e321)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e4 * other.e5) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e5) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            this.e4 * other.e5
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e4 * other.e5)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiDualNum {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<AntiDualNum> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<AntiDualNum>(this);
    }
}
public extension AntiDotProductInfixPartial<AntiDualNum> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            this.e3215 * other.e1234 * -1.0
        );
    }
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            -(this.e3215 * other.e1234) - (this.scalar * other.scalar)
        );
    }
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            this.e3215 * other.e1234 * -1.0
        );
    }
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            -(this.e3215 * other.e1234) - (this.scalar * other.scalar)
        );
    }
}
public extension AntiFlatPoint {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<AntiFlatPoint> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<AntiFlatPoint>(this);
    }
}
public extension AntiDotProductInfixPartial<AntiFlatPoint> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) - (other.e321 * this.e321)
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e321 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            this.e321 * other.e321 * -1.0
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
public extension AntiFlector {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<AntiFlector> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<AntiFlector>(this);
    }
}
public extension AntiDotProductInfixPartial<AntiFlector> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e4 * this.e5) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e321 * -1.0
        );
    }
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            -(other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            -(this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiLine {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<AntiLine> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<AntiLine>(this);
    }
}
public extension AntiDotProductInfixPartial<AntiLine> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension AntiLine {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)
        );
    }
}
public extension AntiLine {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)
        );
    }
}
public extension AntiLine {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12)
        );
    }
}
public extension AntiLine {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension AntiLine {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension AntiLine {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension AntiLine {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension AntiMotor {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<AntiMotor> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<AntiMotor>(this);
    }
}
public extension AntiDotProductInfixPartial<AntiMotor> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar)
        );
    }
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)
        );
    }
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar)
        );
    }
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234)
        );
    }
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.scalar * other.scalar) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234)
        );
    }
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            this.e3215 * other.e1234 * -1.0
        );
    }
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.scalar * other.scalar) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234)
        );
    }
}
public extension AntiPlane {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<AntiPlane> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<AntiPlane>(this);
    }
}
public extension AntiDotProductInfixPartial<AntiPlane> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e5) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            -(other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            -(other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension AntiScalar {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<AntiScalar> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<AntiScalar>(this);
    }
}
public extension AntiDotProductInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345
        );
    }
}
public extension Circle {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<Circle> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Circle>(this);
    }
}
public extension AntiDotProductInfixPartial<Circle> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Circle {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension Circle {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension Circle {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension Circle {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension Circle {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
public extension Circle {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435)
        );
    }
}
public extension Circle {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435)
        );
    }
}
public extension Circle {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
public extension Circle {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
public extension CircleRotor {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<CircleRotor> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<CircleRotor>(this);
    }
}
public extension AntiDotProductInfixPartial<CircleRotor> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e12345 * this.e12345) - (other.e321 * this.e321)
        );
    }
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345
        );
    }
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435)
        );
    }
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e12345 * other.e12345)
        );
    }
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e12345 * other.e12345) - (this.e321 * other.e321)
        );
    }
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e12345 * other.e12345) - (this.e321 * other.e321)
        );
    }
}
public extension Dipole {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<Dipole> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Dipole>(this);
    }
}
public extension AntiDotProductInfixPartial<Dipole> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Dipole {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension Dipole {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension Dipole {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension Dipole {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension Dipole {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension Dipole {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35)
        );
    }
}
public extension Dipole {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35)
        );
    }
}
public extension Dipole {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension Dipole {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension DipoleInversion {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<DipoleInversion> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<DipoleInversion>(this);
    }
}
public extension AntiDotProductInfixPartial<DipoleInversion> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234 * -1.0
        );
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e1234 * other.e3215)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e1234 * other.e3215) - (this.e3215 * other.e1234)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e1234 * other.e3215)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e1234 * other.e3215) - (this.e3215 * other.e1234)
        );
    }
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e1234 * other.e3215) - (this.e3215 * other.e1234)
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<DualNum> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<DualNum>(this);
    }
}
public extension AntiDotProductInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension DualNum {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            other.e4 * this.e5
        );
    }
}
public extension DualNum {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
public extension DualNum {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
public extension DualNum {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
public extension DualNum {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345
        );
    }
}
public extension DualNum {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e5 * other.e4) + (this.e12345 * other.e12345)
        );
    }
}
public extension DualNum {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4
        );
    }
}
public extension DualNum {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e5 * other.e4) + (this.e12345 * other.e12345)
        );
    }
}
public extension FlatPoint {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<FlatPoint> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<FlatPoint>(this);
    }
}
public extension AntiDotProductInfixPartial<FlatPoint> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35)
        );
    }
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35)
        );
    }
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35)
        );
    }
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45
        );
    }
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            this.e45 * other.e45
        );
    }
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<Flector> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Flector>(this);
    }
}
public extension AntiDotProductInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35)
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35)
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e1234 * this.e3215)
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125)
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234)
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125)
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234)
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234)
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<Line> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Line>(this);
    }
}
public extension AntiDotProductInfixPartial<Line> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Line {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
        );
    }
}
public extension Line {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
        );
    }
}
public extension Line {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
        );
    }
}
public extension Line {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
        );
    }
}
public extension Line {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435)
        );
    }
}
public extension Line {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412)
        );
    }
}
public extension Line {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412)
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<Motor> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Motor>(this);
    }
}
public extension AntiDotProductInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e4 * this.e5)
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345)
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345)
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e12345 * other.e12345) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4)
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e12345 * other.e12345) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4)
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<MultiVector> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<MultiVector>(this);
    }
}
public extension AntiDotProductInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.scalar * this.scalar)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            -(other.e3215 * this.e1234) - (other.scalar * this.scalar)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e12345 * this.e12345) - (other.e321 * this.e321)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            (other.e5 * this.e4) + (other.e12345 * this.e12345)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (other.e12345 * this.e12345) + (other.e4 * this.e5) + (other.e5 * this.e4) + (other.e45 * this.e45) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.scalar * this.scalar) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e321 * this.e321) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e1234 * other.e3215)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234) - (this.e1234 * other.e3215)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e12345 * other.e12345) + (this.e4 * other.e5) + (this.e5 * other.e4) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3) - (this.e321 * other.e321)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.scalar * other.scalar) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e3215 * other.e1234) - (this.e1234 * other.e3215)
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<Plane> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Plane>(this);
    }
}
public extension AntiDotProductInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(DipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Plane {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e1234 * this.e3215)
        );
    }
}
public extension Plane {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125)
        );
    }
}
public extension Plane {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e1234 * this.e3215)
        );
    }
}
public extension Plane {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125)
        );
    }
}
public extension Plane {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234)
        );
    }
}
public extension Plane {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234)
        );
    }
}
public extension RoundPoint {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<RoundPoint> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<RoundPoint>(this);
    }
}
public extension AntiDotProductInfixPartial<RoundPoint> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e5 * this.e4
        );
    }
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            other.e5 * this.e4
        );
    }
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<Scalar> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Scalar>(this);
    }
}
public extension AntiDotProductInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Scalar {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
public extension Scalar {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
public extension Scalar {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
public extension Scalar {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
public extension Scalar {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
public extension Scalar {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
public extension Sphere {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<Sphere> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Sphere>(this);
    }
}
public extension AntiDotProductInfixPartial<Sphere> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDualNum rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Sphere {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234 * -1.0
        );
    }
}
public extension Sphere {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234 * -1.0
        );
    }
}
public extension Sphere {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234)
        );
    }
}
public extension Sphere {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234)
        );
    }
}
public extension Sphere {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215)
        );
    }
}
public extension Sphere {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234)
        );
    }
}
public extension Sphere {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215)
        );
    }
}
public extension Sphere {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234) - (this.e1234 * other.e3215)
        );
    }
}
public extension VersorEven {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<VersorEven> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<VersorEven>(this);
    }
}
public extension AntiDotProductInfixPartial<VersorEven> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e12345 * this.e12345) - (other.e321 * this.e321)
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            (other.e5 * this.e4) + (other.e12345 * this.e12345)
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412)
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4)
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (other.e12345 * this.e12345) + (other.e4 * this.e5) + (other.e5 * this.e4) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3) - (other.e321 * this.e321)
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension VersorEven {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e12345 * this.e12345) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4) + (other.e4 * this.e5) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
public extension VersorOdd {
    // Fancy infix trick (first half)
    public AntiDotProductInfixPartial<VersorOdd> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<VersorOdd>(this);
    }
}
public extension AntiDotProductInfixPartial<VersorOdd> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return this.a.anti_dot_product(rhs);
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.scalar * this.scalar)
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            -(other.e3215 * this.e1234) - (other.scalar * this.scalar)
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234)
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234)
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234)
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.scalar * this.scalar) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215)
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234)
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215)
        );
    }
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.scalar * this.scalar) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234)
        );
    }
}

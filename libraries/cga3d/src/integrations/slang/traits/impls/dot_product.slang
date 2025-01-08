implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<AntiCircleRotor> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<AntiCircleRotor>(this);
    }
}
public extension DotProductInfixPartial<AntiCircleRotor> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension AntiCircleRotor {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (other.e45 * this.e45)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension AntiCircleRotor {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.scalar * other.scalar) - (this.e45 * other.e45)
        );
    }
}
public extension AntiCircleRotor {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension AntiCircleRotor {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.scalar * other.scalar) - (this.e45 * other.e45)
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<AntiDipoleInversion> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<AntiDipoleInversion>(this);
    }
}
public extension DotProductInfixPartial<AntiDipoleInversion> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e4 * other.e5)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            this.e4 * other.e5 * -1.0
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e4 * other.e5)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
public extension AntiDualNum {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<AntiDualNum> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<AntiDualNum>(this);
    }
}
public extension DotProductInfixPartial<AntiDualNum> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension AntiDualNum {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension AntiDualNum {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension AntiDualNum {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension AntiDualNum {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
public extension AntiDualNum {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) + (this.scalar * other.scalar)
        );
    }
}
public extension AntiDualNum {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension AntiDualNum {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
public extension AntiDualNum {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) + (this.scalar * other.scalar)
        );
    }
}
public extension AntiFlatPoint {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<AntiFlatPoint> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<AntiFlatPoint>(this);
    }
}
public extension DotProductInfixPartial<AntiFlatPoint> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension AntiFlatPoint {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
public extension AntiFlatPoint {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension AntiFlatPoint {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
public extension AntiFlatPoint {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension AntiFlatPoint {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension AntiFlatPoint {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension AntiFlatPoint {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension AntiFlector {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<AntiFlector> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<AntiFlector>(this);
    }
}
public extension DotProductInfixPartial<AntiFlector> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension AntiFlector {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e4 * this.e5)
        );
    }
}
public extension AntiFlector {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension AntiFlector {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
public extension AntiFlector {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
        );
    }
}
public extension AntiFlector {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension AntiFlector {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension AntiFlector {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4)
        );
    }
}
public extension AntiFlector {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4)
        );
    }
}
public extension AntiFlector {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4)
        );
    }
}
public extension AntiLine {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<AntiLine> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<AntiLine>(this);
    }
}
public extension DotProductInfixPartial<AntiLine> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension AntiLine {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
public extension AntiLine {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
public extension AntiLine {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
public extension AntiLine {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
public extension AntiLine {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
public extension AntiLine {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
public extension AntiLine {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
public extension AntiMotor {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<AntiMotor> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<AntiMotor>(this);
    }
}
public extension DotProductInfixPartial<AntiMotor> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension AntiMotor {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
public extension AntiMotor {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension AntiMotor {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
public extension AntiMotor {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
public extension AntiMotor {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
public extension AntiMotor {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234)
        );
    }
}
public extension AntiMotor {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234)
        );
    }
}
public extension AntiMotor {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension AntiMotor {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
public extension AntiMotor {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234)
        );
    }
}
public extension AntiPlane {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<AntiPlane> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<AntiPlane>(this);
    }
}
public extension DotProductInfixPartial<AntiPlane> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension AntiPlane {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5)
        );
    }
}
public extension AntiPlane {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
public extension AntiPlane {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
public extension AntiPlane {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4)
        );
    }
}
public extension AntiPlane {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4)
        );
    }
}
public extension AntiPlane {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4)
        );
    }
}
public extension AntiScalar {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<AntiScalar> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<AntiScalar>(this);
    }
}
public extension DotProductInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiScalar rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension AntiScalar {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
public extension Circle {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<Circle> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<Circle>(this);
    }
}
public extension DotProductInfixPartial<Circle> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Circle {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension Circle {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension Circle {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension Circle {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension Circle {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension Circle {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
public extension Circle {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
public extension Circle {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension Circle {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension CircleRotor {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<CircleRotor> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<CircleRotor>(this);
    }
}
public extension DotProductInfixPartial<CircleRotor> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension CircleRotor {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension CircleRotor {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension CircleRotor {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension CircleRotor {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension CircleRotor {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension CircleRotor {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e12345 * this.e12345)
        );
    }
}
public extension CircleRotor {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
public extension CircleRotor {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
public extension CircleRotor {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e12345 * other.e12345)
        );
    }
}
public extension CircleRotor {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e12345 * other.e12345)
        );
    }
}
public extension CircleRotor {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e12345 * other.e12345)
        );
    }
}
public extension Dipole {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<Dipole> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<Dipole>(this);
    }
}
public extension DotProductInfixPartial<Dipole> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Dipole {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
public extension Dipole {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
public extension Dipole {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
public extension Dipole {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
public extension Dipole {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
public extension Dipole {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
public extension Dipole {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
public extension Dipole {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
public extension Dipole {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
public extension DipoleInversion {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<DipoleInversion> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<DipoleInversion>(this);
    }
}
public extension DotProductInfixPartial<DipoleInversion> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension DipoleInversion {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
public extension DipoleInversion {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.e3215 * this.e1234
        );
    }
}
public extension DipoleInversion {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
public extension DipoleInversion {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234)
        );
    }
}
public extension DipoleInversion {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
public extension DipoleInversion {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension DipoleInversion {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
public extension DipoleInversion {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e1234 * other.e3215) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension DipoleInversion {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension DipoleInversion {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            (this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension DipoleInversion {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension DipoleInversion {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<DualNum> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<DualNum>(this);
    }
}
public extension DotProductInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension DualNum {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            other.e4 * this.e5 * -1.0
        );
    }
}
public extension DualNum {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension DualNum {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension DualNum {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension DualNum {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
public extension DualNum {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            -(this.e5 * other.e4) - (this.e12345 * other.e12345)
        );
    }
}
public extension DualNum {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            this.e5 * other.e4 * -1.0
        );
    }
}
public extension DualNum {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            -(this.e5 * other.e4) - (this.e12345 * other.e12345)
        );
    }
}
public extension FlatPoint {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<FlatPoint> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<FlatPoint>(this);
    }
}
public extension DotProductInfixPartial<FlatPoint> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension FlatPoint {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
public extension FlatPoint {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
public extension FlatPoint {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
public extension FlatPoint {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * this.e45 * -1.0
        );
    }
}
public extension FlatPoint {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            this.e45 * other.e45 * -1.0
        );
    }
}
public extension FlatPoint {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
public extension FlatPoint {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<Flector> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<Flector>(this);
    }
}
public extension DotProductInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Flector {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
public extension Flector {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
public extension Flector {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e1234 * this.e3215) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension Flector {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * this.e45 * -1.0
        );
    }
}
public extension Flector {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            -(other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension Flector {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension Flector {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            -(this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension Flector {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension Flector {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<Line> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<Line>(this);
    }
}
public extension DotProductInfixPartial<Line> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Line {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
public extension Line {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
public extension Line {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
public extension Line {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
public extension Line {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
public extension Line {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension Line {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<Motor> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<Motor>(this);
    }
}
public extension DotProductInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Motor {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e4 * this.e5)
        );
    }
}
public extension Motor {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension Motor {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
public extension Motor {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345)
        );
    }
}
public extension Motor {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension Motor {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
public extension Motor {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345)
        );
    }
}
public extension Motor {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e12345 * other.e12345) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4)
        );
    }
}
public extension Motor {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            this.e5 * other.e4 * -1.0
        );
    }
}
public extension Motor {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e12345 * other.e12345) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4)
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<MultiVector> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<MultiVector>(this);
    }
}
public extension DotProductInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension MultiVector {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (other.e45 * this.e45)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) + (other.scalar * this.scalar)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e12345 * this.e12345)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            -(other.e5 * this.e4) - (other.e12345 * this.e12345)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.scalar * this.scalar) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e321 * this.e321) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e12345 * this.e12345) - (other.e4 * this.e5) - (other.e5 * this.e4) - (other.e45 * this.e45) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            (this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) + (this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) + (this.e321 * other.e321) - (this.e12345 * other.e12345) - (this.e4 * other.e5) - (this.e5 * other.e4) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e3215 * other.e1234) + (this.e1234 * other.e3215) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<Plane> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<Plane>(this);
    }
}
public extension DotProductInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Scalar operator /(DipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Plane {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension Plane {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            -(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension Plane {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension Plane {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            -(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension Plane {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension Plane {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension RoundPoint {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<RoundPoint> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<RoundPoint>(this);
    }
}
public extension DotProductInfixPartial<RoundPoint> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension RoundPoint {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
public extension RoundPoint {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4)
        );
    }
}
public extension RoundPoint {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4)
        );
    }
}
public extension RoundPoint {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.e5 * this.e4 * -1.0
        );
    }
}
public extension RoundPoint {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            other.e5 * this.e4 * -1.0
        );
    }
}
public extension RoundPoint {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
public extension RoundPoint {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
public extension RoundPoint {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
public extension Scalar {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<Scalar> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<Scalar>(this);
    }
}
public extension DotProductInfixPartial<Scalar> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Scalar {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension Sphere {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<Sphere> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<Sphere>(this);
    }
}
public extension DotProductInfixPartial<Sphere> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDualNum rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Sphere {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.e3215 * this.e1234
        );
    }
}
public extension Sphere {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            other.e3215 * this.e1234
        );
    }
}
public extension Sphere {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension Sphere {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension Sphere {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension Sphere {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension Sphere {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension Sphere {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) + (this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
public extension VersorEven {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<VersorEven> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<VersorEven>(this);
    }
}
public extension DotProductInfixPartial<VersorEven> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiDipoleInversion rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension VersorEven {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4)
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e12345 * this.e12345)
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            -(other.e5 * this.e4) - (other.e12345 * this.e12345)
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321) - (other.e12345 * this.e12345) - (other.e4 * this.e5) - (other.e5 * this.e4) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
public extension VersorEven {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e12345 * this.e12345) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4) - (other.e4 * this.e5)
        );
    }
}
public extension VersorOdd {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<VersorOdd> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<VersorOdd>(this);
    }
}
public extension DotProductInfixPartial<VersorOdd> {
    // Fancy infix trick (second half)
    public Scalar operator /(AntiCircleRotor rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension VersorOdd {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (other.e45 * this.e45)
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) + (other.scalar * this.scalar)
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234)
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.scalar * this.scalar) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
public extension VersorOdd {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.scalar * this.scalar) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}

public extension AntiScalar {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<AntiScalar> {
        return AntiDotProductInfixPartial(this);
    }
}
public extension AntiDotProductInfixPartial<AntiScalar> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> AntiScalar.Output {
        return this.a.anti_dot_product(rhs);
    }
}
public extension AntiScalar: AntiDotProduct<AntiScalar> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension AntiScalar: AntiDotProduct<DualNum> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
public extension AntiScalar: AntiDotProduct<Motor> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
public extension AntiScalar: AntiDotProduct<MultiVector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
public extension DualNum {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<DualNum> {
        return AntiDotProductInfixPartial(this);
    }
}
public extension AntiDotProductInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> DualNum.Output {
        return this.a.anti_dot_product(rhs);
    }
}
public extension DualNum: AntiDotProduct<AntiScalar> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension DualNum: AntiDotProduct<DualNum> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension DualNum: AntiDotProduct<Motor> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
public extension DualNum: AntiDotProduct<MultiVector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
public extension Flector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Flector> {
        return AntiDotProductInfixPartial(this);
    }
}
public extension AntiDotProductInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Flector.Output {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Flector: AntiDotProduct<Flector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
public extension Flector: AntiDotProduct<MultiVector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e4 * other.e4) + (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
public extension Flector: AntiDotProduct<Origin> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
public extension Flector: AntiDotProduct<Plane> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
public extension Flector: AntiDotProduct<Point> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
public extension Line {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Line> {
        return AntiDotProductInfixPartial(this);
    }
}
public extension AntiDotProductInfixPartial<Line> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Line.Output {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Line: AntiDotProduct<Line> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Line) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
public extension Line: AntiDotProduct<Motor> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43)
        );
    }
}
public extension Line: AntiDotProduct<MultiVector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43)
        );
    }
}
public extension Motor {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Motor> {
        return AntiDotProductInfixPartial(this);
    }
}
public extension AntiDotProductInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Motor.Output {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Motor: AntiDotProduct<AntiScalar> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension Motor: AntiDotProduct<DualNum> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension Motor: AntiDotProduct<Line> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Line) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
public extension Motor: AntiDotProduct<Motor> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e1234 * this.e1234)
        );
    }
}
public extension Motor: AntiDotProduct<MultiVector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43) + (this.e1234 * other.e1234)
        );
    }
}
public extension MultiVector {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<MultiVector> {
        return AntiDotProductInfixPartial(this);
    }
}
public extension AntiDotProductInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> MultiVector.Output {
        return this.a.anti_dot_product(rhs);
    }
}
public extension MultiVector: AntiDotProduct<AntiScalar> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension MultiVector: AntiDotProduct<DualNum> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension MultiVector: AntiDotProduct<Flector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
public extension MultiVector: AntiDotProduct<Line> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Line) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
public extension MultiVector: AntiDotProduct<Motor> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e1234 * this.e1234)
        );
    }
}
public extension MultiVector: AntiDotProduct<MultiVector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e1234 * this.e1234) + (other.e4 * this.e4) + (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
public extension MultiVector: AntiDotProduct<Origin> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
public extension MultiVector: AntiDotProduct<Plane> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
public extension MultiVector: AntiDotProduct<Point> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
public extension Origin {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Origin> {
        return AntiDotProductInfixPartial(this);
    }
}
public extension AntiDotProductInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Origin.Output {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Origin: AntiDotProduct<Flector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Origin: AntiDotProduct<MultiVector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Origin: AntiDotProduct<Origin> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Origin: AntiDotProduct<Point> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
public extension Plane {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Plane> {
        return AntiDotProductInfixPartial(this);
    }
}
public extension AntiDotProductInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Plane.Output {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Plane: AntiDotProduct<Flector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
public extension Plane: AntiDotProduct<MultiVector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
public extension Plane: AntiDotProduct<Plane> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Point> {
        return AntiDotProductInfixPartial(this);
    }
}
public extension AntiDotProductInfixPartial<Point> {
    // Fancy infix trick (second half)
    public func operator/(rhs: B) -> Point.Output {
        return this.a.anti_dot_product(rhs);
    }
}
public extension Point: AntiDotProduct<Flector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Point: AntiDotProduct<MultiVector> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Point: AntiDotProduct<Origin> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Point: AntiDotProduct<Point> {
    public typedef AntiScalar AntiDotProductOutput;
    public func anti_dot_product(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}

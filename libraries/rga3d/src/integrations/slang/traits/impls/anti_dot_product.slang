implementing rga3d;
using traits;
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
    public AntiDotProductInfixPartial<DualNum> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<DualNum>(this);
    }
}
public extension AntiDotProductInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
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
    public AntiDotProductInfixPartial<Flector> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Flector>(this);
    }
}
public extension AntiDotProductInfixPartial<Flector> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(Flector rhs) {
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
    public AntiDotProductInfixPartial<Line> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Line>(this);
    }
}
public extension AntiDotProductInfixPartial<Line> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(Line rhs) {
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
    public AntiDotProductInfixPartial<Motor> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Motor>(this);
    }
}
public extension AntiDotProductInfixPartial<Motor> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
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
    public AntiDotProductInfixPartial<MultiVector> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<MultiVector>(this);
    }
}
public extension AntiDotProductInfixPartial<MultiVector> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(AntiScalar rhs) {
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
    public AntiDotProductInfixPartial<Origin> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Origin>(this);
    }
}
public extension AntiDotProductInfixPartial<Origin> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(Flector rhs) {
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
    public AntiDotProductInfixPartial<Plane> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Plane>(this);
    }
}
public extension AntiDotProductInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(Flector rhs) {
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
    public AntiDotProductInfixPartial<Point> operator /(AntiDotProductInfix rhs) {
        return AntiDotProductInfixPartial<Point>(this);
    }
}
public extension AntiDotProductInfixPartial<Point> {
    // Fancy infix trick (second half)
    public AntiScalar operator /(Flector rhs) {
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

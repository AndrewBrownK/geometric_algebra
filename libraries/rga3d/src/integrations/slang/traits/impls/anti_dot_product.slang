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
public extension AntiScalar {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
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
public extension DualNum {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension DualNum {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension DualNum {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
public extension DualNum {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
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
public extension Flector {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e1234 */
            (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            (this.e4 * other.e4) + (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(Origin other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e1234 */
            (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
public extension Flector {
    public AntiScalar anti_dot_product(Point other) {
        return AntiScalar(
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
public extension Line {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
public extension Line {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43)
        );
    }
}
public extension Line {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
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
public extension Motor {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e1234 * this.e1234)
        );
    }
}
public extension Motor {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
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
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e1234 */
            (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e1234 * this.e1234)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            (other.e1234 * this.e1234) + (other.e4 * this.e4) + (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Origin other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e1234 */
            (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Point other) {
        return AntiScalar(
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
public extension Origin {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Origin {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Origin {
    public AntiScalar anti_dot_product(Origin other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Origin {
    public AntiScalar anti_dot_product(Point other) {
        return AntiScalar(
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
public extension Plane {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
public extension Plane {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
public extension Plane {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
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
public extension Point {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Point {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Point {
    public AntiScalar anti_dot_product(Origin other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
public extension Point {
    public AntiScalar anti_dot_product(Point other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}

implementing rga3d;
using traits;
public extension DualNum {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<DualNum> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<DualNum>(this);
    }
}
public extension DotProductInfixPartial<DualNum> {
    // Fancy infix trick (second half)
    public Scalar operator /(DualNum rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension DualNum {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension DualNum {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension DualNum {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension DualNum {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
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
    public Scalar operator /(Flector rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Flector {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321)
        );
    }
}
public extension Flector {
    public Scalar dot_product(Horizon other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
public extension Flector {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) + (this.e321 * other.e321)
        );
    }
}
public extension Flector {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
public extension Flector {
    public Scalar dot_product(Point other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
        );
    }
}
public extension Horizon {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<Horizon> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<Horizon>(this);
    }
}
public extension DotProductInfixPartial<Horizon> {
    // Fancy infix trick (second half)
    public Scalar operator /(Flector rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Horizon {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension Horizon {
    public Scalar dot_product(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension Horizon {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
public extension Horizon {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
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
    public Scalar operator /(Line rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Line {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
public extension Line {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
public extension Line {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
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
    public Scalar operator /(DualNum rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Motor {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Motor {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
public extension Motor {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
public extension Motor {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar)
        );
    }
}
public extension Motor {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
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
    public Scalar operator /(DualNum rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension MultiVector {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.scalar * this.scalar) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e321 * this.e321)
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
public extension MultiVector {
    public Scalar dot_product(Point other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
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
public extension Plane {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<Plane> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<Plane>(this);
    }
}
public extension DotProductInfixPartial<Plane> {
    // Fancy infix trick (second half)
    public Scalar operator /(Flector rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Plane {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension Plane {
    public Scalar dot_product(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension Plane {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension Plane {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension Point {
    // Fancy infix trick (first half)
    public DotProductInfixPartial<Point> operator /(DotProductInfix rhs) {
        return DotProductInfixPartial<Point>(this);
    }
}
public extension DotProductInfixPartial<Point> {
    // Fancy infix trick (second half)
    public Scalar operator /(Flector rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Point {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
public extension Point {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
public extension Point {
    public Scalar dot_product(Point other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
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
    public Scalar operator /(DualNum rhs) {
        return this.a.dot_product(rhs);
    }
}
public extension Scalar {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar {
    public Scalar dot_product(Motor other) {
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

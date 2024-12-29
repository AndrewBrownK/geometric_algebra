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
public extension DualNum: DotProduct<DualNum> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension DualNum: DotProduct<Motor> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension DualNum: DotProduct<MultiVector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
public extension DualNum: DotProduct<Scalar> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
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
public extension Flector: DotProduct<Flector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321)
        );
    }
}
public extension Flector: DotProduct<Horizon> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
public extension Flector: DotProduct<MultiVector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) + (this.e321 * other.e321)
        );
    }
}
public extension Flector: DotProduct<Plane> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
public extension Flector: DotProduct<Point> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Point) -> Scalar {
        return Scalar.from_groups(
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
public extension Horizon: DotProduct<Flector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension Horizon: DotProduct<Horizon> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension Horizon: DotProduct<MultiVector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
public extension Horizon: DotProduct<Plane> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Plane) -> Scalar {
        return Scalar.from_groups(
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
public extension Line: DotProduct<Line> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Line) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
public extension Line: DotProduct<Motor> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
public extension Line: DotProduct<MultiVector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
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
public extension Motor: DotProduct<DualNum> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Motor: DotProduct<Line> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Line) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
public extension Motor: DotProduct<Motor> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
public extension Motor: DotProduct<MultiVector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar)
        );
    }
}
public extension Motor: DotProduct<Scalar> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
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
public extension MultiVector: DotProduct<DualNum> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension MultiVector: DotProduct<Flector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321)
        );
    }
}
public extension MultiVector: DotProduct<Horizon> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension MultiVector: DotProduct<Line> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Line) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
public extension MultiVector: DotProduct<Motor> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
public extension MultiVector: DotProduct<MultiVector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.scalar * this.scalar) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e321 * this.e321)
        );
    }
}
public extension MultiVector: DotProduct<Plane> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
public extension MultiVector: DotProduct<Point> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
        );
    }
}
public extension MultiVector: DotProduct<Scalar> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
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
public extension Plane: DotProduct<Flector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension Plane: DotProduct<Horizon> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension Plane: DotProduct<MultiVector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
public extension Plane: DotProduct<Plane> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Plane) -> Scalar {
        return Scalar.from_groups(
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
public extension Point: DotProduct<Flector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
public extension Point: DotProduct<MultiVector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
public extension Point: DotProduct<Point> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Point) -> Scalar {
        return Scalar.from_groups(
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
public extension Scalar: DotProduct<DualNum> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar: DotProduct<Motor> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar: DotProduct<MultiVector> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
public extension Scalar: DotProduct<Scalar> {
    public typedef Scalar DotProductOutput;
    public func dot_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}

extension DualNum {
    func div(_rhs: dot_product) -> dot_product_partial<DualNum> {
        dot_product_partial(this)
    }
}
extension DualNum: DotProduct<DualNum> {
    func dot_product(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
extension DualNum: DotProduct<Motor> {
    func dot_product(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
extension DualNum: DotProduct<MultiVector> {
    func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
extension DualNum: DotProduct<Scalar> {
    func dot_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
extension Flector {
    func div(_rhs: dot_product) -> dot_product_partial<Flector> {
        dot_product_partial(this)
    }
}
extension Flector: DotProduct<Flector> {
    func dot_product(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321)
        );
    }
}
extension Flector: DotProduct<Horizon> {
    func dot_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
extension Flector: DotProduct<MultiVector> {
    func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) + (this.e321 * other.e321)
        );
    }
}
extension Flector: DotProduct<Plane> {
    func dot_product(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
extension Flector: DotProduct<Point> {
    func dot_product(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
        );
    }
}
extension Horizon {
    func div(_rhs: dot_product) -> dot_product_partial<Horizon> {
        dot_product_partial(this)
    }
}
extension Horizon: DotProduct<Flector> {
    func dot_product(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension Horizon: DotProduct<Horizon> {
    func dot_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension Horizon: DotProduct<MultiVector> {
    func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
extension Horizon: DotProduct<Plane> {
    func dot_product(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
extension Line {
    func div(_rhs: dot_product) -> dot_product_partial<Line> {
        dot_product_partial(this)
    }
}
extension Line: DotProduct<Line> {
    func dot_product(other: Line) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
extension Line: DotProduct<Motor> {
    func dot_product(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
extension Line: DotProduct<MultiVector> {
    func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
extension Motor {
    func div(_rhs: dot_product) -> dot_product_partial<Motor> {
        dot_product_partial(this)
    }
}
extension Motor: DotProduct<DualNum> {
    func dot_product(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
extension Motor: DotProduct<Line> {
    func dot_product(other: Line) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
extension Motor: DotProduct<Motor> {
    func dot_product(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
extension Motor: DotProduct<MultiVector> {
    func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar)
        );
    }
}
extension Motor: DotProduct<Scalar> {
    func dot_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
extension MultiVector {
    func div(_rhs: dot_product) -> dot_product_partial<MultiVector> {
        dot_product_partial(this)
    }
}
extension MultiVector: DotProduct<DualNum> {
    func dot_product(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
extension MultiVector: DotProduct<Flector> {
    func dot_product(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321)
        );
    }
}
extension MultiVector: DotProduct<Horizon> {
    func dot_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension MultiVector: DotProduct<Line> {
    func dot_product(other: Line) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
extension MultiVector: DotProduct<Motor> {
    func dot_product(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
extension MultiVector: DotProduct<MultiVector> {
    func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.scalar * this.scalar) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e321 * this.e321)
        );
    }
}
extension MultiVector: DotProduct<Plane> {
    func dot_product(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
extension MultiVector: DotProduct<Point> {
    func dot_product(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
        );
    }
}
extension MultiVector: DotProduct<Scalar> {
    func dot_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
extension Plane {
    func div(_rhs: dot_product) -> dot_product_partial<Plane> {
        dot_product_partial(this)
    }
}
extension Plane: DotProduct<Flector> {
    func dot_product(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension Plane: DotProduct<Horizon> {
    func dot_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension Plane: DotProduct<MultiVector> {
    func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension Plane: DotProduct<Plane> {
    func dot_product(other: Plane) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension Point {
    func div(_rhs: dot_product) -> dot_product_partial<Point> {
        dot_product_partial(this)
    }
}
extension Point: DotProduct<Flector> {
    func dot_product(other: Flector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
extension Point: DotProduct<MultiVector> {
    func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
extension Point: DotProduct<Point> {
    func dot_product(other: Point) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
extension Scalar {
    func div(_rhs: dot_product) -> dot_product_partial<Scalar> {
        dot_product_partial(this)
    }
}
extension Scalar: DotProduct<DualNum> {
    func dot_product(other: DualNum) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
extension Scalar: DotProduct<Motor> {
    func dot_product(other: Motor) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
extension Scalar: DotProduct<MultiVector> {
    func dot_product(other: MultiVector) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
extension Scalar: DotProduct<Scalar> {
    func dot_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}

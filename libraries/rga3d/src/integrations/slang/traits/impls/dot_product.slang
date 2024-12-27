extension DualNum: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
    func operator/(_rhs: DotProductInfix) -> dot_product_partial<DualNum> {
        dot_product_partial(this)
    }
}
extension DualNum: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
extension DualNum: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
extension DualNum: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
extension Flector: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321)
        );
    }
    func operator/(_rhs: DotProductInfix) -> dot_product_partial<Flector> {
        dot_product_partial(this)
    }
}
extension Flector: DotProduct<Horizon> {
    associatedtype Output = Scalar;
    func dot_product(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
extension Flector: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) + (this.e321 * other.e321)
        );
    }
}
extension Flector: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
extension Flector: DotProduct<Point> {
    associatedtype Output = Scalar;
    func dot_product(other: Point) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
        );
    }
}
extension Horizon: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
    func operator/(_rhs: DotProductInfix) -> dot_product_partial<Horizon> {
        dot_product_partial(this)
    }
}
extension Horizon: DotProduct<Horizon> {
    associatedtype Output = Scalar;
    func dot_product(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension Horizon: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
extension Horizon: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
extension Line: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
    func operator/(_rhs: DotProductInfix) -> dot_product_partial<Line> {
        dot_product_partial(this)
    }
}
extension Line: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
extension Line: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
extension Motor: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
    func operator/(_rhs: DotProductInfix) -> dot_product_partial<Motor> {
        dot_product_partial(this)
    }
}
extension Motor: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
extension Motor: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
extension Motor: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar)
        );
    }
}
extension Motor: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
extension MultiVector: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
    func operator/(_rhs: DotProductInfix) -> dot_product_partial<MultiVector> {
        dot_product_partial(this)
    }
}
extension MultiVector: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321)
        );
    }
}
extension MultiVector: DotProduct<Horizon> {
    associatedtype Output = Scalar;
    func dot_product(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension MultiVector: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
extension MultiVector: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
extension MultiVector: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (other.scalar * this.scalar) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e321 * this.e321)
        );
    }
}
extension MultiVector: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
extension MultiVector: DotProduct<Point> {
    associatedtype Output = Scalar;
    func dot_product(other: Point) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
        );
    }
}
extension MultiVector: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
extension Plane: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
    func operator/(_rhs: DotProductInfix) -> dot_product_partial<Plane> {
        dot_product_partial(this)
    }
}
extension Plane: DotProduct<Horizon> {
    associatedtype Output = Scalar;
    func dot_product(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension Plane: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension Plane: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
extension Point: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
    func operator/(_rhs: DotProductInfix) -> dot_product_partial<Point> {
        dot_product_partial(this)
    }
}
extension Point: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
extension Point: DotProduct<Point> {
    associatedtype Output = Scalar;
    func dot_product(other: Point) -> Output {
        return Scalar.from_groups(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
extension Scalar: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
    func operator/(_rhs: DotProductInfix) -> dot_product_partial<Scalar> {
        dot_product_partial(this)
    }
}
extension Scalar: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
extension Scalar: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
extension Scalar: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}

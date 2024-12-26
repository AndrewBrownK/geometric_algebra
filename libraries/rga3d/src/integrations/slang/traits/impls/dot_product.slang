extension DualNum {
    func div(_rhs: dot_product) -> dot_product_partial<DualNum> {
        dot_product_partial(this)
    }
}
extension DualNum: DotProduct<DualNum> {
    func dot_product(self, other: DualNum) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[scalar] * self[scalar]
        );
    }
}
extension DualNum: DotProduct<Motor> {
    func dot_product(self, other: Motor) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] * other[scalar]
        );
    }
}
extension DualNum: DotProduct<MultiVector> {
    func dot_product(self, other: MultiVector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] * other[scalar]
        );
    }
}
extension DualNum: DotProduct<Scalar> {
    func dot_product(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] * other[scalar]
        );
    }
}
extension Flector {
    func div(_rhs: dot_product) -> dot_product_partial<Flector> {
        dot_product_partial(this)
    }
}
extension Flector: DotProduct<Flector> {
    func dot_product(self, other: Flector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) + (other[e321] * self[e321])
        );
    }
}
extension Flector: DotProduct<Horizon> {
    func dot_product(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e321] * other[e321]
        );
    }
}
extension Flector: DotProduct<MultiVector> {
    func dot_product(self, other: MultiVector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]) + (self[e321] * other[e321])
        );
    }
}
extension Flector: DotProduct<Plane> {
    func dot_product(self, other: Plane) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e321] * other[e321]
        );
    }
}
extension Flector: DotProduct<Point> {
    func dot_product(self, other: Point) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3])
        );
    }
}
extension Horizon {
    func div(_rhs: dot_product) -> dot_product_partial<Horizon> {
        dot_product_partial(this)
    }
}
extension Horizon: DotProduct<Flector> {
    func dot_product(self, other: Flector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e321]
        );
    }
}
extension Horizon: DotProduct<Horizon> {
    func dot_product(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e321]
        );
    }
}
extension Horizon: DotProduct<MultiVector> {
    func dot_product(self, other: MultiVector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e321] * other[e321]
        );
    }
}
extension Horizon: DotProduct<Plane> {
    func dot_product(self, other: Plane) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e321] * other[e321]
        );
    }
}
extension Line {
    func div(_rhs: dot_product) -> dot_product_partial<Line> {
        dot_product_partial(this)
    }
}
extension Line: DotProduct<Line> {
    func dot_product(self, other: Line) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12])
        );
    }
}
extension Line: DotProduct<Motor> {
    func dot_product(self, other: Motor) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12])
        );
    }
}
extension Line: DotProduct<MultiVector> {
    func dot_product(self, other: MultiVector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12])
        );
    }
}
extension Motor {
    func div(_rhs: dot_product) -> dot_product_partial<Motor> {
        dot_product_partial(this)
    }
}
extension Motor: DotProduct<DualNum> {
    func dot_product(self, other: DualNum) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[scalar] * self[scalar]
        );
    }
}
extension Motor: DotProduct<Line> {
    func dot_product(self, other: Line) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12])
        );
    }
}
extension Motor: DotProduct<Motor> {
    func dot_product(self, other: Motor) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[scalar] * self[scalar])
        );
    }
}
extension Motor: DotProduct<MultiVector> {
    func dot_product(self, other: MultiVector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[scalar] * other[scalar])
        );
    }
}
extension Motor: DotProduct<Scalar> {
    func dot_product(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] * other[scalar]
        );
    }
}
extension MultiVector {
    func div(_rhs: dot_product) -> dot_product_partial<MultiVector> {
        dot_product_partial(this)
    }
}
extension MultiVector: DotProduct<DualNum> {
    func dot_product(self, other: DualNum) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[scalar] * self[scalar]
        );
    }
}
extension MultiVector: DotProduct<Flector> {
    func dot_product(self, other: Flector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) + (other[e321] * self[e321])
        );
    }
}
extension MultiVector: DotProduct<Horizon> {
    func dot_product(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e321]
        );
    }
}
extension MultiVector: DotProduct<Line> {
    func dot_product(self, other: Line) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12])
        );
    }
}
extension MultiVector: DotProduct<Motor> {
    func dot_product(self, other: Motor) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[scalar] * self[scalar])
        );
    }
}
extension MultiVector: DotProduct<MultiVector> {
    func dot_product(self, other: MultiVector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[scalar] * self[scalar]) + (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e321] * self[e321])
        );
    }
}
extension MultiVector: DotProduct<Plane> {
    func dot_product(self, other: Plane) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e321] * other[e321]
        );
    }
}
extension MultiVector: DotProduct<Point> {
    func dot_product(self, other: Point) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3])
        );
    }
}
extension MultiVector: DotProduct<Scalar> {
    func dot_product(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] * other[scalar]
        );
    }
}
extension Plane {
    func div(_rhs: dot_product) -> dot_product_partial<Plane> {
        dot_product_partial(this)
    }
}
extension Plane: DotProduct<Flector> {
    func dot_product(self, other: Flector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e321]
        );
    }
}
extension Plane: DotProduct<Horizon> {
    func dot_product(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e321]
        );
    }
}
extension Plane: DotProduct<MultiVector> {
    func dot_product(self, other: MultiVector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e321]
        );
    }
}
extension Plane: DotProduct<Plane> {
    func dot_product(self, other: Plane) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e321]
        );
    }
}
extension Point {
    func div(_rhs: dot_product) -> dot_product_partial<Point> {
        dot_product_partial(this)
    }
}
extension Point: DotProduct<Flector> {
    func dot_product(self, other: Flector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])
        );
    }
}
extension Point: DotProduct<MultiVector> {
    func dot_product(self, other: MultiVector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])
        );
    }
}
extension Point: DotProduct<Point> {
    func dot_product(self, other: Point) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])
        );
    }
}
extension Scalar {
    func div(_rhs: dot_product) -> dot_product_partial<Scalar> {
        dot_product_partial(this)
    }
}
extension Scalar: DotProduct<DualNum> {
    func dot_product(self, other: DualNum) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[scalar] * self[scalar]
        );
    }
}
extension Scalar: DotProduct<Motor> {
    func dot_product(self, other: Motor) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[scalar] * self[scalar]
        );
    }
}
extension Scalar: DotProduct<MultiVector> {
    func dot_product(self, other: MultiVector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[scalar] * self[scalar]
        );
    }
}
extension Scalar: DotProduct<Scalar> {
    func dot_product(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[scalar] * self[scalar]
        );
    }
}

extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<DualNum> {
        return DotProductInfixPartial { a: this };
    }
}
extension DualNum: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Flector> {
        return DotProductInfixPartial { a: this };
    }
}
extension Flector: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<Horizon> {
    associatedtype Output = Scalar;
    func dot_product(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) + (this.e321 * other.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector: DotProduct<Point> {
    associatedtype Output = Scalar;
    func dot_product(other: Point) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Horizon> {
        return DotProductInfixPartial { a: this };
    }
}
extension Horizon: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: DotProduct<Horizon> {
    associatedtype Output = Scalar;
    func dot_product(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Horizon: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Line> {
        return DotProductInfixPartial { a: this };
    }
}
extension Line: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension Line: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Motor> {
        return DotProductInfixPartial { a: this };
    }
}
extension Motor: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension Motor: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<MultiVector> {
        return DotProductInfixPartial { a: this };
    }
}
extension MultiVector: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Horizon> {
    associatedtype Output = Scalar;
    func dot_product(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Line> {
    associatedtype Output = Scalar;
    func dot_product(other: Line) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.scalar * this.scalar) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e321 * this.e321), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e321 * other.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Point> {
    associatedtype Output = Scalar;
    func dot_product(other: Point) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension MultiVector: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar * other.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Plane> {
        return DotProductInfixPartial { a: this };
    }
}
extension Plane: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: DotProduct<Horizon> {
    associatedtype Output = Scalar;
    func dot_product(other: Horizon) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: DotProduct<Plane> {
    associatedtype Output = Scalar;
    func dot_product(other: Plane) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.e321 * this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Point> {
        return DotProductInfixPartial { a: this };
    }
}
extension Point: DotProduct<Flector> {
    associatedtype Output = Scalar;
    func dot_product(other: Flector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension Point: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension Point: DotProduct<Point> {
    associatedtype Output = Scalar;
    func dot_product(other: Point) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: DotProductInfix) -> DotProductInfixPartial<Scalar> {
        return DotProductInfixPartial { a: this };
    }
}
extension Scalar: DotProduct<DualNum> {
    associatedtype Output = Scalar;
    func dot_product(other: DualNum) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: DotProduct<Motor> {
    associatedtype Output = Scalar;
    func dot_product(other: Motor) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: DotProduct<MultiVector> {
    associatedtype Output = Scalar;
    func dot_product(other: MultiVector) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: DotProduct<Scalar> {
    associatedtype Output = Scalar;
    func dot_product(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(other.scalar * this.scalar, 0.0, 0.0, 0.0)
        );
    }
}

extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<AntiScalar> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension AntiScalar: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension AntiScalar: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
extension AntiScalar: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
extension AntiScalar: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<DualNum> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension DualNum: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension DualNum: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension DualNum: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
extension DualNum: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Flector> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Flector: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension Flector: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e4 * other.e4) + (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
extension Flector: AntiDotProduct<Origin> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Origin) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
extension Flector: AntiDotProduct<Plane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
extension Flector: AntiDotProduct<Point> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Point) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Line> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Line: AntiDotProduct<Line> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
extension Line: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43)
        );
    }
}
extension Line: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Motor> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Motor: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension Motor: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension Motor: AntiDotProduct<Line> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
extension Motor: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e1234 * this.e1234)
        );
    }
}
extension Motor: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43) + (this.e1234 * other.e1234)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<MultiVector> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension MultiVector: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension MultiVector: AntiDotProduct<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension MultiVector: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension MultiVector: AntiDotProduct<Line> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
extension MultiVector: AntiDotProduct<Motor> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Motor) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e1234 * this.e1234)
        );
    }
}
extension MultiVector: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e1234 * this.e1234) + (other.e4 * this.e4) + (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension MultiVector: AntiDotProduct<Origin> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Origin) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
extension MultiVector: AntiDotProduct<Plane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
extension MultiVector: AntiDotProduct<Point> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Point) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Origin> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Origin: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Origin: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Origin: AntiDotProduct<Origin> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Origin) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Origin: AntiDotProduct<Point> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Point) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Plane> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Plane: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension Plane: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension Plane: AntiDotProduct<Plane> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: AntiDotProductInfix) -> AntiDotProductInfixPartial<Point> {
        return AntiDotProductInfixPartial { a: this };
    }
}
extension Point: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Point: AntiDotProduct<MultiVector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: MultiVector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Point: AntiDotProduct<Origin> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Origin) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Point: AntiDotProduct<Point> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Point) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}

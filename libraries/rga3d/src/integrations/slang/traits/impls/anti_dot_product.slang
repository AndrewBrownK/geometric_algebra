extension AntiScalar: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
    func operator/(_rhs: AntiDotProductInfix) -> anti_dot_product_partial<AntiScalar> {
        anti_dot_product_partial(this)
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
extension DualNum: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
    func operator/(_rhs: AntiDotProductInfix) -> anti_dot_product_partial<DualNum> {
        anti_dot_product_partial(this)
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
extension Flector: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
    func operator/(_rhs: AntiDotProductInfix) -> anti_dot_product_partial<Flector> {
        anti_dot_product_partial(this)
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
extension Line: AntiDotProduct<Line> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Line) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
    func operator/(_rhs: AntiDotProductInfix) -> anti_dot_product_partial<Line> {
        anti_dot_product_partial(this)
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
extension Motor: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
    func operator/(_rhs: AntiDotProductInfix) -> anti_dot_product_partial<Motor> {
        anti_dot_product_partial(this)
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
extension MultiVector: AntiDotProduct<AntiScalar> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
    func operator/(_rhs: AntiDotProductInfix) -> anti_dot_product_partial<MultiVector> {
        anti_dot_product_partial(this)
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
extension Origin: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
    func operator/(_rhs: AntiDotProductInfix) -> anti_dot_product_partial<Origin> {
        anti_dot_product_partial(this)
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
extension Plane: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
    func operator/(_rhs: AntiDotProductInfix) -> anti_dot_product_partial<Plane> {
        anti_dot_product_partial(this)
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
extension Point: AntiDotProduct<Flector> {
    associatedtype Output = AntiScalar;
    func anti_dot_product(other: Flector) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
    func operator/(_rhs: AntiDotProductInfix) -> anti_dot_product_partial<Point> {
        anti_dot_product_partial(this)
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

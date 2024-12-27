extension AntiScalar {
    func div(_rhs: anti_dot_product) -> anti_dot_product_partial<AntiScalar> {
        anti_dot_product_partial(this)
    }
}
extension AntiScalar: AntiDotProduct<AntiScalar> {
    func anti_dot_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension AntiScalar: AntiDotProduct<DualNum> {
    func anti_dot_product(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
extension AntiScalar: AntiDotProduct<Motor> {
    func anti_dot_product(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
extension AntiScalar: AntiDotProduct<MultiVector> {
    func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
extension DualNum {
    func div(_rhs: anti_dot_product) -> anti_dot_product_partial<DualNum> {
        anti_dot_product_partial(this)
    }
}
extension DualNum: AntiDotProduct<AntiScalar> {
    func anti_dot_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension DualNum: AntiDotProduct<DualNum> {
    func anti_dot_product(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension DualNum: AntiDotProduct<Motor> {
    func anti_dot_product(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
extension DualNum: AntiDotProduct<MultiVector> {
    func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
extension Flector {
    func div(_rhs: anti_dot_product) -> anti_dot_product_partial<Flector> {
        anti_dot_product_partial(this)
    }
}
extension Flector: AntiDotProduct<Flector> {
    func anti_dot_product(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension Flector: AntiDotProduct<MultiVector> {
    func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e4 * other.e4) + (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
extension Flector: AntiDotProduct<Origin> {
    func anti_dot_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
extension Flector: AntiDotProduct<Plane> {
    func anti_dot_product(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
extension Flector: AntiDotProduct<Point> {
    func anti_dot_product(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
extension Line {
    func div(_rhs: anti_dot_product) -> anti_dot_product_partial<Line> {
        anti_dot_product_partial(this)
    }
}
extension Line: AntiDotProduct<Line> {
    func anti_dot_product(other: Line) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
extension Line: AntiDotProduct<Motor> {
    func anti_dot_product(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43)
        );
    }
}
extension Line: AntiDotProduct<MultiVector> {
    func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43)
        );
    }
}
extension Motor {
    func div(_rhs: anti_dot_product) -> anti_dot_product_partial<Motor> {
        anti_dot_product_partial(this)
    }
}
extension Motor: AntiDotProduct<AntiScalar> {
    func anti_dot_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension Motor: AntiDotProduct<DualNum> {
    func anti_dot_product(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension Motor: AntiDotProduct<Line> {
    func anti_dot_product(other: Line) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
extension Motor: AntiDotProduct<Motor> {
    func anti_dot_product(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e1234 * this.e1234)
        );
    }
}
extension Motor: AntiDotProduct<MultiVector> {
    func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43) + (this.e1234 * other.e1234)
        );
    }
}
extension MultiVector {
    func div(_rhs: anti_dot_product) -> anti_dot_product_partial<MultiVector> {
        anti_dot_product_partial(this)
    }
}
extension MultiVector: AntiDotProduct<AntiScalar> {
    func anti_dot_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension MultiVector: AntiDotProduct<DualNum> {
    func anti_dot_product(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
extension MultiVector: AntiDotProduct<Flector> {
    func anti_dot_product(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension MultiVector: AntiDotProduct<Line> {
    func anti_dot_product(other: Line) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
extension MultiVector: AntiDotProduct<Motor> {
    func anti_dot_product(other: Motor) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e1234 * this.e1234)
        );
    }
}
extension MultiVector: AntiDotProduct<MultiVector> {
    func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e1234 * this.e1234) + (other.e4 * this.e4) + (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension MultiVector: AntiDotProduct<Origin> {
    func anti_dot_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
extension MultiVector: AntiDotProduct<Plane> {
    func anti_dot_product(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
extension MultiVector: AntiDotProduct<Point> {
    func anti_dot_product(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
extension Origin {
    func div(_rhs: anti_dot_product) -> anti_dot_product_partial<Origin> {
        anti_dot_product_partial(this)
    }
}
extension Origin: AntiDotProduct<Flector> {
    func anti_dot_product(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Origin: AntiDotProduct<MultiVector> {
    func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Origin: AntiDotProduct<Origin> {
    func anti_dot_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Origin: AntiDotProduct<Point> {
    func anti_dot_product(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
extension Plane {
    func div(_rhs: anti_dot_product) -> anti_dot_product_partial<Plane> {
        anti_dot_product_partial(this)
    }
}
extension Plane: AntiDotProduct<Flector> {
    func anti_dot_product(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension Plane: AntiDotProduct<MultiVector> {
    func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension Plane: AntiDotProduct<Plane> {
    func anti_dot_product(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
extension Point {
    func div(_rhs: anti_dot_product) -> anti_dot_product_partial<Point> {
        anti_dot_product_partial(this)
    }
}
extension Point: AntiDotProduct<Flector> {
    func anti_dot_product(other: Flector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Point: AntiDotProduct<MultiVector> {
    func anti_dot_product(other: MultiVector) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Point: AntiDotProduct<Origin> {
    func anti_dot_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
extension Point: AntiDotProduct<Point> {
    func anti_dot_product(other: Point) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}

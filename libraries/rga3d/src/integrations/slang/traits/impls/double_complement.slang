extension AntiScalar: DoubleComplement {
    associatedtype Output = AntiScalar;
    func double_complement() -> Output {
        return this;
    }
    func operator/(_rhs: DoubleComplementInfix) -> AntiScalar {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementInfix& _rhs) -> AntiScalar& {
        this = this.double_complement();
        return *this;
    }
}
extension DualNum: DoubleComplement {
    associatedtype Output = DualNum;
    func double_complement() -> Output {
        return this;
    }
    func operator/(_rhs: DoubleComplementInfix) -> DualNum {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementInfix& _rhs) -> DualNum& {
        this = this.double_complement();
        return *this;
    }
}
extension Flector: DoubleComplement {
    associatedtype Output = Flector;
    func double_complement() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
    func operator/(_rhs: DoubleComplementInfix) -> Flector {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementInfix& _rhs) -> Flector& {
        this = this.double_complement();
        return *this;
    }
}
extension Horizon: DoubleComplement {
    associatedtype Output = Horizon;
    func double_complement() -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * -1.0
        );
    }
    func operator/(_rhs: DoubleComplementInfix) -> Horizon {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementInfix& _rhs) -> Horizon& {
        this = this.double_complement();
        return *this;
    }
}
extension Line: DoubleComplement {
    associatedtype Output = Line;
    func double_complement() -> Output {
        return this;
    }
    func operator/(_rhs: DoubleComplementInfix) -> Line {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementInfix& _rhs) -> Line& {
        this = this.double_complement();
        return *this;
    }
}
extension Motor: DoubleComplement {
    associatedtype Output = Motor;
    func double_complement() -> Output {
        return this;
    }
    func operator/(_rhs: DoubleComplementInfix) -> Motor {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementInfix& _rhs) -> Motor& {
        this = this.double_complement();
        return *this;
    }
}
extension MultiVector: DoubleComplement {
    associatedtype Output = MultiVector;
    func double_complement() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2, 
            /* e23, e31, e12 */
            this.group3, 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
    func operator/(_rhs: DoubleComplementInfix) -> MultiVector {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementInfix& _rhs) -> MultiVector& {
        this = this.double_complement();
        return *this;
    }
}
extension Origin: DoubleComplement {
    associatedtype Output = Origin;
    func double_complement() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
    func operator/(_rhs: DoubleComplementInfix) -> Origin {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementInfix& _rhs) -> Origin& {
        this = this.double_complement();
        return *this;
    }
}
extension Plane: DoubleComplement {
    associatedtype Output = Plane;
    func double_complement() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
    func operator/(_rhs: DoubleComplementInfix) -> Plane {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementInfix& _rhs) -> Plane& {
        this = this.double_complement();
        return *this;
    }
}
extension Point: DoubleComplement {
    associatedtype Output = Point;
    func double_complement() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
    func operator/(_rhs: DoubleComplementInfix) -> Point {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementInfix& _rhs) -> Point& {
        this = this.double_complement();
        return *this;
    }
}
extension Scalar: DoubleComplement {
    associatedtype Output = Scalar;
    func double_complement() -> Output {
        return this;
    }
    func operator/(_rhs: DoubleComplementInfix) -> Scalar {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementInfix& _rhs) -> Scalar& {
        this = this.double_complement();
        return *this;
    }
}

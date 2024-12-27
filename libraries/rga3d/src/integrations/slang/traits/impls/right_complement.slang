extension AntiScalar: RightComplement {
    associatedtype Output = Scalar;
    func right_complement() -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.e1234
        );
    }
    func operator/(_rhs: RightComplementInfix) -> Scalar {
        return this.right_complement();
    }
}
extension DualNum: RightComplement {
    associatedtype Output = DualNum;
    func right_complement() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            this.group0.yx
        );
    }
    func operator/(_rhs: RightComplementInfix) -> DualNum {
        return this.right_complement();
    }
    func operator/=(const RightComplementInfix& _rhs) -> DualNum& {
        this = this.right_complement();
        return *this;
    }
}
extension Flector: RightComplement {
    associatedtype Output = Flector;
    func right_complement() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
    func operator/(_rhs: RightComplementInfix) -> Flector {
        return this.right_complement();
    }
    func operator/=(const RightComplementInfix& _rhs) -> Flector& {
        this = this.right_complement();
        return *this;
    }
}
extension Horizon: RightComplement {
    associatedtype Output = Origin;
    func right_complement() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e321 * -1.0
        );
    }
    func operator/(_rhs: RightComplementInfix) -> Origin {
        return this.right_complement();
    }
}
extension Line: RightComplement {
    associatedtype Output = Line;
    func right_complement() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group0 * float3(-1.0)
        );
    }
    func operator/(_rhs: RightComplementInfix) -> Line {
        return this.right_complement();
    }
    func operator/=(const RightComplementInfix& _rhs) -> Line& {
        this = this.right_complement();
        return *this;
    }
}
extension Motor: RightComplement {
    associatedtype Output = Motor;
    func right_complement() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
    func operator/(_rhs: RightComplementInfix) -> Motor {
        return this.right_complement();
    }
    func operator/=(const RightComplementInfix& _rhs) -> Motor& {
        this = this.right_complement();
        return *this;
    }
}
extension MultiVector: RightComplement {
    associatedtype Output = MultiVector;
    func right_complement() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.yx, 
            /* e1, e2, e3, e4 */
            this.group4 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group2 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
    func operator/(_rhs: RightComplementInfix) -> MultiVector {
        return this.right_complement();
    }
    func operator/=(const RightComplementInfix& _rhs) -> MultiVector& {
        this = this.right_complement();
        return *this;
    }
}
extension Origin: RightComplement {
    associatedtype Output = Horizon;
    func right_complement() -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e4
        );
    }
    func operator/(_rhs: RightComplementInfix) -> Horizon {
        return this.right_complement();
    }
}
extension Plane: RightComplement {
    associatedtype Output = Point;
    func right_complement() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
    func operator/(_rhs: RightComplementInfix) -> Point {
        return this.right_complement();
    }
}
extension Point: RightComplement {
    associatedtype Output = Plane;
    func right_complement() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
    func operator/(_rhs: RightComplementInfix) -> Plane {
        return this.right_complement();
    }
}
extension Scalar: RightComplement {
    associatedtype Output = AntiScalar;
    func right_complement() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
    func operator/(_rhs: RightComplementInfix) -> AntiScalar {
        return this.right_complement();
    }
}

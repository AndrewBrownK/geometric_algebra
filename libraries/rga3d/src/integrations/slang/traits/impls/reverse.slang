extension AntiScalar: Reverse {
    associatedtype Output = AntiScalar;
    func reverse() -> Output {
        return this;
    }
    func operator/(_rhs: ReverseInfix) -> AntiScalar {
        return this.reverse();
    }
    func operator/=(const ReverseInfix& _rhs) -> AntiScalar& {
        this = this.reverse();
        return *this;
    }
}
extension DualNum: Reverse {
    associatedtype Output = DualNum;
    func reverse() -> Output {
        return this;
    }
    func operator/(_rhs: ReverseInfix) -> DualNum {
        return this.reverse();
    }
    func operator/=(const ReverseInfix& _rhs) -> DualNum& {
        this = this.reverse();
        return *this;
    }
}
extension Flector: Reverse {
    associatedtype Output = Flector;
    func reverse() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
    func operator/(_rhs: ReverseInfix) -> Flector {
        return this.reverse();
    }
    func operator/=(const ReverseInfix& _rhs) -> Flector& {
        this = this.reverse();
        return *this;
    }
}
extension Horizon: Reverse {
    associatedtype Output = Horizon;
    func reverse() -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * -1.0
        );
    }
    func operator/(_rhs: ReverseInfix) -> Horizon {
        return this.reverse();
    }
    func operator/=(const ReverseInfix& _rhs) -> Horizon& {
        this = this.reverse();
        return *this;
    }
}
extension Line: Reverse {
    associatedtype Output = Line;
    func reverse() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1 * float3(-1.0)
        );
    }
    func operator/(_rhs: ReverseInfix) -> Line {
        return this.reverse();
    }
    func operator/=(const ReverseInfix& _rhs) -> Line& {
        this = this.reverse();
        return *this;
    }
}
extension Motor: Reverse {
    associatedtype Output = Motor;
    func reverse() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
    func operator/(_rhs: ReverseInfix) -> Motor {
        return this.reverse();
    }
    func operator/=(const ReverseInfix& _rhs) -> Motor& {
        this = this.reverse();
        return *this;
    }
}
extension MultiVector: Reverse {
    associatedtype Output = MultiVector;
    func reverse() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
    func operator/(_rhs: ReverseInfix) -> MultiVector {
        return this.reverse();
    }
    func operator/=(const ReverseInfix& _rhs) -> MultiVector& {
        this = this.reverse();
        return *this;
    }
}
extension Origin: Reverse {
    associatedtype Output = Origin;
    func reverse() -> Output {
        return this;
    }
    func operator/(_rhs: ReverseInfix) -> Origin {
        return this.reverse();
    }
    func operator/=(const ReverseInfix& _rhs) -> Origin& {
        this = this.reverse();
        return *this;
    }
}
extension Plane: Reverse {
    associatedtype Output = Plane;
    func reverse() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
    func operator/(_rhs: ReverseInfix) -> Plane {
        return this.reverse();
    }
    func operator/=(const ReverseInfix& _rhs) -> Plane& {
        this = this.reverse();
        return *this;
    }
}
extension Point: Reverse {
    associatedtype Output = Point;
    func reverse() -> Output {
        return this;
    }
    func operator/(_rhs: ReverseInfix) -> Point {
        return this.reverse();
    }
    func operator/=(const ReverseInfix& _rhs) -> Point& {
        this = this.reverse();
        return *this;
    }
}
extension Scalar: Reverse {
    associatedtype Output = Scalar;
    func reverse() -> Output {
        return this;
    }
    func operator/(_rhs: ReverseInfix) -> Scalar {
        return this.reverse();
    }
    func operator/=(const ReverseInfix& _rhs) -> Scalar& {
        this = this.reverse();
        return *this;
    }
}

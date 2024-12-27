extension AntiScalar: AntiReverse {
    associatedtype Output = AntiScalar;
    func anti_reverse() -> Output {
        return this;
    }
    func operator/(_rhs: AntiReverseInfix) -> AntiScalar {
        return this.anti_reverse();
    }
    func operator/=(const AntiReverseInfix& _rhs) -> AntiScalar& {
        this = this.anti_reverse();
        return *this;
    }
}
extension DualNum: AntiReverse {
    associatedtype Output = DualNum;
    func anti_reverse() -> Output {
        return this;
    }
    func operator/(_rhs: AntiReverseInfix) -> DualNum {
        return this.anti_reverse();
    }
    func operator/=(const AntiReverseInfix& _rhs) -> DualNum& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Flector: AntiReverse {
    associatedtype Output = Flector;
    func anti_reverse() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
    func operator/(_rhs: AntiReverseInfix) -> Flector {
        return this.anti_reverse();
    }
    func operator/=(const AntiReverseInfix& _rhs) -> Flector& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Horizon: AntiReverse {
    associatedtype Output = Horizon;
    func anti_reverse() -> Output {
        return this;
    }
    func operator/(_rhs: AntiReverseInfix) -> Horizon {
        return this.anti_reverse();
    }
    func operator/=(const AntiReverseInfix& _rhs) -> Horizon& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Line: AntiReverse {
    associatedtype Output = Line;
    func anti_reverse() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1 * float3(-1.0)
        );
    }
    func operator/(_rhs: AntiReverseInfix) -> Line {
        return this.anti_reverse();
    }
    func operator/=(const AntiReverseInfix& _rhs) -> Line& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Motor: AntiReverse {
    associatedtype Output = Motor;
    func anti_reverse() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
    func operator/(_rhs: AntiReverseInfix) -> Motor {
        return this.anti_reverse();
    }
    func operator/=(const AntiReverseInfix& _rhs) -> Motor& {
        this = this.anti_reverse();
        return *this;
    }
}
extension MultiVector: AntiReverse {
    associatedtype Output = MultiVector;
    func anti_reverse() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
    func operator/(_rhs: AntiReverseInfix) -> MultiVector {
        return this.anti_reverse();
    }
    func operator/=(const AntiReverseInfix& _rhs) -> MultiVector& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Origin: AntiReverse {
    associatedtype Output = Origin;
    func anti_reverse() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
    func operator/(_rhs: AntiReverseInfix) -> Origin {
        return this.anti_reverse();
    }
    func operator/=(const AntiReverseInfix& _rhs) -> Origin& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Plane: AntiReverse {
    associatedtype Output = Plane;
    func anti_reverse() -> Output {
        return this;
    }
    func operator/(_rhs: AntiReverseInfix) -> Plane {
        return this.anti_reverse();
    }
    func operator/=(const AntiReverseInfix& _rhs) -> Plane& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Point: AntiReverse {
    associatedtype Output = Point;
    func anti_reverse() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
    func operator/(_rhs: AntiReverseInfix) -> Point {
        return this.anti_reverse();
    }
    func operator/=(const AntiReverseInfix& _rhs) -> Point& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Scalar: AntiReverse {
    associatedtype Output = Scalar;
    func anti_reverse() -> Output {
        return this;
    }
    func operator/(_rhs: AntiReverseInfix) -> Scalar {
        return this.anti_reverse();
    }
    func operator/=(const AntiReverseInfix& _rhs) -> Scalar& {
        this = this.anti_reverse();
        return *this;
    }
}

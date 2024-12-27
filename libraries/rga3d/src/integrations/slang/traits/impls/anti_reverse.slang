extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> AntiScalar {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.anti_reverse();
        return *this;
    }
}
extension AntiScalar: AntiReverse {
    associatedtype Output = AntiScalar;
    func anti_reverse() -> Output {
        return this;
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> DualNum {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.anti_reverse();
        return *this;
    }
}
extension DualNum: AntiReverse {
    associatedtype Output = DualNum;
    func anti_reverse() -> Output {
        return this;
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Flector {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Flector& {
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
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Horizon {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Horizon& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Horizon: AntiReverse {
    associatedtype Output = Horizon;
    func anti_reverse() -> Output {
        return this;
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Line {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Line& {
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
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Motor {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Motor& {
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
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> MultiVector {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> MultiVector& {
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
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Origin {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Origin& {
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
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Plane {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Plane& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Plane: AntiReverse {
    associatedtype Output = Plane;
    func anti_reverse() -> Output {
        return this;
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Point {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Point& {
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
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiReversePrefixOrPostfix) -> Scalar {
        return this.anti_reverse();
    }
    func operator/=(const AntiReversePrefixOrPostfix& _rhs) -> Scalar& {
        this = this.anti_reverse();
        return *this;
    }
}
extension Scalar: AntiReverse {
    associatedtype Output = Scalar;
    func anti_reverse() -> Output {
        return this;
    }
}

extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> AntiScalar {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.reverse();
        return *this;
    }
}
extension AntiScalar: Reverse {
    associatedtype Output = AntiScalar;
    func reverse() -> Output {
        return this;
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> DualNum {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.reverse();
        return *this;
    }
}
extension DualNum: Reverse {
    associatedtype Output = DualNum;
    func reverse() -> Output {
        return this;
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Flector {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Flector& {
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
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Horizon {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Horizon& {
        this = this.reverse();
        return *this;
    }
}
extension Horizon: Reverse {
    associatedtype Output = Horizon;
    func reverse() -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Line {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Line& {
        this = this.reverse();
        return *this;
    }
}
extension Line: Reverse {
    associatedtype Output = Line;
    func reverse() -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Motor {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Motor& {
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
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> MultiVector {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.reverse();
        return *this;
    }
}
extension MultiVector: Reverse {
    associatedtype Output = MultiVector;
    func reverse() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Origin {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Origin& {
        this = this.reverse();
        return *this;
    }
}
extension Origin: Reverse {
    associatedtype Output = Origin;
    func reverse() -> Output {
        return this;
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Plane {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Plane& {
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
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Point {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Point& {
        this = this.reverse();
        return *this;
    }
}
extension Point: Reverse {
    associatedtype Output = Point;
    func reverse() -> Output {
        return this;
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Scalar {
        return this.reverse();
    }
    func operator/=(const ReversePrefixOrPostfix& _rhs) -> Scalar& {
        this = this.reverse();
        return *this;
    }
}
extension Scalar: Reverse {
    associatedtype Output = Scalar;
    func reverse() -> Output {
        return this;
    }
}

extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Scalar {
        return this.left_complement();
    }
}
extension AntiScalar: LeftComplement {
    associatedtype Output = Scalar;
    func left_complement() -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.e1234
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> DualNum {
        return this.left_complement();
    }
    func operator/=(const LeftComplementPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.left_complement();
        return *this;
    }
}
extension DualNum: LeftComplement {
    associatedtype Output = DualNum;
    func left_complement() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            this.group0.yx
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Flector {
        return this.left_complement();
    }
    func operator/=(const LeftComplementPrefixOrPostfix& _rhs) -> Flector& {
        this = this.left_complement();
        return *this;
    }
}
extension Flector: LeftComplement {
    associatedtype Output = Flector;
    func left_complement() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Origin {
        return this.left_complement();
    }
}
extension Horizon: LeftComplement {
    associatedtype Output = Origin;
    func left_complement() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e321
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Line {
        return this.left_complement();
    }
    func operator/=(const LeftComplementPrefixOrPostfix& _rhs) -> Line& {
        this = this.left_complement();
        return *this;
    }
}
extension Line: LeftComplement {
    associatedtype Output = Line;
    func left_complement() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group0 * float3(-1.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Motor {
        return this.left_complement();
    }
    func operator/=(const LeftComplementPrefixOrPostfix& _rhs) -> Motor& {
        this = this.left_complement();
        return *this;
    }
}
extension Motor: LeftComplement {
    associatedtype Output = Motor;
    func left_complement() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> MultiVector {
        return this.left_complement();
    }
    func operator/=(const LeftComplementPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.left_complement();
        return *this;
    }
}
extension MultiVector: LeftComplement {
    associatedtype Output = MultiVector;
    func left_complement() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.yx, 
            /* e1, e2, e3, e4 */
            this.group4, 
            /* e41, e42, e43 */
            this.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group2 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Horizon {
        return this.left_complement();
    }
}
extension Origin: LeftComplement {
    associatedtype Output = Horizon;
    func left_complement() -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e4 * -1.0
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Point {
        return this.left_complement();
    }
}
extension Plane: LeftComplement {
    associatedtype Output = Point;
    func left_complement() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Plane {
        return this.left_complement();
    }
}
extension Point: LeftComplement {
    associatedtype Output = Plane;
    func left_complement() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> AntiScalar {
        return this.left_complement();
    }
}
extension Scalar: LeftComplement {
    associatedtype Output = AntiScalar;
    func left_complement() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
}

extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> AntiScalar {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.double_complement();
        return *this;
    }
}
extension AntiScalar: DoubleComplement {
    associatedtype Output = AntiScalar;
    func double_complement() -> Output {
        return this;
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> DualNum {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.double_complement();
        return *this;
    }
}
extension DualNum: DoubleComplement {
    associatedtype Output = DualNum;
    func double_complement() -> Output {
        return this;
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Flector {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Flector& {
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
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Horizon {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.double_complement();
        return *this;
    }
}
extension Horizon: DoubleComplement {
    associatedtype Output = Horizon;
    func double_complement() -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Line {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Line& {
        this = this.double_complement();
        return *this;
    }
}
extension Line: DoubleComplement {
    associatedtype Output = Line;
    func double_complement() -> Output {
        return this;
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Motor {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Motor& {
        this = this.double_complement();
        return *this;
    }
}
extension Motor: DoubleComplement {
    associatedtype Output = Motor;
    func double_complement() -> Output {
        return this;
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> MultiVector {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.double_complement();
        return *this;
    }
}
extension MultiVector: DoubleComplement {
    associatedtype Output = MultiVector;
    func double_complement() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Origin {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Origin& {
        this = this.double_complement();
        return *this;
    }
}
extension Origin: DoubleComplement {
    associatedtype Output = Origin;
    func double_complement() -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Plane {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Plane& {
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
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Point {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Point& {
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
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Scalar {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.double_complement();
        return *this;
    }
}
extension Scalar: DoubleComplement {
    associatedtype Output = Scalar;
    func double_complement() -> Output {
        return this;
    }
}

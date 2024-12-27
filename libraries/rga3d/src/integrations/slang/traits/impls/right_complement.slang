extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Scalar {
        return this.right_complement();
    }
}
extension AntiScalar: RightComplement {
    associatedtype Output = Scalar;
    func right_complement() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> DualNum {
        return this.right_complement();
    }
    func operator/=(const RightComplementPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.right_complement();
        return *this;
    }
}
extension DualNum: RightComplement {
    associatedtype Output = DualNum;
    func right_complement() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy.yx, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Flector {
        return this.right_complement();
    }
    func operator/=(const RightComplementPrefixOrPostfix& _rhs) -> Flector& {
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
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Origin {
        return this.right_complement();
    }
}
extension Horizon: RightComplement {
    associatedtype Output = Origin;
    func right_complement() -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e321 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Line {
        return this.right_complement();
    }
    func operator/=(const RightComplementPrefixOrPostfix& _rhs) -> Line& {
        this = this.right_complement();
        return *this;
    }
}
extension Line: RightComplement {
    associatedtype Output = Line;
    func right_complement() -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Motor {
        return this.right_complement();
    }
    func operator/=(const RightComplementPrefixOrPostfix& _rhs) -> Motor& {
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
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> MultiVector {
        return this.right_complement();
    }
    func operator/=(const RightComplementPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.right_complement();
        return *this;
    }
}
extension MultiVector: RightComplement {
    associatedtype Output = MultiVector;
    func right_complement() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(this.group0.xy.yx, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group4 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group3.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Horizon {
        return this.right_complement();
    }
}
extension Origin: RightComplement {
    associatedtype Output = Horizon;
    func right_complement() -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Point {
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
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Plane {
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
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> AntiScalar {
        return this.right_complement();
    }
}
extension Scalar: RightComplement {
    associatedtype Output = AntiScalar;
    func right_complement() -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.scalar, 0.0, 0.0, 0.0)
        );
    }
}

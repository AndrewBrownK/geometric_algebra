extension AntiScalar: FlatWeight {
    associatedtype Output = AntiScalar;
    func flat_weight() -> Output {
        return this;
    }
    func operator/(_rhs: FlatWeightInfix) -> AntiScalar {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightInfix& _rhs) -> AntiScalar& {
        this = this.flat_weight();
        return *this;
    }
}
extension DualNum: FlatWeight {
    associatedtype Output = AntiScalar;
    func flat_weight() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234
        );
    }
    func operator/(_rhs: FlatWeightInfix) -> AntiScalar {
        return this.flat_weight();
    }
}
extension Flector: FlatWeight {
    associatedtype Output = Flector;
    func flat_weight() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e423, this.e431, this.e412, 0.0)
        );
    }
    func operator/(_rhs: FlatWeightInfix) -> Flector {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightInfix& _rhs) -> Flector& {
        this = this.flat_weight();
        return *this;
    }
}
extension Line: FlatWeight {
    associatedtype Output = Line;
    func flat_weight() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
    func operator/(_rhs: FlatWeightInfix) -> Line {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightInfix& _rhs) -> Line& {
        this = this.flat_weight();
        return *this;
    }
}
extension Motor: FlatWeight {
    associatedtype Output = Motor;
    func flat_weight() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
    func operator/(_rhs: FlatWeightInfix) -> Motor {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightInfix& _rhs) -> Motor& {
        this = this.flat_weight();
        return *this;
    }
}
extension MultiVector: FlatWeight {
    associatedtype Output = MultiVector;
    func flat_weight() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            this.group2, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e423, this.e431, this.e412, 0.0)
        );
    }
    func operator/(_rhs: FlatWeightInfix) -> MultiVector {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightInfix& _rhs) -> MultiVector& {
        this = this.flat_weight();
        return *this;
    }
}
extension Origin: FlatWeight {
    associatedtype Output = Origin;
    func flat_weight() -> Output {
        return this;
    }
    func operator/(_rhs: FlatWeightInfix) -> Origin {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightInfix& _rhs) -> Origin& {
        this = this.flat_weight();
        return *this;
    }
}
extension Plane: FlatWeight {
    associatedtype Output = Plane;
    func flat_weight() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e423, this.e431, this.e412, 0.0)
        );
    }
    func operator/(_rhs: FlatWeightInfix) -> Plane {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightInfix& _rhs) -> Plane& {
        this = this.flat_weight();
        return *this;
    }
}
extension Point: FlatWeight {
    associatedtype Output = Origin;
    func flat_weight() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e4
        );
    }
    func operator/(_rhs: FlatWeightInfix) -> Origin {
        return this.flat_weight();
    }
}

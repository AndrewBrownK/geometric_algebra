extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> AntiScalar {
        return this.weight();
    }
    func operator/=(const WeightPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.weight();
        return *this;
    }
}
extension AntiScalar: Weight {
    associatedtype Output = AntiScalar;
    func weight() -> Output {
        return this;
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> AntiScalar {
        return this.weight();
    }
}
extension DualNum: Weight {
    associatedtype Output = AntiScalar;
    func weight() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Flector {
        return this.weight();
    }
    func operator/=(const WeightPrefixOrPostfix& _rhs) -> Flector& {
        this = this.weight();
        return *this;
    }
}
extension Flector: Weight {
    associatedtype Output = Flector;
    func weight() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e423, this.e431, this.e412, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Line {
        return this.weight();
    }
    func operator/=(const WeightPrefixOrPostfix& _rhs) -> Line& {
        this = this.weight();
        return *this;
    }
}
extension Line: Weight {
    associatedtype Output = Line;
    func weight() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Motor {
        return this.weight();
    }
    func operator/=(const WeightPrefixOrPostfix& _rhs) -> Motor& {
        this = this.weight();
        return *this;
    }
}
extension Motor: Weight {
    associatedtype Output = Motor;
    func weight() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> MultiVector {
        return this.weight();
    }
    func operator/=(const WeightPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.weight();
        return *this;
    }
}
extension MultiVector: Weight {
    associatedtype Output = MultiVector;
    func weight() -> Output {
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
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Origin {
        return this.weight();
    }
    func operator/=(const WeightPrefixOrPostfix& _rhs) -> Origin& {
        this = this.weight();
        return *this;
    }
}
extension Origin: Weight {
    associatedtype Output = Origin;
    func weight() -> Output {
        return this;
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Plane {
        return this.weight();
    }
    func operator/=(const WeightPrefixOrPostfix& _rhs) -> Plane& {
        this = this.weight();
        return *this;
    }
}
extension Plane: Weight {
    associatedtype Output = Plane;
    func weight() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e423, this.e431, this.e412, 0.0)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Origin {
        return this.weight();
    }
}
extension Point: Weight {
    associatedtype Output = Origin;
    func weight() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e4
        );
    }
}

extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Scalar {
        return this.flat_bulk();
    }
}
extension DualNum: FlatBulk {
    associatedtype Output = Scalar;
    func flat_bulk() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Flector {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Flector& {
        this = this.flat_bulk();
        return *this;
    }
}
extension Flector: FlatBulk {
    associatedtype Output = Flector;
    func flat_bulk() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Horizon {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.flat_bulk();
        return *this;
    }
}
extension Horizon: FlatBulk {
    associatedtype Output = Horizon;
    func flat_bulk() -> Output {
        return this;
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Line {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Line& {
        this = this.flat_bulk();
        return *this;
    }
}
extension Line: FlatBulk {
    associatedtype Output = Line;
    func flat_bulk() -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Motor {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Motor& {
        this = this.flat_bulk();
        return *this;
    }
}
extension Motor: FlatBulk {
    associatedtype Output = Motor;
    func flat_bulk() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> MultiVector {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.flat_bulk();
        return *this;
    }
}
extension MultiVector: FlatBulk {
    associatedtype Output = MultiVector;
    func flat_bulk() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Horizon {
        return this.flat_bulk();
    }
}
extension Plane: FlatBulk {
    associatedtype Output = Horizon;
    func flat_bulk() -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Point {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Point& {
        this = this.flat_bulk();
        return *this;
    }
}
extension Point: FlatBulk {
    associatedtype Output = Point;
    func flat_bulk() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Scalar {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.flat_bulk();
        return *this;
    }
}
extension Scalar: FlatBulk {
    associatedtype Output = Scalar;
    func flat_bulk() -> Output {
        return this;
    }
}

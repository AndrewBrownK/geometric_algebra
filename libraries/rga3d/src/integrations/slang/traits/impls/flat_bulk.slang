extension DualNum: FlatBulk {
    associatedtype Output = Scalar;
    func flat_bulk() -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.scalar
        );
    }
    func operator/(_rhs: FlatBulkInfix) -> Scalar {
        return this.flat_bulk();
    }
}
extension Flector: FlatBulk {
    associatedtype Output = Flector;
    func flat_bulk() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
    func operator/(_rhs: FlatBulkInfix) -> Flector {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkInfix& _rhs) -> Flector& {
        this = this.flat_bulk();
        return *this;
    }
}
extension Horizon: FlatBulk {
    associatedtype Output = Horizon;
    func flat_bulk() -> Output {
        return this;
    }
    func operator/(_rhs: FlatBulkInfix) -> Horizon {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkInfix& _rhs) -> Horizon& {
        this = this.flat_bulk();
        return *this;
    }
}
extension Line: FlatBulk {
    associatedtype Output = Line;
    func flat_bulk() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group1
        );
    }
    func operator/(_rhs: FlatBulkInfix) -> Line {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkInfix& _rhs) -> Line& {
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
    func operator/(_rhs: FlatBulkInfix) -> Motor {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkInfix& _rhs) -> Motor& {
        this = this.flat_bulk();
        return *this;
    }
}
extension MultiVector: FlatBulk {
    associatedtype Output = MultiVector;
    func flat_bulk() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group3, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
    func operator/(_rhs: FlatBulkInfix) -> MultiVector {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkInfix& _rhs) -> MultiVector& {
        this = this.flat_bulk();
        return *this;
    }
}
extension Plane: FlatBulk {
    associatedtype Output = Horizon;
    func flat_bulk() -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e321
        );
    }
    func operator/(_rhs: FlatBulkInfix) -> Horizon {
        return this.flat_bulk();
    }
}
extension Point: FlatBulk {
    associatedtype Output = Point;
    func flat_bulk() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
    }
    func operator/(_rhs: FlatBulkInfix) -> Point {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkInfix& _rhs) -> Point& {
        this = this.flat_bulk();
        return *this;
    }
}
extension Scalar: FlatBulk {
    associatedtype Output = Scalar;
    func flat_bulk() -> Output {
        return this;
    }
    func operator/(_rhs: FlatBulkInfix) -> Scalar {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkInfix& _rhs) -> Scalar& {
        this = this.flat_bulk();
        return *this;
    }
}

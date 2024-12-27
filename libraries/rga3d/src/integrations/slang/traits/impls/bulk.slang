extension DualNum: Bulk {
    associatedtype Output = Scalar;
    func bulk() -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.scalar
        );
    }
    func operator/(_rhs: BulkInfix) -> Scalar {
        return this.bulk();
    }
}
extension Flector: Bulk {
    associatedtype Output = Flector;
    func bulk() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
    func operator/(_rhs: BulkInfix) -> Flector {
        return this.bulk();
    }
    func operator/=(const BulkInfix& _rhs) -> Flector& {
        this = this.bulk();
        return *this;
    }
}
extension Horizon: Bulk {
    associatedtype Output = Horizon;
    func bulk() -> Output {
        return this;
    }
    func operator/(_rhs: BulkInfix) -> Horizon {
        return this.bulk();
    }
    func operator/=(const BulkInfix& _rhs) -> Horizon& {
        this = this.bulk();
        return *this;
    }
}
extension Line: Bulk {
    associatedtype Output = Line;
    func bulk() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group1
        );
    }
    func operator/(_rhs: BulkInfix) -> Line {
        return this.bulk();
    }
    func operator/=(const BulkInfix& _rhs) -> Line& {
        this = this.bulk();
        return *this;
    }
}
extension Motor: Bulk {
    associatedtype Output = Motor;
    func bulk() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
    func operator/(_rhs: BulkInfix) -> Motor {
        return this.bulk();
    }
    func operator/=(const BulkInfix& _rhs) -> Motor& {
        this = this.bulk();
        return *this;
    }
}
extension MultiVector: Bulk {
    associatedtype Output = MultiVector;
    func bulk() -> Output {
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
    func operator/(_rhs: BulkInfix) -> MultiVector {
        return this.bulk();
    }
    func operator/=(const BulkInfix& _rhs) -> MultiVector& {
        this = this.bulk();
        return *this;
    }
}
extension Plane: Bulk {
    associatedtype Output = Horizon;
    func bulk() -> Output {
        return Horizon.from_groups(
            /* e321 */
            this.e321
        );
    }
    func operator/(_rhs: BulkInfix) -> Horizon {
        return this.bulk();
    }
}
extension Point: Bulk {
    associatedtype Output = Point;
    func bulk() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
    }
    func operator/(_rhs: BulkInfix) -> Point {
        return this.bulk();
    }
    func operator/=(const BulkInfix& _rhs) -> Point& {
        this = this.bulk();
        return *this;
    }
}
extension Scalar: Bulk {
    associatedtype Output = Scalar;
    func bulk() -> Output {
        return this;
    }
    func operator/(_rhs: BulkInfix) -> Scalar {
        return this.bulk();
    }
    func operator/=(const BulkInfix& _rhs) -> Scalar& {
        this = this.bulk();
        return *this;
    }
}

extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Scalar {
        return this.bulk();
    }
}
extension DualNum: Bulk {
    associatedtype Output = Scalar;
    func bulk() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Flector {
        return this.bulk();
    }
    func operator/=(const BulkPrefixOrPostfix& _rhs) -> Flector& {
        this = this.bulk();
        return *this;
    }
}
extension Flector: Bulk {
    associatedtype Output = Flector;
    func bulk() -> Output {
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
    func operator/(_rhs: BulkPrefixOrPostfix) -> Horizon {
        return this.bulk();
    }
    func operator/=(const BulkPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.bulk();
        return *this;
    }
}
extension Horizon: Bulk {
    associatedtype Output = Horizon;
    func bulk() -> Output {
        return this;
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Line {
        return this.bulk();
    }
    func operator/=(const BulkPrefixOrPostfix& _rhs) -> Line& {
        this = this.bulk();
        return *this;
    }
}
extension Line: Bulk {
    associatedtype Output = Line;
    func bulk() -> Output {
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
    func operator/(_rhs: BulkPrefixOrPostfix) -> Motor {
        return this.bulk();
    }
    func operator/=(const BulkPrefixOrPostfix& _rhs) -> Motor& {
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
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> MultiVector {
        return this.bulk();
    }
    func operator/=(const BulkPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.bulk();
        return *this;
    }
}
extension MultiVector: Bulk {
    associatedtype Output = MultiVector;
    func bulk() -> Output {
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
    func operator/(_rhs: BulkPrefixOrPostfix) -> Horizon {
        return this.bulk();
    }
}
extension Plane: Bulk {
    associatedtype Output = Horizon;
    func bulk() -> Output {
        return Horizon.from_groups(
            /* e321, 0, 0, 0 */
            float4(this.e321, 0.0, 0.0, 0.0)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Point {
        return this.bulk();
    }
    func operator/=(const BulkPrefixOrPostfix& _rhs) -> Point& {
        this = this.bulk();
        return *this;
    }
}
extension Point: Bulk {
    associatedtype Output = Point;
    func bulk() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Scalar {
        return this.bulk();
    }
    func operator/=(const BulkPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.bulk();
        return *this;
    }
}
extension Scalar: Bulk {
    associatedtype Output = Scalar;
    func bulk() -> Output {
        return this;
    }
}

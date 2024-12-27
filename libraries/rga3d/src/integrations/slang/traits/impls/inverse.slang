extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> DualNum {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.inverse();
        return *this;
    }
}
extension DualNum: Inverse {
    associatedtype Output = DualNum;
    func inverse() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(this.scalar, -2)) * this.group0
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Flector {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Flector& {
        this = this.inverse();
        return *this;
    }
}
extension Flector: Inverse {
    associatedtype Output = Flector;
    func inverse() -> Output {
Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
Scalar other = Scalar.from_groups(
            /* scalar */
            powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2) + powi(this.e321, 2)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * reverse.group1
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Horizon {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Horizon& {
        this = this.inverse();
        return *this;
    }
}
extension Horizon: Inverse {
    associatedtype Output = Horizon;
    func inverse() -> Output {
        return Horizon.from_groups(
            /* e321 */
            1.0/this.e321 * -1.0
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Line {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Line& {
        this = this.inverse();
        return *this;
    }
}
extension Line: Inverse {
    associatedtype Output = Line;
    func inverse() -> Output {
Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            this.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1 * float3(-1.0)
        );
Scalar other = Scalar.from_groups(
            /* scalar */
            powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * reverse.group0, 
            /* e23, e31, e12 */
            float3(other.scalar) * reverse.group1
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Motor {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Motor& {
        this = this.inverse();
        return *this;
    }
}
extension Motor: Inverse {
    associatedtype Output = Motor;
    func inverse() -> Output {
Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other = Scalar.from_groups(
            /* scalar */
            powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) + powi(this.scalar, 2)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * reverse.group1
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> MultiVector {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.inverse();
        return *this;
    }
}
extension MultiVector: Inverse {
    associatedtype Output = MultiVector;
    func inverse() -> Output {
MultiVector reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
Scalar other = Scalar.from_groups(
            /* scalar */
            powi(this.scalar, 2) + powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2) + powi(this.e23, 2) + powi(this.e31, 2) + powi(this.e12, 2) + powi(this.e321, 2)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * reverse.group2, 
            /* e23, e31, e12 */
            float3(other.scalar) * reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * reverse.group4
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Plane {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Plane& {
        this = this.inverse();
        return *this;
    }
}
extension Plane: Inverse {
    associatedtype Output = Plane;
    func inverse() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(this.e321, -2)) * float4(this.e423 * -1.0, this.e431 * -1.0, this.e412 * -1.0, this.e321 * -1.0)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Point {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Point& {
        this = this.inverse();
        return *this;
    }
}
extension Point: Inverse {
    associatedtype Output = Point;
    func inverse() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2)) * this.group0
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Scalar {
        return this.inverse();
    }
    func operator/=(const InversePrefixOrPostfix& _rhs) -> Scalar& {
        this = this.inverse();
        return *this;
    }
}
extension Scalar: Inverse {
    associatedtype Output = Scalar;
    func inverse() -> Output {
        return Scalar.from_groups(
            /* scalar */
            1.0/this.scalar
        );
    }
}

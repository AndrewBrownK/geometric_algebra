public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Scalar {
        return this.bulk();
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.bulk();
    }
}
public extension DualNum: Bulk {
    public typedef Scalar BulkOutput;
    public func bulk() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Flector {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const BulkPrefixOrPostfix& _rhs) -> Flector& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.bulk();
    }
}
public extension Flector: Bulk {
    public typedef Flector BulkOutput;
    public func bulk() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Horizon {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const BulkPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
        return rhs.bulk();
    }
}
public extension Horizon: Bulk {
    public typedef Horizon BulkOutput;
    public func bulk() -> Horizon {
        return this;
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Line {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const BulkPrefixOrPostfix& _rhs) -> Line& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.bulk();
    }
}
public extension Line: Bulk {
    public typedef Line BulkOutput;
    public func bulk() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Motor {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const BulkPrefixOrPostfix& _rhs) -> Motor& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.bulk();
    }
}
public extension Motor: Bulk {
    public typedef Motor BulkOutput;
    public func bulk() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> MultiVector {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const BulkPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.bulk();
    }
}
public extension MultiVector: Bulk {
    public typedef MultiVector BulkOutput;
    public func bulk() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Horizon {
        return this.bulk();
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.bulk();
    }
}
public extension Plane: Bulk {
    public typedef Horizon BulkOutput;
    public func bulk() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Point {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const BulkPrefixOrPostfix& _rhs) -> Point& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.bulk();
    }
}
public extension Point: Bulk {
    public typedef Point BulkOutput;
    public func bulk() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: BulkPrefixOrPostfix) -> Scalar {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const BulkPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.bulk();
    }
}
public extension Scalar: Bulk {
    public typedef Scalar BulkOutput;
    public func bulk() -> Scalar {
        return this;
    }
}

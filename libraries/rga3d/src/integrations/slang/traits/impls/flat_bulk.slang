public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Scalar {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.flat_bulk();
    }
}
public extension DualNum: FlatBulk {
    public typedef Scalar FlatBulkOutput;
    public func flat_bulk() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Flector {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Flector& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.flat_bulk();
    }
}
public extension Flector: FlatBulk {
    public typedef Flector FlatBulkOutput;
    public func flat_bulk() -> Flector {
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
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Horizon {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
        return rhs.flat_bulk();
    }
}
public extension Horizon: FlatBulk {
    public typedef Horizon FlatBulkOutput;
    public func flat_bulk() -> Horizon {
        return this;
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Line {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Line& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.flat_bulk();
    }
}
public extension Line: FlatBulk {
    public typedef Line FlatBulkOutput;
    public func flat_bulk() -> Line {
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
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Motor {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Motor& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.flat_bulk();
    }
}
public extension Motor: FlatBulk {
    public typedef Motor FlatBulkOutput;
    public func flat_bulk() -> Motor {
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
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> MultiVector {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.flat_bulk();
    }
}
public extension MultiVector: FlatBulk {
    public typedef MultiVector FlatBulkOutput;
    public func flat_bulk() -> MultiVector {
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
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Horizon {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.flat_bulk();
    }
}
public extension Plane: FlatBulk {
    public typedef Horizon FlatBulkOutput;
    public func flat_bulk() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Point {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Point& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.flat_bulk();
    }
}
public extension Point: FlatBulk {
    public typedef Point FlatBulkOutput;
    public func flat_bulk() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Scalar {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.flat_bulk();
    }
}
public extension Scalar: FlatBulk {
    public typedef Scalar FlatBulkOutput;
    public func flat_bulk() -> Scalar {
        return this;
    }
}

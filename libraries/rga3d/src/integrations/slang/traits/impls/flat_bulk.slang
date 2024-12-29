implementing rga3d;
using traits;
public extension DualNum {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(DualNum rhs) {
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
    public Flector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const FlatBulkPrefixOrPostfix& rhs) -> Flector& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
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
    public Horizon operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const FlatBulkPrefixOrPostfix& rhs) -> Horizon& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
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
    public Line operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const FlatBulkPrefixOrPostfix& rhs) -> Line& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
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
    public Motor operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const FlatBulkPrefixOrPostfix& rhs) -> Motor& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
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
    public MultiVector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const FlatBulkPrefixOrPostfix& rhs) -> MultiVector& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
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
    public Horizon operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Plane rhs) {
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
    public Point operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const FlatBulkPrefixOrPostfix& rhs) -> Point& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
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
    public Scalar operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const FlatBulkPrefixOrPostfix& rhs) -> Scalar& {
        this = this.flat_bulk();
        return *this;
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.flat_bulk();
    }
}
public extension Scalar: FlatBulk {
    public typedef Scalar FlatBulkOutput;
    public func flat_bulk() -> Scalar {
        return this;
    }
}

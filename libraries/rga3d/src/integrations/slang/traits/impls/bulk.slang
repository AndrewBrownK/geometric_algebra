implementing rga3d;
using traits;
public extension DualNum {
    // Fancy postfix trick
    public Scalar operator /(BulkPrefixOrPostfix rhs)  {
        return this.bulk();
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(DualNum rhs) {
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
    public Flector operator /(BulkPrefixOrPostfix rhs)  {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const BulkPrefixOrPostfix& rhs) -> Flector& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
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
    public Horizon operator /(BulkPrefixOrPostfix rhs)  {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const BulkPrefixOrPostfix& rhs) -> Horizon& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
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
    public Line operator /(BulkPrefixOrPostfix rhs)  {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const BulkPrefixOrPostfix& rhs) -> Line& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
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
    public Motor operator /(BulkPrefixOrPostfix rhs)  {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const BulkPrefixOrPostfix& rhs) -> Motor& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
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
    public MultiVector operator /(BulkPrefixOrPostfix rhs)  {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const BulkPrefixOrPostfix& rhs) -> MultiVector& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
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
    public Horizon operator /(BulkPrefixOrPostfix rhs)  {
        return this.bulk();
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Plane rhs) {
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
    public Point operator /(BulkPrefixOrPostfix rhs)  {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const BulkPrefixOrPostfix& rhs) -> Point& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
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
    public Scalar operator /(BulkPrefixOrPostfix rhs)  {
        return this.bulk();
    }
    // Fancy postfix self-assign
    public func operator /=(const BulkPrefixOrPostfix& rhs) -> Scalar& {
        this = this.bulk();
        return *this;
    }
}
public extension BulkPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.bulk();
    }
}
public extension Scalar: Bulk {
    public typedef Scalar BulkOutput;
    public func bulk() -> Scalar {
        return this;
    }
}

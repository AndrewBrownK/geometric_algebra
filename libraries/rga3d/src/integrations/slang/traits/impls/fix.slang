implementing rga3d;
using traits;
public extension Horizon {
    // Fancy postfix trick
    public Horizon operator /(FixPrefixOrPostfix rhs)  {
        return this.fix();
    }
    // Fancy postfix self-assign
    public func operator /=(const FixPrefixOrPostfix& rhs) -> Horizon& {
        this = this.fix();
        return *this;
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.fix();
    }
}
public extension Horizon: Fix {
    public typedef Horizon FixOutput;
    public func fix() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(FixPrefixOrPostfix rhs)  {
        return this.fix();
    }
    // Fancy postfix self-assign
    public func operator /=(const FixPrefixOrPostfix& rhs) -> Plane& {
        this = this.fix();
        return *this;
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.fix();
    }
}
public extension Plane: Fix {
    public typedef Plane FixOutput;
    public func fix() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow((this.group0 * float4(-1.0))[3], -0.5) * pow(this.e321, -0.5) * -1.0) * this.group0
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(FixPrefixOrPostfix rhs)  {
        return this.fix();
    }
    // Fancy postfix self-assign
    public func operator /=(const FixPrefixOrPostfix& rhs) -> Point& {
        this = this.fix();
        return *this;
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.fix();
    }
}
public extension Point: Fix {
    public typedef Point FixOutput;
    public func fix() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2)) * this.group0
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(FixPrefixOrPostfix rhs)  {
        return this.fix();
    }
    // Fancy postfix self-assign
    public func operator /=(const FixPrefixOrPostfix& rhs) -> Scalar& {
        this = this.fix();
        return *this;
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.fix();
    }
}
public extension Scalar: Fix {
    public typedef Scalar FixOutput;
    public func fix() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            1.0
        );
    }
}

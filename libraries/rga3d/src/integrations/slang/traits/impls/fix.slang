public extension Horizon {
    // Fancy postfix trick
    func operator/(_rhs: FixPrefixOrPostfix) -> Horizon {
        return this.fix();
    }
    // Fancy postfix self-assign
    public func operator/=(const FixPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.fix();
        return *this;
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
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
    func operator/(_rhs: FixPrefixOrPostfix) -> Plane {
        return this.fix();
    }
    // Fancy postfix self-assign
    public func operator/=(const FixPrefixOrPostfix& _rhs) -> Plane& {
        this = this.fix();
        return *this;
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
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
    func operator/(_rhs: FixPrefixOrPostfix) -> Point {
        return this.fix();
    }
    // Fancy postfix self-assign
    public func operator/=(const FixPrefixOrPostfix& _rhs) -> Point& {
        this = this.fix();
        return *this;
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
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
    func operator/(_rhs: FixPrefixOrPostfix) -> Scalar {
        return this.fix();
    }
    // Fancy postfix self-assign
    public func operator/=(const FixPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.fix();
        return *this;
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
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

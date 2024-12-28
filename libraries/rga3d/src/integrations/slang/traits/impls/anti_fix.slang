public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> AntiScalar {
        return this.anti_fix();
    }
    // Fancy postfix self-assign
    public func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.anti_fix();
        return *this;
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.anti_fix();
    }
}
public extension AntiScalar: AntiFix {
    public typedef AntiScalar AntiFixOutput;
    public func anti_fix() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> Origin {
        return this.anti_fix();
    }
    // Fancy postfix self-assign
    public func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> Origin& {
        this = this.anti_fix();
        return *this;
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.anti_fix();
    }
}
public extension Origin: AntiFix {
    public typedef Origin AntiFixOutput;
    public func anti_fix() -> Origin {
        return Origin.from_groups(
            /* e4 */
            1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> Plane {
        return this.anti_fix();
    }
    // Fancy postfix self-assign
    public func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> Plane& {
        this = this.anti_fix();
        return *this;
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.anti_fix();
    }
}
public extension Plane: AntiFix {
    public typedef Plane AntiFixOutput;
    public func anti_fix() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(this.e423, 2) + pow(this.e431, 2) + pow(this.e412, 2)) * this.group0
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> Point {
        return this.anti_fix();
    }
    // Fancy postfix self-assign
    public func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> Point& {
        this = this.anti_fix();
        return *this;
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.anti_fix();
    }
}
public extension Point: AntiFix {
    public typedef Point AntiFixOutput;
    public func anti_fix() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow((this.group0 * float4(-1.0))[3], -0.5) * pow(this.e4, -0.5) * -1.0) * this.group0
        );
    }
}

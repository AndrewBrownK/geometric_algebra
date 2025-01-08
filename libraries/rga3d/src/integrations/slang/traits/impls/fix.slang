implementing rga3d;
using traits;
public extension Horizon {
    // Fancy postfix trick
    public Horizon operator /(FixPrefixOrPostfix rhs)  {
        return this.fix();
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public Horizon operator /(Horizon rhs) {
        return rhs.fix();
    }
}
public extension Horizon {
    public Horizon fix() {
        return Horizon(
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
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.fix();
    }
}
public extension Plane {
    public Plane fix() {
        return Plane(
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
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.fix();
    }
}
public extension Point {
    public Point fix() {
        return Point(
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
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.fix();
    }
}
public extension Scalar {
    public Scalar fix() {
        return Scalar(
            /* scalar */
            1.0
        );
    }
}

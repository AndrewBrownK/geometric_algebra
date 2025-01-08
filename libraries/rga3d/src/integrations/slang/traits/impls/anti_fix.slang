implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AntiFixPrefixOrPostfix rhs)  {
        return this.anti_fix();
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.anti_fix();
    }
}
public extension AntiScalar {
    public AntiScalar anti_fix() {
        return AntiScalar(
            /* e1234 */
            1.0
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    public Origin operator /(AntiFixPrefixOrPostfix rhs)  {
        return this.anti_fix();
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.anti_fix();
    }
}
public extension Origin {
    public Origin anti_fix() {
        return Origin(
            /* e4 */
            1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(AntiFixPrefixOrPostfix rhs)  {
        return this.anti_fix();
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.anti_fix();
    }
}
public extension Plane {
    public Plane anti_fix() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(pow(this.e423, 2) + pow(this.e431, 2) + pow(this.e412, 2)) * this.group0
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(AntiFixPrefixOrPostfix rhs)  {
        return this.anti_fix();
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.anti_fix();
    }
}
public extension Point {
    public Point anti_fix() {
        return Point(
            /* e1, e2, e3, e4 */
            float4(pow((this.group0 * float4(-1.0))[3], -0.5) * pow(this.e4, -0.5) * -1.0) * this.group0
        );
    }
}

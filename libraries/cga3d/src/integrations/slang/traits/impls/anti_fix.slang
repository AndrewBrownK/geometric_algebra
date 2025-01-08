implementing cga3d;
using traits;
public extension AntiFlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(AntiFixPrefixOrPostfix rhs)  {
        return this.anti_fix();
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.anti_fix();
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_fix() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(pow((this.group0 * float4(-1.0))[3], -0.5) * pow(this.e321, -0.5)) * this.group0
        );
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public AntiPlane operator /(AntiFixPrefixOrPostfix rhs)  {
        return this.anti_fix();
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(AntiPlane rhs) {
        return rhs.anti_fix();
    }
}
public extension AntiPlane {
    public AntiPlane anti_fix() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-pow(this.e1, 2) - pow(this.e2, 2) - pow(this.e3, 2)) * this.group0
        );
    }
}
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
            /* e12345 */
            1.0
        );
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(AntiFixPrefixOrPostfix rhs)  {
        return this.anti_fix();
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(FlatPoint rhs) {
        return rhs.anti_fix();
    }
}
public extension FlatPoint {
    public FlatPoint anti_fix() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(pow((this.group0 * float4(-1.0))[3], -0.5) * pow(this.e45, -0.5) * -1.0) * this.group0
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
            /* e4235, e4315, e4125, e3215 */
            float4(pow(this.e4235, 2) + pow(this.e4315, 2) + pow(this.e4125, 2)) * this.group0
        );
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public RoundPoint operator /(AntiFixPrefixOrPostfix rhs)  {
        return this.anti_fix();
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(RoundPoint rhs) {
        return rhs.anti_fix();
    }
}
public extension RoundPoint {
    public RoundPoint anti_fix() {
        AntiScalar geometric_anti_product_2 = AntiScalar(
            /* e12345 */
            2.0*(this.e4 * this.e5) - pow(this.e1, 2) - pow(this.e2, 2) - pow(this.e3, 2)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product_2.e12345) * this.group0, 
            /* e5 */
            geometric_anti_product_2.e12345 * this.e5
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(AntiFixPrefixOrPostfix rhs)  {
        return this.anti_fix();
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.anti_fix();
    }
}
public extension Scalar {
    public Scalar anti_fix() {
        return Scalar(
            /* scalar */
            -1.0
        );
    }
}
public extension Sphere {
    // Fancy postfix trick
    public Sphere operator /(AntiFixPrefixOrPostfix rhs)  {
        return this.anti_fix();
    }
}
public extension AntiFixPrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(Sphere rhs) {
        return rhs.anti_fix();
    }
}
public extension Sphere {
    public Sphere anti_fix() {
        AntiScalar geometric_anti_product_2 = AntiScalar(
            /* e12345 */
            pow(this.e4235, 2) + pow(this.e4315, 2) + pow(this.e4125, 2) - 2.0 * (this.e3215 * this.e1234)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product_2.e12345) * this.group0, 
            /* e1234 */
            geometric_anti_product_2.e12345 * this.e1234
        );
    }
}

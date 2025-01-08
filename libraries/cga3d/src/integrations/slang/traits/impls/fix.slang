implementing cga3d;
using traits;
public extension AntiFlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(FixPrefixOrPostfix rhs)  {
        return this.fix();
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.fix();
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint fix() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(pow((this.group0 * float4(-1.0))[3], -0.5) * pow(this.e321, -0.5) * -1.0) * this.group0
        );
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public AntiPlane operator /(FixPrefixOrPostfix rhs)  {
        return this.fix();
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(AntiPlane rhs) {
        return rhs.fix();
    }
}
public extension AntiPlane {
    public AntiPlane fix() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2)) * this.group0
        );
    }
}
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(FixPrefixOrPostfix rhs)  {
        return this.fix();
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.fix();
    }
}
public extension AntiScalar {
    public AntiScalar fix() {
        return AntiScalar(
            /* e12345 */
            -1.0
        );
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(FixPrefixOrPostfix rhs)  {
        return this.fix();
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(FlatPoint rhs) {
        return rhs.fix();
    }
}
public extension FlatPoint {
    public FlatPoint fix() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(pow((this.group0 * float4(-1.0))[3], -0.5) * pow(this.e45, -0.5)) * this.group0
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
            /* e4235, e4315, e4125, e3215 */
            float4(-pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)) * this.group0
        );
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public RoundPoint operator /(FixPrefixOrPostfix rhs)  {
        return this.fix();
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(RoundPoint rhs) {
        return rhs.fix();
    }
}
public extension RoundPoint {
    public RoundPoint fix() {
        Scalar geometric_product_2 = Scalar(
            /* scalar */
            pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) - 2.0 * (this.e4 * this.e5)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(geometric_product_2.scalar) * this.group0, 
            /* e5 */
            this.e5 * geometric_product_2.scalar
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
public extension Sphere {
    // Fancy postfix trick
    public Sphere operator /(FixPrefixOrPostfix rhs)  {
        return this.fix();
    }
}
public extension FixPrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(Sphere rhs) {
        return rhs.fix();
    }
}
public extension Sphere {
    public Sphere fix() {
        Scalar geometric_product_2 = Scalar(
            /* scalar */
            2.0*(this.e3215 * this.e1234) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_product_2.scalar) * this.group0, 
            /* e1234 */
            geometric_product_2.scalar * this.e1234
        );
    }
}

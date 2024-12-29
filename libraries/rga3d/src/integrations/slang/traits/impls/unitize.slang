implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.unitize();
    }
}
public extension AntiScalar {
    public AntiScalar unitize() {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 / (this.flat_weight_norm_squared().anti_square_root().e1234)
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.unitize();
    }
}
public extension DualNum {
    public DualNum unitize() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0.xy
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.unitize();
    }
}
public extension Flector {
    public Flector unitize() {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/this.flat_weight_norm_squared().anti_square_root().e1234
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group1
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.unitize();
    }
}
public extension Line {
    public Line unitize() {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/this.flat_weight_norm_squared().anti_square_root().e1234
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e1234) * this.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.unitize();
    }
}
public extension Motor {
    public Motor unitize() {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/this.flat_weight_norm_squared().anti_square_root().e1234
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product.e1234) * this.group1
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.unitize();
    }
}
public extension MultiVector {
    public MultiVector unitize() {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/this.flat_weight_norm_squared().anti_square_root().e1234
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product.e1234) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e1234) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group4
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    public Origin operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.unitize();
    }
}
public extension Origin {
    public Origin unitize() {
        return Origin.from_groups(
            /* e4 */
            this.e4 / (this.flat_weight_norm_squared().anti_square_root().e1234)
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.unitize();
    }
}
public extension Plane {
    public Plane unitize() {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.unitize();
    }
}
public extension Point {
    public Point unitize() {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0
        );
    }
}

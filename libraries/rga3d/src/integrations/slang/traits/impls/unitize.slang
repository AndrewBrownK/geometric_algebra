implementing rga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public AntiScalar operator /(UnitizePrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.unitize();
}
public extension AntiScalar {
    public AntiScalar unitize() {
        return AntiScalar(
            /* e1234 */
            this.e1234 / (this.flat_weight_norm_squared().anti_square_root().e1234)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public DualNum operator /(UnitizePrefixOrPostfix lhs, DualNum rhs) {
    return rhs.unitize();
}
public extension DualNum {
    public DualNum unitize() {
        return DualNum(
            /* scalar, e1234 */
            float2(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0.xy
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Flector operator /(UnitizePrefixOrPostfix lhs, Flector rhs) {
    return rhs.unitize();
}
public extension Flector {
    public Flector unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e1234 */
            1.0/this.flat_weight_norm_squared().anti_square_root().e1234
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group1
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Line operator /(UnitizePrefixOrPostfix lhs, Line rhs) {
    return rhs.unitize();
}
public extension Line {
    public Line unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e1234 */
            1.0/this.flat_weight_norm_squared().anti_square_root().e1234
        );
        return Line(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e1234) * this.group1.xyz
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Motor operator /(UnitizePrefixOrPostfix lhs, Motor rhs) {
    return rhs.unitize();
}
public extension Motor {
    public Motor unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e1234 */
            1.0/this.flat_weight_norm_squared().anti_square_root().e1234
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product.e1234) * this.group1
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public MultiVector operator /(UnitizePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitize();
}
public extension MultiVector {
    public MultiVector unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e1234 */
            1.0/this.flat_weight_norm_squared().anti_square_root().e1234
        );
        return MultiVector(
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
// Fancy postfix trick
public Origin operator /(Origin lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Origin operator /(UnitizePrefixOrPostfix lhs, Origin rhs) {
    return rhs.unitize();
}
public extension Origin {
    public Origin unitize() {
        return Origin(
            /* e4 */
            this.e4 / (this.flat_weight_norm_squared().anti_square_root().e1234)
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Plane operator /(UnitizePrefixOrPostfix lhs, Plane rhs) {
    return rhs.unitize();
}
public extension Plane {
    public Plane unitize() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Point operator /(UnitizePrefixOrPostfix lhs, Point rhs) {
    return rhs.unitize();
}
public extension Point {
    public Point unitize() {
        return Point(
            /* e1, e2, e3, e4 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0
        );
    }
}

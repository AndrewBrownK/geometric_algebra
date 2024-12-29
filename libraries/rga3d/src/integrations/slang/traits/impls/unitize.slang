implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
    // Fancy postfix self-assign
    public func operator /=(const UnitizePrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.unitize();
    }
}
public extension AntiScalar: Unitize {
    public typedef AntiScalar UnitizeOutput;
    public func unitize() -> AntiScalar {
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
    // Fancy postfix self-assign
    public func operator /=(const UnitizePrefixOrPostfix& rhs) -> DualNum& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.unitize();
    }
}
public extension DualNum: Unitize {
    public typedef DualNum UnitizeOutput;
    public func unitize() -> DualNum {
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
    // Fancy postfix self-assign
    public func operator /=(const UnitizePrefixOrPostfix& rhs) -> Flector& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.unitize();
    }
}
public extension Flector: Unitize {
    public typedef Flector UnitizeOutput;
    public func unitize() -> Flector {
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
    // Fancy postfix self-assign
    public func operator /=(const UnitizePrefixOrPostfix& rhs) -> Line& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.unitize();
    }
}
public extension Line: Unitize {
    public typedef Line UnitizeOutput;
    public func unitize() -> Line {
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
    // Fancy postfix self-assign
    public func operator /=(const UnitizePrefixOrPostfix& rhs) -> Motor& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.unitize();
    }
}
public extension Motor: Unitize {
    public typedef Motor UnitizeOutput;
    public func unitize() -> Motor {
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
    // Fancy postfix self-assign
    public func operator /=(const UnitizePrefixOrPostfix& rhs) -> MultiVector& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.unitize();
    }
}
public extension MultiVector: Unitize {
    public typedef MultiVector UnitizeOutput;
    public func unitize() -> MultiVector {
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
    // Fancy postfix self-assign
    public func operator /=(const UnitizePrefixOrPostfix& rhs) -> Origin& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.unitize();
    }
}
public extension Origin: Unitize {
    public typedef Origin UnitizeOutput;
    public func unitize() -> Origin {
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
    // Fancy postfix self-assign
    public func operator /=(const UnitizePrefixOrPostfix& rhs) -> Plane& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.unitize();
    }
}
public extension Plane: Unitize {
    public typedef Plane UnitizeOutput;
    public func unitize() -> Plane {
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
    // Fancy postfix self-assign
    public func operator /=(const UnitizePrefixOrPostfix& rhs) -> Point& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.unitize();
    }
}
public extension Point: Unitize {
    public typedef Point UnitizeOutput;
    public func unitize() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0
        );
    }
}

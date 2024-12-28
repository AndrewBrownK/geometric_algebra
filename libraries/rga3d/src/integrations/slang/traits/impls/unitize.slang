using traits;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> AntiScalar {
        return this.unitize();
    }
    // Fancy postfix self-assign
    public func operator/=(const UnitizePrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
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
    func operator/(_rhs: UnitizePrefixOrPostfix) -> DualNum {
        return this.unitize();
    }
    // Fancy postfix self-assign
    public func operator/=(const UnitizePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
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
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Flector {
        return this.unitize();
    }
    // Fancy postfix self-assign
    public func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Flector& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
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
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Line {
        return this.unitize();
    }
    // Fancy postfix self-assign
    public func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Line& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
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
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Motor {
        return this.unitize();
    }
    // Fancy postfix self-assign
    public func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Motor& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
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
    func operator/(_rhs: UnitizePrefixOrPostfix) -> MultiVector {
        return this.unitize();
    }
    // Fancy postfix self-assign
    public func operator/=(const UnitizePrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
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
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Origin {
        return this.unitize();
    }
    // Fancy postfix self-assign
    public func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Origin& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
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
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Plane {
        return this.unitize();
    }
    // Fancy postfix self-assign
    public func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Plane& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
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
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Point {
        return this.unitize();
    }
    // Fancy postfix self-assign
    public func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Point& {
        this = this.unitize();
        return *this;
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
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

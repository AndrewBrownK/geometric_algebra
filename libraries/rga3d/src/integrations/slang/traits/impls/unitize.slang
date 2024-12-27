using traits::AntiSquareRoot;
using traits::FlatWeightNormSquared;
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> AntiScalar {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.unitize();
        return *this;
    }
}
extension AntiScalar: Unitize {
    associatedtype Output = AntiScalar;
    func unitize() -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 / (this.flat_weight_norm_squared().anti_square_root().e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> DualNum {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.unitize();
        return *this;
    }
}
extension DualNum: Unitize {
    associatedtype Output = DualNum;
    func unitize() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0.xy, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Flector {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Flector& {
        this = this.unitize();
        return *this;
    }
}
extension Flector: Unitize {
    associatedtype Output = Flector;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234, 0.0, 0.0, 0.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group1
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Line {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Line& {
        this = this.unitize();
        return *this;
    }
}
extension Line: Unitize {
    associatedtype Output = Line;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234, 0.0, 0.0, 0.0)
        );
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e1234) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_anti_product.e1234) * this.group1.xyz, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Motor {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Motor& {
        this = this.unitize();
        return *this;
    }
}
extension Motor: Unitize {
    associatedtype Output = Motor;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234, 0.0, 0.0, 0.0)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product.e1234) * this.group1
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> MultiVector {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.unitize();
        return *this;
    }
}
extension MultiVector: Unitize {
    associatedtype Output = MultiVector;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(geometric_anti_product.e1234) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group1, 
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e1234) * this.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_anti_product.e1234) * this.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group4
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Origin {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Origin& {
        this = this.unitize();
        return *this;
    }
}
extension Origin: Unitize {
    associatedtype Output = Origin;
    func unitize() -> Output {
        return Origin.from_groups(
            /* e4, 0, 0, 0 */
            float4(this.e4 / (this.flat_weight_norm_squared().anti_square_root().e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Plane {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Plane& {
        this = this.unitize();
        return *this;
    }
}
extension Plane: Unitize {
    associatedtype Output = Plane;
    func unitize() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Point {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Point& {
        this = this.unitize();
        return *this;
    }
}
extension Point: Unitize {
    associatedtype Output = Point;
    func unitize() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0
        );
    }
}

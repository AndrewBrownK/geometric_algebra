using traits::AntiSquareRoot;
using traits::FlatWeightNormSquared;
extension AntiScalar: Unitize {
    associatedtype Output = AntiScalar;
    func unitize() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 / (this.flat_weight_norm_squared().anti_square_root().e1234)
        );
    }
    func operator/(_rhs: UnitizeInfix) -> AntiScalar {
        return this.unitize();
    }
    func operator/=(const UnitizeInfix& _rhs) -> AntiScalar& {
        this = this.unitize();
        return *this;
    }
}
extension DualNum: Unitize {
    associatedtype Output = DualNum;
    func unitize() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0
        );
    }
    func operator/(_rhs: UnitizeInfix) -> DualNum {
        return this.unitize();
    }
    func operator/=(const UnitizeInfix& _rhs) -> DualNum& {
        this = this.unitize();
        return *this;
    }
}
extension Flector: Unitize {
    associatedtype Output = Flector;
    func unitize() -> Output {
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
    func operator/(_rhs: UnitizeInfix) -> Flector {
        return this.unitize();
    }
    func operator/=(const UnitizeInfix& _rhs) -> Flector& {
        this = this.unitize();
        return *this;
    }
}
extension Line: Unitize {
    associatedtype Output = Line;
    func unitize() -> Output {
AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/this.flat_weight_norm_squared().anti_square_root().e1234
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e1234) * this.group1
        );
    }
    func operator/(_rhs: UnitizeInfix) -> Line {
        return this.unitize();
    }
    func operator/=(const UnitizeInfix& _rhs) -> Line& {
        this = this.unitize();
        return *this;
    }
}
extension Motor: Unitize {
    associatedtype Output = Motor;
    func unitize() -> Output {
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
    func operator/(_rhs: UnitizeInfix) -> Motor {
        return this.unitize();
    }
    func operator/=(const UnitizeInfix& _rhs) -> Motor& {
        this = this.unitize();
        return *this;
    }
}
extension MultiVector: Unitize {
    associatedtype Output = MultiVector;
    func unitize() -> Output {
AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e1234 */
            1.0/this.flat_weight_norm_squared().anti_square_root().e1234
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product.e1234) * this.group0, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group2, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e1234) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group4
        );
    }
    func operator/(_rhs: UnitizeInfix) -> MultiVector {
        return this.unitize();
    }
    func operator/=(const UnitizeInfix& _rhs) -> MultiVector& {
        this = this.unitize();
        return *this;
    }
}
extension Origin: Unitize {
    associatedtype Output = Origin;
    func unitize() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e4 / (this.flat_weight_norm_squared().anti_square_root().e1234)
        );
    }
    func operator/(_rhs: UnitizeInfix) -> Origin {
        return this.unitize();
    }
    func operator/=(const UnitizeInfix& _rhs) -> Origin& {
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
    func operator/(_rhs: UnitizeInfix) -> Plane {
        return this.unitize();
    }
    func operator/=(const UnitizeInfix& _rhs) -> Plane& {
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
    func operator/(_rhs: UnitizeInfix) -> Point {
        return this.unitize();
    }
    func operator/=(const UnitizeInfix& _rhs) -> Point& {
        this = this.unitize();
        return *this;
    }
}

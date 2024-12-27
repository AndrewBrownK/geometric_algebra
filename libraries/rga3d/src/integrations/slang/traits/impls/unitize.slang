using traits::AntiSquareRoot;
using traits::FlatWeightNormSquared;
extension AntiScalar {
    func div(_rhs: unitize) -> 
AntiScalar {
        this.unitize()
    }
}
extension AntiScalar {
    func div=(const unitize& _rhs) -> AntiScalar& {
        this = this.unitize()
        return *this;
    }
}
extension AntiScalar: Unitize {
    func unitize() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 / (this.flat_weight_norm_squared().anti_square_root().e1234)
        );
    }
}
extension DualNum {
    func div(_rhs: unitize) -> 
DualNum {
        this.unitize()
    }
}
extension DualNum {
    func div=(const unitize& _rhs) -> DualNum& {
        this = this.unitize()
        return *this;
    }
}
extension DualNum: Unitize {
    func unitize() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0
        );
    }
}
extension Flector {
    func div(_rhs: unitize) -> 
Flector {
        this.unitize()
    }
}
extension Flector {
    func div=(const unitize& _rhs) -> Flector& {
        this = this.unitize()
        return *this;
    }
}
extension Flector: Unitize {
    func unitize() -> Flector {
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
extension Line {
    func div(_rhs: unitize) -> 
Line {
        this.unitize()
    }
}
extension Line {
    func div=(const unitize& _rhs) -> Line& {
        this = this.unitize()
        return *this;
    }
}
extension Line: Unitize {
    func unitize() -> Line {
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
}
extension Motor {
    func div(_rhs: unitize) -> 
Motor {
        this.unitize()
    }
}
extension Motor {
    func div=(const unitize& _rhs) -> Motor& {
        this = this.unitize()
        return *this;
    }
}
extension Motor: Unitize {
    func unitize() -> Motor {
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
extension MultiVector {
    func div(_rhs: unitize) -> 
MultiVector {
        this.unitize()
    }
}
extension MultiVector {
    func div=(const unitize& _rhs) -> MultiVector& {
        this = this.unitize()
        return *this;
    }
}
extension MultiVector: Unitize {
    func unitize() -> MultiVector {
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
}
extension Origin {
    func div(_rhs: unitize) -> 
Origin {
        this.unitize()
    }
}
extension Origin {
    func div=(const unitize& _rhs) -> Origin& {
        this = this.unitize()
        return *this;
    }
}
extension Origin: Unitize {
    func unitize() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 / (this.flat_weight_norm_squared().anti_square_root().e1234)
        );
    }
}
extension Plane {
    func div(_rhs: unitize) -> 
Plane {
        this.unitize()
    }
}
extension Plane {
    func div=(const unitize& _rhs) -> Plane& {
        this = this.unitize()
        return *this;
    }
}
extension Plane: Unitize {
    func unitize() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0
        );
    }
}
extension Point {
    func div(_rhs: unitize) -> 
Point {
        this.unitize()
    }
}
extension Point {
    func div=(const unitize& _rhs) -> Point& {
        this = this.unitize()
        return *this;
    }
}
extension Point: Unitize {
    func unitize() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/this.flat_weight_norm_squared().anti_square_root().e1234) * this.group0
        );
    }
}

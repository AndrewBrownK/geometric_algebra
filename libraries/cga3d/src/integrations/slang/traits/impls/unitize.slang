using traits::AntiSquareRoot;
using traits::RoundWeightNormSquared;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> AntiCircleRotor {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> AntiCircleRotor& {
        this = this.unitize();
        return *this;
    }
}
extension AntiCircleRotor: Unitize {
    associatedtype Output = AntiCircleRotor;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.round_weight_norm_squared().anti_square_root().e12345, 0.0, 0.0, 0.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(geometric_anti_product.e12345) * this.group2
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> AntiDipoleInversion {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.unitize();
        return *this;
    }
}
extension AntiDipoleInversion: Unitize {
    associatedtype Output = AntiDipoleInversion;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.round_weight_norm_squared().anti_square_root().e12345, 0.0, 0.0, 0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(geometric_anti_product.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(geometric_anti_product.e12345) * this.group3
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Circle {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Circle& {
        this = this.unitize();
        return *this;
    }
}
extension Circle: Unitize {
    associatedtype Output = Circle;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.round_weight_norm_squared().anti_square_root().e12345, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e235, e315, e125, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> CircleRotor {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> CircleRotor& {
        this = this.unitize();
        return *this;
    }
}
extension CircleRotor: Unitize {
    associatedtype Output = CircleRotor;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.round_weight_norm_squared().anti_square_root().e12345, 0.0, 0.0, 0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(geometric_anti_product.e12345) * this.group2
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Dipole {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Dipole& {
        this = this.unitize();
        return *this;
    }
}
extension Dipole: Unitize {
    associatedtype Output = Dipole;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.round_weight_norm_squared().anti_square_root().e12345, 0.0, 0.0, 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e15, e25, e35, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group2.xyz, 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> DipoleInversion {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.unitize();
        return *this;
    }
}
extension DipoleInversion: Unitize {
    associatedtype Output = DipoleInversion;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.round_weight_norm_squared().anti_square_root().e12345, 0.0, 0.0, 0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(geometric_anti_product.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product.e12345) * this.group3
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
            /* e12345, 0, 0, 0 */
            float4(1.0/this.round_weight_norm_squared().anti_square_root().e12345, 0.0, 0.0, 0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(geometric_anti_product.e12345) * this.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e5, 0, 0, 0 */
            float4(geometric_anti_product.e12345 * this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(geometric_anti_product.e12345) * this.group3, 
            /* e41, e42, e43, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group6, 
            /* e423, e431, e412, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(geometric_anti_product.e12345) * this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product.e12345) * this.group9, 
            /* e1234, 0, 0, 0 */
            float4(geometric_anti_product.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> RoundPoint {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.unitize();
        return *this;
    }
}
extension RoundPoint: Unitize {
    associatedtype Output = RoundPoint;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.round_weight_norm_squared().anti_square_root().e12345, 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(geometric_anti_product.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> Sphere {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> Sphere& {
        this = this.unitize();
        return *this;
    }
}
extension Sphere: Unitize {
    associatedtype Output = Sphere;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.round_weight_norm_squared().anti_square_root().e12345, 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(geometric_anti_product.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> VersorEven {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.unitize();
        return *this;
    }
}
extension VersorEven: Unitize {
    associatedtype Output = VersorEven;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.round_weight_norm_squared().anti_square_root().e12345, 0.0, 0.0, 0.0)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(geometric_anti_product.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(geometric_anti_product.e12345) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e12345) * this.group3
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: UnitizePrefixOrPostfix) -> VersorOdd {
        return this.unitize();
    }
    func operator/=(const UnitizePrefixOrPostfix& _rhs) -> VersorOdd& {
        this = this.unitize();
        return *this;
    }
}
extension VersorOdd: Unitize {
    associatedtype Output = VersorOdd;
    func unitize() -> Output {
        AntiScalar geometric_anti_product = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0/this.round_weight_norm_squared().anti_square_root().e12345, 0.0, 0.0, 0.0)
        );
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(geometric_anti_product.e12345) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(geometric_anti_product.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product.e12345) * this.group3
        );
    }
}

implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public AntiCircleRotor operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return rhs.unitize();
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.round_weight_norm_squared().anti_square_root().e12345
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(geometric_anti_product.e12345) * this.group2
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiDipoleInversion operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public AntiDipoleInversion operator /(AntiDipoleInversion rhs) {
        return rhs.unitize();
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.round_weight_norm_squared().anti_square_root().e12345
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(geometric_anti_product.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(geometric_anti_product.e12345) * this.group3
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public Circle operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Circle operator /(Circle rhs) {
        return rhs.unitize();
    }
}
public extension Circle {
    public Circle unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.round_weight_norm_squared().anti_square_root().e12345
        );
        return Circle(
            /* e423, e431, e412 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(geometric_anti_product.e12345) * this.group2.xyz
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public CircleRotor operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public CircleRotor operator /(CircleRotor rhs) {
        return rhs.unitize();
    }
}
public extension CircleRotor {
    public CircleRotor unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.round_weight_norm_squared().anti_square_root().e12345
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(geometric_anti_product.e12345) * this.group2
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Dipole operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Dipole operator /(Dipole rhs) {
        return rhs.unitize();
    }
}
public extension Dipole {
    public Dipole unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.round_weight_norm_squared().anti_square_root().e12345
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(geometric_anti_product.e12345) * this.group2.xyz
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public DipoleInversion operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public DipoleInversion operator /(DipoleInversion rhs) {
        return rhs.unitize();
    }
}
public extension DipoleInversion {
    public DipoleInversion unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.round_weight_norm_squared().anti_square_root().e12345
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(geometric_anti_product.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product.e12345) * this.group3
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
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.round_weight_norm_squared().anti_square_root().e12345
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(geometric_anti_product.e12345) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e5 */
            geometric_anti_product.e12345 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(geometric_anti_product.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(geometric_anti_product.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(geometric_anti_product.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product.e12345) * this.group9, 
            /* e1234 */
            geometric_anti_product.e12345 * this.e1234
        );
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public RoundPoint operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(RoundPoint rhs) {
        return rhs.unitize();
    }
}
public extension RoundPoint {
    public RoundPoint unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.round_weight_norm_squared().anti_square_root().e12345
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e12345) * this.group0, 
            /* e5 */
            geometric_anti_product.e12345 * this.e5
        );
    }
}
public extension Sphere {
    // Fancy postfix trick
    public Sphere operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(Sphere rhs) {
        return rhs.unitize();
    }
}
public extension Sphere {
    public Sphere unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.round_weight_norm_squared().anti_square_root().e12345
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product.e12345) * this.group0, 
            /* e1234 */
            geometric_anti_product.e12345 * this.e1234
        );
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public VersorEven operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public VersorEven operator /(VersorEven rhs) {
        return rhs.unitize();
    }
}
public extension VersorEven {
    public VersorEven unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.round_weight_norm_squared().anti_square_root().e12345
        );
        return VersorEven(
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
public extension VersorOdd {
    // Fancy postfix trick
    public VersorOdd operator /(UnitizePrefixOrPostfix rhs)  {
        return this.unitize();
    }
}
public extension UnitizePrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(VersorOdd rhs) {
        return rhs.unitize();
    }
}
public extension VersorOdd {
    public VersorOdd unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.round_weight_norm_squared().anti_square_root().e12345
        );
        return VersorOdd(
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

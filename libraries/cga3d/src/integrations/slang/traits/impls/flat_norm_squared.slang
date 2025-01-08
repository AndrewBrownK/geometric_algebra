implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(AntiCircleRotor rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension AntiCircleRotor {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(AntiDipoleInversion rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension AntiDipoleInversion {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(Circle rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension Circle {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(CircleRotor rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension CircleRotor {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(Dipole rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension Dipole {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(DipoleInversion rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension DipoleInversion {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(DualNum rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension DualNum {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(FlatPoint rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension FlatPoint {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(Flector rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension Flector {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(Line rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension Line {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(Motor rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension Motor {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension MultiVector {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(Plane rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension Plane {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Sphere {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(Sphere rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension Sphere {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(VersorEven rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension VersorEven {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public MultiVector operator /(FlatNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_norm_squared();
    }
}
public extension FlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(VersorOdd rhs) {
        return rhs.flat_norm_squared();
    }
}
public extension VersorOdd {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm_squared().scalar, this.flat_weight_norm_squared().e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}

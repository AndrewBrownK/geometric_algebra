implementing cga3d;
using traits;
// Fancy postfix trick
public MultiVector operator /(AntiCircleRotor lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(AntiDipoleInversion lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(Circle lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(CircleRotor lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(Dipole lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(DipoleInversion lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(DualNum lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(FlatPoint lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(Flector lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(Line lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(Motor lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(Plane lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(Sphere lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(VersorEven lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_norm_squared();
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
// Fancy postfix trick
public MultiVector operator /(VersorOdd lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_norm_squared();
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

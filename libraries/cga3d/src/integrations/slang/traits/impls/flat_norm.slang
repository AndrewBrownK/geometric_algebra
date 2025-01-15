implementing cga3d;
using traits;
// Fancy postfix trick
public MultiVector operator /(AntiCircleRotor lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_norm();
}
public extension AntiCircleRotor {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(AntiDipoleInversion lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_norm();
}
public extension AntiDipoleInversion {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(Circle lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_norm();
}
public extension Circle {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(CircleRotor lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_norm();
}
public extension CircleRotor {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(Dipole lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_norm();
}
public extension Dipole {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(DipoleInversion lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_norm();
}
public extension DipoleInversion {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(DualNum lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_norm();
}
public extension DualNum {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(FlatPoint lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_norm();
}
public extension FlatPoint {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(Flector lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_norm();
}
public extension Flector {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(Line lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_norm();
}
public extension Line {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(Motor lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_norm();
}
public extension Motor {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(MultiVector lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_norm();
}
public extension MultiVector {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(Plane lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_norm();
}
public extension Plane {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(Sphere lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_norm();
}
public extension Sphere {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(VersorEven lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_norm();
}
public extension VersorEven {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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
public MultiVector operator /(VersorOdd lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_norm();
}
public extension VersorOdd {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.flat_bulk_norm().scalar, this.flat_weight_norm().e12345), 
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

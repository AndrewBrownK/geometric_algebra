implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public Dipole operator /(RoundWeightPrefixOrPostfix rhs)  {
        return this.round_weight();
    }
}
public extension RoundWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Dipole operator /(AntiCircleRotor rhs) {
        return rhs.round_weight();
    }
}
public extension AntiCircleRotor {
    public Dipole round_weight() {
        return Dipole(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiDipoleInversion operator /(RoundWeightPrefixOrPostfix rhs)  {
        return this.round_weight();
    }
}
public extension RoundWeightPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDipoleInversion operator /(AntiDipoleInversion rhs) {
        return rhs.round_weight();
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion round_weight() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), this.e4), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public Circle operator /(RoundWeightPrefixOrPostfix rhs)  {
        return this.round_weight();
    }
}
public extension RoundWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Circle operator /(Circle rhs) {
        return rhs.round_weight();
    }
}
public extension Circle {
    public Circle round_weight() {
        return Circle(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public Circle operator /(RoundWeightPrefixOrPostfix rhs)  {
        return this.round_weight();
    }
}
public extension RoundWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Circle operator /(CircleRotor rhs) {
        return rhs.round_weight();
    }
}
public extension CircleRotor {
    public Circle round_weight() {
        return Circle(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Dipole operator /(RoundWeightPrefixOrPostfix rhs)  {
        return this.round_weight();
    }
}
public extension RoundWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Dipole operator /(Dipole rhs) {
        return rhs.round_weight();
    }
}
public extension Dipole {
    public Dipole round_weight() {
        return Dipole(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public DipoleInversion operator /(RoundWeightPrefixOrPostfix rhs)  {
        return this.round_weight();
    }
}
public extension RoundWeightPrefixOrPostfix {
    // Fancy prefix trick
    public DipoleInversion operator /(DipoleInversion rhs) {
        return rhs.round_weight();
    }
}
public extension DipoleInversion {
    public DipoleInversion round_weight() {
        return DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(RoundWeightPrefixOrPostfix rhs)  {
        return this.round_weight();
    }
}
public extension RoundWeightPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.round_weight();
    }
}
public extension MultiVector {
    public MultiVector round_weight() {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            this.e1234
        );
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public RoundPoint operator /(RoundWeightPrefixOrPostfix rhs)  {
        return this.round_weight();
    }
}
public extension RoundWeightPrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(RoundPoint rhs) {
        return rhs.round_weight();
    }
}
public extension RoundPoint {
    public RoundPoint round_weight() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e5 */
            0.0
        );
    }
}
public extension Sphere {
    // Fancy postfix trick
    public Sphere operator /(RoundWeightPrefixOrPostfix rhs)  {
        return this.round_weight();
    }
}
public extension RoundWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(Sphere rhs) {
        return rhs.round_weight();
    }
}
public extension Sphere {
    public Sphere round_weight() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            this.e1234
        );
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public AntiDipoleInversion operator /(RoundWeightPrefixOrPostfix rhs)  {
        return this.round_weight();
    }
}
public extension RoundWeightPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDipoleInversion operator /(VersorEven rhs) {
        return rhs.round_weight();
    }
}
public extension VersorEven {
    public AntiDipoleInversion round_weight() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), this.e4), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public DipoleInversion operator /(RoundWeightPrefixOrPostfix rhs)  {
        return this.round_weight();
    }
}
public extension RoundWeightPrefixOrPostfix {
    // Fancy prefix trick
    public DipoleInversion operator /(VersorOdd rhs) {
        return rhs.round_weight();
    }
}
public extension VersorOdd {
    public DipoleInversion round_weight() {
        return DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}

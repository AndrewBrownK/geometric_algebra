implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public FlatPoint operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(AntiCircleRotor rhs) {
        return rhs.flat_weight();
    }
}
public extension AntiCircleRotor {
    public FlatPoint flat_weight() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45)
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public Line operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(AntiDipoleInversion rhs) {
        return rhs.flat_weight();
    }
}
public extension AntiDipoleInversion {
    public Line flat_weight() {
        return Line(
            /* e415, e425, e435 */
            this.group1.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.flat_weight();
    }
}
public extension AntiScalar {
    public AntiScalar flat_weight() {
        return this;
    }
}
public extension Circle {
    // Fancy postfix trick
    public Line operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Circle rhs) {
        return rhs.flat_weight();
    }
}
public extension Circle {
    public Line flat_weight() {
        return Line(
            /* e415, e425, e435 */
            this.group1.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public Motor operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(CircleRotor rhs) {
        return rhs.flat_weight();
    }
}
public extension CircleRotor {
    public Motor flat_weight() {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e415, this.e425, this.e435, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public FlatPoint operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(Dipole rhs) {
        return rhs.flat_weight();
    }
}
public extension Dipole {
    public FlatPoint flat_weight() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45)
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public Flector operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(DipoleInversion rhs) {
        return rhs.flat_weight();
    }
}
public extension DipoleInversion {
    public Flector flat_weight() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, 0.0)
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public AntiScalar operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DualNum rhs) {
        return rhs.flat_weight();
    }
}
public extension DualNum {
    public AntiScalar flat_weight() {
        return AntiScalar(
            /* e12345 */
            this.e12345
        );
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(FlatPoint rhs) {
        return rhs.flat_weight();
    }
}
public extension FlatPoint {
    public FlatPoint flat_weight() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45)
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.flat_weight();
    }
}
public extension Flector {
    public Flector flat_weight() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.flat_weight();
    }
}
public extension Line {
    public Line flat_weight() {
        return Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.flat_weight();
    }
}
public extension Motor {
    public Motor flat_weight() {
        return Motor(
            /* e415, e425, e435, e12345 */
            this.group0, 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.flat_weight();
    }
}
public extension MultiVector {
    public MultiVector flat_weight() {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group6.xyz, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group9.xyz, 0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.flat_weight();
    }
}
public extension Plane {
    public Plane flat_weight() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Sphere {
    // Fancy postfix trick
    public Plane operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Sphere rhs) {
        return rhs.flat_weight();
    }
}
public extension Sphere {
    public Plane flat_weight() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public Motor operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(VersorEven rhs) {
        return rhs.flat_weight();
    }
}
public extension VersorEven {
    public Motor flat_weight() {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e415, this.e425, this.e435, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public Flector operator /(FlatWeightPrefixOrPostfix rhs)  {
        return this.flat_weight();
    }
}
public extension FlatWeightPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(VersorOdd rhs) {
        return rhs.flat_weight();
    }
}
public extension VersorOdd {
    public Flector flat_weight() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, 0.0)
        );
    }
}

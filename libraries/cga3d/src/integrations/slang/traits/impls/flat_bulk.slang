implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public FlatPoint operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(AntiCircleRotor rhs) {
        return rhs.flat_bulk();
    }
}
public extension AntiCircleRotor {
    public FlatPoint flat_bulk() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiFlector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(AntiDipoleInversion rhs) {
        return rhs.flat_bulk();
    }
}
public extension AntiDipoleInversion {
    public AntiFlector flat_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public AntiDualNum operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(AntiDualNum rhs) {
        return rhs.flat_bulk();
    }
}
public extension AntiDualNum {
    public AntiDualNum flat_bulk() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e3215, 0.0)
        );
    }
}
public extension AntiFlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.flat_bulk();
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint flat_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public AntiFlector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(AntiFlector rhs) {
        return rhs.flat_bulk();
    }
}
public extension AntiFlector {
    public AntiFlector flat_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public FlatPoint operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(AntiLine rhs) {
        return rhs.flat_bulk();
    }
}
public extension AntiLine {
    public FlatPoint flat_bulk() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiMotor {
    // Fancy postfix trick
    public Flector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(AntiMotor rhs) {
        return rhs.flat_bulk();
    }
}
public extension AntiMotor {
    public Flector flat_bulk() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public DualNum operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(AntiPlane rhs) {
        return rhs.flat_bulk();
    }
}
public extension AntiPlane {
    public DualNum flat_bulk() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e5, 0.0)
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public AntiFlatPoint operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(Circle rhs) {
        return rhs.flat_bulk();
    }
}
public extension Circle {
    public AntiFlatPoint flat_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public AntiFlatPoint operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(CircleRotor rhs) {
        return rhs.flat_bulk();
    }
}
public extension CircleRotor {
    public AntiFlatPoint flat_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public FlatPoint operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(Dipole rhs) {
        return rhs.flat_bulk();
    }
}
public extension Dipole {
    public FlatPoint flat_bulk() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public Flector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(DipoleInversion rhs) {
        return rhs.flat_bulk();
    }
}
public extension DipoleInversion {
    public Flector flat_bulk() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.flat_bulk();
    }
}
public extension DualNum {
    public DualNum flat_bulk() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e5, 0.0)
        );
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(FlatPoint rhs) {
        return rhs.flat_bulk();
    }
}
public extension FlatPoint {
    public FlatPoint flat_bulk() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.flat_bulk();
    }
}
public extension Flector {
    public Flector flat_bulk() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public AntiFlatPoint operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(Line rhs) {
        return rhs.flat_bulk();
    }
}
public extension Line {
    public AntiFlatPoint flat_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public AntiFlector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(Motor rhs) {
        return rhs.flat_bulk();
    }
}
public extension Motor {
    public AntiFlector flat_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.flat_bulk();
    }
}
public extension MultiVector {
    public MultiVector flat_bulk() {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(this.group3.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public AntiDualNum operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(Plane rhs) {
        return rhs.flat_bulk();
    }
}
public extension Plane {
    public AntiDualNum flat_bulk() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e3215, 0.0)
        );
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public DualNum operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(RoundPoint rhs) {
        return rhs.flat_bulk();
    }
}
public extension RoundPoint {
    public DualNum flat_bulk() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e5, 0.0)
        );
    }
}
public extension Sphere {
    // Fancy postfix trick
    public AntiDualNum operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(Sphere rhs) {
        return rhs.flat_bulk();
    }
}
public extension Sphere {
    public AntiDualNum flat_bulk() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e3215, 0.0)
        );
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public AntiFlector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(VersorEven rhs) {
        return rhs.flat_bulk();
    }
}
public extension VersorEven {
    public AntiFlector flat_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public Flector operator /(FlatBulkPrefixOrPostfix rhs)  {
        return this.flat_bulk();
    }
}
public extension FlatBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(VersorOdd rhs) {
        return rhs.flat_bulk();
    }
}
public extension VersorOdd {
    public Flector flat_bulk() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}

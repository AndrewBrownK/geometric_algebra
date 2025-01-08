implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public CircleRotor operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public CircleRotor operator /(AntiCircleRotor rhs) {
        return rhs.right_dual();
    }
}
public extension AntiCircleRotor {
    public CircleRotor right_dual() {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public DipoleInversion operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public DipoleInversion operator /(AntiDipoleInversion rhs) {
        return rhs.right_dual();
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion right_dual() {
        return DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public DualNum operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(AntiDualNum rhs) {
        return rhs.right_dual();
    }
}
public extension AntiDualNum {
    public DualNum right_dual() {
        return DualNum(
            /* e5, e12345 */
            this.group0.xy
        );
    }
}
public extension AntiFlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.right_dual();
    }
}
public extension AntiFlatPoint {
    public FlatPoint right_dual() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public Flector operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(AntiFlector rhs) {
        return rhs.right_dual();
    }
}
public extension AntiFlector {
    public Flector right_dual() {
        return Flector(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public Line operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(AntiLine rhs) {
        return rhs.right_dual();
    }
}
public extension AntiLine {
    public Line right_dual() {
        return Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
public extension AntiMotor {
    // Fancy postfix trick
    public Motor operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(AntiMotor rhs) {
        return rhs.right_dual();
    }
}
public extension AntiMotor {
    public Motor right_dual() {
        return Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public Plane operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(AntiPlane rhs) {
        return rhs.right_dual();
    }
}
public extension AntiPlane {
    public Plane right_dual() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension AntiScalar {
    // Fancy postfix trick
    public Scalar operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiScalar rhs) {
        return rhs.right_dual();
    }
}
public extension AntiScalar {
    public Scalar right_dual() {
        return Scalar(
            /* scalar */
            this.e12345 * -1.0
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public Dipole operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Dipole operator /(Circle rhs) {
        return rhs.right_dual();
    }
}
public extension Circle {
    public Dipole right_dual() {
        return Dipole(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            this.group2.xyz
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public AntiCircleRotor operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public AntiCircleRotor operator /(CircleRotor rhs) {
        return rhs.right_dual();
    }
}
public extension CircleRotor {
    public AntiCircleRotor right_dual() {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Circle operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Circle operator /(Dipole rhs) {
        return rhs.right_dual();
    }
}
public extension Dipole {
    public Circle right_dual() {
        return Circle(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            this.group2.xyz * float3(-1.0)
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public AntiDipoleInversion operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDipoleInversion operator /(DipoleInversion rhs) {
        return rhs.right_dual();
    }
}
public extension DipoleInversion {
    public AntiDipoleInversion right_dual() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public AntiDualNum operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(DualNum rhs) {
        return rhs.right_dual();
    }
}
public extension DualNum {
    public AntiDualNum right_dual() {
        return AntiDualNum(
            /* e3215, scalar */
            this.group0.xy * float2(-1.0)
        );
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(FlatPoint rhs) {
        return rhs.right_dual();
    }
}
public extension FlatPoint {
    public AntiFlatPoint right_dual() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public AntiFlector operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(Flector rhs) {
        return rhs.right_dual();
    }
}
public extension Flector {
    public AntiFlector right_dual() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public AntiLine operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public AntiLine operator /(Line rhs) {
        return rhs.right_dual();
    }
}
public extension Line {
    public AntiLine right_dual() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz, 
            /* e15, e25, e35 */
            this.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public AntiMotor operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public AntiMotor operator /(Motor rhs) {
        return rhs.right_dual();
    }
}
public extension Motor {
    public AntiMotor right_dual() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.right_dual();
    }
}
public extension MultiVector {
    public MultiVector right_dual() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.group9.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            this.e3215, 
            /* e15, e25, e35, e45 */
            float4(this.group8.xyz, this.e321 * -1.0), 
            /* e41, e42, e43 */
            this.group7.xyz, 
            /* e23, e31, e12 */
            this.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group5.xyz, this.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            this.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.e4 * -1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public AntiPlane operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(Plane rhs) {
        return rhs.right_dual();
    }
}
public extension Plane {
    public AntiPlane right_dual() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public Sphere operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(RoundPoint rhs) {
        return rhs.right_dual();
    }
}
public extension RoundPoint {
    public Sphere right_dual() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.e4 * -1.0
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public AntiScalar operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Scalar rhs) {
        return rhs.right_dual();
    }
}
public extension Scalar {
    public AntiScalar right_dual() {
        return AntiScalar(
            /* e12345 */
            this.scalar
        );
    }
}
public extension Sphere {
    // Fancy postfix trick
    public RoundPoint operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(Sphere rhs) {
        return rhs.right_dual();
    }
}
public extension Sphere {
    public RoundPoint right_dual() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            this.e3215
        );
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public VersorOdd operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(VersorEven rhs) {
        return rhs.right_dual();
    }
}
public extension VersorEven {
    public VersorOdd right_dual() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, this.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public VersorEven operator /(RightDualPrefixOrPostfix rhs)  {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public VersorEven operator /(VersorOdd rhs) {
        return rhs.right_dual();
    }
}
public extension VersorOdd {
    public VersorEven right_dual() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.group3.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}

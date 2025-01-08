implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public AntiCircleRotor operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor conformal_conjugate() {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiDipoleInversion operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public AntiDipoleInversion operator /(AntiDipoleInversion rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion conformal_conjugate() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public AntiDualNum operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(AntiDualNum rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension AntiDualNum {
    public AntiDualNum conformal_conjugate() {
        return AntiDualNum(
            /* e3215, scalar */
            this.group0.xy * float2(-1.0, 1.0)
        );
    }
}
public extension AntiFlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint conformal_conjugate() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public AntiFlector operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(AntiFlector rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension AntiFlector {
    public AntiFlector conformal_conjugate() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public AntiLine operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public AntiLine operator /(AntiLine rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension AntiLine {
    public AntiLine conformal_conjugate() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz, 
            /* e15, e25, e35 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
public extension AntiMotor {
    // Fancy postfix trick
    public AntiMotor operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public AntiMotor operator /(AntiMotor rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension AntiMotor {
    public AntiMotor conformal_conjugate() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0, 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public AntiPlane operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(AntiPlane rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension AntiPlane {
    public AntiPlane conformal_conjugate() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension AntiScalar {
    public AntiScalar conformal_conjugate() {
        return AntiScalar(
            /* e12345 */
            this.e12345 * -1.0
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public Circle operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public Circle operator /(Circle rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension Circle {
    public Circle conformal_conjugate() {
        return Circle(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            this.group2.xyz * float3(-1.0)
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public CircleRotor operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public CircleRotor operator /(CircleRotor rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension CircleRotor {
    public CircleRotor conformal_conjugate() {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0)
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Dipole operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public Dipole operator /(Dipole rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension Dipole {
    public Dipole conformal_conjugate() {
        return Dipole(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            this.group2.xyz * float3(-1.0)
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public DipoleInversion operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public DipoleInversion operator /(DipoleInversion rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension DipoleInversion {
    public DipoleInversion conformal_conjugate() {
        return DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension DualNum {
    public DualNum conformal_conjugate() {
        return DualNum(
            /* e5, e12345 */
            this.group0.xy * float2(-1.0)
        );
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(FlatPoint rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension FlatPoint {
    public FlatPoint conformal_conjugate() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension Flector {
    public Flector conformal_conjugate() {
        return Flector(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension Line {
    public Line conformal_conjugate() {
        return Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension Motor {
    public Motor conformal_conjugate() {
        return Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension MultiVector {
    public MultiVector conformal_conjugate() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            this.group3 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 * float4(-1.0), 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension Plane {
    public Plane conformal_conjugate() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public RoundPoint operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(RoundPoint rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension RoundPoint {
    public RoundPoint conformal_conjugate() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e5 */
            this.e5 * -1.0
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension Scalar {
    public Scalar conformal_conjugate() {
        return this;
    }
}
public extension Sphere {
    // Fancy postfix trick
    public Sphere operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(Sphere rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension Sphere {
    public Sphere conformal_conjugate() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0), 
            /* e1234 */
            this.e1234
        );
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public VersorEven operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public VersorEven operator /(VersorEven rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension VersorEven {
    public VersorEven conformal_conjugate() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public VersorOdd operator /(ConformalConjugatePrefixOrPostfix rhs)  {
        return this.conformal_conjugate();
    }
}
public extension ConformalConjugatePrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(VersorOdd rhs) {
        return rhs.conformal_conjugate();
    }
}
public extension VersorOdd {
    public VersorOdd conformal_conjugate() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}

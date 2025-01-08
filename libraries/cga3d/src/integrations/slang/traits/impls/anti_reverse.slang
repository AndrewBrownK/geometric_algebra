implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public AntiCircleRotor operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return rhs.anti_reverse();
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reverse() {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiDipoleInversion operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiDipoleInversion operator /(AntiDipoleInversion rhs) {
        return rhs.anti_reverse();
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reverse() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public AntiDualNum operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(AntiDualNum rhs) {
        return rhs.anti_reverse();
    }
}
public extension AntiDualNum {
    public AntiDualNum anti_reverse() {
        return this;
    }
}
public extension AntiFlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.anti_reverse();
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_reverse() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public AntiFlector operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(AntiFlector rhs) {
        return rhs.anti_reverse();
    }
}
public extension AntiFlector {
    public AntiFlector anti_reverse() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public AntiLine operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiLine operator /(AntiLine rhs) {
        return rhs.anti_reverse();
    }
}
public extension AntiLine {
    public AntiLine anti_reverse() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
public extension AntiMotor {
    // Fancy postfix trick
    public AntiMotor operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiMotor operator /(AntiMotor rhs) {
        return rhs.anti_reverse();
    }
}
public extension AntiMotor {
    public AntiMotor anti_reverse() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public AntiPlane operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(AntiPlane rhs) {
        return rhs.anti_reverse();
    }
}
public extension AntiPlane {
    public AntiPlane anti_reverse() {
        return this;
    }
}
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.anti_reverse();
    }
}
public extension AntiScalar {
    public AntiScalar anti_reverse() {
        return this;
    }
}
public extension Circle {
    // Fancy postfix trick
    public Circle operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Circle operator /(Circle rhs) {
        return rhs.anti_reverse();
    }
}
public extension Circle {
    public Circle anti_reverse() {
        return Circle(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125 */
            this.group2.xyz * float3(-1.0)
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public CircleRotor operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public CircleRotor operator /(CircleRotor rhs) {
        return rhs.anti_reverse();
    }
}
public extension CircleRotor {
    public CircleRotor anti_reverse() {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Dipole operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Dipole operator /(Dipole rhs) {
        return rhs.anti_reverse();
    }
}
public extension Dipole {
    public Dipole anti_reverse() {
        return Dipole(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35 */
            this.group2.xyz * float3(-1.0)
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public DipoleInversion operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public DipoleInversion operator /(DipoleInversion rhs) {
        return rhs.anti_reverse();
    }
}
public extension DipoleInversion {
    public DipoleInversion anti_reverse() {
        return DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.anti_reverse();
    }
}
public extension DualNum {
    public DualNum anti_reverse() {
        return this;
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(FlatPoint rhs) {
        return rhs.anti_reverse();
    }
}
public extension FlatPoint {
    public FlatPoint anti_reverse() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.anti_reverse();
    }
}
public extension Flector {
    public Flector anti_reverse() {
        return Flector(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.anti_reverse();
    }
}
public extension Line {
    public Line anti_reverse() {
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
    public Motor operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.anti_reverse();
    }
}
public extension Motor {
    public Motor anti_reverse() {
        return Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.anti_reverse();
    }
}
public extension MultiVector {
    public MultiVector anti_reverse() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group5.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group6 * float4(-1.0), 
            /* e423, e431, e412 */
            this.group7.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group8.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.anti_reverse();
    }
}
public extension Plane {
    public Plane anti_reverse() {
        return this;
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public RoundPoint operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(RoundPoint rhs) {
        return rhs.anti_reverse();
    }
}
public extension RoundPoint {
    public RoundPoint anti_reverse() {
        return this;
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.anti_reverse();
    }
}
public extension Scalar {
    public Scalar anti_reverse() {
        return this;
    }
}
public extension Sphere {
    // Fancy postfix trick
    public Sphere operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(Sphere rhs) {
        return rhs.anti_reverse();
    }
}
public extension Sphere {
    public Sphere anti_reverse() {
        return this;
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public VersorEven operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public VersorEven operator /(VersorEven rhs) {
        return rhs.anti_reverse();
    }
}
public extension VersorEven {
    public VersorEven anti_reverse() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public VersorOdd operator /(AntiReversePrefixOrPostfix rhs)  {
        return this.anti_reverse();
    }
}
public extension AntiReversePrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(VersorOdd rhs) {
        return rhs.anti_reverse();
    }
}
public extension VersorOdd {
    public VersorOdd anti_reverse() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}

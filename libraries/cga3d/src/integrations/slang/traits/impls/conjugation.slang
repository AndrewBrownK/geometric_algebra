implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public AntiCircleRotor operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return rhs.conjugation();
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor conjugation() {
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
    public AntiDipoleInversion operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDipoleInversion operator /(AntiDipoleInversion rhs) {
        return rhs.conjugation();
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion conjugation() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public AntiDualNum operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(AntiDualNum rhs) {
        return rhs.conjugation();
    }
}
public extension AntiDualNum {
    public AntiDualNum conjugation() {
        return this;
    }
}
public extension AntiFlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.conjugation();
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint conjugation() {
        return this;
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public AntiFlector operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(AntiFlector rhs) {
        return rhs.conjugation();
    }
}
public extension AntiFlector {
    public AntiFlector conjugation() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0, 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public AntiLine operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiLine operator /(AntiLine rhs) {
        return rhs.conjugation();
    }
}
public extension AntiLine {
    public AntiLine conjugation() {
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
    public AntiMotor operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiMotor operator /(AntiMotor rhs) {
        return rhs.conjugation();
    }
}
public extension AntiMotor {
    public AntiMotor conjugation() {
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
    public AntiPlane operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(AntiPlane rhs) {
        return rhs.conjugation();
    }
}
public extension AntiPlane {
    public AntiPlane conjugation() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.conjugation();
    }
}
public extension AntiScalar {
    public AntiScalar conjugation() {
        return AntiScalar(
            /* e12345 */
            this.e12345 * -1.0
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public Circle operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Circle operator /(Circle rhs) {
        return rhs.conjugation();
    }
}
public extension Circle {
    public Circle conjugation() {
        return this;
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public CircleRotor operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public CircleRotor operator /(CircleRotor rhs) {
        return rhs.conjugation();
    }
}
public extension CircleRotor {
    public CircleRotor conjugation() {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Dipole operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Dipole operator /(Dipole rhs) {
        return rhs.conjugation();
    }
}
public extension Dipole {
    public Dipole conjugation() {
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
    public DipoleInversion operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public DipoleInversion operator /(DipoleInversion rhs) {
        return rhs.conjugation();
    }
}
public extension DipoleInversion {
    public DipoleInversion conjugation() {
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
    public DualNum operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.conjugation();
    }
}
public extension DualNum {
    public DualNum conjugation() {
        return DualNum(
            /* e5, e12345 */
            this.group0.xy * float2(-1.0)
        );
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(FlatPoint rhs) {
        return rhs.conjugation();
    }
}
public extension FlatPoint {
    public FlatPoint conjugation() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.conjugation();
    }
}
public extension Flector {
    public Flector conjugation() {
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
    public Line operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.conjugation();
    }
}
public extension Line {
    public Line conjugation() {
        return this;
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.conjugation();
    }
}
public extension Motor {
    public Motor conjugation() {
        return Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.conjugation();
    }
}
public extension MultiVector {
    public MultiVector conjugation() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e5 */
            this.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            this.group3 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group5.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.conjugation();
    }
}
public extension Plane {
    public Plane conjugation() {
        return this;
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public RoundPoint operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(RoundPoint rhs) {
        return rhs.conjugation();
    }
}
public extension RoundPoint {
    public RoundPoint conjugation() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e5 */
            this.e5 * -1.0
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.conjugation();
    }
}
public extension Scalar {
    public Scalar conjugation() {
        return this;
    }
}
public extension Sphere {
    // Fancy postfix trick
    public Sphere operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(Sphere rhs) {
        return rhs.conjugation();
    }
}
public extension Sphere {
    public Sphere conjugation() {
        return this;
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public VersorEven operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public VersorEven operator /(VersorEven rhs) {
        return rhs.conjugation();
    }
}
public extension VersorEven {
    public VersorEven conjugation() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1, e2, e3, e4 */
            this.group3 * float4(-1.0)
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public VersorOdd operator /(ConjugationPrefixOrPostfix rhs)  {
        return this.conjugation();
    }
}
public extension ConjugationPrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(VersorOdd rhs) {
        return rhs.conjugation();
    }
}
public extension VersorOdd {
    public VersorOdd conjugation() {
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

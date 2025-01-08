implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public AntiCircleRotor operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return rhs.auto_morphism();
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor auto_morphism() {
        return this;
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiDipoleInversion operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDipoleInversion operator /(AntiDipoleInversion rhs) {
        return rhs.auto_morphism();
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion auto_morphism() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0)
        );
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public AntiDualNum operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(AntiDualNum rhs) {
        return rhs.auto_morphism();
    }
}
public extension AntiDualNum {
    public AntiDualNum auto_morphism() {
        return this;
    }
}
public extension AntiFlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.auto_morphism();
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint auto_morphism() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public AntiFlector operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(AntiFlector rhs) {
        return rhs.auto_morphism();
    }
}
public extension AntiFlector {
    public AntiFlector auto_morphism() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public AntiLine operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public AntiLine operator /(AntiLine rhs) {
        return rhs.auto_morphism();
    }
}
public extension AntiLine {
    public AntiLine auto_morphism() {
        return this;
    }
}
public extension AntiMotor {
    // Fancy postfix trick
    public AntiMotor operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public AntiMotor operator /(AntiMotor rhs) {
        return rhs.auto_morphism();
    }
}
public extension AntiMotor {
    public AntiMotor auto_morphism() {
        return this;
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public AntiPlane operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(AntiPlane rhs) {
        return rhs.auto_morphism();
    }
}
public extension AntiPlane {
    public AntiPlane auto_morphism() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.auto_morphism();
    }
}
public extension AntiScalar {
    public AntiScalar auto_morphism() {
        return AntiScalar(
            /* e12345 */
            this.e12345 * -1.0
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public Circle operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Circle operator /(Circle rhs) {
        return rhs.auto_morphism();
    }
}
public extension Circle {
    public Circle auto_morphism() {
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
    public CircleRotor operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public CircleRotor operator /(CircleRotor rhs) {
        return rhs.auto_morphism();
    }
}
public extension CircleRotor {
    public CircleRotor auto_morphism() {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0)
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Dipole operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Dipole operator /(Dipole rhs) {
        return rhs.auto_morphism();
    }
}
public extension Dipole {
    public Dipole auto_morphism() {
        return this;
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public DipoleInversion operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public DipoleInversion operator /(DipoleInversion rhs) {
        return rhs.auto_morphism();
    }
}
public extension DipoleInversion {
    public DipoleInversion auto_morphism() {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.auto_morphism();
    }
}
public extension DualNum {
    public DualNum auto_morphism() {
        return DualNum(
            /* e5, e12345 */
            this.group0.xy * float2(-1.0)
        );
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(FlatPoint rhs) {
        return rhs.auto_morphism();
    }
}
public extension FlatPoint {
    public FlatPoint auto_morphism() {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.auto_morphism();
    }
}
public extension Flector {
    public Flector auto_morphism() {
        return this;
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.auto_morphism();
    }
}
public extension Line {
    public Line auto_morphism() {
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
    public Motor operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.auto_morphism();
    }
}
public extension Motor {
    public Motor auto_morphism() {
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
    public MultiVector operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.auto_morphism();
    }
}
public extension MultiVector {
    public MultiVector auto_morphism() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e5 */
            this.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
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
    public Plane operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.auto_morphism();
    }
}
public extension Plane {
    public Plane auto_morphism() {
        return this;
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public RoundPoint operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(RoundPoint rhs) {
        return rhs.auto_morphism();
    }
}
public extension RoundPoint {
    public RoundPoint auto_morphism() {
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
    public Scalar operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.auto_morphism();
    }
}
public extension Scalar {
    public Scalar auto_morphism() {
        return this;
    }
}
public extension Sphere {
    // Fancy postfix trick
    public Sphere operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(Sphere rhs) {
        return rhs.auto_morphism();
    }
}
public extension Sphere {
    public Sphere auto_morphism() {
        return this;
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public VersorEven operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public VersorEven operator /(VersorEven rhs) {
        return rhs.auto_morphism();
    }
}
public extension VersorEven {
    public VersorEven auto_morphism() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0), 
            /* e1, e2, e3, e4 */
            this.group3 * float4(-1.0)
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public VersorOdd operator /(AutoMorphismPrefixOrPostfix rhs)  {
        return this.auto_morphism();
    }
}
public extension AutoMorphismPrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(VersorOdd rhs) {
        return rhs.auto_morphism();
    }
}
public extension VersorOdd {
    public VersorOdd auto_morphism() {
        return this;
    }
}

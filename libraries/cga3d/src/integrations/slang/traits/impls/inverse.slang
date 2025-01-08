implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public AntiCircleRotor operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiCircleRotor operator /(AntiCircleRotor rhs) {
        return rhs.inverse();
    }
}
public extension AntiCircleRotor {
    public AntiCircleRotor inverse() {
        AntiCircleRotor reverse = AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.scalar, 2) - pow(this.e45, 2)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * reverse.group1, 
            /* e15, e25, e35, scalar */
            float4(other.scalar) * reverse.group2
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiDipoleInversion operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiDipoleInversion operator /(AntiDipoleInversion rhs) {
        return rhs.inverse();
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion inverse() {
        AntiDipoleInversion reverse = AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
        Scalar other = Scalar(
            /* scalar */
            pow(this.e321, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e4 * this.e5)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.scalar) * reverse.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * reverse.group1, 
            /* e235, e315, e125, e4 */
            float4(other.scalar) * reverse.group2, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * reverse.group3
        );
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public AntiDualNum operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiDualNum operator /(AntiDualNum rhs) {
        return rhs.inverse();
    }
}
public extension AntiDualNum {
    public AntiDualNum inverse() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(pow(this.scalar, -2)) * this.group0.xy
        );
    }
}
public extension AntiFlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.inverse();
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint inverse() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(pow(this.e321, -2)) * float4(this.e235 * -1.0, this.e315 * -1.0, this.e125 * -1.0, this.e321 * -1.0)
        );
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public AntiFlector operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(AntiFlector rhs) {
        return rhs.inverse();
    }
}
public extension AntiFlector {
    public AntiFlector inverse() {
        AntiFlector reverse = AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
        Scalar other = Scalar(
            /* scalar */
            pow(this.e321, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * reverse.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * reverse.group1
        );
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public AntiLine operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiLine operator /(AntiLine rhs) {
        return rhs.inverse();
    }
}
public extension AntiLine {
    public AntiLine inverse() {
        AntiLine reverse = AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            this.group1.xyz * float3(-1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar) * reverse.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar) * reverse.group1.xyz
        );
    }
}
public extension AntiMotor {
    // Fancy postfix trick
    public AntiMotor operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiMotor operator /(AntiMotor rhs) {
        return rhs.inverse();
    }
}
public extension AntiMotor {
    public AntiMotor inverse() {
        AntiMotor reverse = AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.scalar, 2)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * reverse.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar) * reverse.group1
        );
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public AntiPlane operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(AntiPlane rhs) {
        return rhs.inverse();
    }
}
public extension AntiPlane {
    public AntiPlane inverse() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2)) * this.group0
        );
    }
}
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.inverse();
    }
}
public extension AntiScalar {
    public AntiScalar inverse() {
        return AntiScalar(
            /* e12345 */
            1.0/this.e12345 * -1.0
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public Circle operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Circle operator /(Circle rhs) {
        return rhs.inverse();
    }
}
public extension Circle {
    public Circle inverse() {
        Circle reverse = Circle(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125 */
            this.group2.xyz * float3(-1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            pow(this.e321, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(other.scalar) * reverse.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * reverse.group1, 
            /* e235, e315, e125 */
            float3(other.scalar) * reverse.group2.xyz
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public CircleRotor operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public CircleRotor operator /(CircleRotor rhs) {
        return rhs.inverse();
    }
}
public extension CircleRotor {
    public CircleRotor inverse() {
        CircleRotor reverse = CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            pow(this.e321, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - pow(this.e12345, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * reverse.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * reverse.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.scalar) * reverse.group2
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Dipole operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Dipole operator /(Dipole rhs) {
        return rhs.inverse();
    }
}
public extension Dipole {
    public Dipole inverse() {
        Dipole reverse = Dipole(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35 */
            this.group2.xyz * float3(-1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) - pow(this.e45, 2)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(other.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * reverse.group1, 
            /* e15, e25, e35 */
            float3(other.scalar) * reverse.group2.xyz
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public DipoleInversion operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public DipoleInversion operator /(DipoleInversion rhs) {
        return rhs.inverse();
    }
}
public extension DipoleInversion {
    public DipoleInversion inverse() {
        DipoleInversion reverse = DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        Scalar other = Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) - pow(this.e45, 2) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * reverse.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * reverse.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * reverse.group3
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.inverse();
    }
}
public extension DualNum {
    public DualNum inverse() {
        return DualNum(
            /* e5, e12345 */
            float2(pow(this.e12345, -2) * -1.0) * this.group0.xy
        );
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public FlatPoint operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public FlatPoint operator /(FlatPoint rhs) {
        return rhs.inverse();
    }
}
public extension FlatPoint {
    public FlatPoint inverse() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(pow(this.e45, -2) * -1.0) * float4(this.e15 * -1.0, this.e25 * -1.0, this.e35 * -1.0, this.e45 * -1.0)
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.inverse();
    }
}
public extension Flector {
    public Flector inverse() {
        Flector reverse = Flector(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
        Scalar other = Scalar(
            /* scalar */
            -pow(this.e45, 2) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * reverse.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * reverse.group1
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.inverse();
    }
}
public extension Line {
    public Line inverse() {
        Line reverse = Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            -pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2)
        );
        return Line(
            /* e415, e425, e435 */
            float3(other.scalar) * reverse.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * reverse.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.inverse();
    }
}
public extension Motor {
    public Motor inverse() {
        Motor reverse = Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other = Scalar(
            /* scalar */
            -pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - pow(this.e12345, 2)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * reverse.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * reverse.group1
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.inverse();
    }
}
public extension MultiVector {
    public MultiVector inverse() {
        MultiVector reverse = MultiVector(
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
        Scalar other = Scalar(
            /* scalar */
            2.0*(this.e15 * this.e41) + 2.0 * (this.e25 * this.e42) + 2.0 * (this.e35 * this.e43) + 2.0 * (this.e3215 * this.e1234) + pow(this.scalar, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.e321, 2) - pow(this.e12345, 2) - pow(this.e45, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2) - 2.0 * (this.e4 * this.e5) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * reverse.group1, 
            /* e5 */
            reverse.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * reverse.group3, 
            /* e41, e42, e43 */
            float3(other.scalar) * reverse.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * reverse.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * reverse.group6, 
            /* e423, e431, e412 */
            float3(other.scalar) * reverse.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * reverse.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * reverse.group9, 
            /* e1234 */
            reverse.e1234 * other.scalar
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.inverse();
    }
}
public extension Plane {
    public Plane inverse() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)) * this.group0
        );
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public RoundPoint operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public RoundPoint operator /(RoundPoint rhs) {
        return rhs.inverse();
    }
}
public extension RoundPoint {
    public RoundPoint inverse() {
        Scalar other = Scalar(
            /* scalar */
            pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) - 2.0 * (this.e4 * this.e5)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            this.e5 * other.scalar
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.inverse();
    }
}
public extension Scalar {
    public Scalar inverse() {
        return Scalar(
            /* scalar */
            1.0/this.scalar
        );
    }
}
public extension Sphere {
    // Fancy postfix trick
    public Sphere operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public Sphere operator /(Sphere rhs) {
        return rhs.inverse();
    }
}
public extension Sphere {
    public Sphere inverse() {
        Scalar other = Scalar(
            /* scalar */
            2.0*(this.e3215 * this.e1234) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public VersorEven operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public VersorEven operator /(VersorEven rhs) {
        return rhs.inverse();
    }
}
public extension VersorEven {
    public VersorEven inverse() {
        VersorEven reverse = VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
        Scalar other = Scalar(
            /* scalar */
            pow(this.e321, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) - pow(this.e12345, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e5 * this.e4)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * reverse.group0, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * reverse.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * reverse.group2, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * reverse.group3
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public VersorOdd operator /(InversePrefixOrPostfix rhs)  {
        return this.inverse();
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public VersorOdd operator /(VersorOdd rhs) {
        return rhs.inverse();
    }
}
public extension VersorOdd {
    public VersorOdd inverse() {
        VersorOdd reverse = VersorOdd(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
        Scalar other = Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + pow(this.scalar, 2) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) - pow(this.e45, 2) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * reverse.group0, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * reverse.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * reverse.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * reverse.group3
        );
    }
}

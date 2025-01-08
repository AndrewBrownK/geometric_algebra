implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public AntiMotor operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiMotor operator /(AntiCircleRotor rhs) {
        return rhs.round_bulk();
    }
}
public extension AntiCircleRotor {
    public AntiMotor round_bulk() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiFlector operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(AntiDipoleInversion rhs) {
        return rhs.round_bulk();
    }
}
public extension AntiDipoleInversion {
    public AntiFlector round_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiDualNum rhs) {
        return rhs.round_bulk();
    }
}
public extension AntiDualNum {
    public Scalar round_bulk() {
        return Scalar(
            /* scalar */
            this.scalar
        );
    }
}
public extension AntiFlatPoint {
    // Fancy postfix trick
    public AntiFlatPoint operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(AntiFlatPoint rhs) {
        return rhs.round_bulk();
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint round_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public AntiFlector operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(AntiFlector rhs) {
        return rhs.round_bulk();
    }
}
public extension AntiFlector {
    public AntiFlector round_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public AntiLine operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiLine operator /(AntiLine rhs) {
        return rhs.round_bulk();
    }
}
public extension AntiLine {
    public AntiLine round_bulk() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
public extension AntiMotor {
    // Fancy postfix trick
    public AntiMotor operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiMotor operator /(AntiMotor rhs) {
        return rhs.round_bulk();
    }
}
public extension AntiMotor {
    public AntiMotor round_bulk() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0, 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public AntiPlane operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(AntiPlane rhs) {
        return rhs.round_bulk();
    }
}
public extension AntiPlane {
    public AntiPlane round_bulk() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public AntiFlatPoint operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(Circle rhs) {
        return rhs.round_bulk();
    }
}
public extension Circle {
    public AntiFlatPoint round_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public AntiFlatPoint operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlatPoint operator /(CircleRotor rhs) {
        return rhs.round_bulk();
    }
}
public extension CircleRotor {
    public AntiFlatPoint round_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public AntiLine operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiLine operator /(Dipole rhs) {
        return rhs.round_bulk();
    }
}
public extension Dipole {
    public AntiLine round_bulk() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public AntiLine operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiLine operator /(DipoleInversion rhs) {
        return rhs.round_bulk();
    }
}
public extension DipoleInversion {
    public AntiLine round_bulk() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.round_bulk();
    }
}
public extension MultiVector {
    public MultiVector round_bulk() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
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
public extension RoundPoint {
    // Fancy postfix trick
    public AntiPlane operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiPlane operator /(RoundPoint rhs) {
        return rhs.round_bulk();
    }
}
public extension RoundPoint {
    public AntiPlane round_bulk() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.round_bulk();
    }
}
public extension Scalar {
    public Scalar round_bulk() {
        return this;
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public AntiFlector operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiFlector operator /(VersorEven rhs) {
        return rhs.round_bulk();
    }
}
public extension VersorEven {
    public AntiFlector round_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public AntiMotor operator /(RoundBulkPrefixOrPostfix rhs)  {
        return this.round_bulk();
    }
}
public extension RoundBulkPrefixOrPostfix {
    // Fancy prefix trick
    public AntiMotor operator /(VersorOdd rhs) {
        return rhs.round_bulk();
    }
}
public extension VersorOdd {
    public AntiMotor round_bulk() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}

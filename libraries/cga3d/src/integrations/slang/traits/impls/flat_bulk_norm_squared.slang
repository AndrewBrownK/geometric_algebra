implementing cga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiCircleRotor {
    public Scalar flat_bulk_norm_squared() {
        Line flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiDipoleInversion {
    public Scalar flat_bulk_norm_squared() {
        DipoleInversion flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDualNum lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiDualNum {
    public Scalar flat_bulk_norm_squared() {
        VersorEven flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlatPoint lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiFlatPoint {
    public Scalar flat_bulk_norm_squared() {
        Sphere flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlector lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiFlector {
    public Scalar flat_bulk_norm_squared() {
        DipoleInversion flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(AntiLine lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiLine {
    public Scalar flat_bulk_norm_squared() {
        Line flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(AntiMotor lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiMotor {
    public Scalar flat_bulk_norm_squared() {
        Motor flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(AntiPlane lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiPlane {
    public Scalar flat_bulk_norm_squared() {
        FlatPoint flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Circle {
    public Scalar flat_bulk_norm_squared() {
        Sphere flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension CircleRotor {
    public Scalar flat_bulk_norm_squared() {
        Sphere flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Dipole {
    public Scalar flat_bulk_norm_squared() {
        Line flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension DipoleInversion {
    public Scalar flat_bulk_norm_squared() {
        Motor flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(DualNum lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension DualNum {
    public Scalar flat_bulk_norm_squared() {
        FlatPoint flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(FlatPoint lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension FlatPoint {
    public Scalar flat_bulk_norm_squared() {
        Line flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Flector lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Flector {
    public Scalar flat_bulk_norm_squared() {
        Motor flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Line {
    public Scalar flat_bulk_norm_squared() {
        Sphere flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Motor {
    public Scalar flat_bulk_norm_squared() {
        DipoleInversion flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension MultiVector {
    public Scalar flat_bulk_norm_squared() {
        MultiVector flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Plane lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Plane {
    public Scalar flat_bulk_norm_squared() {
        VersorEven flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(RoundPoint lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension RoundPoint {
    public Scalar flat_bulk_norm_squared() {
        FlatPoint flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(Sphere lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Sphere {
    public Scalar flat_bulk_norm_squared() {
        VersorEven flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension VersorEven {
    public Scalar flat_bulk_norm_squared() {
        DipoleInversion flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension VersorOdd {
    public Scalar flat_bulk_norm_squared() {
        Motor flat_bulk_thing = this.flat_bulk().wedge(RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}

implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiCircleRotor rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiDipoleInversion rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension AntiDualNum {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiDualNum rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension AntiFlatPoint {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiFlatPoint rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension AntiFlector {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiFlector rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension AntiLine {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiLine rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension AntiMotor {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiMotor rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension AntiPlane {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiPlane rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension Circle {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Circle rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension CircleRotor {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(CircleRotor rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension Dipole {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Dipole rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension DipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(DipoleInversion rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension DualNum {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(DualNum rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension FlatPoint {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(FlatPoint rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension Flector {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Flector rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension Line {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Line rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension Motor {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Motor rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension MultiVector {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(MultiVector rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension Plane {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Plane rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension RoundPoint {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(RoundPoint rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension Sphere {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Sphere rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension VersorEven {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorEven rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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
public extension VersorOdd {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm_squared();
    }
}
public extension FlatBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorOdd rhs) {
        return rhs.flat_bulk_norm_squared();
    }
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

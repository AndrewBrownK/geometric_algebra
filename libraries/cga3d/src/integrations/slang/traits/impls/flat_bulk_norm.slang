implementing cga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiCircleRotor {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiDipoleInversion {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDualNum lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiDualNum {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlatPoint lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiFlatPoint {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlector lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiFlector {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(AntiLine lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiLine {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(AntiMotor lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiMotor {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(AntiPlane lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiPlane {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_bulk_norm();
}
public extension Circle {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_bulk_norm();
}
public extension CircleRotor {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_bulk_norm();
}
public extension Dipole {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_bulk_norm();
}
public extension DipoleInversion {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(DualNum lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_bulk_norm();
}
public extension DualNum {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(FlatPoint lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_bulk_norm();
}
public extension FlatPoint {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Flector lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_bulk_norm();
}
public extension Flector {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_bulk_norm();
}
public extension Line {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_bulk_norm();
}
public extension Motor {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_bulk_norm();
}
public extension MultiVector {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Plane lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_bulk_norm();
}
public extension Plane {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(RoundPoint lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.flat_bulk_norm();
}
public extension RoundPoint {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Sphere lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_bulk_norm();
}
public extension Sphere {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_bulk_norm();
}
public extension VersorEven {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_bulk_norm();
}
public extension VersorOdd {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}

implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiCircleRotor rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension AntiCircleRotor {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiDipoleInversion rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension AntiDipoleInversion {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiDualNum rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension AntiDualNum {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension AntiFlatPoint {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiFlatPoint rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension AntiFlatPoint {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiFlector rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension AntiFlector {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiLine rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension AntiLine {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension AntiMotor {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiMotor rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension AntiMotor {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiPlane rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension AntiPlane {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Circle {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Circle rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension Circle {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(CircleRotor rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension CircleRotor {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Dipole rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension Dipole {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(DipoleInversion rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension DipoleInversion {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension DualNum {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(DualNum rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension DualNum {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension FlatPoint {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(FlatPoint rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension FlatPoint {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Flector {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Flector rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension Flector {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Line {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Line rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension Line {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Motor {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Motor rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension Motor {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(MultiVector rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension MultiVector {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Plane {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Plane rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension Plane {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(RoundPoint rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension RoundPoint {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension Sphere {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Sphere rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension Sphere {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorEven rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension VersorEven {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public Scalar operator /(FlatBulkNormPrefixOrPostfix rhs)  {
        return this.flat_bulk_norm();
    }
}
public extension FlatBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorOdd rhs) {
        return rhs.flat_bulk_norm();
    }
}
public extension VersorOdd {
    public Scalar flat_bulk_norm() {
        return this.flat_bulk_norm_squared().square_root();
    }
}

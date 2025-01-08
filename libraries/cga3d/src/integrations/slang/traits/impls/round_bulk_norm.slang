implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiCircleRotor rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension AntiCircleRotor {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiDipoleInversion rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension AntiDipoleInversion {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiDualNum rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension AntiDualNum {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension AntiFlatPoint {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiFlatPoint rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension AntiFlatPoint {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiFlector rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension AntiFlector {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiLine rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension AntiLine {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension AntiMotor {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiMotor rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension AntiMotor {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiPlane rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension AntiPlane {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension Circle {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Circle rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension Circle {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(CircleRotor rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension CircleRotor {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Dipole rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension Dipole {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(DipoleInversion rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension DipoleInversion {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(MultiVector rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension MultiVector {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(RoundPoint rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension RoundPoint {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension Scalar {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorEven rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension VersorEven {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormPrefixOrPostfix rhs)  {
        return this.round_bulk_norm();
    }
}
public extension RoundBulkNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorOdd rhs) {
        return rhs.round_bulk_norm();
    }
}
public extension VersorOdd {
    public Scalar round_bulk_norm() {
        return this.round_bulk_norm_squared().square_root();
    }
}

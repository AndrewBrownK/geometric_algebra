implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiCircleRotor rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension AntiCircleRotor {
    public Scalar round_bulk_norm_squared() {
        AntiMotor round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiDipoleInversion rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension AntiDipoleInversion {
    public Scalar round_bulk_norm_squared() {
        AntiFlector round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension AntiDualNum {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiDualNum rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension AntiDualNum {
    public Scalar round_bulk_norm_squared() {
        Scalar round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension AntiFlatPoint {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiFlatPoint rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension AntiFlatPoint {
    public Scalar round_bulk_norm_squared() {
        AntiFlatPoint round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension AntiFlector {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiFlector rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension AntiFlector {
    public Scalar round_bulk_norm_squared() {
        AntiFlector round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension AntiLine {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiLine rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension AntiLine {
    public Scalar round_bulk_norm_squared() {
        AntiLine round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension AntiMotor {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiMotor rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension AntiMotor {
    public Scalar round_bulk_norm_squared() {
        AntiMotor round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension AntiPlane {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiPlane rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension AntiPlane {
    public Scalar round_bulk_norm_squared() {
        AntiPlane round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension Circle {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Circle rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension Circle {
    public Scalar round_bulk_norm_squared() {
        AntiFlatPoint round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(CircleRotor rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension CircleRotor {
    public Scalar round_bulk_norm_squared() {
        AntiFlatPoint round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Dipole rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension Dipole {
    public Scalar round_bulk_norm_squared() {
        AntiLine round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(DipoleInversion rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension DipoleInversion {
    public Scalar round_bulk_norm_squared() {
        AntiLine round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(MultiVector rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension MultiVector {
    public Scalar round_bulk_norm_squared() {
        MultiVector round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(RoundPoint rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension RoundPoint {
    public Scalar round_bulk_norm_squared() {
        AntiPlane round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension Scalar {
    public Scalar round_bulk_norm_squared() {
        Scalar round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorEven rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension VersorEven {
    public Scalar round_bulk_norm_squared() {
        AntiFlector round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix rhs)  {
        return this.round_bulk_norm_squared();
    }
}
public extension RoundBulkNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorOdd rhs) {
        return rhs.round_bulk_norm_squared();
    }
}
public extension VersorOdd {
    public Scalar round_bulk_norm_squared() {
        AntiMotor round_bulk = this.round_bulk();
        return round_bulk.dot_product(round_bulk);
    }
}

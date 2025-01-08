implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.round_weight_norm_squared();
    }
}
public extension RoundWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiCircleRotor rhs) {
        return rhs.round_weight_norm_squared();
    }
}
public extension AntiCircleRotor {
    public AntiScalar round_weight_norm_squared() {
        Line round_weight_carrier = this.round_weight().wedge(DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.round_weight_norm_squared();
    }
}
public extension RoundWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiDipoleInversion rhs) {
        return rhs.round_weight_norm_squared();
    }
}
public extension AntiDipoleInversion {
    public AntiScalar round_weight_norm_squared() {
        Flector round_weight_carrier = this.round_weight().wedge(DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
public extension Circle {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.round_weight_norm_squared();
    }
}
public extension RoundWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Circle rhs) {
        return rhs.round_weight_norm_squared();
    }
}
public extension Circle {
    public AntiScalar round_weight_norm_squared() {
        Plane round_weight_carrier = this.round_weight().wedge(DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.round_weight_norm_squared();
    }
}
public extension RoundWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(CircleRotor rhs) {
        return rhs.round_weight_norm_squared();
    }
}
public extension CircleRotor {
    public AntiScalar round_weight_norm_squared() {
        Plane round_weight_carrier = this.round_weight().wedge(DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
public extension Dipole {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.round_weight_norm_squared();
    }
}
public extension RoundWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Dipole rhs) {
        return rhs.round_weight_norm_squared();
    }
}
public extension Dipole {
    public AntiScalar round_weight_norm_squared() {
        Line round_weight_carrier = this.round_weight().wedge(DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.round_weight_norm_squared();
    }
}
public extension RoundWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(DipoleInversion rhs) {
        return rhs.round_weight_norm_squared();
    }
}
public extension DipoleInversion {
    public AntiScalar round_weight_norm_squared() {
        Motor round_weight_carrier = this.round_weight().wedge(DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.round_weight_norm_squared();
    }
}
public extension RoundWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(MultiVector rhs) {
        return rhs.round_weight_norm_squared();
    }
}
public extension MultiVector {
    public AntiScalar round_weight_norm_squared() {
        MultiVector round_weight_carrier = this.round_weight().wedge(DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
public extension RoundPoint {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.round_weight_norm_squared();
    }
}
public extension RoundWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(RoundPoint rhs) {
        return rhs.round_weight_norm_squared();
    }
}
public extension RoundPoint {
    public AntiScalar round_weight_norm_squared() {
        FlatPoint round_weight_carrier = this.round_weight().wedge(DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
public extension Sphere {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.round_weight_norm_squared();
    }
}
public extension RoundWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(Sphere rhs) {
        return rhs.round_weight_norm_squared();
    }
}
public extension Sphere {
    public AntiScalar round_weight_norm_squared() {
        AntiScalar round_weight_carrier = this.round_weight().wedge(DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.round_weight_norm_squared();
    }
}
public extension RoundWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(VersorEven rhs) {
        return rhs.round_weight_norm_squared();
    }
}
public extension VersorEven {
    public AntiScalar round_weight_norm_squared() {
        Flector round_weight_carrier = this.round_weight().wedge(DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix rhs)  {
        return this.round_weight_norm_squared();
    }
}
public extension RoundWeightNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(VersorOdd rhs) {
        return rhs.round_weight_norm_squared();
    }
}
public extension VersorOdd {
    public AntiScalar round_weight_norm_squared() {
        Motor round_weight_carrier = this.round_weight().wedge(DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        ));
        return round_weight_carrier.anti_dot_product(round_weight_carrier);
    }
}

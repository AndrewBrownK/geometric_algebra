implementing cga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiCircleRotor lhs, RoundWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.round_weight_norm_squared();
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
// Fancy postfix trick
public AntiScalar operator /(AntiDipoleInversion lhs, RoundWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.round_weight_norm_squared();
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
// Fancy postfix trick
public AntiScalar operator /(Circle lhs, RoundWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.round_weight_norm_squared();
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
// Fancy postfix trick
public AntiScalar operator /(CircleRotor lhs, RoundWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.round_weight_norm_squared();
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
// Fancy postfix trick
public AntiScalar operator /(Dipole lhs, RoundWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.round_weight_norm_squared();
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
// Fancy postfix trick
public AntiScalar operator /(DipoleInversion lhs, RoundWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.round_weight_norm_squared();
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
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, RoundWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.round_weight_norm_squared();
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
// Fancy postfix trick
public AntiScalar operator /(RoundPoint lhs, RoundWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.round_weight_norm_squared();
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
// Fancy postfix trick
public AntiScalar operator /(Sphere lhs, RoundWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.round_weight_norm_squared();
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
// Fancy postfix trick
public AntiScalar operator /(VersorEven lhs, RoundWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.round_weight_norm_squared();
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
// Fancy postfix trick
public AntiScalar operator /(VersorOdd lhs, RoundWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.round_weight_norm_squared();
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

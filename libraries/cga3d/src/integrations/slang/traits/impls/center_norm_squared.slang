implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public Scalar operator /(CenterNormSquaredPrefixOrPostfix rhs)  {
        return this.center_norm_squared();
    }
}
public extension CenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiCircleRotor rhs) {
        return rhs.center_norm_squared();
    }
}
public extension AntiCircleRotor {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(CenterNormSquaredPrefixOrPostfix rhs)  {
        return this.center_norm_squared();
    }
}
public extension CenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiDipoleInversion rhs) {
        return rhs.center_norm_squared();
    }
}
public extension AntiDipoleInversion {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
public extension Circle {
    // Fancy postfix trick
    public Scalar operator /(CenterNormSquaredPrefixOrPostfix rhs)  {
        return this.center_norm_squared();
    }
}
public extension CenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Circle rhs) {
        return rhs.center_norm_squared();
    }
}
public extension Circle {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public Scalar operator /(CenterNormSquaredPrefixOrPostfix rhs)  {
        return this.center_norm_squared();
    }
}
public extension CenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(CircleRotor rhs) {
        return rhs.center_norm_squared();
    }
}
public extension CircleRotor {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Scalar operator /(CenterNormSquaredPrefixOrPostfix rhs)  {
        return this.center_norm_squared();
    }
}
public extension CenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Dipole rhs) {
        return rhs.center_norm_squared();
    }
}
public extension Dipole {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(CenterNormSquaredPrefixOrPostfix rhs)  {
        return this.center_norm_squared();
    }
}
public extension CenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(DipoleInversion rhs) {
        return rhs.center_norm_squared();
    }
}
public extension DipoleInversion {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public Scalar operator /(CenterNormSquaredPrefixOrPostfix rhs)  {
        return this.center_norm_squared();
    }
}
public extension CenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(MultiVector rhs) {
        return rhs.center_norm_squared();
    }
}
public extension MultiVector {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public Scalar operator /(CenterNormSquaredPrefixOrPostfix rhs)  {
        return this.center_norm_squared();
    }
}
public extension CenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorEven rhs) {
        return rhs.center_norm_squared();
    }
}
public extension VersorEven {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public Scalar operator /(CenterNormSquaredPrefixOrPostfix rhs)  {
        return this.center_norm_squared();
    }
}
public extension CenterNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorOdd rhs) {
        return rhs.center_norm_squared();
    }
}
public extension VersorOdd {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}

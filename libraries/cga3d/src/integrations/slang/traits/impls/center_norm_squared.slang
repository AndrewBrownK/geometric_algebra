implementing cga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.center_norm_squared();
}
public extension AntiCircleRotor {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.center_norm_squared();
}
public extension AntiDipoleInversion {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.center_norm_squared();
}
public extension Circle {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.center_norm_squared();
}
public extension CircleRotor {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.center_norm_squared();
}
public extension Dipole {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.center_norm_squared();
}
public extension DipoleInversion {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.center_norm_squared();
}
public extension MultiVector {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.center_norm_squared();
}
public extension VersorEven {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.center_norm_squared();
}
public extension VersorOdd {
    public Scalar center_norm_squared() {
        return Scalar(
            /* scalar */
            this.flat_weight_norm_squared().right_anti_dual().scalar + this.round_bulk_norm_squared().scalar
        );
    }
}

implementing cga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.center_norm();
}
public extension AntiCircleRotor {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.center_norm();
}
public extension AntiDipoleInversion {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.center_norm();
}
public extension Circle {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.center_norm();
}
public extension CircleRotor {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.center_norm();
}
public extension Dipole {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.center_norm();
}
public extension DipoleInversion {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.center_norm();
}
public extension MultiVector {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.center_norm();
}
public extension VersorEven {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.center_norm();
}
public extension VersorOdd {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}

implementing cga3d;
using traits;
public extension AntiCircleRotor {
    // Fancy postfix trick
    public Scalar operator /(CenterNormPrefixOrPostfix rhs)  {
        return this.center_norm();
    }
}
public extension CenterNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiCircleRotor rhs) {
        return rhs.center_norm();
    }
}
public extension AntiCircleRotor {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
public extension AntiDipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(CenterNormPrefixOrPostfix rhs)  {
        return this.center_norm();
    }
}
public extension CenterNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(AntiDipoleInversion rhs) {
        return rhs.center_norm();
    }
}
public extension AntiDipoleInversion {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
public extension Circle {
    // Fancy postfix trick
    public Scalar operator /(CenterNormPrefixOrPostfix rhs)  {
        return this.center_norm();
    }
}
public extension CenterNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Circle rhs) {
        return rhs.center_norm();
    }
}
public extension Circle {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
public extension CircleRotor {
    // Fancy postfix trick
    public Scalar operator /(CenterNormPrefixOrPostfix rhs)  {
        return this.center_norm();
    }
}
public extension CenterNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(CircleRotor rhs) {
        return rhs.center_norm();
    }
}
public extension CircleRotor {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
public extension Dipole {
    // Fancy postfix trick
    public Scalar operator /(CenterNormPrefixOrPostfix rhs)  {
        return this.center_norm();
    }
}
public extension CenterNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Dipole rhs) {
        return rhs.center_norm();
    }
}
public extension Dipole {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
public extension DipoleInversion {
    // Fancy postfix trick
    public Scalar operator /(CenterNormPrefixOrPostfix rhs)  {
        return this.center_norm();
    }
}
public extension CenterNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(DipoleInversion rhs) {
        return rhs.center_norm();
    }
}
public extension DipoleInversion {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public Scalar operator /(CenterNormPrefixOrPostfix rhs)  {
        return this.center_norm();
    }
}
public extension CenterNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(MultiVector rhs) {
        return rhs.center_norm();
    }
}
public extension MultiVector {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
public extension VersorEven {
    // Fancy postfix trick
    public Scalar operator /(CenterNormPrefixOrPostfix rhs)  {
        return this.center_norm();
    }
}
public extension CenterNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorEven rhs) {
        return rhs.center_norm();
    }
}
public extension VersorEven {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}
public extension VersorOdd {
    // Fancy postfix trick
    public Scalar operator /(CenterNormPrefixOrPostfix rhs)  {
        return this.center_norm();
    }
}
public extension CenterNormPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(VersorOdd rhs) {
        return rhs.center_norm();
    }
}
public extension VersorOdd {
    public Scalar center_norm() {
        return this.center_norm_squared().square_root();
    }
}

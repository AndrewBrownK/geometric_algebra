implementing cga3d;
using traits;
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(SquareRootPrefixOrPostfix rhs)  {
        return this.square_root();
    }
}
public extension SquareRootPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.square_root();
    }
}
public extension Scalar {
    public Scalar square_root() {
        return Scalar(
            /* scalar */
            pow(this.scalar, 0.5)
        );
    }
}

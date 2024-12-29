implementing rga3d;
using traits;
public extension Scalar {
    // Fancy postfix trick
    public Scalar operator /(SquareRootPrefixOrPostfix rhs)  {
        return this.square_root();
    }
    // Fancy postfix self-assign
    public func operator /=(const SquareRootPrefixOrPostfix& rhs) -> Scalar& {
        this = this.square_root();
        return *this;
    }
}
public extension SquareRootPrefixOrPostfix {
    // Fancy prefix trick
    public Scalar operator /(Scalar rhs) {
        return rhs.square_root();
    }
}
public extension Scalar: SquareRoot {
    public typedef Scalar SquareRootOutput;
    public func square_root() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            pow(this.scalar, 0.5)
        );
    }
}

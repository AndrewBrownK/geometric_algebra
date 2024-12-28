public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: SquareRootPrefixOrPostfix) -> Scalar {
        return this.square_root();
    }
    // Fancy postfix self-assign
    public func operator/=(const SquareRootPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.square_root();
        return *this;
    }
}
public extension SquareRootPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
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

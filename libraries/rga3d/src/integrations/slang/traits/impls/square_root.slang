extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: SquareRootPrefixOrPostfix) -> Scalar {
        return this.square_root();
    }
    func operator/=(const SquareRootPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.square_root();
        return *this;
    }
}
extension Scalar: SquareRoot {
    associatedtype Output = Scalar;
    func square_root() -> Output {
        return Scalar.from_groups(
            /* scalar */
            powf(this.scalar, 0.5)
        );
    }
}

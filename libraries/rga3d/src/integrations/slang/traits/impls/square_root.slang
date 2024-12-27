extension Scalar: SquareRoot {
    associatedtype Output = Scalar;
    func square_root() -> Output {
        return Scalar.from_groups(
            /* scalar */
            powf(this.scalar, 0.5)
        );
    }
    func operator/(_rhs: SquareRootInfix) -> Scalar {
        return this.square_root();
    }
    func operator/=(const SquareRootInfix& _rhs) -> Scalar& {
        this = this.square_root();
        return *this;
    }
}

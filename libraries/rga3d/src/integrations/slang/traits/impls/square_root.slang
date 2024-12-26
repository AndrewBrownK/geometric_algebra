extension Scalar {
    func div(_rhs: square_root) -> 
Scalar {
        this.square_root()
    }
}
extension Scalar {
    func div=(const square_root& _rhs) -> Scalar& {
        this = this.square_root()
        return *this;
    }
}
extension Scalar: SquareRoot {
    func square_root(self) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powf(self[scalar], 0.5)
        );
    }
}

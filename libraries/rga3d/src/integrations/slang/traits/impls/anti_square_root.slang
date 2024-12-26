extension AntiScalar {
    func div(_rhs: anti_square_root) -> 
AntiScalar {
        this.anti_square_root()
    }
}
extension AntiScalar {
    func div=(const anti_square_root& _rhs) -> AntiScalar& {
        this = this.anti_square_root()
        return *this;
    }
}
extension AntiScalar: AntiSquareRoot {
    func anti_square_root(self) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            powf(self[e1234], 0.5)
        );
    }
}

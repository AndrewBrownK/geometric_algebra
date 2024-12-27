extension AntiScalar: AntiSquareRoot {
    associatedtype Output = AntiScalar;
    func anti_square_root() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            powf(this.e1234, 0.5)
        );
    }
    func operator/(_rhs: AntiSquareRootInfix) -> AntiScalar {
        return this.anti_square_root();
    }
    func operator/=(const AntiSquareRootInfix& _rhs) -> AntiScalar& {
        this = this.anti_square_root();
        return *this;
    }
}

extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiSquareRootPrefixOrPostfix) -> AntiScalar {
        return this.anti_square_root();
    }
    func operator/=(const AntiSquareRootPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.anti_square_root();
        return *this;
    }
}
extension AntiScalar: AntiSquareRoot {
    associatedtype Output = AntiScalar;
    func anti_square_root() -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(powf(this.e1234, 0.5), 0.0, 0.0, 0.0)
        );
    }
}

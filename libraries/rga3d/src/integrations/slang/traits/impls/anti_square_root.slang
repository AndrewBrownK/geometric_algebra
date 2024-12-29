implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AntiSquareRootPrefixOrPostfix rhs)  {
        return this.anti_square_root();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiSquareRootPrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.anti_square_root();
        return *this;
    }
}
public extension AntiSquareRootPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.anti_square_root();
    }
}
public extension AntiScalar: AntiSquareRoot {
    public typedef AntiScalar AntiSquareRootOutput;
    public func anti_square_root() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(this.e1234, 0.5)
        );
    }
}

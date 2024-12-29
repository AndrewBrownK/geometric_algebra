implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AntiSquareRootPrefixOrPostfix rhs)  {
        return this.anti_square_root();
    }
}
public extension AntiSquareRootPrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.anti_square_root();
    }
}
public extension AntiScalar {
    public AntiScalar anti_square_root() {
        return AntiScalar.from_groups(
            /* e1234 */
            pow(this.e1234, 0.5)
        );
    }
}

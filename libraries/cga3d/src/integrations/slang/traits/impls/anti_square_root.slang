implementing cga3d;
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
        return AntiScalar(
            /* e12345 */
            pow(this.e12345, 0.5)
        );
    }
}

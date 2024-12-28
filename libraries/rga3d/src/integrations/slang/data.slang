implementing rga3d;

internal bool lessThanHelper<T: IComparable>(T a, T b) {
    return a.lessThan(b);
}
internal bool equalsHelper<T: IComparable>(T a, T b) {
    return a.equals(b);
}
internal bool lessThanOrEqualsHelper<T: IComparable>(T a, T b) {
    return a.lessThanOrEquals(b);
}
            __include "data/anti_scalar";
__include "data/dual_num";
__include "data/flector";
__include "data/horizon";
__include "data/line";
__include "data/motor";
__include "data/multi_vector";
__include "data/origin";
__include "data/plane";
__include "data/point";
__include "data/scalar";

implementing rga3d;
using data::*;

/// Point

struct Point {
    e1_: float, e2_: float, e3_: float, e4_: float
}
struct PointGroups {
    // e1, e2, e3, e4
    group0_: float4
}
fn point_grouped(self_: Point) -> PointGroups {
    return PointGroups(
        float4(self_.e1_, self_.e2_, self_.e3_, self_.e4_)
    );
}
fn point_degroup(self_: PointGroups) -> Point {
    return Point(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w
    );
}

}

impl Point {
    pub const LEN: usize = 4;
}

impl PartialOrd for Point {
fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    for i in 0..Self::LEN {
        let a = float_ord::FloatOrd(self[i]);
        let b = float_ord::FloatOrd(other[i]);
        match a.cmp(&b) {
            std::cmp::Ordering::Equal => continue,
            result => return Some(result),
        }
    }
    Some(std::cmp::Ordering::Equal)
}
}
impl Ord for Point {
fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    for i in 0..Self::LEN {
        let a = float_ord::FloatOrd(self[i]);
        let b = float_ord::FloatOrd(other[i]);
        match a.cmp(&b) {
            std::cmp::Ordering::Equal => continue,
            result => return result,
        }
    }
    std::cmp::Ordering::Equal
}
}
impl PartialEq for Point {
fn eq(&self, other: &Self) -> bool {
    for i in 0..Self::LEN {
        let a = float_ord::FloatOrd(self[i]);
        let b = float_ord::FloatOrd(other[i]);
        if a != b {
            return false
        }
    }
    true
}
}
impl Eq for Point {}
impl std::hash::Hash for Point {
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for i in 0..Self::LEN {
        self[i].to_bits().hash(state);
    }
}
}

extension Point std::ops::Index<elements::e1> for Point {
    __subscript(_: elements::e1) -> &Self::Output {
       this[0]
    }
}
extension Point std::ops::Index<elements::e2> for Point {
    __subscript(_: elements::e2) -> &Self::Output {
       this[1]
    }
}
extension Point std::ops::Index<elements::e3> for Point {
    __subscript(_: elements::e3) -> &Self::Output {
       this[2]
    }
}
extension Point std::ops::Index<elements::e4> for Point {
    __subscript(_: elements::e4) -> &Self::Output {
       this[3]
    }
}
__include ./impls/point;

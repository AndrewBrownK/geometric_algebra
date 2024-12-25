implementing rga3d;
using data::*;

/// Plane

struct Plane {
    e423_: float, e431_: float, e412_: float, e321_: float
}
struct PlaneGroups {
    // e423, e431, e412, e321
    group0_: float4
}
fn plane_grouped(self_: Plane) -> PlaneGroups {
    return PlaneGroups(
        float4(self_.e423_, self_.e431_, self_.e412_, self_.e321_)
    );
}
fn plane_degroup(self_: PlaneGroups) -> Plane {
    return Plane(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w
    );
}

}

impl Plane {
    pub const LEN: usize = 4;
}

impl PartialOrd for Plane {
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
impl Ord for Plane {
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
impl PartialEq for Plane {
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
impl Eq for Plane {}
impl std::hash::Hash for Plane {
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for i in 0..Self::LEN {
        self[i].to_bits().hash(state);
    }
}
}

extension Plane std::ops::Index<elements::e423> for Plane {
    __subscript(_: elements::e423) -> &Self::Output {
       this[0]
    }
}
extension Plane std::ops::Index<elements::e431> for Plane {
    __subscript(_: elements::e431) -> &Self::Output {
       this[1]
    }
}
extension Plane std::ops::Index<elements::e412> for Plane {
    __subscript(_: elements::e412) -> &Self::Output {
       this[2]
    }
}
extension Plane std::ops::Index<elements::e321> for Plane {
    __subscript(_: elements::e321) -> &Self::Output {
       this[3]
    }
}
__include ./impls/plane;

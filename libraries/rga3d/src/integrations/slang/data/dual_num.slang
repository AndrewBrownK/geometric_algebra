implementing rga3d;
using data::*;

/// DualNum

struct DualNum {
    scalar: float, e1234_: float
}
struct DualNumGroups {
    // scalar, e1234, 0, 0
    group0_: float4
}
fn dualNum_grouped(self_: DualNum) -> DualNumGroups {
    return DualNumGroups(
        float4(self_.scalar, self_.e1234_, 0.0, 0.0)
    );
}
fn dualNum_degroup(self_: DualNumGroups) -> DualNum {
    return DualNum(
        self_.group0_.x, self_.group0_.y
    );
}

}

impl DualNum {
    pub const LEN: usize = 2;
}

impl PartialOrd for DualNum {
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
impl Ord for DualNum {
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
impl PartialEq for DualNum {
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
impl Eq for DualNum {}
impl std::hash::Hash for DualNum {
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for i in 0..Self::LEN {
        self[i].to_bits().hash(state);
    }
}
}

extension DualNum std::ops::Index<elements::scalar> for DualNum {
    __subscript(_: elements::scalar) -> &Self::Output {
       this[0]
    }
}
extension DualNum std::ops::Index<elements::e1234> for DualNum {
    __subscript(_: elements::e1234) -> &Self::Output {
       this[1]
    }
}
__include ./impls/dual_num;

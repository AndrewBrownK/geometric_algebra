implementing rga3d;
using data::*;

/// Flector

struct Flector {
    e1_: float, e2_: float, e3_: float, e4_: float,
    e423_: float, e431_: float, e412_: float, e321_: float
}
struct FlectorGroups {
    // e1, e2, e3, e4
    group0_: float4,
    // e423, e431, e412, e321
    group1_: float4
}
fn flector_grouped(self_: Flector) -> FlectorGroups {
    return FlectorGroups(
        float4(self_.e1_, self_.e2_, self_.e3_, self_.e4_),
        float4(self_.e423_, self_.e431_, self_.e412_, self_.e321_)
    );
}
fn flector_degroup(self_: FlectorGroups) -> Flector {
    return Flector(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w
    );
}

}

impl Flector {
    pub const LEN: usize = 8;
}

impl PartialOrd for Flector {
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
impl Ord for Flector {
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
impl PartialEq for Flector {
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
impl Eq for Flector {}
impl std::hash::Hash for Flector {
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for i in 0..Self::LEN {
        self[i].to_bits().hash(state);
    }
}
}

extension Flector std::ops::Index<elements::e1> for Flector {
    __subscript(_: elements::e1) -> &Self::Output {
       this[0]
    }
}
extension Flector std::ops::Index<elements::e2> for Flector {
    __subscript(_: elements::e2) -> &Self::Output {
       this[1]
    }
}
extension Flector std::ops::Index<elements::e3> for Flector {
    __subscript(_: elements::e3) -> &Self::Output {
       this[2]
    }
}
extension Flector std::ops::Index<elements::e4> for Flector {
    __subscript(_: elements::e4) -> &Self::Output {
       this[3]
    }
}
extension Flector std::ops::Index<elements::e423> for Flector {
    __subscript(_: elements::e423) -> &Self::Output {
       this[4]
    }
}
extension Flector std::ops::Index<elements::e431> for Flector {
    __subscript(_: elements::e431) -> &Self::Output {
       this[5]
    }
}
extension Flector std::ops::Index<elements::e412> for Flector {
    __subscript(_: elements::e412) -> &Self::Output {
       this[6]
    }
}
extension Flector std::ops::Index<elements::e321> for Flector {
    __subscript(_: elements::e321) -> &Self::Output {
       this[7]
    }
}
__include ./impls/flector;

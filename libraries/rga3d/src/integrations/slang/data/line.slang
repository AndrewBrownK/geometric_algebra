implementing rga3d;
using data::*;

/// Line

struct Line {
    e41_: float, e42_: float, e43_: float,
    e23_: float, e31_: float, e12_: float
}
struct LineGroups {
    // e41, e42, e43, 0
    group0_: float4,
    // e23, e31, e12, 0
    group1_: float4
}
fn line_grouped(self_: Line) -> LineGroups {
    return LineGroups(
        float4(self_.e41_, self_.e42_, self_.e43_, 0.0),
        float4(self_.e23_, self_.e31_, self_.e12_, 0.0)
    );
}
fn line_degroup(self_: LineGroups) -> Line {
    return Line(
        self_.group0_.x, self_.group0_.y, self_.group0_.z,
        self_.group1_.x, self_.group1_.y, self_.group1_.z
    );
}

}

impl Line {
    pub const LEN: usize = 6;
}

impl PartialOrd for Line {
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
impl Ord for Line {
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
impl PartialEq for Line {
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
impl Eq for Line {}
impl std::hash::Hash for Line {
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for i in 0..Self::LEN {
        self[i].to_bits().hash(state);
    }
}
}

extension Line std::ops::Index<elements::e41> for Line {
    __subscript(_: elements::e41) -> &Self::Output {
       this[0]
    }
}
extension Line std::ops::Index<elements::e42> for Line {
    __subscript(_: elements::e42) -> &Self::Output {
       this[1]
    }
}
extension Line std::ops::Index<elements::e43> for Line {
    __subscript(_: elements::e43) -> &Self::Output {
       this[2]
    }
}
extension Line std::ops::Index<elements::e23> for Line {
    __subscript(_: elements::e23) -> &Self::Output {
       this[3]
    }
}
extension Line std::ops::Index<elements::e31> for Line {
    __subscript(_: elements::e31) -> &Self::Output {
       this[4]
    }
}
extension Line std::ops::Index<elements::e12> for Line {
    __subscript(_: elements::e12) -> &Self::Output {
       this[5]
    }
}
__include ./impls/line;

implementing rga3d;
using data::*;

/// Origin.
/// The Origin is the RoundPoint where x, y, z, and radius are all zero.
/// It is the base element e4.
/// Not to be confused with FlatOrigin, which is a Dipole connecting Origin and Infinity.

struct Origin {
    e4_: float
}
struct OriginGroups {
    // e4, 0, 0, 0
    group0_: float4
}
fn origin_grouped(self_: Origin) -> OriginGroups {
    return OriginGroups(
        float4(self_.e4_, 0.0, 0.0, 0.0)
    );
}
fn origin_degroup(self_: OriginGroups) -> Origin {
    return Origin(
        self_.group0_.x
    );
}

}

impl Origin {
    pub const LEN: usize = 1;
}

impl PartialOrd for Origin {
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
impl Ord for Origin {
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
impl PartialEq for Origin {
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
impl Eq for Origin {}
impl std::hash::Hash for Origin {
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for i in 0..Self::LEN {
        self[i].to_bits().hash(state);
    }
}
}

extension Origin std::ops::Index<elements::e4> for Origin {
    __subscript(_: elements::e4) -> &Self::Output {
       this[0]
    }
}
__include ./impls/origin;

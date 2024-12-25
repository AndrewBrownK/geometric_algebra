implementing rga3d;
using data::*;

/// Horizon

struct Horizon {
    e321_: float
}
struct HorizonGroups {
    // e321, 0, 0, 0
    group0_: float4
}
fn horizon_grouped(self_: Horizon) -> HorizonGroups {
    return HorizonGroups(
        float4(self_.e321_, 0.0, 0.0, 0.0)
    );
}
fn horizon_degroup(self_: HorizonGroups) -> Horizon {
    return Horizon(
        self_.group0_.x
    );
}

}

impl Horizon {
    pub const LEN: usize = 1;
}

impl PartialOrd for Horizon {
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
impl Ord for Horizon {
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
impl PartialEq for Horizon {
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
impl Eq for Horizon {}
impl std::hash::Hash for Horizon {
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for i in 0..Self::LEN {
        self[i].to_bits().hash(state);
    }
}
}

extension Horizon std::ops::Index<elements::e321> for Horizon {
    __subscript(_: elements::e321) -> &Self::Output {
       this[0]
    }
}
__include ./impls/horizon;

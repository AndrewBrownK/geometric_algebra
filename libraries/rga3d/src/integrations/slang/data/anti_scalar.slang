implementing rga3d;
using data::*;

/// AntiScalar

struct AntiScalar {
    e1234_: float
}
struct AntiScalarGroups {
    // e1234, 0, 0, 0
    group0_: float4
}
fn antiScalar_grouped(self_: AntiScalar) -> AntiScalarGroups {
    return AntiScalarGroups(
        float4(self_.e1234_, 0.0, 0.0, 0.0)
    );
}
fn antiScalar_degroup(self_: AntiScalarGroups) -> AntiScalar {
    return AntiScalar(
        self_.group0_.x
    );
}

}

impl AntiScalar {
    pub const LEN: usize = 1;
}

impl PartialOrd for AntiScalar {
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
impl Ord for AntiScalar {
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
impl PartialEq for AntiScalar {
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
impl Eq for AntiScalar {}
impl std::hash::Hash for AntiScalar {
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for i in 0..Self::LEN {
        self[i].to_bits().hash(state);
    }
}
}

extension AntiScalar std::ops::Index<elements::e1234> for AntiScalar {
    __subscript(_: elements::e1234) -> &Self::Output {
       this[0]
    }
}
__include ./impls/anti_scalar;

implementing rga3d;
using data::*;

/// Scalar

struct Scalar {
    scalar: float
}
struct ScalarGroups {
    // scalar, 0, 0, 0
    group0_: float4
}
fn scalar_grouped(self_: Scalar) -> ScalarGroups {
    return ScalarGroups(
        float4(self_.scalar, 0.0, 0.0, 0.0)
    );
}
fn scalar_degroup(self_: ScalarGroups) -> Scalar {
    return Scalar(
        self_.group0_.x
    );
}

}

impl Scalar {
    pub const LEN: usize = 1;
}

impl PartialOrd for Scalar {
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
impl Ord for Scalar {
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
impl PartialEq for Scalar {
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
impl Eq for Scalar {}
impl std::hash::Hash for Scalar {
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for i in 0..Self::LEN {
        self[i].to_bits().hash(state);
    }
}
}

extension Scalar std::ops::Index<elements::scalar> for Scalar {
    __subscript(_: elements::scalar) -> &Self::Output {
       this[0]
    }
}
__include ./impls/scalar;

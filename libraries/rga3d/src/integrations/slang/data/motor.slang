implementing rga3d;
using data::*;

/// Motor

struct Motor {
    e41_: float, e42_: float, e43_: float, e1234_: float,
    e23_: float, e31_: float, e12_: float, scalar: float
}
struct MotorGroups {
    // e41, e42, e43, e1234
    group0_: float4,
    // e23, e31, e12, scalar
    group1_: float4
}
fn motor_grouped(self_: Motor) -> MotorGroups {
    return MotorGroups(
        float4(self_.e41_, self_.e42_, self_.e43_, self_.e1234_),
        float4(self_.e23_, self_.e31_, self_.e12_, self_.scalar)
    );
}
fn motor_degroup(self_: MotorGroups) -> Motor {
    return Motor(
        self_.group0_.x, self_.group0_.y, self_.group0_.z, self_.group0_.w,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w
    );
}

}

impl Motor {
    pub const LEN: usize = 8;
}

impl PartialOrd for Motor {
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
impl Ord for Motor {
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
impl PartialEq for Motor {
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
impl Eq for Motor {}
impl std::hash::Hash for Motor {
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for i in 0..Self::LEN {
        self[i].to_bits().hash(state);
    }
}
}

extension Motor std::ops::Index<elements::e41> for Motor {
    __subscript(_: elements::e41) -> &Self::Output {
       this[0]
    }
}
extension Motor std::ops::Index<elements::e42> for Motor {
    __subscript(_: elements::e42) -> &Self::Output {
       this[1]
    }
}
extension Motor std::ops::Index<elements::e43> for Motor {
    __subscript(_: elements::e43) -> &Self::Output {
       this[2]
    }
}
extension Motor std::ops::Index<elements::e1234> for Motor {
    __subscript(_: elements::e1234) -> &Self::Output {
       this[3]
    }
}
extension Motor std::ops::Index<elements::e23> for Motor {
    __subscript(_: elements::e23) -> &Self::Output {
       this[4]
    }
}
extension Motor std::ops::Index<elements::e31> for Motor {
    __subscript(_: elements::e31) -> &Self::Output {
       this[5]
    }
}
extension Motor std::ops::Index<elements::e12> for Motor {
    __subscript(_: elements::e12) -> &Self::Output {
       this[6]
    }
}
extension Motor std::ops::Index<elements::scalar> for Motor {
    __subscript(_: elements::scalar) -> &Self::Output {
       this[7]
    }
}
__include ./impls/motor;

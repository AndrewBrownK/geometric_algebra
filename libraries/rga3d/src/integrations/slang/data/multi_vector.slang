implementing rga3d;
using data::*;

/// MultiVector

struct MultiVector {
    scalar: float, e1234_: float,
    e1_: float, e2_: float, e3_: float, e4_: float,
    e41_: float, e42_: float, e43_: float,
    e23_: float, e31_: float, e12_: float,
    e423_: float, e431_: float, e412_: float, e321_: float
}
struct MultiVectorGroups {
    // scalar, e1234, 0, 0
    group0_: float4,
    // e1, e2, e3, e4
    group1_: float4,
    // e41, e42, e43, 0
    group2_: float4,
    // e23, e31, e12, 0
    group3_: float4,
    // e423, e431, e412, e321
    group4_: float4
}
fn multiVector_grouped(self_: MultiVector) -> MultiVectorGroups {
    return MultiVectorGroups(
        float4(self_.scalar, self_.e1234_, 0.0, 0.0),
        float4(self_.e1_, self_.e2_, self_.e3_, self_.e4_),
        float4(self_.e41_, self_.e42_, self_.e43_, 0.0),
        float4(self_.e23_, self_.e31_, self_.e12_, 0.0),
        float4(self_.e423_, self_.e431_, self_.e412_, self_.e321_)
    );
}
fn multiVector_degroup(self_: MultiVectorGroups) -> MultiVector {
    return MultiVector(
        self_.group0_.x, self_.group0_.y,
        self_.group1_.x, self_.group1_.y, self_.group1_.z, self_.group1_.w,
        self_.group2_.x, self_.group2_.y, self_.group2_.z,
        self_.group3_.x, self_.group3_.y, self_.group3_.z,
        self_.group4_.x, self_.group4_.y, self_.group4_.z, self_.group4_.w
    );
}

}

impl MultiVector {
    pub const LEN: usize = 16;
}

impl PartialOrd for MultiVector {
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
impl Ord for MultiVector {
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
impl PartialEq for MultiVector {
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
impl Eq for MultiVector {}
impl std::hash::Hash for MultiVector {
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for i in 0..Self::LEN {
        self[i].to_bits().hash(state);
    }
}
}

extension MultiVector std::ops::Index<elements::scalar> for MultiVector {
    __subscript(_: elements::scalar) -> &Self::Output {
       this[0]
    }
}
extension MultiVector std::ops::Index<elements::e1234> for MultiVector {
    __subscript(_: elements::e1234) -> &Self::Output {
       this[1]
    }
}
extension MultiVector std::ops::Index<elements::e1> for MultiVector {
    __subscript(_: elements::e1) -> &Self::Output {
       this[2]
    }
}
extension MultiVector std::ops::Index<elements::e2> for MultiVector {
    __subscript(_: elements::e2) -> &Self::Output {
       this[3]
    }
}
extension MultiVector std::ops::Index<elements::e3> for MultiVector {
    __subscript(_: elements::e3) -> &Self::Output {
       this[4]
    }
}
extension MultiVector std::ops::Index<elements::e4> for MultiVector {
    __subscript(_: elements::e4) -> &Self::Output {
       this[5]
    }
}
extension MultiVector std::ops::Index<elements::e41> for MultiVector {
    __subscript(_: elements::e41) -> &Self::Output {
       this[6]
    }
}
extension MultiVector std::ops::Index<elements::e42> for MultiVector {
    __subscript(_: elements::e42) -> &Self::Output {
       this[7]
    }
}
extension MultiVector std::ops::Index<elements::e43> for MultiVector {
    __subscript(_: elements::e43) -> &Self::Output {
       this[8]
    }
}
extension MultiVector std::ops::Index<elements::e23> for MultiVector {
    __subscript(_: elements::e23) -> &Self::Output {
       this[9]
    }
}
extension MultiVector std::ops::Index<elements::e31> for MultiVector {
    __subscript(_: elements::e31) -> &Self::Output {
       this[10]
    }
}
extension MultiVector std::ops::Index<elements::e12> for MultiVector {
    __subscript(_: elements::e12) -> &Self::Output {
       this[11]
    }
}
extension MultiVector std::ops::Index<elements::e423> for MultiVector {
    __subscript(_: elements::e423) -> &Self::Output {
       this[12]
    }
}
extension MultiVector std::ops::Index<elements::e431> for MultiVector {
    __subscript(_: elements::e431) -> &Self::Output {
       this[13]
    }
}
extension MultiVector std::ops::Index<elements::e412> for MultiVector {
    __subscript(_: elements::e412) -> &Self::Output {
       this[14]
    }
}
extension MultiVector std::ops::Index<elements::e321> for MultiVector {
    __subscript(_: elements::e321) -> &Self::Output {
       this[15]
    }
}
__include ./impls/multi_vector;

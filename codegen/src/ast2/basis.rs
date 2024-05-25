use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Write};

bitflags::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct BasisSignature: u16 {
        const scalar = 0x0;
        const e0 = 0x1;
        const e1 = 0x2;
        const e2 = 0x4;
        const e3 = 0x8;
        const e4 = 0x10;
        const e5 = 0x20;
        const e6 = 0x40;
        const e7 = 0x80;
        const e8 = 0x100;
        const e9 = 0x200;
        const eA = 0x400;
        const eB = 0x800;
        const eC = 0x1000;
        const eD = 0x2000;
        const eE = 0x4000;
        const eF = 0x8000;
    }
}

impl PartialOrd for BasisSignature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self.bits();
        let b = other.bits();
        Some(a.count_ones().cmp(&b.count_ones()).then_with(|| {
            b.reverse_bits().cmp(&a.reverse_bits())
        }))
    }
}
impl Ord for BasisSignature {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.bits();
        let b = other.bits();
        a.count_ones().cmp(&b.count_ones()).then_with(|| {
            b.reverse_bits().cmp(&a.reverse_bits())
        })
    }
}

impl Debug for BasisSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let num = self.bits();
        f.write_str(format!("BasisSignature(0b{num:016b})").as_str())
    }
}

impl Display for BasisSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let num = self.bits();
        if num == 0 {
            write!(f, "scalar")
        } else {
            write!(f, "e")?;
            for i in 0..16 {
                if num & (1 << i) != 0 {
                    write!(f, "{}", char::from_digit(i, 16).unwrap())?;
                }
            }
            Ok(())
        }
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BasisElement {
    coefficient: i8,
    signature: BasisSignature,
}
impl PartialOrd for BasisElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.signature.cmp(&other.signature).then_with(|| {
            self.coefficient.cmp(&other.coefficient)
        }))
    }
}
impl Ord for BasisElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.signature.cmp(&other.signature).then_with(|| {
            self.coefficient.cmp(&other.coefficient)
        })
    }
}
impl Display for BasisElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.coefficient {
            0 => write!(f, "0"),
            1 => write!(f, "{}", self.signature),
            -1 => write!(f, "-{}", self.signature),
            c => write!(f, "{}*{}", c, self.signature),
        }
    }
}

impl From<BasisSignature> for BasisElement {
    fn from(signature: BasisSignature) -> Self {
        Self {
            coefficient: 1,
            signature
        }
    }
}

impl BasisElement {
    pub const fn zero() -> Self {
        Self {
            coefficient: 0,
            signature: BasisSignature::scalar
        }
    }

    pub const fn negate(mut self) -> Self {
        self.coefficient *= -1;
        self
    }

    pub fn primitive_wedge(
        &self,
        other: &BasisElement,
        anti_scalar: &BasisElement,
        generator_squares: &[i8]
    ) -> BasisElement {
        let a = self;


        return a.clone();
    }
}

#[allow(non_upper_case_globals, dead_code)]
pub mod elements {
    use crate::ast2::basis::*;

    const fn element(signature: BasisSignature) -> BasisElement {
        BasisElement {
            coefficient: 1,
            signature,
        }
    }

    include!(concat!(env!("OUT_DIR"), "/generated_elements.rs"));

    // And we'll add some custom bases as seen in the wild.
    // (But let's not go overboard by using code generation on this part...
    //  The number of permutations is the factorial of the number of dimensions.)

    // Eric Lengyel's bases:
    pub const e41: BasisElement = e14.negate();
    pub const e42: BasisElement = e24.negate();
    pub const e43: BasisElement = e34.negate();
    pub const e31: BasisElement = e13.negate();
    pub const e423: BasisElement = e234;
    pub const e431: BasisElement = e134.negate();
    pub const e412: BasisElement = e124;
    pub const e321: BasisElement = e123;
    pub const e415: BasisElement = e145.negate();
    pub const e425: BasisElement = e245.negate();
    pub const e435: BasisElement = e345.negate();
    pub const e315: BasisElement = e135.negate();
    pub const e4235: BasisElement = e2345;
    pub const e4315: BasisElement = e1345.negate();
    pub const e4125: BasisElement = e1245;
    pub const e3215: BasisElement = e1235.negate();
}
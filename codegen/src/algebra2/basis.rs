use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

use rand::Rng;

use crate::algebra::basis_element;

bitflags::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

impl Default for BasisSignature {
    fn default() -> Self {
        BasisSignature::scalar
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

impl BasisSignature {
    fn to_primary_bases(&self) -> Vec<PrimaryBasis> {
        let mut result = vec![];
        for basis in PrimaryBasis::array() {
            let sig = BasisSignature::from_bits_retain(1u16 << (basis as u8));
            if self.contains(sig) {
                result.push(basis);
            }
        }
        result
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
impl Default for BasisElement {
    fn default() -> Self {
        Self {
            coefficient: 0,
            signature: BasisSignature::scalar,
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
    pub fn coefficient(&self) -> i8 {
        self.coefficient
    }

    pub fn signature(&self) -> BasisSignature {
        self.signature
    }

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

    pub fn grade(&self) -> u8 {
        self.signature.bits().count_ones() as u8
    }

    /// Wedge product, but "primitive" in the sense that all BasisElements
    /// are assumed to be entirely independent, and there's nothing fancy
    /// like merged e+ and e- into e4 and e5 going on.
    pub fn primitive_wedge(
        &self,
        other: &BasisElement,
        generator_squares: &GeneratorSquares,
    ) -> BasisElement {
        let a = self.signature.to_primary_bases();
        let b = other.signature.to_primary_bases();
        let mut sign = self.coefficient * other.coefficient;
        let mut result_elements = vec![];
        let mut a_idx = 0;
        let mut b_idx = 0;
        while a_idx < a.len() || b_idx < b.len() {
            if a_idx >= a.len() {
                result_elements.push(b[b_idx]);
                b_idx += 1;
                continue
            }
            if b_idx >= b.len() {
                result_elements.push(a[a_idx]);
                a_idx += 1;
                continue
            }
            let a_ = a[a_idx];
            let b_ = b[b_idx];
            match a_.cmp(&b_) {
                Ordering::Less => {
                    result_elements.push(a_);
                    a_idx += 1;
                }
                Ordering::Equal => {
                    sign *= generator_squares.square(a_);
                    if sign == 0 {
                        return BasisElement::zero()
                    }
                    // Must move b_ at least to the right of a_.
                    // Which negates the sign each step.
                    let swaps = ((a.len() - a_idx) - 1) as u32;
                    sign *= i8::pow(-1, swaps % 2);
                    a_idx += 1;
                    b_idx += 1;
                }
                Ordering::Greater => {
                    // Must move b_ all the way to left of a_.
                    // Which negates the sign each step.
                    result_elements.push(b_);
                    let swaps = (a.len() - a_idx) as u32;
                    sign *= i8::pow(-1, swaps % 2);
                    b_idx += 1;
                }
            }
        }
        let mut result_sig = 0u16;
        for primary_basis in result_elements {
            let additional_sig = 1u16 << (primary_basis as u8);
            result_sig = result_sig | additional_sig;
        }
        let signature = BasisSignature::from_bits_retain(result_sig);
        BasisElement {
            coefficient: sign,
            signature,
        }
    }

    /// AntiWedge product, but "primitive" in the sense that all BasisElements
    /// are assumed to be entirely independent, and there's nothing fancy
    /// like merged e+ and e- into e4 and e5 going on.
    pub fn primitive_anti_wedge(
        &self,
        other: &BasisElement,
        generator_squares: &GeneratorSquares,
    ) -> BasisElement {
        let mut a = self.clone();
        let mut b = other.clone();
        let anti_scalar_signature = generator_squares.anti_scalar().signature;
        a.signature = anti_scalar_signature - a.signature;
        b.signature = anti_scalar_signature - b.signature;
        let mut result = a.primitive_wedge(&b, generator_squares);
        result.signature = anti_scalar_signature - result.signature;
        return result;
    }
}

#[test]
fn new_basis_elements_wedge() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let mut a: u16 = rng.gen();
        let mut b: u16 = rng.gen();
        let a_s: i8 = rng.gen::<i8>().signum();
        let b_s: i8 = rng.gen::<i8>().signum();
        let d: usize = (rng.gen::<usize>() % 15) + 1;
        for remove_d in d..16 {
            a = a & !(1u16 << remove_d);
            b = b & !(1u16 << remove_d);
        }

        let mut squares = GeneratorSquares::empty();
        for _ in 0..d {
            let basis = squares.next_available_basis().unwrap();
            let sq: i8 = ((rng.gen::<i8>().max(i8::MIN + 1).abs() % 5) - 2).signum();
            squares = squares.append([(basis, sq)]).unwrap();
        }
        println!("Squares: {squares:?}");
        println!("a: {a_s} {a:016b}");
        println!("b: {b_s} {b:016b}");

        let old_a = basis_element::BasisElement {
            coefficient: a_s as isize,
            index: a,
        };
        let old_b = basis_element::BasisElement {
            coefficient: b_s as isize,
            index: b,
        };

        let mut new_a = BasisElement::from(BasisSignature::from_bits_retain(a));
        new_a.coefficient = a_s;
        let mut new_b = BasisElement::from(BasisSignature::from_bits_retain(b));
        new_b.coefficient = b_s;

        let sq: Vec<_> = squares.raw_squares[0..d].iter().map(|it| it.clone() as isize).collect();
        let old_product = old_a.primitive_anti_product(&old_b, sq.as_slice());
        let new_product = new_a.primitive_anti_wedge(&new_b, &squares);

        let old_coefficient = old_product.coefficient;
        let old_sig = old_product.index;
        let new_coefficient = new_product.coefficient as isize;
        let new_sig = new_product.signature.bits();
        println!("old result: {old_coefficient} {old_sig:016b}");
        println!("new result: {new_coefficient} {new_sig:016b}");

        assert_eq!(old_coefficient, new_coefficient, "coefficients mismatch");
        assert_eq!(old_sig, new_sig, "signature mismatch");
    }
}


#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum PrimaryBasis {
    e0 = 0,
    e1 = 1,
    e2 = 2,
    e3 = 3,
    e4 = 4,
    e5 = 5,
    e6 = 6,
    e7 = 7,
    e8 = 8,
    e9 = 9,
    eA = 10,
    eB = 11,
    eC = 12,
    eD = 13,
    eE = 14,
    eF = 15,
}
impl PrimaryBasis {
    fn array() -> [Self; 16] {
        [
            PrimaryBasis::e0, PrimaryBasis::e1, PrimaryBasis::e2, PrimaryBasis::e3,
            PrimaryBasis::e4, PrimaryBasis::e5, PrimaryBasis::e6, PrimaryBasis::e7,
            PrimaryBasis::e8, PrimaryBasis::e9, PrimaryBasis::eA, PrimaryBasis::eB,
            PrimaryBasis::eC, PrimaryBasis::eD, PrimaryBasis::eE, PrimaryBasis::eF,
        ]
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratorSquares {
    active_bases: BasisSignature,
    raw_squares: [i8; 16],
}
impl GeneratorSquares {
    pub fn anti_scalar(&self) -> BasisElement {
        let signature = self.active_bases.clone();
        BasisElement {
            coefficient: 1,
            signature,
        }
    }

    pub fn next_available_basis(&self) -> anyhow::Result<PrimaryBasis> {
        let mut emptying_signature = self.active_bases.clone();

        // The way this works, if the active_bases is not empty and starts at e1 (or higher) instead
        // of e0, then it will skip over e0 (etc.) unless it runs out of bases all the way to eF,
        // and then it will loop around and try the lower bases again.
        for basis in PrimaryBasis::array().into_iter().chain(PrimaryBasis::array()) {
            if emptying_signature.is_empty() {
                return Ok(basis)
            }
            let sig = BasisSignature::from_bits_retain(1u16 << (basis as u8));
            emptying_signature.remove(sig);
        }
        Err(anyhow::format_err!("There are no more available PrimaryBasis for {self:?}."))
    }

    pub fn empty() -> Self {
        Self {
            active_bases: BasisSignature::empty(),
            raw_squares: [0i8; 16]
        }
    }

    pub fn new<const N: usize>(generator_squares: [(PrimaryBasis, i8); N]) -> Self {
        let mut active_bases = BasisSignature::empty();
        let mut raw_squares = [0i8; 16];
        for (basis, square) in generator_squares {
            let sig = BasisSignature::from_bits_retain(1u16 << (basis as u8));
            active_bases = active_bases.union(sig);
            raw_squares[(basis as u8) as usize] = square;
        }
        Self { active_bases, raw_squares }
    }

    pub fn append<const N: usize>(self, generator_squares: [(PrimaryBasis, i8); N]) -> anyhow::Result<Self> {
        let mut active_bases = self.active_bases;
        let mut raw_squares = self.raw_squares;
        for (basis, square) in generator_squares {
            let sig = BasisSignature::from_bits_retain(1u16 << (basis as u8));
            if active_bases.contains(sig) {
                return Err(anyhow::format_err!("The PrimaryBasis {basis:?} is already taken on {self:?}"))
            }
            active_bases = active_bases.union(sig);
            raw_squares[(basis as u8) as usize] = square;
        }
        Ok(Self { active_bases, raw_squares })
    }

    pub fn overwrite<const N: usize>(self, generator_squares: [(PrimaryBasis, i8); N]) -> Self {
        let mut active_bases = self.active_bases;
        let mut raw_squares = self.raw_squares;
        for (basis, square) in generator_squares {
            let sig = BasisSignature::from_bits_retain(1u16 << (basis as u8));
            active_bases = active_bases.union(sig);
            raw_squares[(basis as u8) as usize] = square;
        }
        Self { active_bases, raw_squares }
    }

    pub fn square(&self, basis: PrimaryBasis) -> i8 {
        self.raw_squares[(basis as u8) as usize]
    }

    pub fn wedge(&self, a: BasisElement, b: BasisElement) -> BasisElement {
        a.primitive_wedge(&b, &self)
    }

    pub fn anti_wedge(&self, a: BasisElement, b: BasisElement) -> BasisElement {
        a.primitive_anti_wedge(&b, &self)
    }
}

#[allow(non_upper_case_globals, dead_code)]
pub mod elements {
    use crate::algebra2::basis::*;

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
use std::cmp::Ordering;
use crate::algebra2::basis::{BasisElement, BasisSignature};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratorSquares {
    active_bases: BasisSignature,
    // TODO un-pub
    pub raw_squares: [i8; 16],
}

// TODO any chance of some of these being const fn?
impl GeneratorSquares {
    pub fn anti_scalar(&self) -> BasisElement {
        let signature = self.active_bases.clone();
        BasisElement {
            coefficient: 1,
            signature,
            display_name: None,
        }
    }

    pub fn next_available_basis(&self) -> anyhow::Result<GeneratorElement> {
        let mut emptying_signature = self.active_bases.clone();

        // The way this works, if the active_bases is not empty and starts at e1 (or higher) instead
        // of e0, then it will skip over e0 (etc.) unless it runs out of bases all the way to eF,
        // and then it will loop around and try the lower bases again.
        for basis in GeneratorElement::array().into_iter().chain(GeneratorElement::array()) {
            if emptying_signature.is_empty() {
                return Ok(basis)
            }
            emptying_signature.remove(basis.signature());
        }
        Err(anyhow::format_err!("There are no more available PrimaryBasis for {self:?}."))
    }

    pub fn empty() -> Self {
        Self {
            active_bases: BasisSignature::empty(),
            raw_squares: [0i8; 16]
        }
    }

    pub fn new<const N: usize>(generator_squares: [(GeneratorElement, i8); N]) -> Self {
        let mut active_bases = BasisSignature::empty();
        let mut raw_squares = [0i8; 16];
        for (basis, square) in generator_squares {
            active_bases = active_bases.union(basis.signature());
            raw_squares[(basis as u8) as usize] = square;
        }
        Self { active_bases, raw_squares }
    }

    pub fn append<const N: usize>(self, generator_squares: [(GeneratorElement, i8); N]) -> anyhow::Result<Self> {
        let mut active_bases = self.active_bases;
        let mut raw_squares = self.raw_squares;
        for (basis, square) in generator_squares {
            let sig = basis.signature();
            if active_bases.contains(sig) {
                return Err(anyhow::format_err!("The PrimaryBasis {basis:?} is already taken on {self:?}"))
            }
            active_bases = active_bases.union(sig);
            raw_squares[(basis as u8) as usize] = square;
        }
        Ok(Self { active_bases, raw_squares })
    }

    pub fn overwrite<const N: usize>(self, generator_squares: [(GeneratorElement, i8); N]) -> Self {
        let mut active_bases = self.active_bases;
        let mut raw_squares = self.raw_squares;
        for (basis, square) in generator_squares {
            active_bases = active_bases.union(basis.signature());
            raw_squares[(basis as u8) as usize] = square;
        }
        Self { active_bases, raw_squares }
    }

    pub fn square_basis(&self, basis: GeneratorElement) -> i8 {
        self.raw_squares[(basis as u8) as usize]
    }

    pub fn square_element(&self, a: BasisElement) -> i8 {
        if a.coefficient == 0 {
            return 0
        }
        // (-1)^2 == 1^2 == 1
        let mut sign = 1i8;

        let mut sig = a.signature;
        sig = self.anti_scalar().signature.intersection(sig);
        let (a_len, a) = sig.into_generator_elements();
        let mut a_idx = 0;
        while a_idx < a_len {
            let a_ = a[a_idx].unwrap();
            // TODO remove this commented bit if indeed we don't need it
            // Must move b_ at least to the right of a_.
            // Which negates the sign each step.
            // let swaps = ((a_len - 1) - a_idx) as u32;
            // sign *= i8::pow(-1, swaps % 2);
            sign = sign * match self.square_basis(a_) {
                0 => return 0i8,
                sq => sq
            };
            a_idx += 1;
        }
        sign
    }
}


#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum GeneratorElement {
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

impl GeneratorElement {
    pub const fn array() -> [Self; 16] {
        [
            GeneratorElement::e0, GeneratorElement::e1, GeneratorElement::e2, GeneratorElement::e3,
            GeneratorElement::e4, GeneratorElement::e5, GeneratorElement::e6, GeneratorElement::e7,
            GeneratorElement::e8, GeneratorElement::e9, GeneratorElement::eA, GeneratorElement::eB,
            GeneratorElement::eC, GeneratorElement::eD, GeneratorElement::eE, GeneratorElement::eF,
        ]
    }

    const fn bits(self) -> u16 {
        1u16 << self as u8
    }

    pub const fn signature(self) -> BasisSignature {
        BasisSignature::from_bits_retain(self.bits())
    }

    pub const fn element(self) -> BasisElement {
        BasisElement {
            coefficient: 1,
            signature: self.signature(),
            display_name: None,
        }
    }

    const fn const_cmp(&self, other: &GeneratorElement) -> Ordering {
        let a = *self as u8;
        let b = *other as u8;
        if a < b {
            return Ordering::Less
        }
        if a > b {
            return Ordering::Greater
        }
        Ordering::Equal
    }
}

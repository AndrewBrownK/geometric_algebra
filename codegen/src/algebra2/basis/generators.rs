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
    pub const fn anti_scalar(&self) -> BasisElement {
        let signature = self.active_bases;
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

    pub const fn square_generator(&self, basis: GeneratorElement) -> i8 {
        self.raw_squares[(basis as u8) as usize]
    }

    pub const fn square_element(&self, a: BasisElement) -> i8 {
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
            sign = sign * match self.square_generator(a_) {
                0 => return 0i8,
                sq => sq
            };
            a_idx += 1;
        }
        sign
    }

    // TODO test
    /// True Geometric Product, in other words without using substituted bases.
    /// If your BasisElements are substituting bases, you'll need to convert to
    /// the underlying bases before you can properly use this function.
    pub const fn true_product(&self, a: BasisElement, b: BasisElement) -> BasisElement {

        // TODO determine if actually const compatible
        // Implementation may look a bit strange because it is const compatible
        let s = a;
        let o = b;

        let (a_len, a) = s.signature.into_grade_1_signatures();
        let (b_len, b) = o.signature.into_grade_1_signatures();
        let mut sign = s.coefficient * o.coefficient;

        let mut result_elements: [Option<BasisSignature>; 16] = [None; 16];
        let mut a_idx = 0;
        let mut b_idx = 0;
        let mut r_idx = 0;
        while a_idx < a_len || b_idx < b_len {
            if a_idx >= a_len {
                result_elements[r_idx] = b[b_idx];
                r_idx += 1;
                b_idx += 1;
                continue
            }
            if b_idx >= b_len {
                result_elements[r_idx] = a[a_idx];
                r_idx += 1;
                a_idx += 1;
                continue
            }
            let a_ = a[a_idx].unwrap();
            let b_ = b[b_idx].unwrap();
            match BasisSignature::const_cmp(&a_, &b_) {
                Ordering::Less => {
                    result_elements[r_idx] = Some(a_);
                    r_idx += 1;
                    a_idx += 1;
                }
                Ordering::Equal => {
                    // TODO if this would be better using square_generator, maybe
                    //  I need to do a refactor and BasisSignature::into_generator_elements
                    //  is nice after all
                    sign *= self.square_element(BasisElement {
                        coefficient: 1,
                        signature: a_,
                        display_name: None,
                    });
                    if sign == 0 {
                        return BasisElement::zero()
                    }
                    // Must move b_ at least to the right of a_.
                    // Which negates the sign each step.
                    let swaps = ((a_len - a_idx) - 1) as u32;
                    sign *= i8::pow(-1, swaps % 2);
                    a_idx += 1;
                    b_idx += 1;
                },
                Ordering::Greater => {
                    // Must move b_ all the way to left of a_.
                    // Which negates the sign each step.
                    result_elements[r_idx] = Some(b_);
                    r_idx += 1;
                    let swaps = (a_len - a_idx) as u32;
                    sign *= i8::pow(-1, swaps % 2);
                    b_idx += 1;
                }
            }
        }
        let mut result_sig = 0u16;
        let mut i = 0;
        while i < r_idx {
            let primary_basis = result_elements[i].unwrap();
            i += 1;
            let additional_sig = primary_basis.bits();
            result_sig = result_sig | additional_sig;
        }
        if sign == 0 {
            result_sig = 0u16;
        }
        let signature = BasisSignature::from_bits_retain(result_sig);
        BasisElement {
            coefficient: sign,
            signature,
            display_name: None,
        }
    }

    /// True Geometric AntiProduct, in other words without using substituted bases.
    /// If your BasisElements are substituting bases, you'll need to convert to
    /// the underlying bases before you can properly use this function.
    pub fn true_anti_product(&self, a: BasisElement, b: BasisElement) -> BasisElement {
        let anti_scalar = self.anti_scalar();
        let a = a.right_complement(anti_scalar);
        let b = b.right_complement(anti_scalar);
        let c = self.true_product(a, b);
        c.left_complement(anti_scalar)
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

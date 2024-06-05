use crate::algebra2::basis::{BasisElement, BasisSignature, PrimaryBasis};

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

    pub fn next_available_basis(&self) -> anyhow::Result<PrimaryBasis> {
        let mut emptying_signature = self.active_bases.clone();

        // The way this works, if the active_bases is not empty and starts at e1 (or higher) instead
        // of e0, then it will skip over e0 (etc.) unless it runs out of bases all the way to eF,
        // and then it will loop around and try the lower bases again.
        for basis in PrimaryBasis::array().into_iter().chain(PrimaryBasis::array()) {
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

    pub fn new<const N: usize>(generator_squares: [(PrimaryBasis, i8); N]) -> Self {
        let mut active_bases = BasisSignature::empty();
        let mut raw_squares = [0i8; 16];
        for (basis, square) in generator_squares {
            active_bases = active_bases.union(basis.signature());
            raw_squares[(basis as u8) as usize] = square;
        }
        Self { active_bases, raw_squares }
    }

    pub fn append<const N: usize>(self, generator_squares: [(PrimaryBasis, i8); N]) -> anyhow::Result<Self> {
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

    pub fn overwrite<const N: usize>(self, generator_squares: [(PrimaryBasis, i8); N]) -> Self {
        let mut active_bases = self.active_bases;
        let mut raw_squares = self.raw_squares;
        for (basis, square) in generator_squares {
            active_bases = active_bases.union(basis.signature());
            raw_squares[(basis as u8) as usize] = square;
        }
        Self { active_bases, raw_squares }
    }

    pub fn square_basis(&self, basis: PrimaryBasis) -> i8 {
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
        let (a_len, a) = sig.into_primary_bases();
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
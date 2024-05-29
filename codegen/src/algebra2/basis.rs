use std::cmp::Ordering;
use std::collections::HashMap;
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
        Some(BasisSignature::const_cmp(self, other))
    }
}
impl Ord for BasisSignature {
    fn cmp(&self, other: &Self) -> Ordering {
        BasisSignature::const_cmp(self, other)
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
    const fn into_primary_bases(self) -> (usize, [Option<PrimaryBasis>; 16]) {
        let mut result = [None; 16];
        let mut i = 0;
        let mut j = 0;
        let mut len = 0;
        let arr = PrimaryBasis::array();
        while i < arr.len() {
            let basis = arr[i];
            i += 1;
            let sig = basis.signature();
            if self.contains(sig) {
                result[j] = Some(basis);
                j += 1;
                len += 1;
            }
        }
        (len, result)
    }

    const fn const_cmp(&self, other: &BasisSignature) -> Ordering {let a = self.bits();
        let a = self.bits();
        let b = other.bits();
        let aco = a.count_ones();
        let bco = b.count_ones();
        if aco < bco {
            return Ordering::Less
        }
        if aco > bco {
            return Ordering::Greater
        }
        let ra = a.reverse_bits();
        let rb = b.reverse_bits();
        if rb < ra {
            return Ordering::Less
        }
        if rb > ra {
            return Ordering::Greater
        }
        return Ordering::Equal
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasisElement {
    // BasisElements will mathematically operate
    // under the assumption that the primary bases
    // are ordered normally, like e123456 etc
    coefficient: i8,
    signature: BasisSignature,

    // However you can also override the name of a
    // basis element (like "e3215" or "e_plus" or "e_minus"
    // or "anti_scalar" or "pseudo_scalar" or "pss" or "i")
    // As long as you also record the sign with
    // respect to the normally ordered element.
    // This way we can tell what to do with all of
    // (for example) +e412, -e412, +e124, and -e124
    display_name: Option<BasisElementDisplayName>,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasisElementDisplayName {
    display_name: &'static str,
    negate_display: bool,
}

impl PartialOrd for BasisElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(BasisElement::const_cmp(self, other))
    }
}
impl Ord for BasisElement {
    fn cmp(&self, other: &Self) -> Ordering {
        BasisElement::const_cmp(self, other)
    }
}
impl Display for BasisElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sign = self.coefficient;
        if let Some(dn) = self.display_name {
            if dn.negate_display {
                sign = sign * -1;
            }
            return match sign {
                0 => write!(f, "0"),
                1 => write!(f, "{}", dn.display_name),
                -1 => write!(f, "-{}", dn.display_name),
                c => write!(f, "{}*{}", c, dn.display_name),
            };
        }
        match sign {
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
            display_name: None,
        }
    }
}

impl From<BasisSignature> for BasisElement {
    fn from(signature: BasisSignature) -> Self {
        Self {
            coefficient: 1,
            signature,
            display_name: None,
        }
    }
}

impl BasisElement {
    const fn const_cmp(&self, other: &Self) -> Ordering {
        match BasisSignature::const_cmp(&self.signature, &other.signature) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => {}
        }
        let ac = self.coefficient;
        let bc = other.coefficient;
        if ac < bc {
            return Ordering::Less
        }
        if ac > bc {
            return Ordering::Greater
        }
        Ordering::Equal
    }

    pub const fn coefficient(&self) -> i8 {
        self.coefficient
    }

    pub const fn signature(&self) -> BasisSignature {
        self.signature
    }

    pub const fn zero() -> Self {
        Self {
            coefficient: 0,
            signature: BasisSignature::scalar,
            display_name: None,
        }
    }

    pub const fn scalar() -> Self {
        Self {
            coefficient: 1,
            signature: BasisSignature::scalar,
            display_name: None,
        }
    }

    pub const fn negate(mut self) -> Self {
        self.coefficient *= -1;
        self
    }

    pub const fn grade(&self) -> u8 {
        self.signature.bits().count_ones() as u8
    }

    pub const fn parsed_display_name(s: &'static str) -> Option<Self> {
        let mut result = BasisElement::zero();
        result.coefficient = 1;
        let mut display_name = BasisElementDisplayName {
            display_name: s,
            negate_display: false,
        };

        // This might not look very idiomatic at first glance,
        // but keep in mind we are operating in a const fn.

        let s = s.as_bytes();
        // if s == b"zero" {
        //     display_name.display_name = "zero";
        //     result.display_name = Some(display_name);
        //     result.coefficient = 0;
        //     return Some(result)
        // }
        // if s == b"scalar" {
        //     display_name.display_name = "scalar";
        //     result.display_name = Some(display_name);
        //     result.coefficient = 1;
        //     return Some(result)
        // }
        let mut i = 0;
        let mut reached_elements = false;
        while i < s.len() {
            let c = s[i];
            i += 1;
            let next_basis = match (reached_elements, c) {
                // (false, b'-') => {
                //     display_name.negate_display = !display_name.negate_display;
                //     continue
                // }
                (false, b'0') => {
                    result.coefficient = 0;
                    result.signature = BasisSignature::empty();
                    // For parsing here, it would be weird for someone to specify stuff
                    // after a 0, like "0e123". So if you want to be weird, do that
                    // with a manually set display_name instead of me figure out
                    // strange cases in const evaluation.
                    return if i == s.len() {
                        result.display_name = Some(display_name);
                        Some(result)
                    } else {
                        None
                    }
                }
                (false, b'1') => {
                    result.coefficient = 1;
                    result.signature = BasisSignature::empty();
                    return if i == s.len() {
                        result.display_name = Some(display_name);
                        Some(result)
                    } else {
                        None
                    }
                }
                (false, b'e') => {
                    reached_elements = true;
                    continue
                }
                (true, b'0') => PrimaryBasis::e0,
                (true, b'1') => PrimaryBasis::e1,
                (true, b'2') => PrimaryBasis::e2,
                (true, b'3') => PrimaryBasis::e3,
                (true, b'4') => PrimaryBasis::e4,
                (true, b'5') => PrimaryBasis::e5,
                (true, b'6') => PrimaryBasis::e6,
                (true, b'7') => PrimaryBasis::e7,
                (true, b'8') => PrimaryBasis::e8,
                (true, b'9') => PrimaryBasis::e9,
                (true, b'A') => PrimaryBasis::eA,
                (true, b'B') => PrimaryBasis::eB,
                (true, b'C') => PrimaryBasis::eC,
                (true, b'D') => PrimaryBasis::eD,
                (true, b'E') => PrimaryBasis::eE,
                (true, b'F') => PrimaryBasis::eF,
                _ => return None
            };
            let sig_addition = next_basis.signature();
            let sig_existing = result.signature;

            // Reject double bases.
            if sig_existing.contains(sig_addition) {
                return None;
            }
            // Alright so the slot is open for this primary element.
            // Now the only question is how many primary elements we have to swap to be in order.
            let sig_behind_this_basis = BasisSignature::from_bits_retain(u16::MAX - (sig_addition.bits() - 1));
            let yes_swap = sig_existing.intersection(sig_behind_this_basis);
            let exp = yes_swap.bits().count_ones();
            let negate = -1 == i8::pow(-1, exp);
            if negate {
                result.coefficient = result.coefficient * -1;
                display_name.negate_display = !display_name.negate_display;
            }
            // Alright and finally actually update the signature
            result.signature = sig_existing.union(sig_addition);
        }
        result.display_name = Some(display_name);
        Some(result)
    }

    pub const fn with_name(mut self, display_name: &'static str, odd_permutation: bool) -> Self {
        let dn = BasisElementDisplayName {
            display_name,
            negate_display: odd_permutation,
        };
        self.display_name = Some(dn);
        self
    }

    pub const fn reverse(&self) -> Self {
        let gr = self.grade() as u32;
        let exp = gr * (gr - 1) / 2;
        let mut copy = *self;
        copy.coefficient = i8::pow(-1i8, exp) * copy.coefficient;
        copy
    }

    pub const fn anti_reverse(&self, anti_scalar: BasisElement) -> Self {
        let r = self.right_complement(anti_scalar);
        let r = r.reverse();
        r.left_complement(anti_scalar)
    }

    pub const fn right_complement(&self, anti_scalar: BasisElement) -> BasisElement {
        if !anti_scalar.signature.contains(self.signature) {
            panic!("Cannot take the right complement of a BasisElement with respect to an \
                AntiScalar that does not contain it.")
        }
        if self.coefficient == 0 {
            return BasisElement::zero();
        }

        let new_sig = anti_scalar.signature.bits() - self.signature.bits();

        // Negative coefficient anti_scalar is allowed
        let mut answer = BasisElement::scalar();
        answer.coefficient = self.coefficient * anti_scalar.coefficient;
        answer.signature = BasisSignature::from_bits_retain(new_sig);

        // Now we can test if we have to actually worry about the sign.
        let d = anti_scalar.grade();
        let gr = self.grade();
        let ag = d - gr;

        // Hmmm it is so inconvenient to define these,
        // maybe I should get them as a dependency instead
        const fn is_even(num: u8) -> bool { num % 2 == 0 }
        const fn is_odd(num: u8) -> bool { num % 2 != 0 }

        if is_odd(d) || is_even(gr) || is_even(ag) {
            return answer;
        }

        // Okay we should double-check the sign.
        let test = self.wedge(answer);
        if test.coefficient == anti_scalar.coefficient {
            return answer
        }
        if test.coefficient == -anti_scalar.coefficient {
            answer.coefficient = -1 * answer.coefficient;
            return answer
        }

        // This basically shouldn't happen unless the i8 coefficient somehow gets corrupted
        // panic!("Cannot figure out right_complement for strange element: {self:?} anti_scalar: {anti_scalar:?}")

        // Limited/no formatting options in const eval
        panic!("Cannot figure out right_complement for strange element")
    }

    pub const fn left_complement(&self, anti_scalar: BasisElement) -> BasisElement {
        let mut rc = self.right_complement(anti_scalar);
        let d = anti_scalar.grade();
        let gr = self.grade();
        let ag = d - gr;
        let exp = gr * ag;
        rc.coefficient = rc.coefficient * i8::pow(-1, exp as u32);
        rc
    }

    /// Wedge product
    pub const fn wedge(
        &self,
        other: BasisElement,
    ) -> BasisElement {
        // Implementation may look a bit strange because it is const compatible

        let (a_len, a) = self.signature.into_primary_bases();
        let (b_len, b) = other.signature.into_primary_bases();
        let mut sign = self.coefficient * other.coefficient;

        let mut result_elements: [Option<PrimaryBasis>; 16] = [None; 16];
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
            match PrimaryBasis::const_cmp(&a_, &b_) {
                Ordering::Less => {
                    result_elements[r_idx] = Some(a_);
                    r_idx += 1;
                    a_idx += 1;
                }
                Ordering::Equal => return BasisElement::zero(),
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

    /// AntiWedge product
    pub const fn anti_wedge(
        &self,
        other: BasisElement,
        anti_scalar: BasisElement,
    ) -> BasisElement {
        let s = self.right_complement(anti_scalar);
        let o = other.right_complement(anti_scalar);
        let w = s.wedge(o);
        w.left_complement(anti_scalar)
    }
}


#[derive(Clone, Debug)]
pub struct BasisElementNames {
    zero: Option<BasisElementDisplayName>,
    elements: HashMap<BasisSignature, BasisElementDisplayName>
}
impl BasisElementNames {
    pub fn new() -> Self {
        BasisElementNames {
            zero: None,
            elements: HashMap::new(),
        }
    }

    /// Give a name to a BasisElement, if one exists.
    pub fn provide_name(&self, mut el: BasisElement) -> BasisElement {
        let existing = if el.coefficient == 0 {
            self.zero
        } else {
            self.elements.get(&el.signature).cloned()
        };
        if let Some(dn) = existing {
            el.display_name = Some(dn);
        }
        return el
    }

    /// Add a name on a BasisElement (if it exists) to the BasisElementNames.
    pub fn accept_name(&mut self, el: BasisElement) -> anyhow::Result<()> {
        let Some(el_dn) = el.display_name else { return Ok(()) };
        let sig = el.signature;
        let existing = if el.coefficient == 0 {
            self.zero
        } else {
            self.elements.get(&sig).cloned()
        };
        if let Some(dn) = existing {
            if el_dn != dn {
                anyhow::bail!("BasisElementNames cannot accept name {el_dn:?} because it already has {dn:?} for the same signature {sig:?}")
            }
        } else {
            self.elements.insert(sig, el_dn);
        }
        Ok(())
    }
}




#[test]
fn new_basis_elements_wedge() {
    let mut rng = rand::thread_rng();
    let mut i = 0;
    while i < 100 {
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
            let sq: i8 = ((rng.gen::<i8>().max(i8::MIN + 1).abs() % 11) - 5).signum();
            squares = squares.append([(basis, sq)]).unwrap();
        }

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
        // The old BasisElement product is a geometric product, not a wedge product
        let old_product = old_a.primitive_product(&old_b, sq.as_slice());
        let new_product = new_a.wedge(new_b);

        // So we do this check to make sure we are comparing apples to apples
        if old_product.coefficient != 0 && old_product.index.count_ones() != (a.count_ones() + b.count_ones()) {
            continue
        }
        i += 1;

        println!("Squares: {squares:?}");
        println!("a: {a_s} {a:016b}");
        println!("b: {b_s} {b:016b}");

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
    pub const fn array() -> [Self; 16] {
        [
            PrimaryBasis::e0, PrimaryBasis::e1, PrimaryBasis::e2, PrimaryBasis::e3,
            PrimaryBasis::e4, PrimaryBasis::e5, PrimaryBasis::e6, PrimaryBasis::e7,
            PrimaryBasis::e8, PrimaryBasis::e9, PrimaryBasis::eA, PrimaryBasis::eB,
            PrimaryBasis::eC, PrimaryBasis::eD, PrimaryBasis::eE, PrimaryBasis::eF,
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

    const fn const_cmp(&self, other: &PrimaryBasis) -> Ordering {
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

    pub fn square(&self, basis: PrimaryBasis) -> i8 {
        self.raw_squares[(basis as u8) as usize]
    }
}

#[allow(non_upper_case_globals, dead_code)]
pub mod elements {
    use crate::algebra2::basis::*;

    include!(concat!(env!("OUT_DIR"), "/generated_elements.rs"));

    // And we'll add some custom bases as seen in the wild.
    // (But let's not go overboard by using code generation on this part...
    //  The number of permutations is the factorial of the number of dimensions.)

    const fn const_parse(s: &'static str) -> BasisElement {
        BasisElement::parsed_display_name(s).expect("Failed to parse const BasisElement")
    }

    // Eric Lengyel's bases:
    pub const e41: BasisElement = const_parse("e41");
    pub const e42: BasisElement = const_parse("e42");
    pub const e43: BasisElement = const_parse("e43");
    pub const e31: BasisElement = const_parse("e31");
    pub const e423: BasisElement = const_parse("e423");
    pub const e431: BasisElement = const_parse("e431");
    pub const e412: BasisElement = const_parse("e412");
    pub const e321: BasisElement = const_parse("e321");
    pub const e415: BasisElement = const_parse("e415");
    pub const e425: BasisElement = const_parse("e425");
    pub const e435: BasisElement = const_parse("e435");
    pub const e315: BasisElement = const_parse("e315");
    pub const e4235: BasisElement = const_parse("e4235");
    pub const e4315: BasisElement = const_parse("e4315");
    pub const e4125: BasisElement = const_parse("e4125");
    pub const e3215: BasisElement = const_parse("e3215");

    #[test]
    fn test_parse_custom_bases() {
        use std::fmt::Write;
        let cases = [
            (e41, "e41", true, e14.negate()),
            (e42, "e42", true, e24.negate()),
            (e43, "e43", true, e34.negate()),
            (e31, "e31", true, e13.negate()),
            (e423, "e423", false, e234),
            (e431, "e431", true, e134.negate()),
            (e412, "e412", false, e124),
            (e321, "e321", true, e123.negate()),
            (e415, "e415", true, e145.negate()),
            (e425, "e425", true, e245.negate()),
            (e435, "e435", true, e345.negate()),
            (e315, "e315", true, e135.negate()),
            (e4235, "e4235", false, e2345),
            (e4315, "e4315", true, e1345.negate()),
            (e4125, "e4125", false, e1245),
            (e3215, "e3215", true, e1235.negate()),
        ];
        for (custom_element, correct_name, display_is_negated, ordered_element) in cases {
            assert_eq!(
                custom_element.signature(), ordered_element.signature(),
                "Custom BasisElement {custom_element:?} does not match signature of {ordered_element:?}"
            );
            assert_eq!(
                custom_element.coefficient(), ordered_element.coefficient(),
                "Custom BasisElement {custom_element:?} does not match coefficient of {ordered_element:?}"
            );
            let dn = custom_element.display_name.expect("Parsed BasisElements should have custom names");
            assert_eq!(
                dn.negate_display, display_is_negated,
                "Custom BasisElement {custom_element:?} has incorrect display negation"
            );
            let mut n = String::new();
            write!(n, "{custom_element}").expect("BasisElements must implement Display without fail");
            assert_eq!(
                n.as_str(), correct_name,
                "Custom BasisElement {custom_element:?} does not display to \"{correct_name}\""
            );

            // Negated display
            let mut custom_element = custom_element;
            custom_element.coefficient = -1 * custom_element.coefficient;
            let correct_name = format!("-{correct_name}");
            let mut n = String::new();
            write!(n, "{custom_element}").expect("BasisElements must implement Display without fail");
            assert_eq!(
                n, correct_name,
                "Custom BasisElement {custom_element:?} does not display to \"{correct_name}\""
            );
        }
    }
}
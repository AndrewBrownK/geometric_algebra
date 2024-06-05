use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use rand::Rng;
use generators::GeneratorSquares;

use crate::algebra::basis_element;

pub mod generators;

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

    // Strange return type is because of const evaluation compatibility
    const fn into_primary_bases(self) -> (usize, [Option<BasisSignature>; 16]) {
        let mut result = [None; 16];
        let mut i = 0;
        let mut j = 0;
        let mut len = 0;
        while i < 16 {
            let sig = BasisSignature::from_bits_retain(u16::pow(2, i as u32));
            i += 1;
            if self.contains(sig) {
                result[j] = Some(sig);
                j += 1;
                len += 1;
            }
        }
        (len, result)
    }

    const fn const_cmp(&self, other: &BasisSignature) -> Ordering {
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
            let next_basis: u16 = match (reached_elements, c) {
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
                (true, b'0') => 0x0,
                (true, b'1') => 0x1,
                (true, b'2') => 0x2,
                (true, b'3') => 0x3,
                (true, b'4') => 0x4,
                (true, b'5') => 0x5,
                (true, b'6') => 0x6,
                (true, b'7') => 0x7,
                (true, b'8') => 0x8,
                (true, b'9') => 0x9,
                (true, b'A') => 0xA,
                (true, b'B') => 0xB,
                (true, b'C') => 0xC,
                (true, b'D') => 0xD,
                (true, b'E') => 0xE,
                (true, b'F') => 0xF,
                _ => return None
            };
            let sig_addition = BasisSignature::from_bits_retain(next_basis);
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

#[test]
fn test_generator_squares() {
    use elements::*;
    let rga = GeneratorSquares::new([(E1, 1), (E2, 1), (E3, 1), (E4, 0)]);
    let cga = GeneratorSquares::new([(E1, 1), (E2, 1), (E3, 1), (E4, 1), (E5, -1)]);

    //

    assert_eq!(1, rga.square_element(scalar));

    assert_eq!(1, rga.square_element(e1));
    assert_eq!(1, rga.square_element(e2));
    assert_eq!(1, rga.square_element(e3));
    assert_eq!(0, rga.square_element(e4));

    assert_eq!(1, rga.square_element(e12));
    assert_eq!(1, rga.square_element(e23));
    assert_eq!(1, rga.square_element(e31));
    assert_eq!(0, rga.square_element(e41));
    assert_eq!(0, rga.square_element(e42));
    assert_eq!(0, rga.square_element(e43));

    assert_eq!(1, rga.square_element(e123));
    assert_eq!(0, rga.square_element(e412));
    assert_eq!(0, rga.square_element(e423));
    assert_eq!(0, rga.square_element(e431));

    assert_eq!(0, rga.square_element(e1234));

    //

    assert_eq!(1, cga.square_element(scalar));

    assert_eq!(1, cga.square_element(e1));
    assert_eq!(1, cga.square_element(e2));
    assert_eq!(1, cga.square_element(e3));
    assert_eq!(1, cga.square_element(e4));
    assert_eq!(-1, cga.square_element(e5));

    assert_eq!(1, cga.square_element(e12));
    assert_eq!(1, cga.square_element(e23));
    assert_eq!(1, cga.square_element(e31));
    assert_eq!(1, cga.square_element(e41));
    assert_eq!(1, cga.square_element(e42));
    assert_eq!(1, cga.square_element(e43));
    assert_eq!(-1, cga.square_element(e15));
    assert_eq!(-1, cga.square_element(e25));
    assert_eq!(-1, cga.square_element(e35));
    assert_eq!(-1, cga.square_element(e45));

    assert_eq!(1, cga.square_element(e123));
    assert_eq!(1, cga.square_element(e412));
    assert_eq!(1, cga.square_element(e423));
    assert_eq!(1, cga.square_element(e431));
    assert_eq!(-1, cga.square_element(e125));
    assert_eq!(-1, cga.square_element(e235));
    assert_eq!(-1, cga.square_element(e315));
    assert_eq!(-1, cga.square_element(e415));
    assert_eq!(-1, cga.square_element(e425));
    assert_eq!(-1, cga.square_element(e435));

    assert_eq!(1, cga.square_element(e1234));
    assert_eq!(-1, cga.square_element(e1235));
    assert_eq!(-1, cga.square_element(e1245));
    assert_eq!(-1, cga.square_element(e1345));
    assert_eq!(-1, cga.square_element(e2345));

    assert_eq!(-1, cga.square_element(e12345));
}


#[allow(non_upper_case_globals, dead_code)]
pub mod elements {
    use crate::algebra2::basis::*;
    use crate::algebra2::basis::generators::GeneratorElement;

    // List some primary basis elements

    pub const E0: GeneratorElement = GeneratorElement::e0;
    pub const E1: GeneratorElement = GeneratorElement::e1;
    pub const E2: GeneratorElement = GeneratorElement::e2;
    pub const E3: GeneratorElement = GeneratorElement::e3;
    pub const E4: GeneratorElement = GeneratorElement::e4;
    pub const E5: GeneratorElement = GeneratorElement::e5;
    pub const E6: GeneratorElement = GeneratorElement::e6;
    pub const E7: GeneratorElement = GeneratorElement::e7;
    pub const E8: GeneratorElement = GeneratorElement::e8;
    pub const E9: GeneratorElement = GeneratorElement::e9;
    pub const EA: GeneratorElement = GeneratorElement::eA;
    pub const EB: GeneratorElement = GeneratorElement::eB;
    pub const EC: GeneratorElement = GeneratorElement::eC;
    pub const ED: GeneratorElement = GeneratorElement::eD;
    pub const EE: GeneratorElement = GeneratorElement::eE;
    pub const EF: GeneratorElement = GeneratorElement::eF;


    // List a bunch of generated elements

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

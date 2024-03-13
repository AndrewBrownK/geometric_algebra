use std::fmt::Write;

/// The BasisElementIndex is a set of bits representing the orthogonal vector BasisElements
/// in (what might be) a product of BasisElements. e0 is binary 1, e1 is binary 10, and e01 is binary 11.
/// This means the scalar "BasisElement" that actually has no multivector parts is binary 0.
pub type BasisElementIndex = u16;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BasisElement {
    // TODO is it supposed to be an f32 for CGA, or is isize still okay?
    pub coefficient: isize,
    pub index: BasisElementIndex,
}

impl BasisElement {
    pub fn from_index(index: BasisElementIndex) -> Self {
        Self { coefficient: 1, index }
    }

    pub fn component_bases(&self) -> impl Iterator<Item = usize> + '_ {
        (0..std::mem::size_of::<BasisElementIndex>() * 8).filter(move |index| (self.index >> index) & 1 != 0)
    }

    pub fn primitive_product(&self, b: &BasisElement, generator_squares: &[isize]) -> BasisElement {
        let a = self;
        // let commutations = a.component_bases().fold((0, a.index, b.index), |(commutations, a, b), index| {
        //     let hurdles_a = a & (BasisElementIndex::MAX << (index + 1));
        //     let hurdles_b = b & ((1 << index) - 1);
        //     (
        //         commutations + BasisElement::from_index(hurdles_a | hurdles_b).grade(),
        //         a & !(1 << index),
        //         b ^ (1 << index),
        //     )
        // }).0;




        // Example:
        // a = e134 = b01101
        // b = e235 = b10110

        let mut commutations = 0;
        let mut remaining_a_index = a.index;
        let mut remaining_b_index = b.index;
        for index in a.component_bases() {
            let hurdles_a = remaining_a_index & (BasisElementIndex::MAX << (index + 1));
            let hurdles_b = remaining_b_index & ((1 << index) - 1);
            commutations = commutations + BasisElement::from_index(hurdles_a | hurdles_b).grade();
            remaining_a_index = remaining_a_index & !(1 << index);
            remaining_b_index = remaining_b_index ^ (1 << index);

            // index = 0
            // hurdles_a = b01101 & (b11111 << (0+1)) = b01101 & b11110 = b01100
            // hurdles_b = b10110 & ((1 << 0) - 1) = b10110 & b00000 = b00000
            // commutations = 0 + grade(b01100) = 2
            // remaining_a_index = b01101 & !(1 << 0) = b01100
            // remaining_b_index = b10110 ^ (1 << 0) = b10111

            // index = 2
            // hurdles_a = b01100 & (b11111 << (2+1)) = b01100 & b11000 = b01000
            // hurdles_b = b10111 & ((1 << 2) - 1) = b10111 & b00011 = b00011
            // commutations = 2 + grade(b01011) = 5
            // remaining_a_index = b01100 & !(1 << 2) = b01000
            // remaining_b_index = b10111 ^ (1 << 2) = b10011

            // index = 3
            // hurdles_a = b01000 & (b11111 << (3+1)) = b01000 & b10000 = b00000
            // hurdles_b = b10011 & ((1 << 3) - 1) = b10011 & b00111 = b00011
            // commutations = 5 + grade(b00011) = 7
            // remaining_a_index = b01000 & !(1 << 3) = b00000
            // remaining_b_index = b10011 ^ (1 << 3) = b11011

            // commutations = 7

            // Or in a table layout....
            // index    hurdles_a    hurdles_b    commutations    remaining_a_index    remaining_b_index
            //     _            _            _               0               b01101               b10110
            //     0       b01100       b00000               2               b01100               b10111
            //     2       b01000       b00011               5               b01000               b10011
            //     3       b00000       b00011               7               b00000               b11011

            // So what are hurdles? And what is this "count of commutations"?
            // Well you can see a few things....
            // - remaining_a_index is digested a 1 at a time, and converted to all 0s
            // - remaining_b_index is converted to all 1s, except a 0 when where it overlaps remainign_a_index
            // so again... what are hurdles?
            // - hurdles_a seems to be the "remaining bases not yet reached"

            // TODO honestly bro it's just really fucking obscure. I'm going to try working around/without
            //  understanding this for now and see if I can still do what I need to do.
        }
        let coefficient = BasisElement::from_index(a.index & b.index)
            .component_bases()
            .map(|i| generator_squares[i])
            .fold(a.coefficient * b.coefficient * if commutations % 2 == 0 { 1 } else { -1 }, |a, b| a * b);
        BasisElement {
            coefficient,
            index: a.index ^ b.index,
        }
    }
}


// NEW PLAN (compared to original geometric_algebra repository)
//  Since all the weird complements and stuff is characteristic to the strange GeometricAlgebra that is CGA...
//  Leave BasisElement as a totally plain data type,
//  And make all the fancy "complement" and "product" and "involution" behavioral stuff associated with the GA instead.
//  (see GeometricAlgebraTrait)
impl BasisElement {
    pub fn get_coefficient(&self) -> isize {
        self.coefficient
    }

    pub fn set_coefficient(&mut self, c: isize) {
        self.coefficient = c;
    }

    pub fn grade(&self) -> usize {
        self.index.count_ones() as usize
    }
}

impl std::fmt::Display for BasisElement {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = format!("e{}", self.component_bases().map(|index| format!("{:X}", index + 1)).collect::<String>());
        if self.coefficient == -1 {
            formatter.write_str("-")?;
        }
        if self.coefficient == 0 {
            formatter.write_str("0")?;
        } else if self.index == 0 {
            formatter.write_str("1")?;
        } else {
            formatter.write_str(name.as_str())?;
        }
        Ok(())
    }
}

impl Ord for BasisElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let grades_order = self.grade().cmp(&other.grade());
        if grades_order != std::cmp::Ordering::Equal {
            return grades_order;
        }
        let a_without_b = self.index & (!other.index);
        let b_without_a = other.index & (!self.index);
        if a_without_b.trailing_zeros() < b_without_a.trailing_zeros() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl PartialOrd for BasisElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


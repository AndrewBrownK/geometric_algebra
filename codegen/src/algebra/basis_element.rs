/// The BasisElementIndex is a set of bits representing the orthogonal vector BasisElements
/// in (what might be) a product of BasisElements. e0 is binary 1, e1 is binary 10, and e01 is binary 11.
/// This means the scalar "BasisElement" that actually has no multivector parts is binary 0.
pub type BasisElementIndex = u16;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BasisElement {
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
        let mut commutations = 0;
        let mut remaining_a_index = a.index;
        let mut remaining_b_index = b.index;
        for index in a.component_bases() {
            let hurdles_a = remaining_a_index & (BasisElementIndex::MAX << (index + 1));
            let hurdles_b = remaining_b_index & ((1 << index) - 1);
            commutations = commutations + BasisElement::from_index(hurdles_a | hurdles_b).grade();
            remaining_a_index = remaining_a_index & !(1 << index);
            remaining_b_index = remaining_b_index ^ (1 << index);
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


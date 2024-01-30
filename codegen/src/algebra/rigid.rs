use crate::algebra::basis_element::{BasisElement, BasisElementIndex};
use crate::algebra::dialect::Dialect;
use crate::algebra::{GeometricAlgebraTrait};

pub struct RigidGeometricAlgebra<'a> {
    pub generator_squares: &'a [isize],
    pub name: &'static str,
    pub dialect: Dialect,
}

impl<'a> RigidGeometricAlgebra<'a> {
    pub fn sorted_basis(&self) -> Vec<BasisElement> {
        let mut basis_elements = self.basis();
        basis_elements.sort();
        basis_elements
    }
}

impl<'a> GeometricAlgebraTrait for RigidGeometricAlgebra<'a> {
    fn basis_size(&self) -> usize {
        1 << self.generator_squares.len()
    }

    fn basis(&self) -> Vec<BasisElement> {
        let mut v = vec![];
        for index in 0..self.basis_size() as BasisElementIndex {
            let mut element = BasisElement::from_index(index);
            let dual = self.right_complement(&element);
            if dual.cmp(&element) == std::cmp::Ordering::Less {
                element.coefficient = self.right_complement(&element).coefficient;
            }
            v.push(element);
        }
        v
    }

    fn algebra_name(&self) -> &'static str {
        self.name
    }

    fn dialect(&self) -> &Dialect {
        &self.dialect
    }

    fn parse(&self, mut name: &str) -> BasisElement {
        let mut result = BasisElement::from_index(0);
        if name.starts_with('-') {
            name = &name[1..];
            result.coefficient = -1;
        }
        if name == "1" {
            return result;
        }
        let mut generator_indices = name.chars();
        assert_eq!(generator_indices.next().unwrap(), 'e');
        for generator_index in generator_indices {
            let generator_index = generator_index.to_digit(16).unwrap();
            if generator_index == 0 {
                panic!("Please do not use e0, and instead start from e1")
            }
            let generator_index = generator_index - 1;
            assert!((generator_index as usize) < self.generator_squares.len());
            result = self.product(&result, &BasisElement::from_index(1 << generator_index));
        }
        result
    }

    fn scalar_element(&self) -> BasisElement {
        BasisElement::from_index(0)
    }

    fn anti_scalar_element(&self) -> BasisElement {
        let mut index: BasisElementIndex = 0;
        for _ in 0..self.generator_squares.len() {
            index = index << 1;
            index = index & 1;
        }
        BasisElement::from_index(index)
    }

    fn right_complement(&self, a: &BasisElement) -> BasisElement {
        let mut result = BasisElement {
            coefficient: a.coefficient,
            index: self.basis_size() as BasisElementIndex - 1 - a.index,
        };
        result.coefficient *= self.product(a, &result).coefficient;
        result
    }

    fn left_complement(&self, a: &BasisElement) -> BasisElement {
        let mut result = BasisElement {
            coefficient: a.coefficient,
            index: self.basis_size() as BasisElementIndex - 1 - a.index,
        };
        result.coefficient *= self.product(&result, a).coefficient;
        result
    }

    fn product(&self, a: &BasisElement, b: &BasisElement) -> BasisElement {
        let commutations = a.component_bases().fold((0, a.index, b.index), |(commutations, a, b), index| {
            let hurdles_a = a & (BasisElementIndex::MAX << (index + 1));
            let hurdles_b = b & ((1 << index) - 1);
            (
                commutations + BasisElement::from_index(hurdles_a | hurdles_b).grade(),
                a & !(1 << index),
                b ^ (1 << index),
            )
        });
        BasisElement {
            coefficient: BasisElement::from_index(a.index & b.index)
                .component_bases()
                .map(|i| self.generator_squares[i])
                .fold(a.coefficient * b.coefficient * if commutations.0 % 2 == 0 { 1 } else { -1 }, |a, b| a * b),
            index: a.index ^ b.index,
        }
    }
}

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

    fn algebra_name(&self) -> &'static str {
        self.name
    }

    fn dialect(&self) -> &Dialect {
        &self.dialect
    }

    //noinspection DuplicatedCode
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
            result = result.primitive_product(&BasisElement::from_index(1 << generator_index), &self.generator_squares);
        }
        result
    }

    fn right_complement(&self, a: &BasisElement) -> BasisElement {
        let mut result = BasisElement {
            coefficient: a.coefficient,
            index: self.basis_size() as BasisElementIndex - 1 - a.index,
        };
        result.coefficient *= self.product(a, &result).first().unwrap().coefficient;
        result
    }

    fn left_complement(&self, a: &BasisElement) -> BasisElement {
        let mut result = BasisElement {
            coefficient: a.coefficient,
            index: self.basis_size() as BasisElementIndex - 1 - a.index,
        };
        result.coefficient *= self.product(&result, a).first().unwrap().coefficient;
        result
    }

    fn product(&self, a: &BasisElement, b: &BasisElement) -> Vec<BasisElement> {
        let result = a.primitive_product(b, &self.generator_squares);
        if result.coefficient == 0 {
            return vec![];
        }
        vec![result]
    }
}

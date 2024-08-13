use crate::algebra::basis_element::{BasisElement, BasisElementIndex};
use crate::algebra::dialect::Dialect;
use crate::algebra::GeometricAlgebraTrait;

pub struct RigidGeometricAlgebra {
    generator_squares: Vec<isize>,
    pub name: &'static str,
    pub dialect: Dialect,
}

impl RigidGeometricAlgebra {
    pub fn new(name: &'static str, dimensions: usize, dialect: Dialect) -> Self {
        let mut generator_squares = vec![1; dimensions];
        generator_squares.push(0);
        return RigidGeometricAlgebra { generator_squares, name, dialect };
    }
    pub fn sorted_basis(&self) -> Vec<BasisElement> {
        let mut basis_elements = self.basis();
        basis_elements.sort();
        basis_elements
    }
}

impl GeometricAlgebraTrait for RigidGeometricAlgebra {
    fn basis_size(&self) -> usize {
        1 << self.generator_squares.len()
    }

    fn algebra_name(&self) -> &'static str {
        self.name
    }

    fn dialect(&self) -> &Dialect {
        &self.dialect
    }

    // noinspection DuplicatedCode
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

    fn is_degenerate(&self) -> bool {
        return true;
    }

    fn has_multiple_complements(&self) -> bool {
        return self.generator_squares.len() % 2 == 0;
    }

    fn right_complement(&self, a: &BasisElement) -> BasisElement {
        let mut result = BasisElement {
            coefficient: a.coefficient,
            index: self.basis_size() as BasisElementIndex - 1 - a.index,
        };
        result.coefficient *= a.primitive_product(&result, &self.generator_squares).coefficient;
        result
    }

    fn left_complement(&self, a: &BasisElement) -> BasisElement {
        let mut result = BasisElement {
            coefficient: a.coefficient,
            index: self.basis_size() as BasisElementIndex - 1 - a.index,
        };
        result.coefficient *= a.primitive_product(&result, &self.generator_squares).coefficient;
        result
    }

    fn dual(&self, a: &BasisElement) -> BasisElement {
        // May return zero because of degenerate metric. See page 67 of book
        let origin = (1 as BasisElementIndex) << self.represented_dimensions();
        let aligned_origin = a.index & origin == origin;
        if aligned_origin {
            return BasisElement { coefficient: 0, index: 0 };
        }
        return self.right_complement(a);
    }

    fn anti_dual(&self, a: &BasisElement) -> BasisElement {
        // May return zero because of degenerate metric. See page 68 of book
        let origin = (1 as BasisElementIndex) << self.represented_dimensions();
        let aligned_origin = a.index & origin == origin;
        if !aligned_origin {
            return BasisElement { coefficient: 0, index: 0 };
        }
        return self.right_complement(a);
    }

    fn product(&self, a: &BasisElement, b: &BasisElement) -> Vec<BasisElement> {
        let result = a.primitive_product(b, &self.generator_squares);
        if result.coefficient == 0 {
            return vec![];
        }
        vec![result]
    }

    fn anti_product(&self, a: &BasisElement, b: &BasisElement) -> Vec<BasisElement> {
        let result = a.primitive_anti_product(b, &self.generator_squares);
        if result.coefficient == 0 {
            return vec![];
        }
        vec![result]
    }

    fn represented_dimensions(&self) -> usize {
        self.generator_squares.len() - 1
    }
}

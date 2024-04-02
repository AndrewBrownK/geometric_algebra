use crate::algebra::basis_element::{BasisElement, BasisElementIndex};
use crate::algebra::dialect::Dialect;
use crate::algebra::GeometricAlgebraTrait;

#[cfg(test)]
mod tests;

pub struct ConformalGeometricAlgebra {
    all_retaining_generator_squares: Vec<isize>,
    surface_generator_squares: Vec<isize>,
    origin: usize,
    infinity: usize,
    pub name: &'static str,
    pub dialect: Dialect,
}

impl ConformalGeometricAlgebra {
    pub fn new(name: &'static str, dimensions: usize, dialect: Dialect) -> Self {
        let all_retaining_generator_squares = vec![1; dimensions + 2];
        let mut surface_generator_squares = vec![1; dimensions];
        surface_generator_squares.push(0);
        surface_generator_squares.push(0);

        let origin = surface_generator_squares.len() - 2;
        let infinity = surface_generator_squares.len() - 1;
        ConformalGeometricAlgebra {
            all_retaining_generator_squares,
            surface_generator_squares,
            origin,
            infinity,
            name,
            dialect,
        }
    }

    pub fn sorted_basis(&self) -> Vec<BasisElement> {
        let mut basis_elements = self.basis();
        basis_elements.sort();
        basis_elements
    }
}

impl GeometricAlgebraTrait for ConformalGeometricAlgebra {
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
            assert!((generator_index as usize) < self.surface_generator_squares.len());
            result = result.primitive_product(&BasisElement::from_index(1 << generator_index), &self.surface_generator_squares);
        }
        result
    }

    fn represented_dimensions(&self) -> usize {
        self.surface_generator_squares.len() - 2
    }

    fn basis_size(&self) -> usize {
        1 << self.surface_generator_squares.len()
    }

    fn basis(&self) -> Vec<BasisElement> {
        let mut v = vec![];
        for index in 0..self.basis_size() as BasisElementIndex {
            let mut element = BasisElement::from_index(index);
            let dual = self.dual(&element);
            if dual.cmp(&element) == std::cmp::Ordering::Less {
                element.coefficient = self.dual(&element).coefficient;
            }
            v.push(element);
        }
        v
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

    fn dual(&self, a: &BasisElement) -> BasisElement {
        //
        // let origin = (1 as BasisElementIndex) << self.origin;
        // let infinity = (1 as BasisElementIndex) << self.infinity;
        // // let projective = origin | infinity;
        // let anti_scalar = self.anti_scalar_element().index;












        // TODO here
        // TODO yeah here next

        // TODO between the RGA poster and the page on CGA Duals...
        //  https://projectivegeometricalgebra.org/projgeomalg.pdf
        //  https://conformalgeometricalgebra.org/wiki/index.php?title=Duals
        //  It seems the "RightBulkDual" is what we're looking for

        let index = a.index;
        let coefficient = a.coefficient;


        // TODO find the pattern that will generalize to all dimensions instead of
        //  hard coding for cga3d
        // let exceptions: &[BasisElementIndex] = &[];
        // let coefficient = a.coefficient;
        let exceptions: &[BasisElementIndex] = &[
            0b10,
            0b1000,
            0b101,
            0b10001,
            0b1010,
            0b10100,
            0b11000,
            0b1011,
            0b10101,
            0b11001,
            0b1110,
            0b10111,
            0b11011,
            0b11110,
            0b11100,
            0b11111
        ];
        let coefficient = if exceptions.contains(&index) {
            -1 * a.coefficient
        } else {
            a.coefficient
        };

        let mut result = a.clone();
        result.coefficient = coefficient;
        let mut pre_complement = a.clone();


        let origin = (1 as BasisElementIndex) << self.origin;
        let infinity = (1 as BasisElementIndex) << self.infinity;
        // let projective = origin | infinity;
        let anti_scalar = self.anti_scalar_element().index;

        let aligned_infinity = index & infinity == infinity;
        let aligned_origin = index & origin == origin;
        if aligned_infinity && !aligned_origin {
            let new_index = (index + origin) - infinity;
            result.index = new_index;
            pre_complement = result.clone();
            result.index = anti_scalar - new_index;
        }
        if !aligned_infinity && aligned_origin {
            let new_index = (index + infinity) - origin;
            result.index = new_index;
            pre_complement = result.clone();
            result.index = anti_scalar - new_index;
        }
        if aligned_origin == aligned_infinity {
            result.index = anti_scalar - index;
        }
        // result.coefficient *= pre_complement.primitive_product(&result, &self.surface_generator_squares).coefficient;
        // result.coefficient *= a.primitive_product(&self.right_complement(&a), &self.surface_generator_squares).coefficient;
        result
    }

    fn anti_dual(&self, a: &BasisElement) -> BasisElement {
        // TODO between the RGA poster and the page on CGA Duals...
        //  https://projectivegeometricalgebra.org/projgeomalg.pdf
        //  https://conformalgeometricalgebra.org/wiki/index.php?title=Duals
        //  It seems the "RightWeightDual" is what we're looking for

        // TODO what is anti_dual in conformal?
        //  It is the right complement of the anti-metric
        //  Not the left complement of whatever
        //  You can also just do right complement first and normal-metric second, if you
        //  don't want to define the anti-metric.
        return self.dual(a)
    }

    fn product(&self, a: &BasisElement, b: &BasisElement) -> Vec<BasisElement> {
        // We need to reproduce this Cayley table.
        // https://conformalgeometricalgebra.org/wiki/index.php?title=Geometric_products

        let mut result = vec![];

        let trivial_product = a.primitive_product(b, &self.surface_generator_squares);
        if trivial_product.coefficient != 0 {
            result.push(trivial_product.clone());
        }

        let origin = (1 as BasisElementIndex) << self.origin;
        let infinity = (1 as BasisElementIndex) << self.infinity;
        let projective = origin | infinity;
        let anti_scalar = self.anti_scalar_element().index;
        let non_projective = anti_scalar - projective;

        let projective_basis = BasisElement::from_index(projective);

        let a_is_projective = (a.index & projective) == projective;
        let b_is_projective = (b.index & projective) == projective;
        if a_is_projective && b_is_projective {
            // primitive product should be zero
            assert!(result.is_empty());
            // Then we add the non-trivial part of this product (the only part of this product)
            let mut a = a.clone();
            let mut b = b.clone();
            a.index = a.index & non_projective;
            b.index = b.index & non_projective;
            let mut result = a.primitive_product(&b, &self.surface_generator_squares);
            assert_eq!(result.index & projective, 0);
            return vec![result];
        }

        let a_is_along_origin = ((a.index & origin) == origin) && !a_is_projective;
        let a_is_along_infinity = ((a.index & infinity) == infinity) && !a_is_projective;
        let a_is_half_projective = a_is_along_origin || a_is_along_infinity;

        let b_is_along_origin = ((b.index & origin) == origin) && !b_is_projective;
        let b_is_along_infinity = ((b.index & infinity) == infinity) && !b_is_projective;
        let b_is_half_projective = b_is_along_origin || b_is_along_infinity;

        if (a_is_along_origin && b_is_along_infinity) || (a_is_along_infinity && b_is_along_origin) {
            // There will be two components to the result
            // So we should already have one of the results
            assert_eq!(result.len(), 1);
            // Note the use of different generator squares here
            let mut second_product_component = result[0].primitive_product(&projective_basis, &self.all_retaining_generator_squares);
            // TODO not yet sure if this coefficient logic correctly generalizes to higher dimensions
            if a_is_along_infinity && b_is_along_origin {
                second_product_component.coefficient = second_product_component.coefficient * -1;
            }
            result.push(second_product_component);
            return result;
        }

        if (a_is_half_projective && b_is_projective) || (a_is_projective && b_is_half_projective) {
            // primitive product should be zero
            assert!(result.is_empty());
            // Then we add the non-trivial part of this product (the only part of this product)
            let mut a = a.clone();
            let mut b = b.clone();
            if a_is_projective {
                a.index = a.index & non_projective;
            }
            if b_is_projective {
                b.index = b.index & non_projective;
            }
            let mut result = a.primitive_product(&b, &self.surface_generator_squares);
            // TODO not yet sure if this coefficient logic correctly generalizes to higher dimensions
            if a_is_along_infinity && b_is_projective || a_is_projective && b_is_along_origin {
                result.coefficient = result.coefficient * -1;
            }
            return vec![result];
        }

        return result;
    }


    fn anti_product(&self, a: &BasisElement, b: &BasisElement) -> Vec<BasisElement> {
        // We need to reproduce this Cayley table.
        // https://conformalgeometricalgebra.org/wiki/index.php?title=Geometric_products

        let a = self.right_complement(a);
        let b = self.right_complement(b);
        let mut results = self.product(&a, &b);
        for mut r in results.iter_mut() {
            *r = self.left_complement(&r);
        }
        return results;
    }
}

//

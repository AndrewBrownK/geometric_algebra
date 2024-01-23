use basis_element::{BasisElement, BasisElementIndex};
use crate::algebra::dialect::Dialect;
use crate::ast::{DataType, Parameter};

pub mod basis_element;
pub mod rigid;
pub mod conformal;
pub mod dialect;

pub trait GeometricAlgebraTrait {
    fn algebra_name(&self) -> &'static str;
    fn dialect(&self) -> &Dialect;
    fn parse(&self, name: &str) -> BasisElement;

    fn basis_size(&self) -> usize;
    fn basis(&self) -> Vec<BasisElement>;

    fn scalar_element(&self) -> BasisElement;
    fn anti_scalar_element(&self) -> BasisElement;
    fn right_complement(&self, a: &BasisElement) -> BasisElement;
    fn left_complement(&self, a: &BasisElement) -> BasisElement;
    fn product(&self, a: &BasisElement, b: &BasisElement) -> BasisElement;
}

#[derive(Clone)]
pub struct Involution {
    pub terms: Vec<(BasisElement, BasisElement)>,
}

impl Involution {
    pub fn  identity<GA: GeometricAlgebraTrait>(algebra: &GA) -> Self {
        Self {
            terms: algebra.basis().into_iter().map(|element| (element.clone(), element)).collect(),
        }
    }

    pub fn projection(class: &MultiVectorClass) -> Self {
        Self {
            terms: class.flat_basis().iter().map(|element| (element.clone(), element.clone())).collect(),
        }
    }

    pub fn negated<F>(&self, grade_negation: F) -> Self
    where
        F: Fn(usize) -> bool,
    {
        Self {
            terms: self
                .terms
                .iter()
                .map(|(key, value)| {
                    let mut element = value.clone();
                    element.set_coefficient(if grade_negation(value.grade()) { -1 } else { 1 });
                    (key.clone(), element)
                })
                .collect(),
        }
    }

    pub fn right_complement<GA: GeometricAlgebraTrait>(&self, algebra: &GA) -> Self {
        Self {
            terms: self.terms.iter().map(|(key, value)| (key.clone(), algebra.right_complement(value))).collect(),
        }
    }

    pub fn left_complement<GA: GeometricAlgebraTrait>(&self, algebra: &GA) -> Self {
        Self {
            terms: self.terms.iter().map(|(key, value)| (key.clone(), algebra.left_complement(value))).collect(),
        }
    }

    pub fn double_complement<GA: GeometricAlgebraTrait>(&self, algebra: &GA) -> Self {
        Self {
            terms: self.terms.iter().map(|(key, value)| {
                (key.clone(), algebra.right_complement(&algebra.right_complement(value)))
            }).collect(),
        }
    }

    pub fn involutions<GA: GeometricAlgebraTrait>(algebra: &GA) -> Vec<(&'static str, Self)> {
        let involution = Self::identity(algebra);
        let dimensions = algebra.basis_size();
        vec![
            ("Neg", involution.negated(|_grade| true)),
            ("Automorphism", involution.negated(|grade| grade % 2 == 1)),
            ("Reversal", involution.negated(|grade| grade % 4 >= 2)),
            ("Conjugation", involution.negated(|grade| (grade + 3) % 4 < 2)),
            ("Dual", involution.right_complement(algebra)),
            // Confirmed accurate: epga3d MultiVector
            ("AntiReversal", involution.negated(|grade| {
                let anti_grade = dimensions - grade;
                anti_grade % 4 >= 2
            })),
            ("RightComplement", involution.right_complement(algebra)),
            ("LeftComplement", involution.left_complement(algebra)),
            ("DoubleComplement", involution.double_complement(algebra)),
        ]
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ProductTerm {
    pub product: BasisElement,
    pub factor_a: BasisElement,
    pub factor_b: BasisElement,
}

#[derive(Clone)]
pub struct Product {
    pub terms: Vec<ProductTerm>,
}

impl Product {
    pub fn new<GA: GeometricAlgebraTrait>(a: &[BasisElement], b: &[BasisElement], algebra: &GA) -> Self {
        Self {
            terms: a
                .iter()
                .flat_map(|a| {
                    b.iter().map(move |b| ProductTerm {
                        product: algebra.product(a, b),
                        factor_a: a.clone(),
                        factor_b: b.clone(),
                    })
                })
                .filter(|term| term.product.get_coefficient() != 0)
                .collect(),
        }
    }

    pub fn projected<F>(&self, grade_projection: F) -> Self
    where
        F: Fn(usize, usize, usize) -> bool,
    {
        Self {
            terms: self
                .terms
                .iter()
                .filter(|term| grade_projection(term.factor_a.grade(), term.factor_b.grade(), term.product.grade()))
                .cloned()
                .collect(),
        }
    }

    pub fn dual<GA: GeometricAlgebraTrait>(&self, algebra: &GA) -> Self {
        Self {
            terms: self
                .terms
                .iter()
                .map(|term| {
                    ProductTerm {
                        product: algebra.right_complement(&term.product),
                        factor_a: algebra.right_complement(&term.factor_a),
                        factor_b: algebra.right_complement(&term.factor_b),
                    }
                })
                .collect(),
        }
    }

    pub fn products<GA: GeometricAlgebraTrait>(algebra: &GA) -> Vec<(&'static str, Self)> {
        let basis = algebra.basis();
        let product = Self::new(&basis, &basis, algebra);

        let wedge_like: fn(usize, usize, usize) -> bool = |factor_a_grade, factor_b_grade, product_grade| {
            product_grade == factor_a_grade + factor_b_grade
        };
        let scalar_result_only: fn(usize, usize, usize) -> bool = |_, _, product_grade| {
            product_grade == 0
        };
        let product_and_a_is_b: fn(usize, usize, usize) -> bool = |factor_a_grade, factor_b_grade, product_grade| {
            product_grade + factor_a_grade == factor_b_grade
        };
        let product_and_b_is_a: fn(usize, usize, usize) -> bool = |factor_a_grade, factor_b_grade, product_grade| {
            product_grade + factor_b_grade == factor_a_grade
        };

        let dialect = algebra.dialect();
        let mut products = vec![];


        // https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_products

        for name in &dialect.geometric_product {
            products.push((*name, product.clone()))
        }
        for name in &dialect.geometric_anti_product {
            products.push((*name, product.clone().dual(algebra)))
        }


        // https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products

        for name in &dialect.exterior_product {
            products.push((*name, product.projected(wedge_like)))
        }
        for name in &dialect.exterior_anti_product {
            products.push((*name, product.projected(wedge_like).dual(algebra)))
        }


        // https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products

        // TODO check that the correct predicates are used for the anti_products here, since left and right
        //  kind of switch places on the anti_product. See Cayley tables.

        for name in &dialect.left_interior_product {
            products.push((*name, product.projected(product_and_a_is_b)))
        }
        for name in &dialect.left_interior_anti_product {
            products.push((*name, product.projected(product_and_a_is_b).dual(algebra)))
        }
        for name in &dialect.right_interior_product {
            products.push((*name, product.projected(product_and_b_is_a)))
        }
        for name in &dialect.right_interior_anti_product {
            products.push((*name, product.projected(product_and_b_is_a).dual(algebra)))
        }


        // https://rigidgeometricalgebra.org/wiki/index.php?title=Dot_products

        for name in &dialect.dot_product {
            products.push((*name, product.projected(scalar_result_only)))
        }
        for name in &dialect.anti_dot_product {
            products.push((*name, product.projected(scalar_result_only).dual(algebra)))
        }

        products
    }
}

#[derive(Default)]
pub struct MultiVectorClassRegistry {
    pub classes: Vec<MultiVectorClass>,
    index_by_signature: std::collections::HashMap<Vec<BasisElementIndex>, usize>,
}

impl MultiVectorClassRegistry {
    pub fn single_parameters<'r>(&'r self) -> impl Iterator<Item=Parameter<'r>> {
        self.classes.iter().map(|class_a| {
            Parameter {
                name: "self",
                data_type: DataType::MultiVector(class_a),
            }
        })
    }
    pub fn pair_parameters(&self) -> impl Iterator<Item=(Parameter, Parameter)> {
        self.classes.iter().map(|class_a| {
            let param_a = Parameter {
                name: "self",
                data_type: DataType::MultiVector(class_a),
            };
            self.classes.iter().map(move |class_b| {
                let param_b = Parameter {
                    name: "other",
                    data_type: DataType::MultiVector(class_b),
                };
                (param_a.clone(), param_b)
            })
        }).flatten()
    }
}

impl MultiVectorClassRegistry {
    pub fn register(&mut self, class: MultiVectorClass) {
        self.index_by_signature.insert(class.signature(), self.classes.len());
        self.classes.push(class);
    }

    pub fn get(&self, signature: &[BasisElementIndex]) -> Option<&MultiVectorClass> {
        self.index_by_signature.get(signature).map(|index| &self.classes[*index])
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct MultiVectorClass {
    pub class_name: String,
    pub grouped_basis: Vec<Vec<BasisElement>>,
}

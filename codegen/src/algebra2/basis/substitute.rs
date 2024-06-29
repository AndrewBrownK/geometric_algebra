use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Mul, MulAssign};

use im::HashMap;

use crate::algebra2::basis::{BasisElement, BasisSignature};
use crate::algebra2::basis::generators::{GeneratorElement, GeneratorSquares};

#[derive(Clone, PartialEq)]
struct Sum {
    sum: Vec<Product>
}

#[derive(Clone, PartialEq)]
struct Product {
    coefficient: f32,
    element: BasisElement
}

impl Mul<f32> for &Product {
    type Output = Product;

    fn mul(self, rhs: f32) -> Self::Output {
        Product {
            coefficient: self.coefficient * rhs,
            element: self.element,
        }
    }
}

impl MulAssign<f32> for Product {
    fn mul_assign(&mut self, rhs: f32) {
        self.coefficient *= rhs;
    }
}

impl Mul<f32> for &Sum {
    type Output = Sum;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = self.clone();
        for term in result.sum.iter_mut() {
            *term = term.mul(rhs);
        }
        result
    }
}

impl MulAssign<f32> for Sum {
    fn mul_assign(&mut self, rhs: f32) {
        for term in self.sum.iter_mut() {
            term.mul_assign(rhs);
        }
    }
}

impl Debug for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.coefficient == 1.0 {
            return Display::fmt(&self.element, f);
        }
        if self.coefficient == -1.0 {
            write!(f, "-")?;
            return Display::fmt(&self.element, f);
        }
        if self.coefficient == 0.0 {
            write!(f, "0*")?;
            return Display::fmt(&self.element, f);
        }
        write!(f, "{}*{}", self.coefficient, self.element)
    }
}

impl Debug for Sum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.sum.is_empty() {
            return write!(f, "0");
        }
        for (i, p) in self.sum.iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            Debug::fmt(p, f)?;
        }
        Ok(())
    }
}


#[derive(Debug, Clone)]
struct SubstituteElement {
    new_element: BasisElement,
    depends_on_underlying: Vec<BasisElement>,
    expr: Sum,
}

impl SubstituteElement {
    pub fn new() -> Self {
        todo!()
    }
}


impl Product {
    fn multiply(&self, other: &Product, underlying: &GeneratorSquares) -> Option<Product> {
        let mut element = underlying.true_product(self.element, other.element);
        if element.coefficient == 0 {
            return None;
        }
        let coefficient = self.coefficient * other.coefficient * element.coefficient as f32;
        element.coefficient = 1;
        Some(Product { coefficient, element })
    }

    fn anti_multiply(&self, other: &Product, underlying: &GeneratorSquares) -> Option<Product> {
        // use crate::algebra2::basis::elements::*;
        // let e123AB = e123.wedge(eA).wedge(eB);
        // if self.element == e123AB && other.element == e123AB {
        //     eprintln!("Attempting anti-product identity");
        // }

        let mut element = underlying.true_anti_product(self.element, other.element);
        if element.coefficient == 0 {
            return None;
        }
        let coefficient = self.coefficient * other.coefficient * element.coefficient as f32;
        element.coefficient = 1;
        Some(Product { coefficient, element })
    }

    fn add(&self, other: &Product) -> Sum {
        let mut s = Sum { sum: vec![self.clone(), other.clone()] };
        s.sort_and_simplify();
        s
    }
}

impl Sum {

    fn sort_and_simplify(&mut self) {
        // println!("Sum::sort_and_simplify {self:?}");
        self.sum.sort_by(|a, b| {
            a.element.cmp(&b.element)
        });
        // println!("Sum::sort_and_simplify -> {self:?}");
        for p in self.sum.iter_mut() {
            if p.element.coefficient != 1 {
                p.coefficient *= p.element.coefficient as f32;
                p.element.coefficient = 1;
            }
        }
        let mut i = 0;
        while !self.sum.is_empty() && i < self.sum.len() - 1 {
            let (a, b) = self.sum.split_at_mut(i + 1);
            let b_len = b.len();
            let a = &mut a[i];
            let mut total = a.coefficient;
            let mut j = 0;
            while j < b_len {
                let b = &mut b[j];
                if b.element != a.element {
                    break;
                }
                total += b.coefficient;
                j += 1;
            }
            if total == 0.0 {
                self.sum.drain(i..(i+j+1));
            } else {
                a.coefficient = total;
                self.sum.drain(i + 1..(i+j+1));
                i += 1;
            }
        }
        // println!("Sum::sort_and_simplify -> {self:?}");
    }

    fn multiply(&self, other: &Sum, underlying: &GeneratorSquares) -> Sum {
        // println!("Sum::multiply {self:?} and {other:?}");
        let mut sum = vec![];
        for a in self.sum.iter() {
            for b in other.sum.iter() {
                if let Some(product) = a.multiply(b, underlying) {
                    sum.push(product);
                }
            }
        }
        let mut s = Sum { sum };
        // println!("Sum::multiply -> {s:?}");
        s.sort_and_simplify();
        // println!("Sum::multiply -> {s:?}");
        s
    }

    fn anti_multiply(&self, other: &Sum, underlying: &GeneratorSquares) -> Sum {
        // println!("Sum::multiply {self:?} and {other:?}");
        let mut sum = vec![];
        for a in self.sum.iter() {
            for b in other.sum.iter() {
                if let Some(product) = a.anti_multiply(b, underlying) {
                    sum.push(product);
                }
            }
        }
        let mut s = Sum { sum };
        // println!("Sum::multiply -> {s:?}");
        s.sort_and_simplify();
        // println!("Sum::multiply -> {s:?}");
        s
    }


    fn wedge(&self, other: &Sum) -> Sum {
        let a = self;
        let b = other;
        let mut sum = vec![];
        for a in &a.sum {
            for b in &b.sum {
                let mut c = a.element.wedge(b.element);
                if c.coefficient != 0 {
                    let coefficient = (c.coefficient() as f32) * a.coefficient * b.coefficient;
                    c.coefficient = 1;
                    sum.push(Product { coefficient, element: c, });
                }
            }
        }
        let mut s = Sum { sum };
        s.sort_and_simplify();
        s
    }

    /// Dot product of superficial basis as vectors on the underlying basis.
    /// If that makes sense...
    /// In other words... Suppose e4 = 0.5*e- - 0.5*e+ and e5 = e+ + e-
    /// So we have two ways we can try to dot product that.
    /// We can do dot(e4, e5) = 0.5 * 1 + -0.5*1 = 0
    /// Or we can (per page 119) take dot(e4, e5) = 0.5(a⟑b + b⟑a) = -1
    /// I'm not entirely comfortable with the fact each of those gives us a different result.
    /// But in any case, it is the former that we implement here, and that can be used
    /// as an orthogonality test. I'm pretty sure the latter cannot be used as an orthogonality
    /// test.
    fn superficial_dot_product(self, other: Sum) -> f32 {
        let mut a = self;
        let mut b = other;
        a.sort_and_simplify();
        b.sort_and_simplify();
        let a: HashMap<_, _> = a.sum.into_iter().map(|it| {
            (it.element.signature, it.coefficient * it.element.coefficient() as f32)
        }).collect();
        let b: HashMap<_, _> = b.sum.into_iter().map(|it| {
            (it.element.signature, it.coefficient * it.element.coefficient() as f32)
        }).collect();
        let c = a.intersection_with(b, |a, b| {
            a * b
        });
        c.iter().map(|it| it.1).sum()
    }

    fn add(&self, other: &Sum) -> Sum {
        let mut sum = self.sum.clone();
        sum.append(&mut other.sum.clone());
        let mut s = Sum { sum };
        s.sort_and_simplify();
        s
    }
}


#[derive(Debug)]
pub struct SubstitutionRepository {
    underlying_squares: GeneratorSquares,
    substitutions_to_underlying: HashMap<BasisElement, Sum>,
    underlying_to_substitutions: HashMap<BasisElement, Sum>,
    substitution_products: HashMap<(BasisElement, BasisElement), Sum>,
    substitution_anti_products: HashMap<(BasisElement, BasisElement), Sum>,
}

impl SubstitutionRepository {
    pub fn new(
        underlying_squares: GeneratorSquares,
        substituted_elements: Vec<(GeneratorElement, Vec<(f32, GeneratorElement)>)>
    ) -> Self {
        let mut substitutions_to_underlying = HashMap::new();
        let mut underlying_to_substitutions = HashMap::new();


        // Get grade 1 elements for underlying basis
        let mut underlying_grade_1_elements: Vec<_> = underlying_squares
            .anti_scalar().signature()
            .into_generator_elements().1.into_iter()
            .filter_map(|it| it)
            .map(|it| it.element()).collect();


        // Get grade 1 elements for substitute basis
        let mut substitute_grade_1_elements = underlying_grade_1_elements.clone();
        for (_, sum) in substituted_elements.iter() {
            for (_, und_el) in sum {
                let und_el = und_el.element();
                substitute_grade_1_elements.retain(|it| it != &und_el)
            }
        }
        for (sub_el, _) in substituted_elements.iter() {
            let sub_el = sub_el.element();
            if !substitute_grade_1_elements.contains(&sub_el) {
                substitute_grade_1_elements.push(sub_el);
            }
        }


        // Sanity check underlying and substitution bases have same dimensionality
        assert_eq!(underlying_grade_1_elements.len(), substitute_grade_1_elements.len(), "The substitute and underlying generator elements must have the same dimensionality");


        // Translate solutions for substitute basis into solutions for underlying basis
        // This allows us to go back and forth instead of only one way
        for (sub, under) in substituted_elements {
            let sub = sub.element();
            for (c, u) in under.iter() {
                // eprintln!("sub: {sub}   c: {c}   u: {u:?}");

                // So with Lengyel CGA we will pass through this block 4 times
                // sub: E4   c:  0.5   u: EB
                // sub: E4   c: -0.5   u: EA
                // sub: E5   c:  1.0   u: EB
                // sub: E5   c:  1.0   u: EA

                if *c == 0.0 {
                    panic!("Don't define substitution elements on underlying elements using a coefficient of zero: {sub:?}")
                }
                let u = u.element();
                let s = underlying_to_substitutions.entry(u)
                    .and_modify(|sum: &mut Sum| {
                        sum.sum.push(Product { coefficient: 1.0 / c, element: sub });
                        sum.sort_and_simplify();
                    })
                    .or_insert(Sum { sum: vec![Product { coefficient: 1.0 / *c, element: sub }] });

                // eprintln!("WIP Sum: {u} := {s:?}");

                // And then after the above modifications/insertions, we get something like...
                // EB :=  2.0*E4 + 1.0*E5
                // EA := -2.0*E4 + 1.0*E5

                assert!(!s.sum.is_empty(), "problem deriving underlying_to_substitutions")
            }
            let sum = under.into_iter().map(|(c, it)| Product { coefficient: c, element: it.element()}).collect();
            let mut sum = Sum { sum };
            sum.sort_and_simplify();

            let existing = substitutions_to_underlying.insert(sub, sum.clone());
            assert!(existing.is_none(), "Basis substitutions must be uniquely defined, but found more than one definition for {sub}");

            let sub = sub.negate();
            sum.mul_assign(-1.0);
            let existing = substitutions_to_underlying.insert(sub, sum);
            assert!(existing.is_none(), "Basis substitutions must be uniquely defined, but found more than one definition for {sub}");
        }


        // Verify that the solutions for underlying basis work correctly and also have proper scale
        for (under, orig_substitutions) in underlying_to_substitutions.iter_mut() {
            let subs_back_to_underlying: Vec<_> = orig_substitutions.clone().sum.into_iter()
                .flat_map(|it| {
                    let c = it.coefficient;
                    let mut und = substitutions_to_underlying.get(&it.element).expect("substitutions are defined here").clone();
                    for it in und.sum.iter_mut() {
                        it.coefficient *= c;
                    }
                    und.sort_and_simplify();
                    und.sum
                })
                .collect();
            let mut back_to_underlying = Sum { sum: subs_back_to_underlying };
            back_to_underlying.sort_and_simplify();
            assert_eq!(back_to_underlying.sum.len(), 1, "Substitution elements ({under} := {orig_substitutions:?}) do not resolve into an independent underlying {under}.");

            // This is a good place to fix proportionality since we haven't done that yet
            let c = back_to_underlying.sum[0].coefficient;
            back_to_underlying.sum[0].mul_assign(1.0 / c);
            orig_substitutions.mul_assign(1.0 / c);

            assert_eq!(back_to_underlying.sum[0], Product { coefficient: 1.0, element: *under }, "Substitution elements do not resolve into underlying {under} properly.");
        }


        // Verify that substitution elements are orthogonal
        let mut substitutions_to_underlying_vec: Vec<_> = substitutions_to_underlying.clone().into_iter().collect();
        for i in 0..(substitutions_to_underlying.len() - 1) {
            for j in (i + 1)..substitutions_to_underlying_vec.len() {
                let (a, a_underlying) = &substitutions_to_underlying_vec[i];
                let (b, b_underlying) = &substitutions_to_underlying_vec[j];
                if a == &b.negate() {
                    continue;
                }
                let dot = a_underlying.clone().superficial_dot_product(b_underlying.clone());
                assert_eq!(dot, 0.0, "Basis substitutions must be orthogonal, violated by {a} and {b}")
            }
        }


        // Start constructing higher grade elements
        let mut substitution_grade_n_elements = substitute_grade_1_elements.clone();
        for n in 1..substitute_grade_1_elements.len() {
            let mut grade_n_plus_1_elements = vec![];
            for a in substitute_grade_1_elements.iter() {
                for b in substitution_grade_n_elements.iter() {
                    let a = a.clone();
                    let b = b.clone();
                    let substitution_wedge = a.wedge(b);
                    if substitution_wedge.coefficient == 0 {
                        continue
                    }
                    let a_ = substitutions_to_underlying.get(&a).map(|it| it.clone())
                        .unwrap_or_else(|| Sum { sum: vec![Product { coefficient: 1.0, element: a }] });
                    let b_ = substitutions_to_underlying.get(&b).map(|it| it.clone())
                        .unwrap_or_else(|| Sum { sum: vec![Product { coefficient: 1.0, element: b }] });

                    let underlying_product = a_.multiply(&b_, &underlying_squares);
                    let underlying_wedge: Vec<_> = underlying_product.sum.iter().filter_map(|it|
                        if it.element.grade() == substitution_wedge.grade() { Some(it.clone()) } else { None }
                    ).collect();
                    if !underlying_wedge.is_empty() {
                        for term in underlying_wedge.iter() {
                            let thing = Sum { sum: vec![Product { coefficient: 1.0 / term.coefficient, element: substitution_wedge }]};
                            let el = term.element;
                            // eprintln!("Adding {thing:?} to {el}");
                            let s = underlying_to_substitutions.entry(el)
                                .and_modify(|it| { *it = it.add(&thing); })
                                .or_insert(thing);
                            // eprintln!("{el} is now looking like: {s:?}");
                        }
                        substitutions_to_underlying.insert(substitution_wedge, Sum { sum: underlying_wedge });
                        grade_n_plus_1_elements.push(substitution_wedge);
                    }
                }
            }

            for (under, orig_substitutions) in underlying_to_substitutions.iter_mut() {
                if under.grade() != (n + 1) as u8 {
                    continue
                }
                // Since the accumulated components of each sum is acquired now, fix the scale
                let subs_back_to_underlying: Vec<_> = orig_substitutions.clone().sum.into_iter()
                    .flat_map(|it| {
                        let c = it.coefficient;
                        let mut und = substitutions_to_underlying.get(&it.element).expect("substitutions are defined here").clone();
                        for it in und.sum.iter_mut() {
                            it.coefficient *= c;
                        }
                        und.sort_and_simplify();
                        und.sum
                    })
                    .collect();
                let mut back_to_underlying = Sum { sum: subs_back_to_underlying };
                back_to_underlying.sort_and_simplify();
                assert_eq!(back_to_underlying.sum.len(), 1, "Substitution elements ({under} := {orig_substitutions:?}) do not resolve into an independent underlying {under}.");

                let c = back_to_underlying.sum[0].coefficient;
                back_to_underlying.sum[0].mul_assign(1.0 / c);
                orig_substitutions.mul_assign(1.0 / c);
                assert_eq!(back_to_underlying.sum[0], Product { coefficient: 1.0, element: *under }, "Substitution elements do not resolve into underlying {under} properly.");
            }

            substitution_grade_n_elements = grade_n_plus_1_elements;
        }

        // Clear out some redundant entries
        substitutions_to_underlying.retain(|k, v| {
            v.sum.len() > 1
                || (v.sum[0].element.coefficient as f32 * v.sum[0].coefficient) != (k.coefficient as f32)
                || v.sum[0].element.signature != k.signature
        });
        underlying_to_substitutions.retain(|k, v| {
            v.sum.len() > 1
                || (v.sum[0].element.coefficient as f32 * v.sum[0].coefficient) != (k.coefficient as f32)
                || v.sum[0].element.signature != k.signature
        });

        // Also add negated entries in this direction too
        for (under, orig_substitutions) in underlying_to_substitutions.clone().iter() {
            underlying_to_substitutions.insert(under.negate(), orig_substitutions.mul(-1.0));
        }


        let mut substitution_products = HashMap::new();
        let mut substitution_anti_products = HashMap::new();
        let s = Self {
            underlying_squares,
            substitutions_to_underlying,
            underlying_to_substitutions,
            substitution_products,
            substitution_anti_products,
        };
        // eprintln!("SubstitutionRepository: {s:?}");
        s
    }

    pub fn product(&mut self, a: &BasisElement, b: &BasisElement) -> Sum {
        // eprintln!("Attempting {a} * {b}");
        self.substitution_products.entry((*a, *b))
            .or_insert_with(|| {
                // TODO should I really be using any unwrap_or here? or should I fill out
                //  substitutions_to_underlying and underlying_to_substitutions all the way?
                let a_ = self.substitutions_to_underlying.get(&a).cloned()
                    .unwrap_or(Sum { sum: vec![Product { coefficient: 1.0, element: *a }] });
                let b_ = self.substitutions_to_underlying.get(&b).cloned()
                    .unwrap_or(Sum { sum: vec![Product { coefficient: 1.0, element: *b }] });
                // eprintln!("    Underlying factors: {a_:?}, {b_:?}");
                let mut result = Sum { sum: vec![] };
                let underlying_product = a_.multiply(&b_, &self.underlying_squares);
                // eprintln!("    Underlying product: {underlying_product:?}");
                for underlying_term in underlying_product.sum.into_iter() {
                    let substitution_terms = self.underlying_to_substitutions.get(&underlying_term.element).cloned()
                        .map(| mut it| {
                            it.mul_assign(underlying_term.coefficient);
                            it
                        })
                        .unwrap_or(Sum { sum: vec![underlying_term] });
                    // eprintln!("    Intermediate sum: {substitution_terms:?}");
                    result = result.add(&substitution_terms);
                }
                // eprintln!("    Substitution product: {result:?}");
                result
            }).clone()
    }

    pub fn anti_product(&mut self, a: &BasisElement, b: &BasisElement) -> Sum {
        self.substitution_anti_products.entry((*a, *b))
            .or_insert_with(|| {
                let a_ = self.substitutions_to_underlying.get(&a).cloned()
                    .unwrap_or(Sum { sum: vec![Product { coefficient: 1.0, element: *a }] });
                let b_ = self.substitutions_to_underlying.get(&b).cloned()
                    .unwrap_or(Sum { sum: vec![Product { coefficient: 1.0, element: *b }] });
                let mut result = Sum { sum: vec![] };
                let underlying_product = a_.anti_multiply(&b_, &self.underlying_squares);
                for underlying_term in underlying_product.sum.into_iter() {
                    let substitution_terms = self.underlying_to_substitutions.get(&underlying_term.element).cloned()
                        .map(| mut it| {
                            it.mul_assign(underlying_term.coefficient);
                            it
                        })
                        .unwrap_or(Sum { sum: vec![underlying_term] });
                    result = result.add(&substitution_terms);
                }
                result
            }).clone()
    }
}

#[test]
fn sum_simplifications() {
    use crate::algebra2::basis::elements::*;

    let mut a = Sum { sum: vec![
        Product { coefficient: 1.0, element: e1 },
        Product { coefficient: -0.5, element: e1 },
        Product { coefficient: -0.25, element: e1 },
        Product { coefficient: -0.25, element: e1 },
    ] };
    let b = Sum { sum: vec![] };
    a.sort_and_simplify();
    assert_eq!(a, b);

    let mut a = Sum { sum: vec![
        Product { coefficient: 1.0, element: e3 },
        Product { coefficient: 1.0, element: e1 },
        Product { coefficient: 1.0, element: e2 },
    ] };
    let b = Sum { sum: vec![
        Product { coefficient: 1.0, element: e1 },
        Product { coefficient: 1.0, element: e2 },
        Product { coefficient: 1.0, element: e3 },
    ] };
    a.sort_and_simplify();
    assert_eq!(a, b);

    let mut a = Sum { sum: vec![
        Product { coefficient: 1.0, element: e1 },
        Product { coefficient: 2.0, element: e2 },
        Product { coefficient: -0.5, element: e1 },
        Product { coefficient: 3.0, element: e3 },
        Product { coefficient: -0.5, element: e1 },
        Product { coefficient: -2.0, element: e3 },
    ] };
    let b = Sum { sum: vec![
        Product { coefficient: 2.0, element: e2 },
        Product { coefficient: 1.0, element: e3 },
    ] };
    a.sort_and_simplify();
    assert_eq!(a, b);

    let mut a = Sum { sum: vec![
        Product { coefficient: 1.0, element: e1 },
        Product { coefficient: -1.0, element: e1 },
        Product { coefficient: 1.0, element: e1 },
    ] };
    let b = Sum { sum: vec![
        Product { coefficient: 1.0, element: e1 },
    ] };
    a.sort_and_simplify();
    assert_eq!(a, b);
}

#[test]
fn conformal_3d_geometric_products() {
    // https://conformalgeometricalgebra.org/wiki/index.php?title=Geometric_products
    let correct_cayley_table = {[
        ("1", "1", vec!["1"]),
        ("1", "e1", vec!["e1"]),
        ("1", "e2", vec!["e2"]),
        ("1", "e3", vec!["e3"]),
        ("1", "e4", vec!["e4"]),
        ("1", "e23", vec!["e23"]),
        ("1", "e31", vec!["e31"]),
        ("1", "e12", vec!["e12"]),
        ("1", "e43", vec!["e43"]),
        ("1", "e42", vec!["e42"]),
        ("1", "e41", vec!["e41"]),
        ("1", "e321", vec!["e321"]),
        ("1", "e412", vec!["e412"]),
        ("1", "e431", vec!["e431"]),
        ("1", "e423", vec!["e423"]),
        ("1", "e1234", vec!["e1234"]),
        ("1", "e5", vec!["e5"]),
        ("1", "e15", vec!["e15"]),
        ("1", "e25", vec!["e25"]),
        ("1", "e35", vec!["e35"]),
        ("1", "e45", vec!["e45"]),
        ("1", "e235", vec!["e235"]),
        ("1", "e315", vec!["e315"]),
        ("1", "e125", vec!["e125"]),
        ("1", "e435", vec!["e435"]),
        ("1", "e425", vec!["e425"]),
        ("1", "e415", vec!["e415"]),
        ("1", "e3215", vec!["e3215"]),
        ("1", "e4125", vec!["e4125"]),
        ("1", "e4315", vec!["e4315"]),
        ("1", "e4235", vec!["e4235"]),
        ("1", "e12345", vec!["e12345"]),
        ("e1", "1", vec!["e1"]),
        ("e1", "e1", vec!["1"]),
        ("e1", "e2", vec!["e12"]),
        ("e1", "e3", vec!["-e31"]),
        ("e1", "e4", vec!["-e41"]),
        ("e1", "e23", vec!["-e321"]),
        ("e1", "e31", vec!["-e3"]),
        ("e1", "e12", vec!["e2"]),
        ("e1", "e43", vec!["e431"]),
        ("e1", "e42", vec!["-e412"]),
        ("e1", "e41", vec!["-e4"]),
        ("e1", "e321", vec!["-e23"]),
        ("e1", "e412", vec!["-e42"]),
        ("e1", "e431", vec!["e43"]),
        ("e1", "e423", vec!["e1234"]),
        ("e1", "e1234", vec!["e423"]),
        ("e1", "e5", vec!["e15"]),
        ("e1", "e15", vec!["e5"]),
        ("e1", "e25", vec!["e125"]),
        ("e1", "e35", vec!["-e315"]),
        ("e1", "e45", vec!["-e415"]),
        ("e1", "e235", vec!["-e3215"]),
        ("e1", "e315", vec!["-e35"]),
        ("e1", "e125", vec!["e25"]),
        ("e1", "e435", vec!["e4315"]),
        ("e1", "e425", vec!["-e4125"]),
        ("e1", "e415", vec!["-e45"]),
        ("e1", "e3215", vec!["-e235"]),
        ("e1", "e4125", vec!["-e425"]),
        ("e1", "e4315", vec!["e435"]),
        ("e1", "e4235", vec!["e12345"]),
        ("e1", "e12345", vec!["e4235"]),
        ("e2", "1", vec!["e2"]),
        ("e2", "e1", vec!["-e12"]),
        ("e2", "e2", vec!["1"]),
        ("e2", "e3", vec!["e23"]),
        ("e2", "e4", vec!["-e42"]),
        ("e2", "e23", vec!["e3"]),
        ("e2", "e31", vec!["-e321"]),
        ("e2", "e12", vec!["-e1"]),
        ("e2", "e43", vec!["-e423"]),
        ("e2", "e42", vec!["-e4"]),
        ("e2", "e41", vec!["e412"]),
        ("e2", "e321", vec!["-e31"]),
        ("e2", "e412", vec!["e41"]),
        ("e2", "e431", vec!["e1234"]),
        ("e2", "e423", vec!["-e43"]),
        ("e2", "e1234", vec!["e431"]),
        ("e2", "e5", vec!["e25"]),
        ("e2", "e15", vec!["-e125"]),
        ("e2", "e25", vec!["e5"]),
        ("e2", "e35", vec!["e235"]),
        ("e2", "e45", vec!["-e425"]),
        ("e2", "e235", vec!["e35"]),
        ("e2", "e315", vec!["-e3215"]),
        ("e2", "e125", vec!["-e15"]),
        ("e2", "e435", vec!["-e4235"]),
        ("e2", "e425", vec!["-e45"]),
        ("e2", "e415", vec!["e4125"]),
        ("e2", "e3215", vec!["-e315"]),
        ("e2", "e4125", vec!["e415"]),
        ("e2", "e4315", vec!["e12345"]),
        ("e2", "e4235", vec!["-e435"]),
        ("e2", "e12345", vec!["e4315"]),
        ("e3", "1", vec!["e3"]),
        ("e3", "e1", vec!["e31"]),
        ("e3", "e2", vec!["-e23"]),
        ("e3", "e3", vec!["1"]),
        ("e3", "e4", vec!["-e43"]),
        ("e3", "e23", vec!["-e2"]),
        ("e3", "e31", vec!["e1"]),
        ("e3", "e12", vec!["-e321"]),
        ("e3", "e43", vec!["-e4"]),
        ("e3", "e42", vec!["e423"]),
        ("e3", "e41", vec!["-e431"]),
        ("e3", "e321", vec!["-e12"]),
        ("e3", "e412", vec!["e1234"]),
        ("e3", "e431", vec!["-e41"]),
        ("e3", "e423", vec!["e42"]),
        ("e3", "e1234", vec!["e412"]),
        ("e3", "e5", vec!["e35"]),
        ("e3", "e15", vec!["e315"]),
        ("e3", "e25", vec!["-e235"]),
        ("e3", "e35", vec!["e5"]),
        ("e3", "e45", vec!["-e435"]),
        ("e3", "e235", vec!["-e25"]),
        ("e3", "e315", vec!["e15"]),
        ("e3", "e125", vec!["-e3215"]),
        ("e3", "e435", vec!["-e45"]),
        ("e3", "e425", vec!["e4235"]),
        ("e3", "e415", vec!["-e4315"]),
        ("e3", "e3215", vec!["-e125"]),
        ("e3", "e4125", vec!["e12345"]),
        ("e3", "e4315", vec!["-e415"]),
        ("e3", "e4235", vec!["e425"]),
        ("e3", "e12345", vec!["e4125"]),
        ("e4", "1", vec!["e4"]),
        ("e4", "e1", vec!["e41"]),
        ("e4", "e2", vec!["e42"]),
        ("e4", "e3", vec!["e43"]),
        ("e4", "e4", vec![]),
        ("e4", "e23", vec!["e423"]),
        ("e4", "e31", vec!["e431"]),
        ("e4", "e12", vec!["e412"]),
        ("e4", "e43", vec![]),
        ("e4", "e42", vec![]),
        ("e4", "e41", vec![]),
        ("e4", "e321", vec!["e1234"]),
        ("e4", "e412", vec![]),
        ("e4", "e431", vec![]),
        ("e4", "e423", vec![]),
        ("e4", "e1234", vec![]),
        ("e4", "e5", vec!["e45", "-1"]),
        ("e4", "e15", vec!["e1", "e415"]),
        ("e4", "e25", vec!["e2", "e425"]),
        ("e4", "e35", vec!["e3", "e435"]),
        ("e4", "e45", vec!["e4"]),
        ("e4", "e235", vec!["e4235", "-e23"]),
        ("e4", "e315", vec!["e4315", "-e31"]),
        ("e4", "e125", vec!["e4125", "-e12"]),
        ("e4", "e435", vec!["-e43"]),
        ("e4", "e425", vec!["-e42"]),
        ("e4", "e415", vec!["-e41"]),
        ("e4", "e3215", vec!["e321", "e12345"]),
        ("e4", "e4125", vec!["e412"]),
        ("e4", "e4315", vec!["e431"]),
        ("e4", "e4235", vec!["e423"]),
        ("e4", "e12345", vec!["-e1234"]),
        ("e23", "1", vec!["e23"]),
        ("e23", "e1", vec!["-e321"]),
        ("e23", "e2", vec!["-e3"]),
        ("e23", "e3", vec!["e2"]),
        ("e23", "e4", vec!["e423"]),
        ("e23", "e23", vec!["-1"]),
        ("e23", "e31", vec!["-e12"]),
        ("e23", "e12", vec!["e31"]),
        ("e23", "e43", vec!["e42"]),
        ("e23", "e42", vec!["-e43"]),
        ("e23", "e41", vec!["-e1234"]),
        ("e23", "e321", vec!["e1"]),
        ("e23", "e412", vec!["e431"]),
        ("e23", "e431", vec!["-e412"]),
        ("e23", "e423", vec!["-e4"]),
        ("e23", "e1234", vec!["e41"]),
        ("e23", "e5", vec!["e235"]),
        ("e23", "e15", vec!["-e3215"]),
        ("e23", "e25", vec!["-e35"]),
        ("e23", "e35", vec!["e25"]),
        ("e23", "e45", vec!["e4235"]),
        ("e23", "e235", vec!["-e5"]),
        ("e23", "e315", vec!["-e125"]),
        ("e23", "e125", vec!["e315"]),
        ("e23", "e435", vec!["e425"]),
        ("e23", "e425", vec!["-e435"]),
        ("e23", "e415", vec!["-e12345"]),
        ("e23", "e3215", vec!["e15"]),
        ("e23", "e4125", vec!["e4315"]),
        ("e23", "e4315", vec!["-e4125"]),
        ("e23", "e4235", vec!["-e45"]),
        ("e23", "e12345", vec!["e415"]),
        ("e31", "1", vec!["e31"]),
        ("e31", "e1", vec!["e3"]),
        ("e31", "e2", vec!["-e321"]),
        ("e31", "e3", vec!["-e1"]),
        ("e31", "e4", vec!["e431"]),
        ("e31", "e23", vec!["e12"]),
        ("e31", "e31", vec!["-1"]),
        ("e31", "e12", vec!["-e23"]),
        ("e31", "e43", vec!["-e41"]),
        ("e31", "e42", vec!["-e1234"]),
        ("e31", "e41", vec!["e43"]),
        ("e31", "e321", vec!["e2"]),
        ("e31", "e412", vec!["-e423"]),
        ("e31", "e431", vec!["-e4"]),
        ("e31", "e423", vec!["e412"]),
        ("e31", "e1234", vec!["e42"]),
        ("e31", "e5", vec!["e315"]),
        ("e31", "e15", vec!["e35"]),
        ("e31", "e25", vec!["-e3215"]),
        ("e31", "e35", vec!["-e15"]),
        ("e31", "e45", vec!["e4315"]),
        ("e31", "e235", vec!["e125"]),
        ("e31", "e315", vec!["-e5"]),
        ("e31", "e125", vec!["-e235"]),
        ("e31", "e435", vec!["-e415"]),
        ("e31", "e425", vec!["-e12345"]),
        ("e31", "e415", vec!["e435"]),
        ("e31", "e3215", vec!["e25"]),
        ("e31", "e4125", vec!["-e4235"]),
        ("e31", "e4315", vec!["-e45"]),
        ("e31", "e4235", vec!["e4125"]),
        ("e31", "e12345", vec!["e425"]),
        ("e12", "1", vec!["e12"]),
        ("e12", "e1", vec!["-e2"]),
        ("e12", "e2", vec!["e1"]),
        ("e12", "e3", vec!["-e321"]),
        ("e12", "e4", vec!["e412"]),
        ("e12", "e23", vec!["-e31"]),
        ("e12", "e31", vec!["e23"]),
        ("e12", "e12", vec!["-1"]),
        ("e12", "e43", vec!["-e1234"]),
        ("e12", "e42", vec!["e41"]),
        ("e12", "e41", vec!["-e42"]),
        ("e12", "e321", vec!["e3"]),
        ("e12", "e412", vec!["-e4"]),
        ("e12", "e431", vec!["e423"]),
        ("e12", "e423", vec!["-e431"]),
        ("e12", "e1234", vec!["e43"]),
        ("e12", "e5", vec!["e125"]),
        ("e12", "e15", vec!["-e25"]),
        ("e12", "e25", vec!["e15"]),
        ("e12", "e35", vec!["-e3215"]),
        ("e12", "e45", vec!["e4125"]),
        ("e12", "e235", vec!["-e315"]),
        ("e12", "e315", vec!["e235"]),
        ("e12", "e125", vec!["-e5"]),
        ("e12", "e435", vec!["-e12345"]),
        ("e12", "e425", vec!["e415"]),
        ("e12", "e415", vec!["-e425"]),
        ("e12", "e3215", vec!["e35"]),
        ("e12", "e4125", vec!["-e45"]),
        ("e12", "e4315", vec!["e4235"]),
        ("e12", "e4235", vec!["-e4315"]),
        ("e12", "e12345", vec!["e435"]),
        ("e43", "1", vec!["e43"]),
        ("e43", "e1", vec!["e431"]),
        ("e43", "e2", vec!["-e423"]),
        ("e43", "e3", vec!["e4"]),
        ("e43", "e4", vec![]),
        ("e43", "e23", vec!["-e42"]),
        ("e43", "e31", vec!["e41"]),
        ("e43", "e12", vec!["-e1234"]),
        ("e43", "e43", vec![]),
        ("e43", "e42", vec![]),
        ("e43", "e41", vec![]),
        ("e43", "e321", vec!["-e412"]),
        ("e43", "e412", vec![]),
        ("e43", "e431", vec![]),
        ("e43", "e423", vec![]),
        ("e43", "e1234", vec![]),
        ("e43", "e5", vec!["e3", "e435"]),
        ("e43", "e15", vec!["e4315", "-e31"]),
        ("e43", "e25", vec!["e23", "-e4235"]),
        ("e43", "e35", vec!["e45", "-1"]),
        ("e43", "e45", vec!["e43"]),
        ("e43", "e235", vec!["-e2", "-e425"]),
        ("e43", "e315", vec!["e1", "e415"]),
        ("e43", "e125", vec!["-e321", "-e12345"]),
        ("e43", "e435", vec!["-e4"]),
        ("e43", "e425", vec!["e423"]),
        ("e43", "e415", vec!["-e431"]),
        ("e43", "e3215", vec!["e12", "-e4125"]),
        ("e43", "e4125", vec!["-e1234"]),
        ("e43", "e4315", vec!["e41"]),
        ("e43", "e4235", vec!["-e42"]),
        ("e43", "e12345", vec!["e412"]),
        ("e42", "1", vec!["e42"]),
        ("e42", "e1", vec!["-e412"]),
        ("e42", "e2", vec!["e4"]),
        ("e42", "e3", vec!["e423"]),
        ("e42", "e4", vec![]),
        ("e42", "e23", vec!["e43"]),
        ("e42", "e31", vec!["-e1234"]),
        ("e42", "e12", vec!["-e41"]),
        ("e42", "e43", vec![]),
        ("e42", "e42", vec![]),
        ("e42", "e41", vec![]),
        ("e42", "e321", vec!["-e431"]),
        ("e42", "e412", vec![]),
        ("e42", "e431", vec![]),
        ("e42", "e423", vec![]),
        ("e42", "e1234", vec![]),
        ("e42", "e5", vec!["e2", "e425"]),
        ("e42", "e15", vec!["e12", "-e4125"]),
        ("e42", "e25", vec!["e45", "-1"]),
        ("e42", "e35", vec!["e4235", "-e23"]),
        ("e42", "e45", vec!["e42"]),
        ("e42", "e235", vec!["e3", "e435"]),
        ("e42", "e315", vec!["-e321", "-e12345"]),
        ("e42", "e125", vec!["-e1", "-e415"]),
        ("e42", "e435", vec!["-e423"]),
        ("e42", "e425", vec!["-e4"]),
        ("e42", "e415", vec!["e412"]),
        ("e42", "e3215", vec!["e31", "-e4315"]),
        ("e42", "e4125", vec!["-e41"]),
        ("e42", "e4315", vec!["-e1234"]),
        ("e42", "e4235", vec!["e43"]),
        ("e42", "e12345", vec!["e431"]),
        ("e41", "1", vec!["e41"]),
        ("e41", "e1", vec!["e4"]),
        ("e41", "e2", vec!["e412"]),
        ("e41", "e3", vec!["-e431"]),
        ("e41", "e4", vec![]),
        ("e41", "e23", vec!["-e1234"]),
        ("e41", "e31", vec!["-e43"]),
        ("e41", "e12", vec!["e42"]),
        ("e41", "e43", vec![]),
        ("e41", "e42", vec![]),
        ("e41", "e41", vec![]),
        ("e41", "e321", vec!["-e423"]),
        ("e41", "e412", vec![]),
        ("e41", "e431", vec![]),
        ("e41", "e423", vec![]),
        ("e41", "e1234", vec![]),
        ("e41", "e5", vec!["e1", "e415"]),
        ("e41", "e15", vec!["e45", "-1"]),
        ("e41", "e25", vec!["e4125", "-e12"]),
        ("e41", "e35", vec!["e31", "-e4315"]),
        ("e41", "e45", vec!["e41"]),
        ("e41", "e235", vec!["-e321", "-e12345"]),
        ("e41", "e315", vec!["-e3", "-e435"]),
        ("e41", "e125", vec!["e2", "e425"]),
        ("e41", "e435", vec!["e431"]),
        ("e41", "e425", vec!["-e412"]),
        ("e41", "e415", vec!["-e4"]),
        ("e41", "e3215", vec!["e23", "-e4235"]),
        ("e41", "e4125", vec!["e42"]),
        ("e41", "e4315", vec!["-e43"]),
        ("e41", "e4235", vec!["-e1234"]),
        ("e41", "e12345", vec!["e423"]),
        ("e321", "1", vec!["e321"]),
        ("e321", "e1", vec!["-e23"]),
        ("e321", "e2", vec!["-e31"]),
        ("e321", "e3", vec!["-e12"]),
        ("e321", "e4", vec!["-e1234"]),
        ("e321", "e23", vec!["e1"]),
        ("e321", "e31", vec!["e2"]),
        ("e321", "e12", vec!["e3"]),
        ("e321", "e43", vec!["e412"]),
        ("e321", "e42", vec!["e431"]),
        ("e321", "e41", vec!["e423"]),
        ("e321", "e321", vec!["-1"]),
        ("e321", "e412", vec!["-e43"]),
        ("e321", "e431", vec!["-e42"]),
        ("e321", "e423", vec!["-e41"]),
        ("e321", "e1234", vec!["e4"]),
        ("e321", "e5", vec!["e3215"]),
        ("e321", "e15", vec!["-e235"]),
        ("e321", "e25", vec!["-e315"]),
        ("e321", "e35", vec!["-e125"]),
        ("e321", "e45", vec!["-e12345"]),
        ("e321", "e235", vec!["e15"]),
        ("e321", "e315", vec!["e25"]),
        ("e321", "e125", vec!["e35"]),
        ("e321", "e435", vec!["e4125"]),
        ("e321", "e425", vec!["e4315"]),
        ("e321", "e415", vec!["e4235"]),
        ("e321", "e3215", vec!["-e5"]),
        ("e321", "e4125", vec!["-e435"]),
        ("e321", "e4315", vec!["-e425"]),
        ("e321", "e4235", vec!["-e415"]),
        ("e321", "e12345", vec!["e45"]),
        ("e412", "1", vec!["e412"]),
        ("e412", "e1", vec!["-e42"]),
        ("e412", "e2", vec!["e41"]),
        ("e412", "e3", vec!["-e1234"]),
        ("e412", "e4", vec![]),
        ("e412", "e23", vec!["-e431"]),
        ("e412", "e31", vec!["e423"]),
        ("e412", "e12", vec!["-e4"]),
        ("e412", "e43", vec![]),
        ("e412", "e42", vec![]),
        ("e412", "e41", vec![]),
        ("e412", "e321", vec!["e43"]),
        ("e412", "e412", vec![]),
        ("e412", "e431", vec![]),
        ("e412", "e423", vec![]),
        ("e412", "e1234", vec![]),
        ("e412", "e5", vec!["e4125", "-e12"]),
        ("e412", "e15", vec!["-e2", "-e425"]),
        ("e412", "e25", vec!["e1", "e415"]),
        ("e412", "e35", vec!["-e321", "-e12345"]),
        ("e412", "e45", vec!["e412"]),
        ("e412", "e235", vec!["e31", "-e4315"]),
        ("e412", "e315", vec!["e4235", "-e23"]),
        ("e412", "e125", vec!["1", "-e45"]),
        ("e412", "e435", vec!["e1234"]),
        ("e412", "e425", vec!["-e41"]),
        ("e412", "e415", vec!["e42"]),
        ("e412", "e3215", vec!["e3", "e435"]),
        ("e412", "e4125", vec!["-e4"]),
        ("e412", "e4315", vec!["e423"]),
        ("e412", "e4235", vec!["-e431"]),
        ("e412", "e12345", vec!["-e43"]),
        ("e431", "1", vec!["e431"]),
        ("e431", "e1", vec!["e43"]),
        ("e431", "e2", vec!["-e1234"]),
        ("e431", "e3", vec!["-e41"]),
        ("e431", "e4", vec![]),
        ("e431", "e23", vec!["e412"]),
        ("e431", "e31", vec!["-e4"]),
        ("e431", "e12", vec!["-e423"]),
        ("e431", "e43", vec![]),
        ("e431", "e42", vec![]),
        ("e431", "e41", vec![]),
        ("e431", "e321", vec!["e42"]),
        ("e431", "e412", vec![]),
        ("e431", "e431", vec![]),
        ("e431", "e423", vec![]),
        ("e431", "e1234", vec![]),
        ("e431", "e5", vec!["e4315", "-e31"]),
        ("e431", "e15", vec!["e3", "e435"]),
        ("e431", "e25", vec!["-e321", "-e12345"]),
        ("e431", "e35", vec!["-e1", "-e415"]),
        ("e431", "e45", vec!["e431"]),
        ("e431", "e235", vec!["e4125", "-e12"]),
        ("e431", "e315", vec!["1", "-e45"]),
        ("e431", "e125", vec!["e23", "-e4235"]),
        ("e431", "e435", vec!["e41"]),
        ("e431", "e425", vec!["e1234"]),
        ("e431", "e415", vec!["-e43"]),
        ("e431", "e3215", vec!["e2", "e425"]),
        ("e431", "e4125", vec!["-e423"]),
        ("e431", "e4315", vec!["-e4"]),
        ("e431", "e4235", vec!["e412"]),
        ("e431", "e12345", vec!["-e42"]),
        ("e423", "1", vec!["e423"]),
        ("e423", "e1", vec!["-e1234"]),
        ("e423", "e2", vec!["-e43"]),
        ("e423", "e3", vec!["e42"]),
        ("e423", "e4", vec![]),
        ("e423", "e23", vec!["-e4"]),
        ("e423", "e31", vec!["-e412"]),
        ("e423", "e12", vec!["e431"]),
        ("e423", "e43", vec![]),
        ("e423", "e42", vec![]),
        ("e423", "e41", vec![]),
        ("e423", "e321", vec!["e41"]),
        ("e423", "e412", vec![]),
        ("e423", "e431", vec![]),
        ("e423", "e423", vec![]),
        ("e423", "e1234", vec![]),
        ("e423", "e5", vec!["e4235", "-e23"]),
        ("e423", "e15", vec!["-e321", "-e12345"]),
        ("e423", "e25", vec!["-e3", "-e435"]),
        ("e423", "e35", vec!["e2", "e425"]),
        ("e423", "e45", vec!["e423"]),
        ("e423", "e235", vec!["1", "-e45"]),
        ("e423", "e315", vec!["e12", "-e4125"]),
        ("e423", "e125", vec!["e4315", "-e31"]),
        ("e423", "e435", vec!["-e42"]),
        ("e423", "e425", vec!["e43"]),
        ("e423", "e415", vec!["e1234"]),
        ("e423", "e3215", vec!["e1", "e415"]),
        ("e423", "e4125", vec!["e431"]),
        ("e423", "e4315", vec!["-e412"]),
        ("e423", "e4235", vec!["-e4"]),
        ("e423", "e12345", vec!["-e41"]),
        ("e1234", "1", vec!["e1234"]),
        ("e1234", "e1", vec!["-e423"]),
        ("e1234", "e2", vec!["-e431"]),
        ("e1234", "e3", vec!["-e412"]),
        ("e1234", "e4", vec![]),
        ("e1234", "e23", vec!["e41"]),
        ("e1234", "e31", vec!["e42"]),
        ("e1234", "e12", vec!["e43"]),
        ("e1234", "e43", vec![]),
        ("e1234", "e42", vec![]),
        ("e1234", "e41", vec![]),
        ("e1234", "e321", vec!["-e4"]),
        ("e1234", "e412", vec![]),
        ("e1234", "e431", vec![]),
        ("e1234", "e423", vec![]),
        ("e1234", "e1234", vec![]),
        ("e1234", "e5", vec!["e321", "e12345"]),
        ("e1234", "e15", vec!["e23", "-e4235"]),
        ("e1234", "e25", vec!["e31", "-e4315"]),
        ("e1234", "e35", vec!["e12", "-e4125"]),
        ("e1234", "e45", vec!["e1234"]),
        ("e1234", "e235", vec!["e1", "e415"]),
        ("e1234", "e315", vec!["e2", "e425"]),
        ("e1234", "e125", vec!["e3", "e435"]),
        ("e1234", "e435", vec!["e412"]),
        ("e1234", "e425", vec!["e431"]),
        ("e1234", "e415", vec!["e423"]),
        ("e1234", "e3215", vec!["1", "-e45"]),
        ("e1234", "e4125", vec!["e43"]),
        ("e1234", "e4315", vec!["e42"]),
        ("e1234", "e4235", vec!["e41"]),
        ("e1234", "e12345", vec!["e4"]),
        ("e5", "1", vec!["e5"]),
        ("e5", "e1", vec!["-e15"]),
        ("e5", "e2", vec!["-e25"]),
        ("e5", "e3", vec!["-e35"]),
        ("e5", "e4", vec!["-1", "-e45"]),
        ("e5", "e23", vec!["e235"]),
        ("e5", "e31", vec!["e315"]),
        ("e5", "e12", vec!["e125"]),
        ("e5", "e43", vec!["e435", "-e3"]),
        ("e5", "e42", vec!["e425", "-e2"]),
        ("e5", "e41", vec!["e415", "-e1"]),
        ("e5", "e321", vec!["-e3215"]),
        ("e5", "e412", vec!["-e12", "-e4125"]),
        ("e5", "e431", vec!["-e31", "-e4315"]),
        ("e5", "e423", vec!["-e23", "-e4235"]),
        ("e5", "e1234", vec!["e12345", "-e321"]),
        ("e5", "e5", vec![]),
        ("e5", "e15", vec![]),
        ("e5", "e25", vec![]),
        ("e5", "e35", vec![]),
        ("e5", "e45", vec!["-e5"]),
        ("e5", "e235", vec![]),
        ("e5", "e315", vec![]),
        ("e5", "e125", vec![]),
        ("e5", "e435", vec!["-e35"]),
        ("e5", "e425", vec!["-e25"]),
        ("e5", "e415", vec!["-e15"]),
        ("e5", "e3215", vec![]),
        ("e5", "e4125", vec!["-e125"]),
        ("e5", "e4315", vec!["-e315"]),
        ("e5", "e4235", vec!["-e235"]),
        ("e5", "e12345", vec!["-e3215"]),
        ("e15", "1", vec!["e15"]),
        ("e15", "e1", vec!["-e5"]),
        ("e15", "e2", vec!["-e125"]),
        ("e15", "e3", vec!["e315"]),
        ("e15", "e4", vec!["e415", "-e1"]),
        ("e15", "e23", vec!["-e3215"]),
        ("e15", "e31", vec!["-e35"]),
        ("e15", "e12", vec!["e25"]),
        ("e15", "e43", vec!["e31", "e4315"]),
        ("e15", "e42", vec!["-e12", "-e4125"]),
        ("e15", "e41", vec!["-1", "-e45"]),
        ("e15", "e321", vec!["e235"]),
        ("e15", "e412", vec!["e425", "-e2"]),
        ("e15", "e431", vec!["e3", "-e435"]),
        ("e15", "e423", vec!["e321", "-e12345"]),
        ("e15", "e1234", vec!["e23", "e4235"]),
        ("e15", "e5", vec![]),
        ("e15", "e15", vec![]),
        ("e15", "e25", vec![]),
        ("e15", "e35", vec![]),
        ("e15", "e45", vec!["-e15"]),
        ("e15", "e235", vec![]),
        ("e15", "e315", vec![]),
        ("e15", "e125", vec![]),
        ("e15", "e435", vec!["e315"]),
        ("e15", "e425", vec!["-e125"]),
        ("e15", "e415", vec!["-e5"]),
        ("e15", "e3215", vec![]),
        ("e15", "e4125", vec!["-e25"]),
        ("e15", "e4315", vec!["e35"]),
        ("e15", "e4235", vec!["e3215"]),
        ("e15", "e12345", vec!["e235"]),
        ("e25", "1", vec!["e25"]),
        ("e25", "e1", vec!["e125"]),
        ("e25", "e2", vec!["-e5"]),
        ("e25", "e3", vec!["-e235"]),
        ("e25", "e4", vec!["e425", "-e2"]),
        ("e25", "e23", vec!["e35"]),
        ("e25", "e31", vec!["-e3215"]),
        ("e25", "e12", vec!["-e15"]),
        ("e25", "e43", vec!["-e23", "-e4235"]),
        ("e25", "e42", vec!["-1", "-e45"]),
        ("e25", "e41", vec!["e12", "e4125"]),
        ("e25", "e321", vec!["e315"]),
        ("e25", "e412", vec!["e1", "-e415"]),
        ("e25", "e431", vec!["e321", "-e12345"]),
        ("e25", "e423", vec!["e435", "-e3"]),
        ("e25", "e1234", vec!["e31", "e4315"]),
        ("e25", "e5", vec![]),
        ("e25", "e15", vec![]),
        ("e25", "e25", vec![]),
        ("e25", "e35", vec![]),
        ("e25", "e45", vec!["-e25"]),
        ("e25", "e235", vec![]),
        ("e25", "e315", vec![]),
        ("e25", "e125", vec![]),
        ("e25", "e435", vec!["-e235"]),
        ("e25", "e425", vec!["-e5"]),
        ("e25", "e415", vec!["e125"]),
        ("e25", "e3215", vec![]),
        ("e25", "e4125", vec!["e15"]),
        ("e25", "e4315", vec!["e3215"]),
        ("e25", "e4235", vec!["-e35"]),
        ("e25", "e12345", vec!["e315"]),
        ("e35", "1", vec!["e35"]),
        ("e35", "e1", vec!["-e315"]),
        ("e35", "e2", vec!["e235"]),
        ("e35", "e3", vec!["-e5"]),
        ("e35", "e4", vec!["e435", "-e3"]),
        ("e35", "e23", vec!["-e25"]),
        ("e35", "e31", vec!["e15"]),
        ("e35", "e12", vec!["-e3215"]),
        ("e35", "e43", vec!["-1", "-e45"]),
        ("e35", "e42", vec!["e23", "e4235"]),
        ("e35", "e41", vec!["-e31", "-e4315"]),
        ("e35", "e321", vec!["e125"]),
        ("e35", "e412", vec!["e321", "-e12345"]),
        ("e35", "e431", vec!["e415", "-e1"]),
        ("e35", "e423", vec!["e2", "-e425"]),
        ("e35", "e1234", vec!["e12", "e4125"]),
        ("e35", "e5", vec![]),
        ("e35", "e15", vec![]),
        ("e35", "e25", vec![]),
        ("e35", "e35", vec![]),
        ("e35", "e45", vec!["-e35"]),
        ("e35", "e235", vec![]),
        ("e35", "e315", vec![]),
        ("e35", "e125", vec![]),
        ("e35", "e435", vec!["-e5"]),
        ("e35", "e425", vec!["e235"]),
        ("e35", "e415", vec!["-e315"]),
        ("e35", "e3215", vec![]),
        ("e35", "e4125", vec!["e3215"]),
        ("e35", "e4315", vec!["-e15"]),
        ("e35", "e4235", vec!["e25"]),
        ("e35", "e12345", vec!["e125"]),
        ("e45", "1", vec!["e45"]),
        ("e45", "e1", vec!["-e415"]),
        ("e45", "e2", vec!["-e425"]),
        ("e45", "e3", vec!["-e435"]),
        ("e45", "e4", vec!["-e4"]),
        ("e45", "e23", vec!["e4235"]),
        ("e45", "e31", vec!["e4315"]),
        ("e45", "e12", vec!["e4125"]),
        ("e45", "e43", vec!["-e43"]),
        ("e45", "e42", vec!["-e42"]),
        ("e45", "e41", vec!["-e41"]),
        ("e45", "e321", vec!["-e12345"]),
        ("e45", "e412", vec!["-e412"]),
        ("e45", "e431", vec!["-e431"]),
        ("e45", "e423", vec!["-e423"]),
        ("e45", "e1234", vec!["-e1234"]),
        ("e45", "e5", vec!["e5"]),
        ("e45", "e15", vec!["e15"]),
        ("e45", "e25", vec!["e25"]),
        ("e45", "e35", vec!["e35"]),
        ("e45", "e45", vec!["1"]),
        ("e45", "e235", vec!["e235"]),
        ("e45", "e315", vec!["e315"]),
        ("e45", "e125", vec!["e125"]),
        ("e45", "e435", vec!["-e3"]),
        ("e45", "e425", vec!["-e2"]),
        ("e45", "e415", vec!["-e1"]),
        ("e45", "e3215", vec!["e3215"]),
        ("e45", "e4125", vec!["e12"]),
        ("e45", "e4315", vec!["e31"]),
        ("e45", "e4235", vec!["e23"]),
        ("e45", "e12345", vec!["-e321"]),
        ("e235", "1", vec!["e235"]),
        ("e235", "e1", vec!["e3215"]),
        ("e235", "e2", vec!["e35"]),
        ("e235", "e3", vec!["-e25"]),
        ("e235", "e4", vec!["-e23", "-e4235"]),
        ("e235", "e23", vec!["-e5"]),
        ("e235", "e31", vec!["-e125"]),
        ("e235", "e12", vec!["e315"]),
        ("e235", "e43", vec!["e425", "-e2"]),
        ("e235", "e42", vec!["e3", "-e435"]),
        ("e235", "e41", vec!["e321", "-e12345"]),
        ("e235", "e321", vec!["-e15"]),
        ("e235", "e412", vec!["-e31", "-e4315"]),
        ("e235", "e431", vec!["e12", "e4125"]),
        ("e235", "e423", vec!["1", "e45"]),
        ("e235", "e1234", vec!["e415", "-e1"]),
        ("e235", "e5", vec![]),
        ("e235", "e15", vec![]),
        ("e235", "e25", vec![]),
        ("e235", "e35", vec![]),
        ("e235", "e45", vec!["-e235"]),
        ("e235", "e235", vec![]),
        ("e235", "e315", vec![]),
        ("e235", "e125", vec![]),
        ("e235", "e435", vec!["-e25"]),
        ("e235", "e425", vec!["e35"]),
        ("e235", "e415", vec!["e3215"]),
        ("e235", "e3215", vec![]),
        ("e235", "e4125", vec!["-e315"]),
        ("e235", "e4315", vec!["e125"]),
        ("e235", "e4235", vec!["e5"]),
        ("e235", "e12345", vec!["-e15"]),
        ("e315", "1", vec!["e315"]),
        ("e315", "e1", vec!["-e35"]),
        ("e315", "e2", vec!["e3215"]),
        ("e315", "e3", vec!["e15"]),
        ("e315", "e4", vec!["-e31", "-e4315"]),
        ("e315", "e23", vec!["e125"]),
        ("e315", "e31", vec!["-e5"]),
        ("e315", "e12", vec!["-e235"]),
        ("e315", "e43", vec!["e1", "-e415"]),
        ("e315", "e42", vec!["e321", "-e12345"]),
        ("e315", "e41", vec!["e435", "-e3"]),
        ("e315", "e321", vec!["-e25"]),
        ("e315", "e412", vec!["e23", "e4235"]),
        ("e315", "e431", vec!["1", "e45"]),
        ("e315", "e423", vec!["-e12", "-e4125"]),
        ("e315", "e1234", vec!["e425", "-e2"]),
        ("e315", "e5", vec![]),
        ("e315", "e15", vec![]),
        ("e315", "e25", vec![]),
        ("e315", "e35", vec![]),
        ("e315", "e45", vec!["-e315"]),
        ("e315", "e235", vec![]),
        ("e315", "e315", vec![]),
        ("e315", "e125", vec![]),
        ("e315", "e435", vec!["e15"]),
        ("e315", "e425", vec!["e3215"]),
        ("e315", "e415", vec!["-e35"]),
        ("e315", "e3215", vec![]),
        ("e315", "e4125", vec!["e235"]),
        ("e315", "e4315", vec!["e5"]),
        ("e315", "e4235", vec!["-e125"]),
        ("e315", "e12345", vec!["-e25"]),
        ("e125", "1", vec!["e125"]),
        ("e125", "e1", vec!["e25"]),
        ("e125", "e2", vec!["-e15"]),
        ("e125", "e3", vec!["e3215"]),
        ("e125", "e4", vec!["-e12", "-e4125"]),
        ("e125", "e23", vec!["-e315"]),
        ("e125", "e31", vec!["e235"]),
        ("e125", "e12", vec!["-e5"]),
        ("e125", "e43", vec!["e321", "-e12345"]),
        ("e125", "e42", vec!["e415", "-e1"]),
        ("e125", "e41", vec!["e2", "-e425"]),
        ("e125", "e321", vec!["-e35"]),
        ("e125", "e412", vec!["1", "e45"]),
        ("e125", "e431", vec!["-e23", "-e4235"]),
        ("e125", "e423", vec!["e31", "e4315"]),
        ("e125", "e1234", vec!["e435", "-e3"]),
        ("e125", "e5", vec![]),
        ("e125", "e15", vec![]),
        ("e125", "e25", vec![]),
        ("e125", "e35", vec![]),
        ("e125", "e45", vec!["-e125"]),
        ("e125", "e235", vec![]),
        ("e125", "e315", vec![]),
        ("e125", "e125", vec![]),
        ("e125", "e435", vec!["e3215"]),
        ("e125", "e425", vec!["-e15"]),
        ("e125", "e415", vec!["e25"]),
        ("e125", "e3215", vec![]),
        ("e125", "e4125", vec!["e5"]),
        ("e125", "e4315", vec!["-e235"]),
        ("e125", "e4235", vec!["e315"]),
        ("e125", "e12345", vec!["-e35"]),
        ("e435", "1", vec!["e435"]),
        ("e435", "e1", vec!["-e4315"]),
        ("e435", "e2", vec!["e4235"]),
        ("e435", "e3", vec!["-e45"]),
        ("e435", "e4", vec!["-e43"]),
        ("e435", "e23", vec!["-e425"]),
        ("e435", "e31", vec!["e415"]),
        ("e435", "e12", vec!["-e12345"]),
        ("e435", "e43", vec!["-e4"]),
        ("e435", "e42", vec!["e423"]),
        ("e435", "e41", vec!["-e431"]),
        ("e435", "e321", vec!["e4125"]),
        ("e435", "e412", vec!["e1234"]),
        ("e435", "e431", vec!["-e41"]),
        ("e435", "e423", vec!["e42"]),
        ("e435", "e1234", vec!["e412"]),
        ("e435", "e5", vec!["-e35"]),
        ("e435", "e15", vec!["-e315"]),
        ("e435", "e25", vec!["e235"]),
        ("e435", "e35", vec!["-e5"]),
        ("e435", "e45", vec!["-e3"]),
        ("e435", "e235", vec!["e25"]),
        ("e435", "e315", vec!["-e15"]),
        ("e435", "e125", vec!["e3215"]),
        ("e435", "e435", vec!["1"]),
        ("e435", "e425", vec!["-e23"]),
        ("e435", "e415", vec!["e31"]),
        ("e435", "e3215", vec!["e125"]),
        ("e435", "e4125", vec!["e321"]),
        ("e435", "e4315", vec!["-e1"]),
        ("e435", "e4235", vec!["e2"]),
        ("e435", "e12345", vec!["-e12"]),
        ("e425", "1", vec!["e425"]),
        ("e425", "e1", vec!["e4125"]),
        ("e425", "e2", vec!["-e45"]),
        ("e425", "e3", vec!["-e4235"]),
        ("e425", "e4", vec!["-e42"]),
        ("e425", "e23", vec!["e435"]),
        ("e425", "e31", vec!["-e12345"]),
        ("e425", "e12", vec!["-e415"]),
        ("e425", "e43", vec!["-e423"]),
        ("e425", "e42", vec!["-e4"]),
        ("e425", "e41", vec!["e412"]),
        ("e425", "e321", vec!["e4315"]),
        ("e425", "e412", vec!["e41"]),
        ("e425", "e431", vec!["e1234"]),
        ("e425", "e423", vec!["-e43"]),
        ("e425", "e1234", vec!["e431"]),
        ("e425", "e5", vec!["-e25"]),
        ("e425", "e15", vec!["e125"]),
        ("e425", "e25", vec!["-e5"]),
        ("e425", "e35", vec!["-e235"]),
        ("e425", "e45", vec!["-e2"]),
        ("e425", "e235", vec!["-e35"]),
        ("e425", "e315", vec!["e3215"]),
        ("e425", "e125", vec!["e15"]),
        ("e425", "e435", vec!["e23"]),
        ("e425", "e425", vec!["1"]),
        ("e425", "e415", vec!["-e12"]),
        ("e425", "e3215", vec!["e315"]),
        ("e425", "e4125", vec!["e1"]),
        ("e425", "e4315", vec!["e321"]),
        ("e425", "e4235", vec!["-e3"]),
        ("e425", "e12345", vec!["-e31"]),
        ("e415", "1", vec!["e415"]),
        ("e415", "e1", vec!["-e45"]),
        ("e415", "e2", vec!["-e4125"]),
        ("e415", "e3", vec!["e4315"]),
        ("e415", "e4", vec!["-e41"]),
        ("e415", "e23", vec!["-e12345"]),
        ("e415", "e31", vec!["-e435"]),
        ("e415", "e12", vec!["e425"]),
        ("e415", "e43", vec!["e431"]),
        ("e415", "e42", vec!["-e412"]),
        ("e415", "e41", vec!["-e4"]),
        ("e415", "e321", vec!["e4235"]),
        ("e415", "e412", vec!["-e42"]),
        ("e415", "e431", vec!["e43"]),
        ("e415", "e423", vec!["e1234"]),
        ("e415", "e1234", vec!["e423"]),
        ("e415", "e5", vec!["-e15"]),
        ("e415", "e15", vec!["-e5"]),
        ("e415", "e25", vec!["-e125"]),
        ("e415", "e35", vec!["e315"]),
        ("e415", "e45", vec!["-e1"]),
        ("e415", "e235", vec!["e3215"]),
        ("e415", "e315", vec!["e35"]),
        ("e415", "e125", vec!["-e25"]),
        ("e415", "e435", vec!["-e31"]),
        ("e415", "e425", vec!["e12"]),
        ("e415", "e415", vec!["1"]),
        ("e415", "e3215", vec!["e235"]),
        ("e415", "e4125", vec!["-e2"]),
        ("e415", "e4315", vec!["e3"]),
        ("e415", "e4235", vec!["e321"]),
        ("e415", "e12345", vec!["-e23"]),
        ("e3215", "1", vec!["e3215"]),
        ("e3215", "e1", vec!["e235"]),
        ("e3215", "e2", vec!["e315"]),
        ("e3215", "e3", vec!["e125"]),
        ("e3215", "e4", vec!["e12345", "-e321"]),
        ("e3215", "e23", vec!["e15"]),
        ("e3215", "e31", vec!["e25"]),
        ("e3215", "e12", vec!["e35"]),
        ("e3215", "e43", vec!["e12", "e4125"]),
        ("e3215", "e42", vec!["e31", "e4315"]),
        ("e3215", "e41", vec!["e23", "e4235"]),
        ("e3215", "e321", vec!["e5"]),
        ("e3215", "e412", vec!["e435", "-e3"]),
        ("e3215", "e431", vec!["e425", "-e2"]),
        ("e3215", "e423", vec!["e415", "-e1"]),
        ("e3215", "e1234", vec!["1", "e45"]),
        ("e3215", "e5", vec![]),
        ("e3215", "e15", vec![]),
        ("e3215", "e25", vec![]),
        ("e3215", "e35", vec![]),
        ("e3215", "e45", vec!["-e3215"]),
        ("e3215", "e235", vec![]),
        ("e3215", "e315", vec![]),
        ("e3215", "e125", vec![]),
        ("e3215", "e435", vec!["e125"]),
        ("e3215", "e425", vec!["e315"]),
        ("e3215", "e415", vec!["e235"]),
        ("e3215", "e3215", vec![]),
        ("e3215", "e4125", vec!["-e35"]),
        ("e3215", "e4315", vec!["-e25"]),
        ("e3215", "e4235", vec!["-e15"]),
        ("e3215", "e12345", vec!["e5"]),
        ("e4125", "1", vec!["e4125"]),
        ("e4125", "e1", vec!["e425"]),
        ("e4125", "e2", vec!["-e415"]),
        ("e4125", "e3", vec!["e12345"]),
        ("e4125", "e4", vec!["-e412"]),
        ("e4125", "e23", vec!["-e4315"]),
        ("e4125", "e31", vec!["e4235"]),
        ("e4125", "e12", vec!["-e45"]),
        ("e4125", "e43", vec!["e1234"]),
        ("e4125", "e42", vec!["-e41"]),
        ("e4125", "e41", vec!["e42"]),
        ("e4125", "e321", vec!["-e435"]),
        ("e4125", "e412", vec!["e4"]),
        ("e4125", "e431", vec!["-e423"]),
        ("e4125", "e423", vec!["e431"]),
        ("e4125", "e1234", vec!["-e43"]),
        ("e4125", "e5", vec!["e125"]),
        ("e4125", "e15", vec!["-e25"]),
        ("e4125", "e25", vec!["e15"]),
        ("e4125", "e35", vec!["-e3215"]),
        ("e4125", "e45", vec!["e12"]),
        ("e4125", "e235", vec!["-e315"]),
        ("e4125", "e315", vec!["e235"]),
        ("e4125", "e125", vec!["-e5"]),
        ("e4125", "e435", vec!["e321"]),
        ("e4125", "e425", vec!["-e1"]),
        ("e4125", "e415", vec!["e2"]),
        ("e4125", "e3215", vec!["e35"]),
        ("e4125", "e4125", vec!["-1"]),
        ("e4125", "e4315", vec!["e23"]),
        ("e4125", "e4235", vec!["-e31"]),
        ("e4125", "e12345", vec!["-e3"]),
        ("e4315", "1", vec!["e4315"]),
        ("e4315", "e1", vec!["-e435"]),
        ("e4315", "e2", vec!["e12345"]),
        ("e4315", "e3", vec!["e415"]),
        ("e4315", "e4", vec!["-e431"]),
        ("e4315", "e23", vec!["e4125"]),
        ("e4315", "e31", vec!["-e45"]),
        ("e4315", "e12", vec!["-e4235"]),
        ("e4315", "e43", vec!["e41"]),
        ("e4315", "e42", vec!["e1234"]),
        ("e4315", "e41", vec!["-e43"]),
        ("e4315", "e321", vec!["-e425"]),
        ("e4315", "e412", vec!["e423"]),
        ("e4315", "e431", vec!["e4"]),
        ("e4315", "e423", vec!["-e412"]),
        ("e4315", "e1234", vec!["-e42"]),
        ("e4315", "e5", vec!["e315"]),
        ("e4315", "e15", vec!["e35"]),
        ("e4315", "e25", vec!["-e3215"]),
        ("e4315", "e35", vec!["-e15"]),
        ("e4315", "e45", vec!["e31"]),
        ("e4315", "e235", vec!["e125"]),
        ("e4315", "e315", vec!["-e5"]),
        ("e4315", "e125", vec!["-e235"]),
        ("e4315", "e435", vec!["e1"]),
        ("e4315", "e425", vec!["e321"]),
        ("e4315", "e415", vec!["-e3"]),
        ("e4315", "e3215", vec!["e25"]),
        ("e4315", "e4125", vec!["-e23"]),
        ("e4315", "e4315", vec!["-1"]),
        ("e4315", "e4235", vec!["e12"]),
        ("e4315", "e12345", vec!["-e2"]),
        ("e4235", "1", vec!["e4235"]),
        ("e4235", "e1", vec!["e12345"]),
        ("e4235", "e2", vec!["e435"]),
        ("e4235", "e3", vec!["-e425"]),
        ("e4235", "e4", vec!["-e423"]),
        ("e4235", "e23", vec!["-e45"]),
        ("e4235", "e31", vec!["-e4125"]),
        ("e4235", "e12", vec!["e4315"]),
        ("e4235", "e43", vec!["-e42"]),
        ("e4235", "e42", vec!["e43"]),
        ("e4235", "e41", vec!["e1234"]),
        ("e4235", "e321", vec!["-e415"]),
        ("e4235", "e412", vec!["-e431"]),
        ("e4235", "e431", vec!["e412"]),
        ("e4235", "e423", vec!["e4"]),
        ("e4235", "e1234", vec!["-e41"]),
        ("e4235", "e5", vec!["e235"]),
        ("e4235", "e15", vec!["-e3215"]),
        ("e4235", "e25", vec!["-e35"]),
        ("e4235", "e35", vec!["e25"]),
        ("e4235", "e45", vec!["e23"]),
        ("e4235", "e235", vec!["-e5"]),
        ("e4235", "e315", vec!["-e125"]),
        ("e4235", "e125", vec!["e315"]),
        ("e4235", "e435", vec!["-e2"]),
        ("e4235", "e425", vec!["e3"]),
        ("e4235", "e415", vec!["e321"]),
        ("e4235", "e3215", vec!["e15"]),
        ("e4235", "e4125", vec!["e31"]),
        ("e4235", "e4315", vec!["-e12"]),
        ("e4235", "e4235", vec!["-1"]),
        ("e4235", "e12345", vec!["-e1"]),
        ("e12345", "1", vec!["e12345"]),
        ("e12345", "e1", vec!["e4235"]),
        ("e12345", "e2", vec!["e4315"]),
        ("e12345", "e3", vec!["e4125"]),
        ("e12345", "e4", vec!["-e1234"]),
        ("e12345", "e23", vec!["e415"]),
        ("e12345", "e31", vec!["e425"]),
        ("e12345", "e12", vec!["e435"]),
        ("e12345", "e43", vec!["e412"]),
        ("e12345", "e42", vec!["e431"]),
        ("e12345", "e41", vec!["e423"]),
        ("e12345", "e321", vec!["e45"]),
        ("e12345", "e412", vec!["-e43"]),
        ("e12345", "e431", vec!["-e42"]),
        ("e12345", "e423", vec!["-e41"]),
        ("e12345", "e1234", vec!["e4"]),
        ("e12345", "e5", vec!["-e3215"]),
        ("e12345", "e15", vec!["e235"]),
        ("e12345", "e25", vec!["e315"]),
        ("e12345", "e35", vec!["e125"]),
        ("e12345", "e45", vec!["-e321"]),
        ("e12345", "e235", vec!["-e15"]),
        ("e12345", "e315", vec!["-e25"]),
        ("e12345", "e125", vec!["-e35"]),
        ("e12345", "e435", vec!["-e12"]),
        ("e12345", "e425", vec!["-e31"]),
        ("e12345", "e415", vec!["-e23"]),
        ("e12345", "e3215", vec!["e5"]),
        ("e12345", "e4125", vec!["-e3"]),
        ("e12345", "e4315", vec!["-e2"]),
        ("e12345", "e4235", vec!["-e1"]),
        ("e12345", "e12345", vec!["-1"]),
    ]};



    use crate::algebra2::basis::elements::*;
    let mut underlying_cga = GeneratorSquares::new([(E1, 1), (E2, 1), (E3, 1), (EA, 1), (EB, -1)]);
    underlying_cga.negative_anti_scalar = true;
    let mut substituted_cga = SubstitutionRepository::new(underlying_cga, vec![
        // TODO i bet I can make this use operators which would actually be very nice and appropriate
        (E4, vec![(0.5, EB), (-0.5, EA)]),
        (E5, vec![(1.0, EB), (1.0, EA)]),
    ]);



    let mut failures = 0;
    let mut correct_products = BTreeMap::new();
    for (a, b, products) in correct_cayley_table {
        let mut a = BasisElement::parsed_display_name(a).expect("a must parse");
        a.display_name = None;
        let mut b = BasisElement::parsed_display_name(b).expect("b must parse");
        b.display_name = None;
        let mut sum = vec![];
        for product in products {
            // TODO go through the entire file and fix non-idiomatic expect(format!().as_str())
            let mut element = BasisElement::parsed_display_name(product).expect(format!("{product} must parse").as_str());
            element.display_name = None;
            sum.push(Product { coefficient: 1.0, element });
        }
        let mut sum = Sum { sum };
        sum.sort_and_simplify();
        correct_products.insert((a.signature, b.signature), (a.coefficient, b.coefficient, sum));
    }

    for a in 0..u16::pow(2, 6) {
        // skip e0
        if a & 1 == 1 {
            continue
        }

        let a_sig = BasisSignature::from_bits_retain(a);
        let mut a = BasisElement::from(a_sig);
        for b in 0..u16::pow(2, 6) {
            // skip e0
            if b & 1 == 1 {
                continue
            }

            let b_sig = BasisSignature::from_bits_retain(b);
            let mut b = BasisElement::from(b_sig);

            let (a_sign, b_sign, correct_product) = correct_products
                .remove(&(a_sig, b_sig))
                .unwrap_or_else(|| panic!("Cayley table must be complete, missing {a} * {b}"));

            a.coefficient = a_sign;
            b.coefficient = b_sign;

            let mut calculated_product = substituted_cga.product(&a, &b);
            if calculated_product != correct_product {
                eprintln!("{a} * {b} was calculated as {calculated_product:?}, but we expected {correct_product:?}");
                failures = failures + 1;
            }
        }
    }
    assert_eq!(failures, 0, "Conformal Geometric Product has {failures} errors.")
}


#[test]
fn conformal_3d_geometric_anti_products() {
    // https://conformalgeometricalgebra.org/wiki/index.php?title=Geometric_products
    let correct_cayley_table = {[
        ("1", "1", vec!["-e12345"]),
        ("1", "e1", vec!["-e4235"]),
        ("1", "e2", vec!["-e4315"]),
        ("1", "e3", vec!["-e4125"]),
        ("1", "e4", vec!["e1234"]),
        ("1", "e23", vec!["-e415"]),
        ("1", "e31", vec!["-e425"]),
        ("1", "e12", vec!["-e435"]),
        ("1", "e43", vec!["-e412"]),
        ("1", "e42", vec!["-e431"]),
        ("1", "e41", vec!["-e423"]),
        ("1", "e321", vec!["-e45"]),
        ("1", "e412", vec!["e43"]),
        ("1", "e431", vec!["e42"]),
        ("1", "e423", vec!["e41"]),
        ("1", "e1234", vec!["-e4"]),
        ("1", "e5", vec!["e3215"]),
        ("1", "e15", vec!["-e235"]),
        ("1", "e25", vec!["-e315"]),
        ("1", "e35", vec!["-e125"]),
        ("1", "e45", vec!["e321"]),
        ("1", "e235", vec!["e15"]),
        ("1", "e315", vec!["e25"]),
        ("1", "e125", vec!["e35"]),
        ("1", "e435", vec!["e12"]),
        ("1", "e425", vec!["e31"]),
        ("1", "e415", vec!["e23"]),
        ("1", "e3215", vec!["-e5"]),
        ("1", "e4125", vec!["e3"]),
        ("1", "e4315", vec!["e2"]),
        ("1", "e4235", vec!["e1"]),
        ("1", "e12345", vec!["1"]),
        ("e1", "1", vec!["-e4235"]),
        ("e1", "e1", vec!["-e12345"]),
        ("e1", "e2", vec!["e435"]),
        ("e1", "e3", vec!["-e425"]),
        ("e1", "e4", vec!["-e423"]),
        ("e1", "e23", vec!["e45"]),
        ("e1", "e31", vec!["-e4125"]),
        ("e1", "e12", vec!["e4315"]),
        ("e1", "e43", vec!["e42"]),
        ("e1", "e42", vec!["-e43"]),
        ("e1", "e41", vec!["e1234"]),
        ("e1", "e321", vec!["e415"]),
        ("e1", "e412", vec!["e431"]),
        ("e1", "e431", vec!["-e412"]),
        ("e1", "e423", vec!["e4"]),
        ("e1", "e1234", vec!["-e41"]),
        ("e1", "e5", vec!["e235"]),
        ("e1", "e15", vec!["-e3215"]),
        ("e1", "e25", vec!["e35"]),
        ("e1", "e35", vec!["-e25"]),
        ("e1", "e45", vec!["-e23"]),
        ("e1", "e235", vec!["-e5"]),
        ("e1", "e315", vec!["e125"]),
        ("e1", "e125", vec!["-e315"]),
        ("e1", "e435", vec!["-e2"]),
        ("e1", "e425", vec!["e3"]),
        ("e1", "e415", vec!["-e321"]),
        ("e1", "e3215", vec!["e15"]),
        ("e1", "e4125", vec!["e31"]),
        ("e1", "e4315", vec!["-e12"]),
        ("e1", "e4235", vec!["1"]),
        ("e1", "e12345", vec!["e1"]),
        ("e2", "1", vec!["-e4315"]),
        ("e2", "e1", vec!["-e435"]),
        ("e2", "e2", vec!["-e12345"]),
        ("e2", "e3", vec!["e415"]),
        ("e2", "e4", vec!["-e431"]),
        ("e2", "e23", vec!["e4125"]),
        ("e2", "e31", vec!["e45"]),
        ("e2", "e12", vec!["-e4235"]),
        ("e2", "e43", vec!["-e41"]),
        ("e2", "e42", vec!["e1234"]),
        ("e2", "e41", vec!["e43"]),
        ("e2", "e321", vec!["e425"]),
        ("e2", "e412", vec!["-e423"]),
        ("e2", "e431", vec!["e4"]),
        ("e2", "e423", vec!["e412"]),
        ("e2", "e1234", vec!["-e42"]),
        ("e2", "e5", vec!["e315"]),
        ("e2", "e15", vec!["-e35"]),
        ("e2", "e25", vec!["-e3215"]),
        ("e2", "e35", vec!["e15"]),
        ("e2", "e45", vec!["-e31"]),
        ("e2", "e235", vec!["-e125"]),
        ("e2", "e315", vec!["-e5"]),
        ("e2", "e125", vec!["e235"]),
        ("e2", "e435", vec!["e1"]),
        ("e2", "e425", vec!["-e321"]),
        ("e2", "e415", vec!["-e3"]),
        ("e2", "e3215", vec!["e25"]),
        ("e2", "e4125", vec!["-e23"]),
        ("e2", "e4315", vec!["1"]),
        ("e2", "e4235", vec!["e12"]),
        ("e2", "e12345", vec!["e2"]),
        ("e3", "1", vec!["-e4125"]),
        ("e3", "e1", vec!["e425"]),
        ("e3", "e2", vec!["-e415"]),
        ("e3", "e3", vec!["-e12345"]),
        ("e3", "e4", vec!["-e412"]),
        ("e3", "e23", vec!["-e4315"]),
        ("e3", "e31", vec!["e4235"]),
        ("e3", "e12", vec!["e45"]),
        ("e3", "e43", vec!["e1234"]),
        ("e3", "e42", vec!["e41"]),
        ("e3", "e41", vec!["-e42"]),
        ("e3", "e321", vec!["e435"]),
        ("e3", "e412", vec!["e4"]),
        ("e3", "e431", vec!["e423"]),
        ("e3", "e423", vec!["-e431"]),
        ("e3", "e1234", vec!["-e43"]),
        ("e3", "e5", vec!["e125"]),
        ("e3", "e15", vec!["e25"]),
        ("e3", "e25", vec!["-e15"]),
        ("e3", "e35", vec!["-e3215"]),
        ("e3", "e45", vec!["-e12"]),
        ("e3", "e235", vec!["e315"]),
        ("e3", "e315", vec!["-e235"]),
        ("e3", "e125", vec!["-e5"]),
        ("e3", "e435", vec!["-e321"]),
        ("e3", "e425", vec!["-e1"]),
        ("e3", "e415", vec!["e2"]),
        ("e3", "e3215", vec!["e35"]),
        ("e3", "e4125", vec!["1"]),
        ("e3", "e4315", vec!["e23"]),
        ("e3", "e4235", vec!["-e31"]),
        ("e3", "e12345", vec!["e3"]),
        ("e4", "1", vec!["e1234"]),
        ("e4", "e1", vec!["e423"]),
        ("e4", "e2", vec!["e431"]),
        ("e4", "e3", vec!["e412"]),
        ("e4", "e4", vec![]),
        ("e4", "e23", vec!["e41"]),
        ("e4", "e31", vec!["e42"]),
        ("e4", "e12", vec!["e43"]),
        ("e4", "e43", vec![]),
        ("e4", "e42", vec![]),
        ("e4", "e41", vec![]),
        ("e4", "e321", vec!["e4"]),
        ("e4", "e412", vec![]),
        ("e4", "e431", vec![]),
        ("e4", "e423", vec![]),
        ("e4", "e1234", vec![]),
        ("e4", "e5", vec!["e12345", "-e321"]),
        ("e4", "e15", vec!["e23", "e4235"]),
        ("e4", "e25", vec!["e31", "e4315"]),
        ("e4", "e35", vec!["e12", "e4125"]),
        ("e4", "e45", vec!["-e1234"]),
        ("e4", "e235", vec!["e415", "-e1"]),
        ("e4", "e315", vec!["e425", "-e2"]),
        ("e4", "e125", vec!["e435", "-e3"]),
        ("e4", "e435", vec!["e412"]),
        ("e4", "e425", vec!["e431"]),
        ("e4", "e415", vec!["e423"]),
        ("e4", "e3215", vec!["1", "e45"]),
        ("e4", "e4125", vec!["-e43"]),
        ("e4", "e4315", vec!["-e42"]),
        ("e4", "e4235", vec!["-e41"]),
        ("e4", "e12345", vec!["e4"]),
        ("e23", "1", vec!["-e415"]),
        ("e23", "e1", vec!["e45"]),
        ("e23", "e2", vec!["-e4125"]),
        ("e23", "e3", vec!["e4315"]),
        ("e23", "e4", vec!["e41"]),
        ("e23", "e23", vec!["e12345"]),
        ("e23", "e31", vec!["-e435"]),
        ("e23", "e12", vec!["e425"]),
        ("e23", "e43", vec!["e431"]),
        ("e23", "e42", vec!["-e412"]),
        ("e23", "e41", vec!["e4"]),
        ("e23", "e321", vec!["-e4235"]),
        ("e23", "e412", vec!["-e42"]),
        ("e23", "e431", vec!["e43"]),
        ("e23", "e423", vec!["-e1234"]),
        ("e23", "e1234", vec!["-e423"]),
        ("e23", "e5", vec!["e15"]),
        ("e23", "e15", vec!["e5"]),
        ("e23", "e25", vec!["-e125"]),
        ("e23", "e35", vec!["e315"]),
        ("e23", "e45", vec!["e1"]),
        ("e23", "e235", vec!["-e3215"]),
        ("e23", "e315", vec!["e35"]),
        ("e23", "e125", vec!["-e25"]),
        ("e23", "e435", vec!["-e31"]),
        ("e23", "e425", vec!["e12"]),
        ("e23", "e415", vec!["-1"]),
        ("e23", "e3215", vec!["-e235"]),
        ("e23", "e4125", vec!["-e2"]),
        ("e23", "e4315", vec!["e3"]),
        ("e23", "e4235", vec!["-e321"]),
        ("e23", "e12345", vec!["e23"]),
        ("e31", "1", vec!["-e425"]),
        ("e31", "e1", vec!["e4125"]),
        ("e31", "e2", vec!["e45"]),
        ("e31", "e3", vec!["-e4235"]),
        ("e31", "e4", vec!["e42"]),
        ("e31", "e23", vec!["e435"]),
        ("e31", "e31", vec!["e12345"]),
        ("e31", "e12", vec!["-e415"]),
        ("e31", "e43", vec!["-e423"]),
        ("e31", "e42", vec!["e4"]),
        ("e31", "e41", vec!["e412"]),
        ("e31", "e321", vec!["-e4315"]),
        ("e31", "e412", vec!["e41"]),
        ("e31", "e431", vec!["-e1234"]),
        ("e31", "e423", vec!["-e43"]),
        ("e31", "e1234", vec!["-e431"]),
        ("e31", "e5", vec!["e25"]),
        ("e31", "e15", vec!["e125"]),
        ("e31", "e25", vec!["e5"]),
        ("e31", "e35", vec!["-e235"]),
        ("e31", "e45", vec!["e2"]),
        ("e31", "e235", vec!["-e35"]),
        ("e31", "e315", vec!["-e3215"]),
        ("e31", "e125", vec!["e15"]),
        ("e31", "e435", vec!["e23"]),
        ("e31", "e425", vec!["-1"]),
        ("e31", "e415", vec!["-e12"]),
        ("e31", "e3215", vec!["-e315"]),
        ("e31", "e4125", vec!["e1"]),
        ("e31", "e4315", vec!["-e321"]),
        ("e31", "e4235", vec!["-e3"]),
        ("e31", "e12345", vec!["e31"]),
        ("e12", "1", vec!["-e435"]),
        ("e12", "e1", vec!["-e4315"]),
        ("e12", "e2", vec!["e4235"]),
        ("e12", "e3", vec!["e45"]),
        ("e12", "e4", vec!["e43"]),
        ("e12", "e23", vec!["-e425"]),
        ("e12", "e31", vec!["e415"]),
        ("e12", "e12", vec!["e12345"]),
        ("e12", "e43", vec!["e4"]),
        ("e12", "e42", vec!["e423"]),
        ("e12", "e41", vec!["-e431"]),
        ("e12", "e321", vec!["-e4125"]),
        ("e12", "e412", vec!["-e1234"]),
        ("e12", "e431", vec!["-e41"]),
        ("e12", "e423", vec!["e42"]),
        ("e12", "e1234", vec!["-e412"]),
        ("e12", "e5", vec!["e35"]),
        ("e12", "e15", vec!["-e315"]),
        ("e12", "e25", vec!["e235"]),
        ("e12", "e35", vec!["e5"]),
        ("e12", "e45", vec!["e3"]),
        ("e12", "e235", vec!["e25"]),
        ("e12", "e315", vec!["-e15"]),
        ("e12", "e125", vec!["-e3215"]),
        ("e12", "e435", vec!["-1"]),
        ("e12", "e425", vec!["-e23"]),
        ("e12", "e415", vec!["e31"]),
        ("e12", "e3215", vec!["-e125"]),
        ("e12", "e4125", vec!["-e321"]),
        ("e12", "e4315", vec!["-e1"]),
        ("e12", "e4235", vec!["e2"]),
        ("e12", "e12345", vec!["e12"]),
        ("e43", "1", vec!["-e412"]),
        ("e43", "e1", vec!["e42"]),
        ("e43", "e2", vec!["-e41"]),
        ("e43", "e3", vec!["-e1234"]),
        ("e43", "e4", vec![]),
        ("e43", "e23", vec!["-e431"]),
        ("e43", "e31", vec!["e423"]),
        ("e43", "e12", vec!["e4"]),
        ("e43", "e43", vec![]),
        ("e43", "e42", vec![]),
        ("e43", "e41", vec![]),
        ("e43", "e321", vec!["e43"]),
        ("e43", "e412", vec![]),
        ("e43", "e431", vec![]),
        ("e43", "e423", vec![]),
        ("e43", "e1234", vec![]),
        ("e43", "e5", vec!["e12", "e4125"]),
        ("e43", "e15", vec!["e2", "-e425"]),
        ("e43", "e25", vec!["e415", "-e1"]),
        ("e43", "e35", vec!["e12345", "-e321"]),
        ("e43", "e45", vec!["e412"]),
        ("e43", "e235", vec!["e31", "e4315"]),
        ("e43", "e315", vec!["-e23", "-e4235"]),
        ("e43", "e125", vec!["-1", "-e45"]),
        ("e43", "e435", vec!["-e1234"]),
        ("e43", "e425", vec!["-e41"]),
        ("e43", "e415", vec!["e42"]),
        ("e43", "e3215", vec!["e3", "-e435"]),
        ("e43", "e4125", vec!["-e4"]),
        ("e43", "e4315", vec!["-e423"]),
        ("e43", "e4235", vec!["e431"]),
        ("e43", "e12345", vec!["e43"]),
        ("e42", "1", vec!["-e431"]),
        ("e42", "e1", vec!["-e43"]),
        ("e42", "e2", vec!["-e1234"]),
        ("e42", "e3", vec!["e41"]),
        ("e42", "e4", vec![]),
        ("e42", "e23", vec!["e412"]),
        ("e42", "e31", vec!["e4"]),
        ("e42", "e12", vec!["-e423"]),
        ("e42", "e43", vec![]),
        ("e42", "e42", vec![]),
        ("e42", "e41", vec![]),
        ("e42", "e321", vec!["e42"]),
        ("e42", "e412", vec![]),
        ("e42", "e431", vec![]),
        ("e42", "e423", vec![]),
        ("e42", "e1234", vec![]),
        ("e42", "e5", vec!["e31", "e4315"]),
        ("e42", "e15", vec!["e435", "-e3"]),
        ("e42", "e25", vec!["e12345", "-e321"]),
        ("e42", "e35", vec!["e1", "-e415"]),
        ("e42", "e45", vec!["e431"]),
        ("e42", "e235", vec!["-e12", "-e4125"]),
        ("e42", "e315", vec!["-1", "-e45"]),
        ("e42", "e125", vec!["e23", "e4235"]),
        ("e42", "e435", vec!["e41"]),
        ("e42", "e425", vec!["-e1234"]),
        ("e42", "e415", vec!["-e43"]),
        ("e42", "e3215", vec!["e2", "-e425"]),
        ("e42", "e4125", vec!["e423"]),
        ("e42", "e4315", vec!["-e4"]),
        ("e42", "e4235", vec!["-e412"]),
        ("e42", "e12345", vec!["e42"]),
        ("e41", "1", vec!["-e423"]),
        ("e41", "e1", vec!["-e1234"]),
        ("e41", "e2", vec!["e43"]),
        ("e41", "e3", vec!["-e42"]),
        ("e41", "e4", vec![]),
        ("e41", "e23", vec!["e4"]),
        ("e41", "e31", vec!["-e412"]),
        ("e41", "e12", vec!["e431"]),
        ("e41", "e43", vec![]),
        ("e41", "e42", vec![]),
        ("e41", "e41", vec![]),
        ("e41", "e321", vec!["e41"]),
        ("e41", "e412", vec![]),
        ("e41", "e431", vec![]),
        ("e41", "e423", vec![]),
        ("e41", "e1234", vec![]),
        ("e41", "e5", vec!["e23", "e4235"]),
        ("e41", "e15", vec!["e12345", "-e321"]),
        ("e41", "e25", vec!["e3", "-e435"]),
        ("e41", "e35", vec!["e425", "-e2"]),
        ("e41", "e45", vec!["e423"]),
        ("e41", "e235", vec!["-1", "-e45"]),
        ("e41", "e315", vec!["e12", "e4125"]),
        ("e41", "e125", vec!["-e31", "-e4315"]),
        ("e41", "e435", vec!["-e42"]),
        ("e41", "e425", vec!["e43"]),
        ("e41", "e415", vec!["-e1234"]),
        ("e41", "e3215", vec!["e1", "-e415"]),
        ("e41", "e4125", vec!["-e431"]),
        ("e41", "e4315", vec!["e412"]),
        ("e41", "e4235", vec!["-e4"]),
        ("e41", "e12345", vec!["e41"]),
        ("e321", "1", vec!["-e45"]),
        ("e321", "e1", vec!["e415"]),
        ("e321", "e2", vec!["e425"]),
        ("e321", "e3", vec!["e435"]),
        ("e321", "e4", vec!["-e4"]),
        ("e321", "e23", vec!["-e4235"]),
        ("e321", "e31", vec!["-e4315"]),
        ("e321", "e12", vec!["-e4125"]),
        ("e321", "e43", vec!["-e43"]),
        ("e321", "e42", vec!["-e42"]),
        ("e321", "e41", vec!["-e41"]),
        ("e321", "e321", vec!["e12345"]),
        ("e321", "e412", vec!["-e412"]),
        ("e321", "e431", vec!["-e431"]),
        ("e321", "e423", vec!["-e423"]),
        ("e321", "e1234", vec!["-e1234"]),
        ("e321", "e5", vec!["e5"]),
        ("e321", "e15", vec!["e15"]),
        ("e321", "e25", vec!["e25"]),
        ("e321", "e35", vec!["e35"]),
        ("e321", "e45", vec!["-1"]),
        ("e321", "e235", vec!["e235"]),
        ("e321", "e315", vec!["e315"]),
        ("e321", "e125", vec!["e125"]),
        ("e321", "e435", vec!["e3"]),
        ("e321", "e425", vec!["e2"]),
        ("e321", "e415", vec!["e1"]),
        ("e321", "e3215", vec!["e3215"]),
        ("e321", "e4125", vec!["-e12"]),
        ("e321", "e4315", vec!["-e31"]),
        ("e321", "e4235", vec!["-e23"]),
        ("e321", "e12345", vec!["e321"]),
        ("e412", "1", vec!["e43"]),
        ("e412", "e1", vec!["e431"]),
        ("e412", "e2", vec!["-e423"]),
        ("e412", "e3", vec!["-e4"]),
        ("e412", "e4", vec![]),
        ("e412", "e23", vec!["e42"]),
        ("e412", "e31", vec!["-e41"]),
        ("e412", "e12", vec!["-e1234"]),
        ("e412", "e43", vec![]),
        ("e412", "e42", vec![]),
        ("e412", "e41", vec![]),
        ("e412", "e321", vec!["e412"]),
        ("e412", "e412", vec![]),
        ("e412", "e431", vec![]),
        ("e412", "e423", vec![]),
        ("e412", "e1234", vec![]),
        ("e412", "e5", vec!["e435", "-e3"]),
        ("e412", "e15", vec!["e31", "e4315"]),
        ("e412", "e25", vec!["-e23", "-e4235"]),
        ("e412", "e35", vec!["-1", "-e45"]),
        ("e412", "e45", vec!["-e43"]),
        ("e412", "e235", vec!["e425", "-e2"]),
        ("e412", "e315", vec!["e1", "-e415"]),
        ("e412", "e125", vec!["e321", "-e12345"]),
        ("e412", "e435", vec!["-e4"]),
        ("e412", "e425", vec!["-e423"]),
        ("e412", "e415", vec!["e431"]),
        ("e412", "e3215", vec!["e12", "e4125"]),
        ("e412", "e4125", vec!["e1234"]),
        ("e412", "e4315", vec!["e41"]),
        ("e412", "e4235", vec!["-e42"]),
        ("e412", "e12345", vec!["e412"]),
        ("e431", "1", vec!["e42"]),
        ("e431", "e1", vec!["-e412"]),
        ("e431", "e2", vec!["-e4"]),
        ("e431", "e3", vec!["e423"]),
        ("e431", "e4", vec![]),
        ("e431", "e23", vec!["-e43"]),
        ("e431", "e31", vec!["-e1234"]),
        ("e431", "e12", vec!["e41"]),
        ("e431", "e43", vec![]),
        ("e431", "e42", vec![]),
        ("e431", "e41", vec![]),
        ("e431", "e321", vec!["e431"]),
        ("e431", "e412", vec![]),
        ("e431", "e431", vec![]),
        ("e431", "e423", vec![]),
        ("e431", "e1234", vec![]),
        ("e431", "e5", vec!["e425", "-e2"]),
        ("e431", "e15", vec!["-e12", "-e4125"]),
        ("e431", "e25", vec!["-1", "-e45"]),
        ("e431", "e35", vec!["e23", "e4235"]),
        ("e431", "e45", vec!["-e42"]),
        ("e431", "e235", vec!["e3", "-e435"]),
        ("e431", "e315", vec!["e321", "-e12345"]),
        ("e431", "e125", vec!["e415", "-e1"]),
        ("e431", "e435", vec!["e423"]),
        ("e431", "e425", vec!["-e4"]),
        ("e431", "e415", vec!["-e412"]),
        ("e431", "e3215", vec!["e31", "e4315"]),
        ("e431", "e4125", vec!["-e41"]),
        ("e431", "e4315", vec!["e1234"]),
        ("e431", "e4235", vec!["e43"]),
        ("e431", "e12345", vec!["e431"]),
        ("e423", "1", vec!["e41"]),
        ("e423", "e1", vec!["-e4"]),
        ("e423", "e2", vec!["e412"]),
        ("e423", "e3", vec!["-e431"]),
        ("e423", "e4", vec![]),
        ("e423", "e23", vec!["-e1234"]),
        ("e423", "e31", vec!["e43"]),
        ("e423", "e12", vec!["-e42"]),
        ("e423", "e43", vec![]),
        ("e423", "e42", vec![]),
        ("e423", "e41", vec![]),
        ("e423", "e321", vec!["e423"]),
        ("e423", "e412", vec![]),
        ("e423", "e431", vec![]),
        ("e423", "e423", vec![]),
        ("e423", "e1234", vec![]),
        ("e423", "e5", vec!["e415", "-e1"]),
        ("e423", "e15", vec!["-1", "-e45"]),
        ("e423", "e25", vec!["e12", "e4125"]),
        ("e423", "e35", vec!["-e31", "-e4315"]),
        ("e423", "e45", vec!["-e41"]),
        ("e423", "e235", vec!["e321", "-e12345"]),
        ("e423", "e315", vec!["e435", "-e3"]),
        ("e423", "e125", vec!["e2", "-e425"]),
        ("e423", "e435", vec!["-e431"]),
        ("e423", "e425", vec!["e412"]),
        ("e423", "e415", vec!["-e4"]),
        ("e423", "e3215", vec!["e23", "e4235"]),
        ("e423", "e4125", vec!["e42"]),
        ("e423", "e4315", vec!["-e43"]),
        ("e423", "e4235", vec!["e1234"]),
        ("e423", "e12345", vec!["e423"]),
        ("e1234", "1", vec!["-e4"]),
        ("e1234", "e1", vec!["e41"]),
        ("e1234", "e2", vec!["e42"]),
        ("e1234", "e3", vec!["e43"]),
        ("e1234", "e4", vec![]),
        ("e1234", "e23", vec!["-e423"]),
        ("e1234", "e31", vec!["-e431"]),
        ("e1234", "e12", vec!["-e412"]),
        ("e1234", "e43", vec![]),
        ("e1234", "e42", vec![]),
        ("e1234", "e41", vec![]),
        ("e1234", "e321", vec!["e1234"]),
        ("e1234", "e412", vec![]),
        ("e1234", "e431", vec![]),
        ("e1234", "e423", vec![]),
        ("e1234", "e1234", vec![]),
        ("e1234", "e5", vec!["1", "e45"]),
        ("e1234", "e15", vec!["e1", "-e415"]),
        ("e1234", "e25", vec!["e2", "-e425"]),
        ("e1234", "e35", vec!["e3", "-e435"]),
        ("e1234", "e45", vec!["e4"]),
        ("e1234", "e235", vec!["e23", "e4235"]),
        ("e1234", "e315", vec!["e31", "e4315"]),
        ("e1234", "e125", vec!["e12", "e4125"]),
        ("e1234", "e435", vec!["e43"]),
        ("e1234", "e425", vec!["e42"]),
        ("e1234", "e415", vec!["e41"]),
        ("e1234", "e3215", vec!["e321", "-e12345"]),
        ("e1234", "e4125", vec!["e412"]),
        ("e1234", "e4315", vec!["e431"]),
        ("e1234", "e4235", vec!["e423"]),
        ("e1234", "e12345", vec!["e1234"]),
        ("e5", "1", vec!["e3215"]),
        ("e5", "e1", vec!["-e235"]),
        ("e5", "e2", vec!["-e315"]),
        ("e5", "e3", vec!["-e125"]),
        ("e5", "e4", vec!["e321", "e12345"]),
        ("e5", "e23", vec!["e15"]),
        ("e5", "e31", vec!["e25"]),
        ("e5", "e12", vec!["e35"]),
        ("e5", "e43", vec!["e12", "-e4125"]),
        ("e5", "e42", vec!["e31", "-e4315"]),
        ("e5", "e41", vec!["e23", "-e4235"]),
        ("e5", "e321", vec!["-e5"]),
        ("e5", "e412", vec!["e3", "e435"]),
        ("e5", "e431", vec!["e2", "e425"]),
        ("e5", "e423", vec!["e1", "e415"]),
        ("e5", "e1234", vec!["1", "-e45"]),
        ("e5", "e5", vec![]),
        ("e5", "e15", vec![]),
        ("e5", "e25", vec![]),
        ("e5", "e35", vec![]),
        ("e5", "e45", vec!["e3215"]),
        ("e5", "e235", vec![]),
        ("e5", "e315", vec![]),
        ("e5", "e125", vec![]),
        ("e5", "e435", vec!["e125"]),
        ("e5", "e425", vec!["e315"]),
        ("e5", "e415", vec!["e235"]),
        ("e5", "e3215", vec![]),
        ("e5", "e4125", vec!["e35"]),
        ("e5", "e4315", vec!["e25"]),
        ("e5", "e4235", vec!["e15"]),
        ("e5", "e12345", vec!["e5"]),
        ("e15", "1", vec!["-e235"]),
        ("e15", "e1", vec!["e3215"]),
        ("e15", "e2", vec!["-e35"]),
        ("e15", "e3", vec!["e25"]),
        ("e15", "e4", vec!["e23", "-e4235"]),
        ("e15", "e23", vec!["e5"]),
        ("e15", "e31", vec!["-e125"]),
        ("e15", "e12", vec!["e315"]),
        ("e15", "e43", vec!["e2", "e425"]),
        ("e15", "e42", vec!["-e3", "-e435"]),
        ("e15", "e41", vec!["e321", "e12345"]),
        ("e15", "e321", vec!["-e15"]),
        ("e15", "e412", vec!["e4315", "-e31"]),
        ("e15", "e431", vec!["e12", "-e4125"]),
        ("e15", "e423", vec!["e45", "-1"]),
        ("e15", "e1234", vec!["-e1", "-e415"]),
        ("e15", "e5", vec![]),
        ("e15", "e15", vec![]),
        ("e15", "e25", vec![]),
        ("e15", "e35", vec![]),
        ("e15", "e45", vec!["-e235"]),
        ("e15", "e235", vec![]),
        ("e15", "e315", vec![]),
        ("e15", "e125", vec![]),
        ("e15", "e435", vec!["-e25"]),
        ("e15", "e425", vec!["e35"]),
        ("e15", "e415", vec!["-e3215"]),
        ("e15", "e3215", vec![]),
        ("e15", "e4125", vec!["e315"]),
        ("e15", "e4315", vec!["-e125"]),
        ("e15", "e4235", vec!["e5"]),
        ("e15", "e12345", vec!["e15"]),
        ("e25", "1", vec!["-e315"]),
        ("e25", "e1", vec!["e35"]),
        ("e25", "e2", vec!["e3215"]),
        ("e25", "e3", vec!["-e15"]),
        ("e25", "e4", vec!["e31", "-e4315"]),
        ("e25", "e23", vec!["e125"]),
        ("e25", "e31", vec!["e5"]),
        ("e25", "e12", vec!["-e235"]),
        ("e25", "e43", vec!["-e1", "-e415"]),
        ("e25", "e42", vec!["e321", "e12345"]),
        ("e25", "e41", vec!["e3", "e435"]),
        ("e25", "e321", vec!["-e25"]),
        ("e25", "e412", vec!["e23", "-e4235"]),
        ("e25", "e431", vec!["e45", "-1"]),
        ("e25", "e423", vec!["e4125", "-e12"]),
        ("e25", "e1234", vec!["-e2", "-e425"]),
        ("e25", "e5", vec![]),
        ("e25", "e15", vec![]),
        ("e25", "e25", vec![]),
        ("e25", "e35", vec![]),
        ("e25", "e45", vec!["-e315"]),
        ("e25", "e235", vec![]),
        ("e25", "e315", vec![]),
        ("e25", "e125", vec![]),
        ("e25", "e435", vec!["e15"]),
        ("e25", "e425", vec!["-e3215"]),
        ("e25", "e415", vec!["-e35"]),
        ("e25", "e3215", vec![]),
        ("e25", "e4125", vec!["-e235"]),
        ("e25", "e4315", vec!["e5"]),
        ("e25", "e4235", vec!["e125"]),
        ("e25", "e12345", vec!["e25"]),
        ("e35", "1", vec!["-e125"]),
        ("e35", "e1", vec!["-e25"]),
        ("e35", "e2", vec!["e15"]),
        ("e35", "e3", vec!["e3215"]),
        ("e35", "e4", vec!["e12", "-e4125"]),
        ("e35", "e23", vec!["-e315"]),
        ("e35", "e31", vec!["e235"]),
        ("e35", "e12", vec!["e5"]),
        ("e35", "e43", vec!["e321", "e12345"]),
        ("e35", "e42", vec!["e1", "e415"]),
        ("e35", "e41", vec!["-e2", "-e425"]),
        ("e35", "e321", vec!["-e35"]),
        ("e35", "e412", vec!["e45", "-1"]),
        ("e35", "e431", vec!["e4235", "-e23"]),
        ("e35", "e423", vec!["e31", "-e4315"]),
        ("e35", "e1234", vec!["-e3", "-e435"]),
        ("e35", "e5", vec![]),
        ("e35", "e15", vec![]),
        ("e35", "e25", vec![]),
        ("e35", "e35", vec![]),
        ("e35", "e45", vec!["-e125"]),
        ("e35", "e235", vec![]),
        ("e35", "e315", vec![]),
        ("e35", "e125", vec![]),
        ("e35", "e435", vec!["-e3215"]),
        ("e35", "e425", vec!["-e15"]),
        ("e35", "e415", vec!["e25"]),
        ("e35", "e3215", vec![]),
        ("e35", "e4125", vec!["e5"]),
        ("e35", "e4315", vec!["e235"]),
        ("e35", "e4235", vec!["-e315"]),
        ("e35", "e12345", vec!["e35"]),
        ("e45", "1", vec!["e321"]),
        ("e45", "e1", vec!["-e23"]),
        ("e45", "e2", vec!["-e31"]),
        ("e45", "e3", vec!["-e12"]),
        ("e45", "e4", vec!["e1234"]),
        ("e45", "e23", vec!["e1"]),
        ("e45", "e31", vec!["e2"]),
        ("e45", "e12", vec!["e3"]),
        ("e45", "e43", vec!["-e412"]),
        ("e45", "e42", vec!["-e431"]),
        ("e45", "e41", vec!["-e423"]),
        ("e45", "e321", vec!["-1"]),
        ("e45", "e412", vec!["e43"]),
        ("e45", "e431", vec!["e42"]),
        ("e45", "e423", vec!["e41"]),
        ("e45", "e1234", vec!["-e4"]),
        ("e45", "e5", vec!["-e3215"]),
        ("e45", "e15", vec!["e235"]),
        ("e45", "e25", vec!["e315"]),
        ("e45", "e35", vec!["e125"]),
        ("e45", "e45", vec!["-e12345"]),
        ("e45", "e235", vec!["-e15"]),
        ("e45", "e315", vec!["-e25"]),
        ("e45", "e125", vec!["-e35"]),
        ("e45", "e435", vec!["e4125"]),
        ("e45", "e425", vec!["e4315"]),
        ("e45", "e415", vec!["e4235"]),
        ("e45", "e3215", vec!["e5"]),
        ("e45", "e4125", vec!["-e435"]),
        ("e45", "e4315", vec!["-e425"]),
        ("e45", "e4235", vec!["-e415"]),
        ("e45", "e12345", vec!["e45"]),
        ("e235", "1", vec!["e15"]),
        ("e235", "e1", vec!["e5"]),
        ("e235", "e2", vec!["-e125"]),
        ("e235", "e3", vec!["e315"]),
        ("e235", "e4", vec!["e1", "e415"]),
        ("e235", "e23", vec!["-e3215"]),
        ("e235", "e31", vec!["e35"]),
        ("e235", "e12", vec!["-e25"]),
        ("e235", "e43", vec!["e4315", "-e31"]),
        ("e235", "e42", vec!["e12", "-e4125"]),
        ("e235", "e41", vec!["e45", "-1"]),
        ("e235", "e321", vec!["-e235"]),
        ("e235", "e412", vec!["-e2", "-e425"]),
        ("e235", "e431", vec!["e3", "e435"]),
        ("e235", "e423", vec!["-e321", "-e12345"]),
        ("e235", "e1234", vec!["e23", "-e4235"]),
        ("e235", "e5", vec![]),
        ("e235", "e15", vec![]),
        ("e235", "e25", vec![]),
        ("e235", "e35", vec![]),
        ("e235", "e45", vec!["e15"]),
        ("e235", "e235", vec![]),
        ("e235", "e315", vec![]),
        ("e235", "e125", vec![]),
        ("e235", "e435", vec!["-e315"]),
        ("e235", "e425", vec!["e125"]),
        ("e235", "e415", vec!["-e5"]),
        ("e235", "e3215", vec![]),
        ("e235", "e4125", vec!["-e25"]),
        ("e235", "e4315", vec!["e35"]),
        ("e235", "e4235", vec!["-e3215"]),
        ("e235", "e12345", vec!["e235"]),
        ("e315", "1", vec!["e25"]),
        ("e315", "e1", vec!["e125"]),
        ("e315", "e2", vec!["e5"]),
        ("e315", "e3", vec!["-e235"]),
        ("e315", "e4", vec!["e2", "e425"]),
        ("e315", "e23", vec!["-e35"]),
        ("e315", "e31", vec!["-e3215"]),
        ("e315", "e12", vec!["e15"]),
        ("e315", "e43", vec!["e23", "-e4235"]),
        ("e315", "e42", vec!["e45", "-1"]),
        ("e315", "e41", vec!["e4125", "-e12"]),
        ("e315", "e321", vec!["-e315"]),
        ("e315", "e412", vec!["e1", "e415"]),
        ("e315", "e431", vec!["-e321", "-e12345"]),
        ("e315", "e423", vec!["-e3", "-e435"]),
        ("e315", "e1234", vec!["e31", "-e4315"]),
        ("e315", "e5", vec![]),
        ("e315", "e15", vec![]),
        ("e315", "e25", vec![]),
        ("e315", "e35", vec![]),
        ("e315", "e45", vec!["e25"]),
        ("e315", "e235", vec![]),
        ("e315", "e315", vec![]),
        ("e315", "e125", vec![]),
        ("e315", "e435", vec!["e235"]),
        ("e315", "e425", vec!["-e5"]),
        ("e315", "e415", vec!["-e125"]),
        ("e315", "e3215", vec![]),
        ("e315", "e4125", vec!["e15"]),
        ("e315", "e4315", vec!["-e3215"]),
        ("e315", "e4235", vec!["-e35"]),
        ("e315", "e12345", vec!["e315"]),
        ("e125", "1", vec!["e35"]),
        ("e125", "e1", vec!["-e315"]),
        ("e125", "e2", vec!["e235"]),
        ("e125", "e3", vec!["e5"]),
        ("e125", "e4", vec!["e3", "e435"]),
        ("e125", "e23", vec!["e25"]),
        ("e125", "e31", vec!["-e15"]),
        ("e125", "e12", vec!["-e3215"]),
        ("e125", "e43", vec!["e45", "-1"]),
        ("e125", "e42", vec!["e4235", "-e23"]),
        ("e125", "e41", vec!["e31", "-e4315"]),
        ("e125", "e321", vec!["-e125"]),
        ("e125", "e412", vec!["-e321", "-e12345"]),
        ("e125", "e431", vec!["-e1", "-e415"]),
        ("e125", "e423", vec!["e2", "e425"]),
        ("e125", "e1234", vec!["e12", "-e4125"]),
        ("e125", "e5", vec![]),
        ("e125", "e15", vec![]),
        ("e125", "e25", vec![]),
        ("e125", "e35", vec![]),
        ("e125", "e45", vec!["e35"]),
        ("e125", "e235", vec![]),
        ("e125", "e315", vec![]),
        ("e125", "e125", vec![]),
        ("e125", "e435", vec!["-e5"]),
        ("e125", "e425", vec!["-e235"]),
        ("e125", "e415", vec!["e315"]),
        ("e125", "e3215", vec![]),
        ("e125", "e4125", vec!["-e3215"]),
        ("e125", "e4315", vec!["-e15"]),
        ("e125", "e4235", vec!["e25"]),
        ("e125", "e12345", vec!["e125"]),
        ("e435", "1", vec!["e12"]),
        ("e435", "e1", vec!["e2"]),
        ("e435", "e2", vec!["-e1"]),
        ("e435", "e3", vec!["-e321"]),
        ("e435", "e4", vec!["e412"]),
        ("e435", "e23", vec!["e31"]),
        ("e435", "e31", vec!["-e23"]),
        ("e435", "e12", vec!["-1"]),
        ("e435", "e43", vec!["-e1234"]),
        ("e435", "e42", vec!["-e41"]),
        ("e435", "e41", vec!["e42"]),
        ("e435", "e321", vec!["e3"]),
        ("e435", "e412", vec!["-e4"]),
        ("e435", "e431", vec!["-e423"]),
        ("e435", "e423", vec!["e431"]),
        ("e435", "e1234", vec!["e43"]),
        ("e435", "e5", vec!["e125"]),
        ("e435", "e15", vec!["e25"]),
        ("e435", "e25", vec!["-e15"]),
        ("e435", "e35", vec!["-e3215"]),
        ("e435", "e45", vec!["e4125"]),
        ("e435", "e235", vec!["e315"]),
        ("e435", "e315", vec!["-e235"]),
        ("e435", "e125", vec!["-e5"]),
        ("e435", "e435", vec!["-e12345"]),
        ("e435", "e425", vec!["-e415"]),
        ("e435", "e415", vec!["e425"]),
        ("e435", "e3215", vec!["e35"]),
        ("e435", "e4125", vec!["-e45"]),
        ("e435", "e4315", vec!["-e4235"]),
        ("e435", "e4235", vec!["e4315"]),
        ("e435", "e12345", vec!["e435"]),
        ("e425", "1", vec!["e31"]),
        ("e425", "e1", vec!["-e3"]),
        ("e425", "e2", vec!["-e321"]),
        ("e425", "e3", vec!["e1"]),
        ("e425", "e4", vec!["e431"]),
        ("e425", "e23", vec!["-e12"]),
        ("e425", "e31", vec!["-1"]),
        ("e425", "e12", vec!["e23"]),
        ("e425", "e43", vec!["e41"]),
        ("e425", "e42", vec!["-e1234"]),
        ("e425", "e41", vec!["-e43"]),
        ("e425", "e321", vec!["e2"]),
        ("e425", "e412", vec!["e423"]),
        ("e425", "e431", vec!["-e4"]),
        ("e425", "e423", vec!["-e412"]),
        ("e425", "e1234", vec!["e42"]),
        ("e425", "e5", vec!["e315"]),
        ("e425", "e15", vec!["-e35"]),
        ("e425", "e25", vec!["-e3215"]),
        ("e425", "e35", vec!["e15"]),
        ("e425", "e45", vec!["e4315"]),
        ("e425", "e235", vec!["-e125"]),
        ("e425", "e315", vec!["-e5"]),
        ("e425", "e125", vec!["e235"]),
        ("e425", "e435", vec!["e415"]),
        ("e425", "e425", vec!["-e12345"]),
        ("e425", "e415", vec!["-e435"]),
        ("e425", "e3215", vec!["e25"]),
        ("e425", "e4125", vec!["e4235"]),
        ("e425", "e4315", vec!["-e45"]),
        ("e425", "e4235", vec!["-e4125"]),
        ("e425", "e12345", vec!["e425"]),
        ("e415", "1", vec!["e23"]),
        ("e415", "e1", vec!["-e321"]),
        ("e415", "e2", vec!["e3"]),
        ("e415", "e3", vec!["-e2"]),
        ("e415", "e4", vec!["e423"]),
        ("e415", "e23", vec!["-1"]),
        ("e415", "e31", vec!["e12"]),
        ("e415", "e12", vec!["-e31"]),
        ("e415", "e43", vec!["-e42"]),
        ("e415", "e42", vec!["e43"]),
        ("e415", "e41", vec!["-e1234"]),
        ("e415", "e321", vec!["e1"]),
        ("e415", "e412", vec!["-e431"]),
        ("e415", "e431", vec!["e412"]),
        ("e415", "e423", vec!["-e4"]),
        ("e415", "e1234", vec!["e41"]),
        ("e415", "e5", vec!["e235"]),
        ("e415", "e15", vec!["-e3215"]),
        ("e415", "e25", vec!["e35"]),
        ("e415", "e35", vec!["-e25"]),
        ("e415", "e45", vec!["e4235"]),
        ("e415", "e235", vec!["-e5"]),
        ("e415", "e315", vec!["e125"]),
        ("e415", "e125", vec!["-e315"]),
        ("e415", "e435", vec!["-e425"]),
        ("e415", "e425", vec!["e435"]),
        ("e415", "e415", vec!["-e12345"]),
        ("e415", "e3215", vec!["e15"]),
        ("e415", "e4125", vec!["-e4315"]),
        ("e415", "e4315", vec!["e4125"]),
        ("e415", "e4235", vec!["-e45"]),
        ("e415", "e12345", vec!["e415"]),
        ("e3215", "1", vec!["-e5"]),
        ("e3215", "e1", vec!["-e15"]),
        ("e3215", "e2", vec!["-e25"]),
        ("e3215", "e3", vec!["-e35"]),
        ("e3215", "e4", vec!["1", "-e45"]),
        ("e3215", "e23", vec!["-e235"]),
        ("e3215", "e31", vec!["-e315"]),
        ("e3215", "e12", vec!["-e125"]),
        ("e3215", "e43", vec!["-e3", "-e435"]),
        ("e3215", "e42", vec!["-e2", "-e425"]),
        ("e3215", "e41", vec!["-e1", "-e415"]),
        ("e3215", "e321", vec!["-e3215"]),
        ("e3215", "e412", vec!["e12", "-e4125"]),
        ("e3215", "e431", vec!["e31", "-e4315"]),
        ("e3215", "e423", vec!["e23", "-e4235"]),
        ("e3215", "e1234", vec!["-e321", "-e12345"]),
        ("e3215", "e5", vec![]),
        ("e3215", "e15", vec![]),
        ("e3215", "e25", vec![]),
        ("e3215", "e35", vec![]),
        ("e3215", "e45", vec!["-e5"]),
        ("e3215", "e235", vec![]),
        ("e3215", "e315", vec![]),
        ("e3215", "e125", vec![]),
        ("e3215", "e435", vec!["e35"]),
        ("e3215", "e425", vec!["e25"]),
        ("e3215", "e415", vec!["e15"]),
        ("e3215", "e3215", vec![]),
        ("e3215", "e4125", vec!["-e125"]),
        ("e3215", "e4315", vec!["-e315"]),
        ("e3215", "e4235", vec!["-e235"]),
        ("e3215", "e12345", vec!["e3215"]),
        ("e4125", "1", vec!["e3"]),
        ("e4125", "e1", vec!["-e31"]),
        ("e4125", "e2", vec!["e23"]),
        ("e4125", "e3", vec!["1"]),
        ("e4125", "e4", vec!["e43"]),
        ("e4125", "e23", vec!["e2"]),
        ("e4125", "e31", vec!["-e1"]),
        ("e4125", "e12", vec!["-e321"]),
        ("e4125", "e43", vec!["e4"]),
        ("e4125", "e42", vec!["e423"]),
        ("e4125", "e41", vec!["-e431"]),
        ("e4125", "e321", vec!["-e12"]),
        ("e4125", "e412", vec!["-e1234"]),
        ("e4125", "e431", vec!["-e41"]),
        ("e4125", "e423", vec!["e42"]),
        ("e4125", "e1234", vec!["-e412"]),
        ("e4125", "e5", vec!["-e35"]),
        ("e4125", "e15", vec!["e315"]),
        ("e4125", "e25", vec!["-e235"]),
        ("e4125", "e35", vec!["-e5"]),
        ("e4125", "e45", vec!["-e435"]),
        ("e4125", "e235", vec!["-e25"]),
        ("e4125", "e315", vec!["e15"]),
        ("e4125", "e125", vec!["e3215"]),
        ("e4125", "e435", vec!["-e45"]),
        ("e4125", "e425", vec!["-e4235"]),
        ("e4125", "e415", vec!["e4315"]),
        ("e4125", "e3215", vec!["e125"]),
        ("e4125", "e4125", vec!["e12345"]),
        ("e4125", "e4315", vec!["e415"]),
        ("e4125", "e4235", vec!["-e425"]),
        ("e4125", "e12345", vec!["e4125"]),
        ("e4315", "1", vec!["e2"]),
        ("e4315", "e1", vec!["e12"]),
        ("e4315", "e2", vec!["1"]),
        ("e4315", "e3", vec!["-e23"]),
        ("e4315", "e4", vec!["e42"]),
        ("e4315", "e23", vec!["-e3"]),
        ("e4315", "e31", vec!["-e321"]),
        ("e4315", "e12", vec!["e1"]),
        ("e4315", "e43", vec!["-e423"]),
        ("e4315", "e42", vec!["e4"]),
        ("e4315", "e41", vec!["e412"]),
        ("e4315", "e321", vec!["-e31"]),
        ("e4315", "e412", vec!["e41"]),
        ("e4315", "e431", vec!["-e1234"]),
        ("e4315", "e423", vec!["-e43"]),
        ("e4315", "e1234", vec!["-e431"]),
        ("e4315", "e5", vec!["-e25"]),
        ("e4315", "e15", vec!["-e125"]),
        ("e4315", "e25", vec!["-e5"]),
        ("e4315", "e35", vec!["e235"]),
        ("e4315", "e45", vec!["-e425"]),
        ("e4315", "e235", vec!["e35"]),
        ("e4315", "e315", vec!["e3215"]),
        ("e4315", "e125", vec!["-e15"]),
        ("e4315", "e435", vec!["e4235"]),
        ("e4315", "e425", vec!["-e45"]),
        ("e4315", "e415", vec!["-e4125"]),
        ("e4315", "e3215", vec!["e315"]),
        ("e4315", "e4125", vec!["-e415"]),
        ("e4315", "e4315", vec!["e12345"]),
        ("e4315", "e4235", vec!["e435"]),
        ("e4315", "e12345", vec!["e4315"]),
        ("e4235", "1", vec!["e1"]),
        ("e4235", "e1", vec!["1"]),
        ("e4235", "e2", vec!["-e12"]),
        ("e4235", "e3", vec!["e31"]),
        ("e4235", "e4", vec!["e41"]),
        ("e4235", "e23", vec!["-e321"]),
        ("e4235", "e31", vec!["e3"]),
        ("e4235", "e12", vec!["-e2"]),
        ("e4235", "e43", vec!["e431"]),
        ("e4235", "e42", vec!["-e412"]),
        ("e4235", "e41", vec!["e4"]),
        ("e4235", "e321", vec!["-e23"]),
        ("e4235", "e412", vec!["-e42"]),
        ("e4235", "e431", vec!["e43"]),
        ("e4235", "e423", vec!["-e1234"]),
        ("e4235", "e1234", vec!["-e423"]),
        ("e4235", "e5", vec!["-e15"]),
        ("e4235", "e15", vec!["-e5"]),
        ("e4235", "e25", vec!["e125"]),
        ("e4235", "e35", vec!["-e315"]),
        ("e4235", "e45", vec!["-e415"]),
        ("e4235", "e235", vec!["e3215"]),
        ("e4235", "e315", vec!["-e35"]),
        ("e4235", "e125", vec!["e25"]),
        ("e4235", "e435", vec!["-e4315"]),
        ("e4235", "e425", vec!["e4125"]),
        ("e4235", "e415", vec!["-e45"]),
        ("e4235", "e3215", vec!["e235"]),
        ("e4235", "e4125", vec!["e425"]),
        ("e4235", "e4315", vec!["-e435"]),
        ("e4235", "e4235", vec!["e12345"]),
        ("e4235", "e12345", vec!["e4235"]),
        ("e12345", "1", vec!["1"]),
        ("e12345", "e1", vec!["e1"]),
        ("e12345", "e2", vec!["e2"]),
        ("e12345", "e3", vec!["e3"]),
        ("e12345", "e4", vec!["e4"]),
        ("e12345", "e23", vec!["e23"]),
        ("e12345", "e31", vec!["e31"]),
        ("e12345", "e12", vec!["e12"]),
        ("e12345", "e43", vec!["e43"]),
        ("e12345", "e42", vec!["e42"]),
        ("e12345", "e41", vec!["e41"]),
        ("e12345", "e321", vec!["e321"]),
        ("e12345", "e412", vec!["e412"]),
        ("e12345", "e431", vec!["e431"]),
        ("e12345", "e423", vec!["e423"]),
        ("e12345", "e1234", vec!["e1234"]),
        ("e12345", "e5", vec!["e5"]),
        ("e12345", "e15", vec!["e15"]),
        ("e12345", "e25", vec!["e25"]),
        ("e12345", "e35", vec!["e35"]),
        ("e12345", "e45", vec!["e45"]),
        ("e12345", "e235", vec!["e235"]),
        ("e12345", "e315", vec!["e315"]),
        ("e12345", "e125", vec!["e125"]),
        ("e12345", "e435", vec!["e435"]),
        ("e12345", "e425", vec!["e425"]),
        ("e12345", "e415", vec!["e415"]),
        ("e12345", "e3215", vec!["e3215"]),
        ("e12345", "e4125", vec!["e4125"]),
        ("e12345", "e4315", vec!["e4315"]),
        ("e12345", "e4235", vec!["e4235"]),
        ("e12345", "e12345", vec!["e12345"]),
    ]};



    use crate::algebra2::basis::elements::*;
    let mut underlying_cga = GeneratorSquares::new([(E1, 1), (E2, 1), (E3, 1), (EA, 1), (EB, -1)]);
    underlying_cga.negative_anti_scalar = true;
    let mut substituted_cga = SubstitutionRepository::new(underlying_cga, vec![
        // TODO i bet I can make this use operators which would actually be very nice and appropriate
        (E4, vec![(0.5, EB), (-0.5, EA)]),
        (E5, vec![(1.0, EB), (1.0, EA)]),
    ]);



    let mut failures = 0;
    let mut correct_anti_products = BTreeMap::new();
    for (a, b, anti_products) in correct_cayley_table {
        let mut a = BasisElement::parsed_display_name(a).expect("a must parse");
        a.display_name = None;
        let mut b = BasisElement::parsed_display_name(b).expect("b must parse");
        b.display_name = None;
        let mut sum = vec![];
        for anti_product in anti_products {
            let mut element = BasisElement::parsed_display_name(anti_product).expect(format!("{anti_product} must parse").as_str());
            element.display_name = None;
            sum.push(Product { coefficient: 1.0, element });
        }
        let mut sum = Sum { sum };
        sum.sort_and_simplify();
        correct_anti_products.insert((a.signature, b.signature), (a.coefficient, b.coefficient, sum));
    }

    for a in 0..u16::pow(2, 6) {
        // skip e0
        if a & 1 == 1 {
            continue
        }

        let a_sig = BasisSignature::from_bits_retain(a);
        let mut a = BasisElement::from(a_sig);
        for b in 0..u16::pow(2, 6) {
            // skip e0
            if b & 1 == 1 {
                continue
            }

            let b_sig = BasisSignature::from_bits_retain(b);
            let mut b = BasisElement::from(b_sig);

            let (a_sign, b_sign, correct_anti_product) = correct_anti_products
                .remove(&(a_sig, b_sig))
                .unwrap_or_else(|| panic!("Cayley table must be complete, missing {a} * {b}"));

            a.coefficient = a_sign;
            b.coefficient = b_sign;

            let mut calculated_anti_product = substituted_cga.anti_product(&a, &b);
            if calculated_anti_product != correct_anti_product {
                eprintln!("{a} * {b} was calculated as {calculated_anti_product:?}, but we expected {correct_anti_product:?}");
                failures = failures + 1;
            }
        }
    }
    assert_eq!(failures, 0, "Conformal Geometric AntiProduct has {failures} errors.")
}

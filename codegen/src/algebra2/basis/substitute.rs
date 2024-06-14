use std::cmp::Ordering;
use im::HashMap;
use crate::algebra2::basis::{BasisElement, BasisSignature};
use crate::algebra2::basis::elements::scalar;
use crate::algebra2::basis::generators::{GeneratorElement, GeneratorSquares};

#[derive(Debug, Clone)]
struct Sum {
    sum: Vec<Product>
}

#[derive(Debug, Clone)]
struct Product {
    coefficient: f32,
    element: BasisElement
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
        let mut i = 0;
        let mut kept_idx = 0;
        while i < (self.sum.len() - 1) {
            let (a, b) = self.sum.split_at_mut(i + 1);
            let a = &mut a[i];
            let b = &mut b[0];
            if a.element == b.element {
                a.coefficient += b.coefficient;
                if a.coefficient != 0.0 {
                    self.sum.swap(kept_idx, i);
                    kept_idx += 1;
                }
                i += 1;
            }
            i += 1;
            // println!("Sum::sort_and_simplify -> {self:?}");
        }
        self.sum.truncate(kept_idx);
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
        c.iter().sum()
    }

    fn add(&self, other: &Sum) -> Sum {
        let mut sum = self.sum.clone();
        sum.append(&mut other.sum.clone());
        let mut s = Sum { sum };
        s.sort_and_simplify();
        s
    }
}


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
                if c == 0.0 {
                    panic!("Don't define substitution elements on underlying elements using a coefficient of zero: {sub:?}")
                }
                let u = u.element();
                underlying_to_substitutions.entry(u)
                    .and_modify(|sum| {
                        sum.sum.push(Product { coefficient: 1.0 / c, element: sub });
                        sum.sort_and_simplify();
                    })
                    .or_insert(Sum { sum: vec![Product { coefficient: c, element: sub }] })
            }
            let sum = under.into_iter().map(|(c, it)| Product { coefficient: c, element: it.element()}).collect();
            let mut sum = Sum { sum };
            sum.sort_and_simplify();
            let existing = substitutions_to_underlying.insert(sub, sum);
            assert!(existing.is_none(), "Basis substitutions must be uniquely defined, but found more than one definition for {sub}")
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
            assert_eq!(back_to_underlying.sum.len(), 1, "Substitution elements do not resolve into an independent underlying {under}.");
            assert_eq!(back_to_underlying.sum[0], under, "Substitution elements do not resolve into underlying {under} properly.");
            let c = back_to_underlying.sum[0].coefficient;
            for sub in orig_substitutions.sum.iter_mut() {
                sub.coefficient /= c;
            }
        }


        // Verify that substitution elements are orthogonal
        let mut substitutions_to_underlying: Vec<_> = substitutions_to_underlying.clone().into_iter().collect();
        for i in 0..(substitutions_to_underlying.len() - 1) {
            for j in (i + 1)..substitutions_to_underlying.len() {
                let (a, a_underlying) = &substitutions_to_underlying[i];
                let (b, b_underlying) = &substitutions_to_underlying[j];
                let dot = a_underlying.clone().superficial_dot_product(b_underlying.clone());
                assert_eq!(dot, 0.0, "Basis substitutions must be orthogonal, violated by {a} and {b}")
            }
        }


        // Start constructing higher grade elements
        let mut next_grade_elements = underlying_grade_1_elements.clone();


        let mut still_finding_wedges = true;
        while still_finding_wedges {
            still_finding_wedges = false;
            let mut next_next_grade_elements = vec![];
            for a in underlying_grade_1_elements.iter() {
                for b in next_grade_elements.iter() {
                    let a = a.clone();
                    let b = b.clone();
                    let superficial_wedge = a.wedge(*b);
                    let a_ = substitutions_to_underlying.get(&a).map(|it| it.clone())
                        .unwrap_or_else(|| Sum { sum: vec![Product { coefficient: 1.0, element: a }] });
                    let b_ = substitutions_to_underlying.get(&b).map(|it| it.clone())
                        .unwrap_or_else(|| Sum { sum: vec![Product { coefficient: 1.0, element: b }] });

                    let true_product = a_.multiply(&b_, &underlying_squares);
                    let wedge_in_product: Vec<_> = true_product.sum.iter().filter_map(|it|
                        if it.element.grade() == a.grade() + b.grade() { Some(it.clone()) } else { None }
                    ).collect();
                    if !wedge_in_product.is_empty() {
                        still_finding_wedges = true;

                        // TODO but what about more than one
                        if wedge_in_product.len() == 1 {
                            let el = &wedge_in_product[0];
                            let p = Product {
                                coefficient: 1.0 / el.coefficient,
                                element: el.element
                            };
                            underlying_to_substitutions.insert(el.element, Sum { sum: vec![p]});
                        }
                        substitutions_to_underlying.insert(superficial_wedge, Sum { sum: wedge_in_product });
                        next_next_grade_elements.push(superficial_wedge);
                    }
                }
            }
            next_grade_elements = next_next_grade_elements;
        }


        let mut substitution_products = HashMap::new();
        let mut substitution_anti_products = HashMap::new();
        Self {
            underlying_squares,
            substitutions_to_underlying,
            underlying_to_substitutions,
            substitution_products,
            substitution_anti_products
        }
    }
}



#[test]
fn try_stuff() {
    use crate::algebra2::basis::elements::*;
    let underlying_cga = GeneratorSquares::new([(E1, 1), (E2, 1), (E3, 1), (EA, 1), (EB, -1)]);

    let new_e4 = SubstituteElement {
        new_element: e4,
        depends_on_underlying: vec![eA, eB],
        expr: Sum { sum: vec![
            Product { coefficient: 0.5, element: eB, },
            Product { coefficient: -0.5, element: eA, },
        ] },
    };
    let new_e5 = SubstituteElement {
        new_element: e5,
        depends_on_underlying: vec![eA, eB],
        expr: Sum { sum: vec![
            Product { coefficient: 1.0, element: eB, },
            Product { coefficient: 1.0, element: eA, },
        ] },
    };

    // TODO this result includes a bivector element, but don't be confused.
    //  This bivector is in the underlying basis, not the outer basis.
    //  I still have to convert it to the outer basis.
    let product_e45 = new_e4.expr.multiply(&new_e5.expr, &underlying_cga);
    println!("{product_e45:?}")
}
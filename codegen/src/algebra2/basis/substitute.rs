use std::cmp::Ordering;
use crate::algebra2::basis::generators::{GeneratorElement, GeneratorSquares};

#[derive(Debug, Clone)]
struct Sum {
    sum: Vec<Product>
}

#[derive(Debug, Clone)]
struct Product {
    coefficient: f32,
    product: Vec<GeneratorElement>
}

// TODO I'm getting a hunch that using real BasisElements instead of GeneratorElements
//  might be a productive call here. Might be possible to define vectors and const eval
//  generator multivectors.
#[derive(Debug, Clone)]
pub struct SubstituteElement {
    new_element: GeneratorElement,
    depends_on_underlying: Vec<GeneratorElement>,
    expr: Sum,
}


impl Product {
    fn sort_and_simplify(&mut self, underlying: &GeneratorSquares) {
        // println!("Product::sort_and_simplify {self:?}");
        if self.coefficient == 0.0 {
            self.product = vec![];
            return
        }
        match self.product.len()  {
            // TODO zero length means scalar for now, until I refactor to use BasisElements instead
            0 => return,
            1 => return,
            _ => {}
        }

        // This is kind of like a hybrid between bubble sort
        // to detect the number of swaps and "retain"
        // to remove squared elements.
        let mut sorted = false;
        while !sorted {
            sorted = true;
            let mut kept_idx = 0;
            let mut range = 0..(self.product.len() - 1);
            while let Some(i) = range.next() {
                let a = self.product[i];
                let b = self.product[i + 1];
                let p_len = self.product.len();
                let last_comparison = p_len == i + 2;
                // println!("Product::sort_and_simplify -> {i} {p_len} {last_comparison}");
                match a.cmp(&b) {
                    Ordering::Less => {
                        self.product[kept_idx] = a;
                        kept_idx += 1;
                        if last_comparison {
                            self.product[kept_idx] = b;
                            kept_idx += 1;
                        }
                    }
                    Ordering::Equal => {
                        let sq = underlying.square_basis(a);
                        if sq == 0 {
                            self.coefficient = 0.0;
                            self.product = vec![];
                            return
                        }
                        self.coefficient *= sq as f32;
                        // Skip the next element
                        let _ = range.next();
                        // Do not increment kept_idx
                        // kept elements will overwrite current position
                    }
                    Ordering::Greater => {
                        self.product.swap(i, i + 1);
                        if i != kept_idx {
                            self.product.swap(i, kept_idx);
                        }
                        self.coefficient *= -1.0;
                        sorted = false;
                        kept_idx += 1;
                        if last_comparison {
                            self.product.swap(i + 1, kept_idx);
                            kept_idx += 1;
                        }
                    }
                }
            }
            // Truncate to kept elements
            self.product.truncate(kept_idx);
            // println!("Product::sort_and_simplify -> {self:?}");
        }
        if self.product.is_empty() && self.coefficient != 0.0 {
            // TODO I need to push scalar here, but GeneratorElement does not have scalar, only
            //  BasisSignature and BasisElement do. So that's a strong argument for making this work
            //  with BasisElements and not just GeneratorElements.
            // self.product.push()
        }
    }

    fn multiply(&self, other: &Product, underlying: &GeneratorSquares) -> Option<Product> {
        // println!("Product::multiply {self:?} and {other:?}");
        let mut product = self.product.clone();
        product.append(&mut other.product.clone());
        let mut coefficient = self.coefficient * other.coefficient;
        let mut product = Product { product, coefficient };
        product.sort_and_simplify(&underlying);
        if product.coefficient == 0.0 {
            // println!("Product::multiply -> zero");
            return None
        }
        // println!("Product::multiply -> {product:?}");
        Some(product)
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
            a.product.len().cmp(&b.product.len()).then_with(|| {
                a.product.cmp(&b.product)
            })
        });
        // println!("Sum::sort_and_simplify -> {self:?}");
        let mut i = 0;
        let mut kept_idx = 0;
        while i < (self.sum.len() - 1) {
            let (a, b) = self.sum.split_at_mut(i + 1);
            let a = &mut a[i];
            let b = &mut b[0];
            if a.product == b.product {
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
        s.sort_and_simplify();
        // println!("Sum::multiply -> {s:?}");
        s
    }

    fn add(&self, other: &Sum) -> Sum {
        let mut sum = self.sum.clone();
        sum.append(&mut other.sum.clone());
        let mut s = Sum { sum };
        s.sort_and_simplify();
        s
    }
}


#[test]
fn try_stuff() {
    use crate::algebra2::basis::elements::*;
    let underlying_cga = GeneratorSquares::new([(E1, 1), (E2, 1), (E3, 1), (EA, 1), (EB, -1)]);

    let new_e4 = SubstituteElement {
        new_element: E4,
        depends_on_underlying: vec![EA, EB],
        expr: Sum { sum: vec![
            Product { coefficient: 0.5, product: vec![EB], },
            Product { coefficient: -0.5, product: vec![EA], },
        ] },
    };
    let new_e5 = SubstituteElement {
        new_element: E5,
        depends_on_underlying: vec![EA, EB],
        expr: Sum { sum: vec![
            Product { coefficient: 1.0, product: vec![EB], },
            Product { coefficient: 1.0, product: vec![EA], },
        ] },
    };

    // TODO this result includes a bivector element, but don't be confused.
    //  This bivector is in the underlying basis, not the outer basis.
    //  I still have to convert it to the outer basis.
    let product_e45 = new_e4.expr.multiply(&new_e5.expr, &underlying_cga);
    println!("{product_e45:?}")
}
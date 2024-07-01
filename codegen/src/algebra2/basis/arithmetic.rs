use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use im::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use crate::algebra2::basis::BasisElement;
use crate::algebra2::basis::generators::{GeneratorElement, GeneratorSquares};
use crate::algebra2::basis::grades::{grade1, Grades};
use crate::grade_constraint;

#[derive(Clone, Copy, PartialEq)]
pub struct Product {
    // TODO un-pub?
    pub coefficient: f32,
    // TODO un-pub?
    pub element: BasisElement
}

#[derive(Clone, PartialEq)]
pub struct Sum {
    // TODO un-pub?
    pub sum: Vec<Product>
}


#[derive(Clone, Copy, PartialEq)]
pub struct GradedProduct<const G: Grades>(PhantomData<Grades>, Product);
#[derive(Clone, PartialEq)]
pub struct GradedSum<const G: Grades>(PhantomData<Grades>, Sum);

macro_rules! graded_sum {
    ($g:ty, $h:ty) => {
        GradedSum<{<$crate::algebra2::basis::grades::AddGradesImpl as $crate::algebra2::basis::grades::AddGradesTrait<$g, $h>>::OUTPUT}>
    };
}


impl From<GeneratorElement> for GradedProduct<{grade1}> {
    fn from(value: GeneratorElement) -> Self {
        GradedProduct(
            PhantomData,
            Product {
                coefficient: 1.0,
                element: value.element(),
            }
        )
    }
}
impl<const G: Grades> From<GradedProduct<G>> for Product {
    fn from(value: GradedProduct<G>) -> Self {
        value.1
    }
}
impl<const G: Grades> From<GradedSum<G>> for Sum {
    fn from(value: GradedSum<G>) -> Self {
        value.1
    }
}




impl Mul<f32> for Product {
    type Output = Product;

    fn mul(self, rhs: f32) -> Self::Output {
        Product {
            coefficient: self.coefficient * rhs,
            element: self.element,
        }
    }
}
impl Mul<Product> for f32 {
    type Output = Product;

    fn mul(self, rhs: Product) -> Self::Output {
        rhs * self
    }
}
impl MulAssign<f32> for Product {
    fn mul_assign(&mut self, rhs: f32) {
        self.coefficient *= rhs;
    }
}
impl Div<f32> for Product {
    type Output = Product;

    fn div(self, rhs: f32) -> Self::Output {
        Product {
            coefficient: self.coefficient / rhs,
            element: self.element,
        }
    }
}
impl DivAssign<f32> for Product {
    fn div_assign(&mut self, rhs: f32) {
        self.coefficient /= rhs;
    }
}
impl Add<Product> for Product {
    type Output = Sum;

    fn add(self, rhs: Product) -> Self::Output {
        let mut s = Sum { sum: vec![self.clone(), rhs] };
        s.sort_and_simplify();
        s
    }
}
impl Sub<Product> for Product {
    type Output = Sum;

    fn sub(self, mut rhs: Product) -> Self::Output {
        rhs *= -1.0;
        let mut s = Sum { sum: vec![self.clone(), rhs] };
        s.sort_and_simplify();
        s
    }
}




impl<const G: Grades> Mul<f32> for GradedProduct<G> {
    type Output = GradedProduct<G>;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.1.coefficient *= rhs;
        self
    }
}
impl<const G: Grades> Mul<GradedProduct<G>> for f32 {
    type Output = GradedProduct<G>;

    fn mul(self, rhs: GradedProduct<G>) -> Self::Output {
        rhs * self
    }
}
impl<const G: Grades> MulAssign<f32> for GradedProduct<G> {
    fn mul_assign(&mut self, rhs: f32) {
        self.1.coefficient *= rhs;
    }
}
impl<const G: Grades> Div<f32> for GradedProduct<G> {
    type Output = GradedProduct<G>;

    fn div(mut self, rhs: f32) -> Self::Output {
        self.1.coefficient /= rhs;
        self
    }
}
impl<const G: Grades> DivAssign<f32> for GradedProduct<G> {
    fn div_assign(&mut self, rhs: f32) {
        self.1.coefficient /= rhs;
    }
}
impl<const G: Grades, const H: Grades> Add<GradedProduct<G>> for GradedProduct<H> where grade_constraint!(G, H): Sized {
    type Output = graded_sum!(G, H);

    fn add(self, rhs: GradedProduct<G>) -> Self::Output {
        GradedSum(
            PhantomData,
            self.1 + rhs.1
        )
    }
}
impl<const G: Grades, const H: Grades> Sub<GradedProduct<G>> for GradedProduct<H> where grade_constraint!(G, H): Sized {
    type Output = graded_sum!(G, H);

    fn sub(self, rhs: GradedProduct<G>) -> Self::Output {
        GradedSum(
            PhantomData,
            self.1 - rhs.1
        )
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
impl Mul<&Sum> for f32 {
    type Output = Sum;

    fn mul(self, rhs: &Sum) -> Self::Output {
        rhs * self
    }
}
impl MulAssign<f32> for Sum {
    fn mul_assign(&mut self, rhs: f32) {
        for term in self.sum.iter_mut() {
            term.mul_assign(rhs);
        }
    }
}
impl Div<f32> for &Sum {
    type Output = Sum;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = self.clone();
        for term in result.sum.iter_mut() {
            *term = term.div(rhs);
        }
        result
    }
}
impl DivAssign<f32> for Sum {
    fn div_assign(&mut self, rhs: f32) {
        for term in self.sum.iter_mut() {
            term.div_assign(rhs);
        }
    }
}
impl Add<Product> for &Sum {
    type Output = Sum;

    fn add(self, rhs: Product) -> Self::Output {
        let mut s = self.clone();
        s.sum.push(rhs);
        s.sort_and_simplify();
        s
    }
}
impl Add<Sum> for &Product {
    type Output = Sum;

    fn add(self, rhs: Sum) -> Self::Output {
        &rhs + *self
    }
}
impl AddAssign<Product> for Sum {
    fn add_assign(&mut self, rhs: Product) {
        self.sum.push(rhs);
        self.sort_and_simplify();
    }
}
impl Sub<Product> for &Sum {
    type Output = Sum;

    fn sub(self, mut rhs: Product) -> Self::Output {
        let mut s = self.clone();
        rhs *= -1.0;
        s.sum.push(rhs);
        s.sort_and_simplify();
        s
    }
}
impl Sub<Sum> for &Product {
    type Output = Sum;

    fn sub(self, mut rhs: Sum) -> Self::Output {
        rhs *= -1.0;
        &rhs + *self
    }
}
impl SubAssign<Product> for Sum {
    fn sub_assign(&mut self, mut rhs: Product) {
        rhs *= -1.0;
        self.sum.push(rhs);
        self.sort_and_simplify();
    }
}
impl Add<&Sum> for &Sum {
    type Output = Sum;

    fn add(self, rhs: &Sum) -> Self::Output {
        let mut s = self.clone();
        let mut rhs = rhs.clone();
        s.sum.append(&mut rhs.sum);
        s.sort_and_simplify();
        s
    }
}
impl AddAssign<&Sum> for Sum {
    fn add_assign(&mut self, rhs: &Sum) {
        let mut rhs = rhs.clone();
        self.sum.append(&mut rhs.sum);
        self.sort_and_simplify();
    }
}
impl Sub<&Sum> for &Sum {
    type Output = Sum;

    fn sub(self, rhs: &Sum) -> Self::Output {
        let mut rhs = rhs.clone();
        rhs *= -1.0;
        self.add(&rhs)
    }
}
impl SubAssign<&Sum> for Sum {
    fn sub_assign(&mut self, rhs: &Sum) {
        let mut rhs = rhs.clone();
        rhs *= -1.0;
        self.add_assign(&rhs);
    }
}




impl<const G: Grades> Mul<f32> for GradedSum<G> {
    type Output = GradedSum<G>;

    fn mul(self, rhs: f32) -> Self::Output {
        GradedSum(
            PhantomData,
            &self.1 * rhs
        )
    }
}
impl<const G: Grades> Mul<GradedSum<G>> for f32 {
    type Output = GradedSum<G>;

    fn mul(self, rhs: GradedSum<G>) -> Self::Output {
        rhs * self
    }
}
impl<const G: Grades> MulAssign<f32> for GradedSum<G> {
    fn mul_assign(&mut self, rhs: f32) {
        self.1 *= rhs;
    }
}
impl<const G: Grades> Div<f32> for GradedSum<G> {
    type Output = GradedSum<G>;

    fn div(self, rhs: f32) -> Self::Output {
        GradedSum(
            PhantomData,
            &self.1 / rhs
        )
    }
}
impl<const G: Grades> DivAssign<f32> for GradedSum<G> {
    fn div_assign(&mut self, rhs: f32) {
        self.1 /= rhs;
    }
}
impl<const G: Grades, const H: Grades> Add<GradedProduct<H>> for GradedSum<G> where grade_constraint!(G, H): Sized {
    type Output = graded_sum!(G, H);

    fn add(self, rhs: GradedProduct<H>) -> Self::Output {
        GradedSum(
            PhantomData,
            &self.1 + rhs.1
        )
    }
}
impl<const G: Grades, const H: Grades> Sub<GradedProduct<H>> for GradedSum<G> where grade_constraint!(G, H): Sized {
    type Output = graded_sum!(G, H);

    fn sub(self, rhs: GradedProduct<H>) -> Self::Output {
        GradedSum(
            PhantomData,
            &self.1 - rhs.1
        )
    }
}
impl<const G: Grades, const H: Grades> Add<GradedSum<H>> for GradedProduct<G> where grade_constraint!(G, H): Sized {
    type Output = graded_sum!(G, H);

    fn add(self, rhs: GradedSum<H>) -> Self::Output {
        GradedSum(
            PhantomData,
            &rhs.1 + self.1
        )
    }
}
impl<const G: Grades, const H: Grades> Sub<GradedSum<H>> for GradedProduct<G> where grade_constraint!(G, H): Sized {
    type Output = graded_sum!(G, H);

    fn sub(self, rhs: GradedSum<H>) -> Self::Output {
        let s = Sum { sum: vec![self.1] };
        GradedSum(
            PhantomData,
            &s - &rhs.1
        )
    }
}
impl<const G: Grades, const H: Grades> Add<GradedSum<H>> for GradedSum<G> where grade_constraint!(G, H): Sized {
    type Output = graded_sum!(G, H);

    fn add(self, rhs: GradedSum<H>) -> Self::Output {
        GradedSum(
            PhantomData,
            &self.1 + &rhs.1
        )
    }
}
impl<const G: Grades> AddAssign<GradedSum<G>> for GradedSum<G> {
    fn add_assign(&mut self, rhs: GradedSum<G>) {
        self.1 += &rhs.1;
    }
}
impl<const G: Grades, const H: Grades> Sub<GradedSum<H>> for GradedSum<G> where grade_constraint!(G, H): Sized {
    type Output = graded_sum!(G, H);

    fn sub(self, rhs: GradedSum<H>) -> Self::Output {
        GradedSum(
            PhantomData,
            &self.1 - &rhs.1
        )
    }
}
impl<const G: Grades> SubAssign<GradedSum<G>> for GradedSum<G> {
    fn sub_assign(&mut self, rhs: GradedSum<G>) {
        self.1 -= &rhs.1;
    }
}



impl Debug for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.coefficient == 1.0 {
            return Display::fmt(&self.element, f);
        }
        if self.coefficient == -1.0 {
            let mut el = self.element;
            if el.coefficient == -1 {
                el.coefficient = 1;
            } else {
                write!(f, "-")?;
            }
            return Display::fmt(&self.element, f);
        }
        if self.coefficient == 0.0 {
            write!(f, "0*")?;
            return Display::fmt(&self.element, f);
        }
        write!(f, "{}*{}", self.coefficient, self.element)
    }
}
impl Display for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
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
impl Display for Sum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Product {
    pub fn multiply(&self, other: &Product, underlying: &GeneratorSquares) -> Option<Product> {
        let mut element = underlying.geometric_product(self.element, other.element);
        if element.coefficient == 0 {
            return None;
        }
        let coefficient = self.coefficient * other.coefficient * element.coefficient as f32;
        element.coefficient = 1;
        Some(Product { coefficient, element })
    }

    pub fn anti_multiply(&self, other: &Product, underlying: &GeneratorSquares) -> Option<Product> {
        // use crate::algebra2::basis::elements::*;
        // let e123AB = e123.wedge(eA).wedge(eB);
        // if self.element == e123AB && other.element == e123AB {
        //     eprintln!("Attempting anti-product identity");
        // }

        let mut element = underlying.geometric_anti_product(self.element, other.element);
        if element.coefficient == 0 {
            return None;
        }
        let coefficient = self.coefficient * other.coefficient * element.coefficient as f32;
        element.coefficient = 1;
        Some(Product { coefficient, element })
    }

    pub fn zero() -> Self {
        use crate::algebra2::basis::elements::*;
        Product { coefficient: 0.0, element: scalar }
    }
}

impl Sum {

    pub fn sort_and_simplify(&mut self) {
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

    pub fn multiply(&self, other: &Sum, underlying: &GeneratorSquares) -> Sum {
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

    pub fn anti_multiply(&self, other: &Sum, underlying: &GeneratorSquares) -> Sum {
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

    pub fn wedge(&self, other: &Sum) -> Sum {
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
    pub fn superficial_dot_product(self, other: Sum) -> f32 {
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

    pub fn zero() -> Self {
        Self { sum: vec![] }
    }
}




use std::fmt::{Debug, Display, Formatter};
use lazy_static::lazy_static;
use regex::Regex;
use tinyvec::{array_vec, ArrayVec, TinyVec};
use crate::algebra2::basis::{BasisElement, BasisSignature};
use crate::algebra2::basis::grades::Grades;

pub(crate) const fn num_elements(d: u8) -> usize {
    let d = d as u32;

    // Scalar counts as an element
    // let n = usize::pow(2, d) - 1;
    let n = usize::pow(2, d);

    n
}

// TODO Is there anything I can do to un-pub this, or make the constraints outside this module more simple?
pub const fn mono_grade_elements(d: u8) -> usize {
    const fn factorial(mut n: usize) -> usize {
        let mut result = 1;
        while n > 1 {
            result = result * n;
            n -= 1;
        }
        result
    }

    let n = d as usize;
    let k = (d/2) as usize;
    let n_k = n-k;
    factorial(n) / (factorial(k) * factorial(n_k))
}
pub const fn mono_grade_groups(d: u8) -> usize {
    let elements = mono_grade_elements(d);
    // In the end every group is 4 floats wide, regardless.
    // This helps keep things simple when passing data back and forth from GPU.
    // However, that does mean we might have up to 3 vacant spaces.
    // The whole point of mono_grade_elements is to prevent any mono_grade class
    // from allocating to heap. So we use 3 as our margin of error.
    (elements + 3) / 4
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MultiVectorSignature<const D: u8>(ArrayVec<[BasisSignature; num_elements(D)]>)
    where [(); num_elements(D)]: Sized;

// TODO there could be an argument for using an enum instead, which is integrating the AST
pub type BasisElementGroup = ArrayVec<[BasisElement; 4]>;




// pub struct MultiVecConstraint<const D: u8>([(); mono_grade_groups(D)], [(); num_elements(D)]) where
//     [(); mono_grade_groups(D)]: Sized,
//     [(); num_elements(D)]: Sized;




// Cannot be Copy because TinyVec is never Copy
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MultiVec<const D: u8> where
    [(); mono_grade_groups(D)]: Sized,
    [(); num_elements(D)]: Sized {
    name: &'static str,
    grades: Grades,
    element_groups: TinyVec<[BasisElementGroup; mono_grade_groups(D)]>,
    // It is important to keep the vec in the signature sorted, so it can serve its purpose.
    // So we should keep very strict control over construction and mutation of MultiVec,
    // and it's signature.
    signature: MultiVectorSignature<D>
}

impl<const D: u8> Debug for MultiVec<D> where
    [(); mono_grade_groups(D)]: Sized,
    [(); num_elements(D)]: Sized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.name;
        write!(f, "MultiVec {{ name: \"{n}\", element_groups: [")?;
        for (i, group) in self.element_groups.iter().enumerate() {
            let comma = if i == 0 { "" } else { ", " };
            write!(f, "{comma}[")?;
            for (j, el) in group.iter().enumerate() {
                let comma = if j == 0 { "" } else { ", " };
                write!(f, "{comma}{el}")?;
            }
            write!(f, "]")?;
        }
        write!(f, "] }}")
    }
}
impl<const D: u8> Display for MultiVec<D> where
    [(); mono_grade_groups(D)]: Sized,
    [(); num_elements(D)]: Sized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.name;
        write!(f, "{n}(")?;
        let mut i = 0;
        for (i, group) in self.element_groups.iter().enumerate() {
            let comma = if i == 0 { "" } else { ", " };
            write!(f, "{comma}(")?;
            for (j, el) in group.iter().enumerate() {
                let comma = if j == 0 { "" } else { ", " };
                write!(f, "{comma}{el}")?;
            }
            write!(f, ")")?;
        }
        write!(f, ")")
    }
}



lazy_static! {
    static ref MULTIVECTOR_NAME_REGEX: Regex = Regex::new("^[A-Z][a-zA-Z0-9]+$").expect("MultiVector name regex is valid");
}

impl<const D: u8> MultiVec<D> where
    [(); mono_grade_groups(D)]: Sized,
    [(); num_elements(D)]: Sized {
    pub fn signature(&self) -> MultiVectorSignature<D> {
        self.signature
    }

    pub fn elements(&self) -> TinyVec<[BasisElement; mono_grade_elements(D)]> {
        let mut elements = TinyVec::new();
        for group in self.element_groups.iter() {
            for element in group {
                elements.push(element.clone());
            }
        }
        elements
    }

    pub fn groups(&self) -> TinyVec<[BasisElementGroup; mono_grade_groups(D)]> {
        let mut groups = TinyVec::new();
        for group in self.element_groups.iter() {
            groups.push(group.clone());
        }
        groups
    }

    pub fn new<E: IntoIterator<Item=BasisElement>>(name: &'static str, elements: E) -> Self {
        let mut elements = elements.into_iter().collect::<Vec<_>>();
        elements.sort();
        let mut active_grade = elements.get(0).map(|it| it.grade()).unwrap_or(0);
        let mut grouped = vec![];
        let mut group = ArrayVec::new();
        for element in elements {
            let element_grade = element.grade();
            if element_grade == active_grade {
                // Same grade, same group
                group.push(element);
                if group.len() == 4 {
                    grouped.push(group);
                    group = ArrayVec::new();
                }
            } else {
                // New grade, new group.
                // If you want extra-compact grouping, then use new_by_groups with manually
                // specified groups instead. Or I guess we could add more heuristics here
                // at some point, but not feeling rushed for it.
                grouped.push(group);
                group = ArrayVec::new();
                group.push(element);
                active_grade = element_grade;
            }
        }
        if !group.is_empty() {
            grouped.push(group);
        }

        Self::new_by_groups(name, grouped)
    }

    pub fn new_by_groups<G: IntoIterator<Item=BasisElementGroup>>(name: &'static str, element_groups: G) -> Self {
        if !MULTIVECTOR_NAME_REGEX.is_match(name) {
            panic!("MultiVector names must be UpperCamelCase without any funny business or \
                special characters, but this is violated by \"{name}\".")
        }
        let mut used_dimensions = BasisSignature::empty();
        let arg = element_groups;
        let mut element_groups = TinyVec::new();
        let mut signature = MultiVectorSignature(ArrayVec::new());
        let mut grades = Grades::none;
        for group in arg {
            for el in group {
                let el_sig = el.signature();
                if signature.0.contains(&el_sig) {
                    panic!("{name} already has {el}. Do not define MultiVectors using redundant or \
                        duplicate BasisSignatures. Don't forget that reordered or sign flipped \
                        BasisElements can share the same BasisSignature")
                }
                signature.0.push(el_sig);
                used_dimensions |= el_sig;
                let i = used_dimensions.bits().count_ones();
                if i > D as u32 {
                    panic!("MultiVector embedded in {D} dimensions is defined with {i} or \
                        more primary basis vectors. Already reserved: {used_dimensions} Latest \
                        addition: {el_sig}")
                }
                grades |= Grades::from_sig(el_sig);
            }
            element_groups.push(group);
        }
        signature.0.sort();
        MultiVec { name, grades, element_groups, signature, }
    }
}

#[macro_export]
macro_rules! multi_vec {
    // Accepts a string literal, a u8 literal, and a list of BasisElementGroups (as tuples)
    ($str_lit:expr, $u8_lit:expr, $( ($($basis_element:expr),+ $(,)?)),+ $(,)?) => {
        {
            use crate::algebra2::multivector::TupleToGroup;
            let name: &'static str = $str_lit;
            let groups: std::vec::Vec<crate::algebra2::multivector::BasisElementGroup> = vec![
                $( ($($basis_element),+).tuple_to_group() ),+
            ];
            crate::algebra2::multivector::MultiVec::<$u8_lit>::new_by_groups($str_lit, groups)
        }
    };
    // Accepts a string literal, a u8 literal, and a list of BasisElementGroups (as arrays)
    ($str_lit:expr, $u8_lit:expr, $( [$($basis_element:expr),+ $(,)?]),+ $(,)?) => {
        {
            use crate::algebra2::multivector::TupleToGroup;
            let name: &'static str = $str_lit;
            let groups: std::vec::Vec<crate::algebra2::multivector::BasisElementGroup> = vec![
                $( ($($basis_element),+).tuple_to_group() ),+
            ];
            crate::algebra2::multivector::MultiVec::<$u8_lit>::new_by_groups($str_lit, groups)
        }
    };
    // Accepts a string literal, a u8 literal, and a list of BasisElement
    ($str_lit:expr, $u8_lit:expr, $( $basis_element:expr ),+ $(,)?) => {
        {
            let name: &'static str = $str_lit;
            let elements: std::vec::Vec<crate::algebra2::multivector::BasisElement> = vec![$($basis_element),+];
            crate::algebra2::multivector::MultiVec::<$u8_lit>::new($str_lit, elements)
        }
    };
}

trait TupleToGroup<T: Default> {
    fn tuple_to_group(self) -> ArrayVec<[T; 4]>;
}
impl<T: Default> TupleToGroup<T> for (T,) {
    fn tuple_to_group(self) -> ArrayVec<[T; 4]> {
        array_vec!(self.0)
    }
}
impl<T: Default> TupleToGroup<T> for (T, T) {
    fn tuple_to_group(self) -> ArrayVec<[T; 4]> {
        array_vec!(self.0, self.1)
    }
}
impl<T: Default> TupleToGroup<T> for (T, T, T) {
    fn tuple_to_group(self) -> ArrayVec<[T; 4]> {
        array_vec!(self.0, self.1, self.2)
    }
}
impl<T: Default> TupleToGroup<T> for (T, T, T, T) {
    fn tuple_to_group(self) -> ArrayVec<[T; 4]> {
        array_vec!(self.0, self.1, self.2, self.3)
    }
}
impl<T: Default> TupleToGroup<T> for [T; 4] {
    fn tuple_to_group(self) -> ArrayVec<[T; 4]> {
        ArrayVec::from(self)
    }
}
impl<T: Default> TupleToGroup<T> for [T; 3] {
    fn tuple_to_group(self) -> ArrayVec<[T; 4]> {
        let mut v = ArrayVec::new();
        for t in self.into_iter() {
            v.push(t);
        }
        v
    }
}
impl<T: Default> TupleToGroup<T> for [T; 2] {
    fn tuple_to_group(self) -> ArrayVec<[T; 4]> {
        let mut v = ArrayVec::new();
        for t in self.into_iter() {
            v.push(t);
        }
        v
    }
}
impl<T: Default> TupleToGroup<T> for [T; 1] {
    fn tuple_to_group(self) -> ArrayVec<[T; 4]> {
        let mut v = ArrayVec::new();
        for t in self.into_iter() {
            v.push(t);
        }
        v
    }
}

pub trait BoxIt {
    fn box_it(self) -> BoxedMultiVec;
}
impl BoxIt for MultiVec<1> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D1(Box::new(self))
    }
}
impl BoxIt for MultiVec<2> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D2(Box::new(self))
    }
}
impl BoxIt for MultiVec<3> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D3(Box::new(self))
    }
}
impl BoxIt for MultiVec<4> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D4(Box::new(self))
    }
}
impl BoxIt for MultiVec<5> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D5(Box::new(self))
    }
}
impl BoxIt for MultiVec<6> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D6(Box::new(self))
    }
}
impl BoxIt for MultiVec<7> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D7(Box::new(self))
    }
}
impl BoxIt for MultiVec<8> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D8(Box::new(self))
    }
}
impl BoxIt for MultiVec<9> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D9(Box::new(self))
    }
}
impl BoxIt for MultiVec<10> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D10(Box::new(self))
    }
}
impl BoxIt for MultiVec<11> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D11(Box::new(self))
    }
}
impl BoxIt for MultiVec<12> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D12(Box::new(self))
    }
}
impl BoxIt for MultiVec<13> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D13(Box::new(self))
    }
}
impl BoxIt for MultiVec<14> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D14(Box::new(self))
    }
}
impl BoxIt for MultiVec<15> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D15(Box::new(self))
    }
}
impl BoxIt for MultiVec<16> {
    fn box_it(self) -> BoxedMultiVec {
        BoxedMultiVec::D16(Box::new(self))
    }
}



#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum BoxedMultiVec {
    D1(Box<MultiVec<1>>),
    D2(Box<MultiVec<2>>),
    D3(Box<MultiVec<3>>),
    D4(Box<MultiVec<4>>),
    D5(Box<MultiVec<5>>),
    D6(Box<MultiVec<6>>),
    D7(Box<MultiVec<7>>),
    D8(Box<MultiVec<8>>),
    D9(Box<MultiVec<9>>),
    D10(Box<MultiVec<10>>),
    D11(Box<MultiVec<11>>),
    D12(Box<MultiVec<12>>),
    D13(Box<MultiVec<13>>),
    D14(Box<MultiVec<14>>),
    D15(Box<MultiVec<15>>),
    D16(Box<MultiVec<16>>),
}
impl BoxedMultiVec {
    pub fn adapt_eq<const D: u8>(&self, other: &MultiVec<D>) -> bool where
        MultiVec<D>: BoxIt,
        [(); mono_grade_groups(D)]: Sized,
        [(); num_elements(D)]: Sized {

        self == &other.clone().box_it()
    }

    pub fn element_groups(&self) -> impl Iterator<Item=&BasisElementGroup> {
        match &self {
            BoxedMultiVec::D1(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D2(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D3(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D4(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D5(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D6(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D7(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D8(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D9(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D10(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D11(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D12(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D13(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D14(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D15(mv) => mv.element_groups.iter(),
            BoxedMultiVec::D16(mv) => mv.element_groups.iter(),
        }
    }

    pub fn elements(&self) -> impl Iterator<Item=BasisElement> {
        let mut v = vec![];
        for el in self.element_groups().flat_map(|it| it.iter()) {
            v.push(*el);
        }
        v.into_iter()
    }
}


#[test]
fn test_construction() {
    use crate::algebra2::basis::elements::*;
    let circle = MultiVec::<5>::new_by_groups("Circle", [
        array_vec!(e423, e431, e412, e321), array_vec!(e415, e425, e435), array_vec!(e235, e315, e125)
    ]);
    println!("{circle:?}");
    println!("{circle}");

    // Note this one will print with a different order than displayed because it will sort
    // the BasisElements first. If you want a fixed order, then specify the grouping
    // manually (since everything will end up in groups anyway). If you'd rather have it sorted
    // without fretting about sorting it yourself, then use it like this with ungrouped input.
    let dipole = multi_vec!("Dipole", 5, e41, e42, e43, e23, e31, e12, e15, e25, e35, e45);
    println!("{dipole}");

    let circle_again = multi_vec!("Circle", 5, (e423, e431, e412, e321), (e415, e425, e435), (e235, e315, e125));
    println!("{circle_again}");

    let dipole_again = multi_vec!("Dipole", 5, [e41, e42, e43], [e23, e31, e12], [e15, e25, e35, e45]);
    println!("{dipole_again}");

    if dipole == dipole_again {
        //
    }
}
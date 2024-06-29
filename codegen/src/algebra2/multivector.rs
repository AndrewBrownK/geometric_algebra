use std::fmt::{Debug, Display, Formatter};
use lazy_static::lazy_static;
use regex::Regex;
use tinyvec::{array_vec, ArrayVec, TinyVec};

use crate::algebra2::basis::{BasisElement, BasisSignature};
use crate::algebra2::basis::grades::Grades;

const fn num_elements(d: u8) -> usize {
    let d = d as u32;
    let n = usize::pow(2, d) - 1;
    n
}

const fn mono_grade_elements(d: u8) -> usize {
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
const fn mono_grade_groups(d: u8) -> usize {
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


// Cannot be Copy because TinyVec is never Copy
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MultiVec<const D: u8> where
    [(); mono_grade_groups(D)]: Sized,
    [(); num_elements(D)]: Sized
{
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
                used_dimensions = used_dimensions.union(el_sig);
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
}
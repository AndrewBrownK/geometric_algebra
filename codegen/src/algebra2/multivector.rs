use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use std::sync::atomic::Ordering;

use atom::AtomSetOnce;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use paste::paste;
use regex::Regex;
use tinyvec::{array_vec, ArrayVec, TinyVec};

use MultiVecRepository::D1;

use crate::algebra2::basis::{BasisElement, BasisSignature};
use crate::algebra2::basis::grades::Grades;
use crate::algebra2::GeometricAlgebra;
use crate::ast2::traits::RawTraitImplementation;

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



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MultiVecSignature<const D: u8>(ArrayVec<[BasisSignature; num_elements(D)]>)
    where [(); num_elements(D)]: Sized;

// TODO there could be an argument for using an enum instead, which is integrating the AST
pub type BasisElementGroup = ArrayVec<[BasisElement; 4]>;




// pub struct MultiVecConstraint<const D: u8>([(); mono_grade_groups(D)], [(); num_elements(D)]) where
//     [(); mono_grade_groups(D)]: Sized,
//     [(); num_elements(D)]: Sized;


// TODO I am so very torn and conflicted with MultiVec. It feels so close but so far from being
//  const. I keep flip flopping. All this effort to use sized arrays, only to drop the ball like
//  a coward and depend on the heap anyway. The truly really interesting problem is handling
//  impl ClassesFromRegistry for Specifically. The concern is less about choosing between
//  MultiVec<D> and MultiVecEnum. The real concern is, where is it getting this value? Hell..
//  for any implementation of TraitDef_2Class_2Param... where is it providing a value at all,
//  instead of just specifying an associated type? Nowhere yet, it seems.... alright alright alright
//  but let's assume we're going to provide it by value somewhere at some point. We probably don't
//  want these MultiVec values declared willy nilly in trait implementations, we want them to
//  be properly handled data through the MultiVecRepository. It might be possible to make something
//  like Elaborated that can set the Owner and Other associated types. Hell.... maybe Owner and
//  Other associated types should be trait methods instead of Associated types to begin with?
//  ClassesFromRegistry is technically not used anywhere right now. I think I'm onto something
//  there. Anyway..... well... that's another conundrum anyway. Because we're back in the class
//  implementation again, instead of our script scope with lovely MultiVec declarations.
//  So yeah, about MultiVec declarations. Could they be static? Would we like that? Well right now
//  they depend on the heap, so we'd have to use something like lazy_static!, and then they're
//  less succinct. Additionally I want the ability to dynamically generate additional MultiVecs
//  based on various conditions and suffixes, like being on the Origin or inside the Horizon.
//  However a modifiable name more or less means the name needs to be a String, which more or less
//  means MultiVec won't have a Size, or will depend on heap yet again. So to talk more about
//  MultiVec on heap, never mind String name, mind it now. If some insane person really goes up to
//  16 dimensions, then that is 65,535 unique BasisElements, which means a MultiVecSignature that is
//  131kB long. Then multiply that by 18 for one class per grade plus DualNum and full MultiVector,
//  then the MultiVecSignatures alone (nevermind BasisElementGroups) is taking 2.4 MB, without
//  including mixed grade or partial grade classes at all. So? If someone is crazy and they do
//  that, then what? If we start by saying we don't want that on our stack, and we're going to need
//  the heap, then the heap is the heap no matter how you slice it, and maybe you want to make them
//  lazy static so you can get away with only a single copy of each. It might seem absurd or crazy
//  to make them take up so much space statically in the binary instead of the heap, but at the end
//  of the day, if someone chooses 16 bases, they are going to spend a lot of memory, and I should
//  be less concerned about if it is using heap, or implements Copy, and more concerned about only
//  ensuring it is created once, and every use site borrows it, and it never overflows the stack.
//  To me that looks like... I want static declarations. AS LONG AS THE FOLLOWING CONDITIONS ARE
//  SATISFIED:
//  - I prefer const eval static instead of lazy heap static, as long as the compiler isn't
//    literally destroyed from chewing such intensive const eval. This allows me to use ArrayVec
//    instead of TinyVec, and simplify some of the constraints on D.
//  - Obviously the existing macros are kick-ass and there's no way we can settle for anything
//    less elegant for those declarations. Can the macros play nice in const eval though?
//  - I might/should redo MultiVecRepository to just take a bunch of &'static MultiVec and go from
//    there. The entire app could use these &'static MultiVecs without causing lifetime annoyances.
//  - Even though I do have intentions to make suffix/variant MultiVecs, it might be better as a
//    suggestion spat out in the console, and we still require all MultiVecs to be statically
//    declared (instead of generating new MultiVecs using rules and patterns dynamically).
//  ..
//  Here are some other thoughts/concerns...
//  Maybe MultiVecSignature is not serving its purpose well if it is not a convenient/useful lookup
//  device. With that in mind, I could scrap it and save some memory. If I need to match signatures,
//  I may be better off (overall) just scanning through the BasisElements already included in the
//  MultiVec definition. I know BasisSignature is great and would also make a nice lookup tool,
//  but the fact that MultiVecSignature could get 131 kB long starts to defeat the purpose.
//  Next... since everything is getting hidden in a static position behind a static reference anyway,
//  I could honestly get a little more liberal with the const generics. I could size every
//  MultiVec according to it's ACTUAL quantity of BasisElements, not the total possible elements
//  for the dimensionality, then have a separate const generic for the dimensionality, and then
//  have another type that holds a reference to the &'static MultiVec (not worried about size at
//  all) and keeping track of the dimensionality only (but not the MultiVec-specific quantity of
//  BasisElements).
//  Yeah this seems great. Refactors coming!!

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
    signature: MultiVecSignature<D>
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
    pub fn signature(&self) -> MultiVecSignature<D> {
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
        let mut signature = MultiVecSignature(ArrayVec::new());
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
    // Accepts a u8 literal, an ident, and a list of BasisElementGroups (as tuples)
    (D=$u8_lit:expr; $mv_name:ident => $( ($($basis_element:expr),+ $(,)?)),+ $(,)?) => {
        {
            use $crate::algebra2::basis::elements::*;
            use $crate::algebra2::multivector::TupleToGroup;
            let name: &'static str = stringify!($mv_name);
            let groups: std::vec::Vec<$crate::algebra2::multivector::BasisElementGroup> = vec![
                $( ($($basis_element),+,).tuple_to_group() ),+
            ];
            $crate::algebra2::multivector::MultiVec::<$u8_lit>::new_by_groups(name, groups)
        }
    };
    // Accepts a u8 literal, an ident, and a list of BasisElementGroups (as arrays)
    (D=$u8_lit:expr; $mv_name:ident => $( [$($basis_element:expr),+ $(,)?]),+ $(,)?) => {
        {
            use $crate::algebra2::basis::elements::*;
            use $crate::algebra2::multivector::TupleToGroup;
            let name: &'static str = stringify!($mv_name);
            let groups: std::vec::Vec<$crate::algebra2::multivector::BasisElementGroup> = vec![
                $( ($($basis_element),+,).tuple_to_group() ),+
            ];
            $crate::algebra2::multivector::MultiVec::<$u8_lit>::new_by_groups(name, groups)
        }
    };
    // Accepts a u8 literal, an ident, and a list of BasisElement
    (D=$u8_lit:expr; $mv_name:ident => $( $basis_element:expr ),+ $(,)?) => {
        {
            use $crate::algebra2::basis::elements::*;
            let name: &'static str = stringify!($mv_name);
            let elements: std::vec::Vec<$crate::algebra2::basis::BasisElement> = vec![$($basis_element),+,];
            $crate::algebra2::multivector::MultiVec::<$u8_lit>::new(name, elements)
        }
    };
    // Accepts a u8 literal, an ident, and a list of BasisElementGroups
    // (non-enclosed, but separated by pipes). This is kind of cheating
    // because we are using an ident instead of an expr to allow us to chain
    // with pipes.
    (D=$u8_lit:expr; $mv_name:ident as $( $($basis_element:ident),+ $(,)?)|+ ) => {
        {
            use $crate::algebra2::basis::elements::*;
            use $crate::algebra2::multivector::TupleToGroup;
            let name: &'static str = stringify!($mv_name);
            let groups: std::vec::Vec<$crate::algebra2::multivector::BasisElementGroup> = vec![
                $( ($($basis_element),+,).tuple_to_group() ),+
            ];
            $crate::algebra2::multivector::MultiVec::<$u8_lit>::new_by_groups(name, groups)
        }
    };
}
#[macro_export]
macro_rules! multi_vecs {
    (D=$u8_lit:expr; $( $mv_name:ident => $( ($($basis_element:expr),+ $(,)?)),+ $(,)? );+ $(;)?) => {
        {
            use $crate::algebra2::basis::elements::*;
            vec![
                $( multi_vec!(D=$u8_lit; $mv_name => $( ( $( $basis_element ),+ ) ),+ ) ),+
            ]
        }
    };
    (D=$u8_lit:expr; $( $mv_name:ident => $( [$($basis_element:expr),+ $(,)?]),+ $(,)? );+ $(;)?) => {
        {
            use $crate::algebra2::basis::elements::*;
            vec![
                $( multi_vec!(D=$u8_lit; $mv_name => $( [ $( $basis_element ),+ ] ),+ ) ),+
            ]
        }
    };
    (D=$u8_lit:expr; $( $mv_name:ident => $($basis_element:expr),+ $(,)? );+ $(;)?) => {
        {
            use $crate::algebra2::basis::elements::*;
            vec![
                $( multi_vec!(D=$u8_lit; $mv_name => $( $basis_element ),+) ),+
            ]
        }
    };
    // Accepts a u8 literal, an ident, and a list of BasisElementGroups
    // (non-enclosed, but separated by pipes). This is kind of cheating
    // because we are using an ident instead of an expr to allow us to chain
    // with pipes.
    (D=$u8_lit:expr; $( $mv_name:ident as $( $($basis_element:ident),+ $(,)?)|+ );+ $(;)?) => {
        {
            use $crate::algebra2::basis::elements::*;
            vec![
                $( multi_vec!(D=$u8_lit; $mv_name as $( $( $basis_element ),+ )|+ ) ),+
            ]
        }
    };
}


pub trait TupleToGroup<T: Default> {
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

pub trait ConsolidateEnum {
    type Output;
    fn consolidate_enum(self) -> Self::Output;
}
macro_rules! consolidate_enum {
    // literal based, with boxing
    ($(derive($($d:ident),+ $(,)?))? box $impl_type:ident $variant_prefix:ident => $( $variants:literal ),+ $(,)?) => {
        paste! {
            #[derive(Clone, PartialEq, Eq, Hash, Debug)]
            pub enum [<$impl_type Enum>] {
                $([<$variant_prefix $variants>](Box<$impl_type<$variants>>)),+
            }

            $(impl ConsolidateEnum for $impl_type<$variants> {
                type Output = [<$impl_type Enum>];
                fn consolidate_enum(self) -> Self::Output {
                    [<$impl_type Enum>]::[<$variant_prefix $variants>](Box::new(self))
                }
            })+
        }
    };
    // literal based, no boxing
    ($(derive($($d:ident),+ $(,)?))? $impl_type:ident $variant_prefix:ident => $( $variants:literal ),+ $(,)?) => {
        paste! {
            $(#[derive($($d),+)])?
            pub enum [<$impl_type Enum>] {
                $([<$variant_prefix $variants>]($impl_type<$variants>)),+
            }

            $(impl ConsolidateEnum for $impl_type<$variants> {
                type Output = [<$impl_type Enum>];
                fn consolidate_enum(self) -> Self::Output {
                    [<$impl_type Enum>]::[<$variant_prefix $variants>](self)
                }
            })+
        }
    };
}

consolidate_enum! {
    derive(Clone, PartialEq, Eq, Hash, Debug)
    box MultiVec D => 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
}

consolidate_enum! {
    derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)
    box MultiVecSignature D => 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
}

impl MultiVecEnum {
    fn signature(&self) -> MultiVecSignatureEnum {
        match self {
            MultiVecEnum::D1(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D2(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D3(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D4(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D5(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D6(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D7(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D8(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D9(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D10(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D11(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D12(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D13(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D14(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D15(d) => d.signature.consolidate_enum(),
            MultiVecEnum::D16(d) => d.signature.consolidate_enum(),
        }
    }

    pub fn element_groups(&self) -> impl Iterator<Item=&BasisElementGroup> {
        match &self {
            MultiVecEnum::D1(mv) => mv.element_groups.iter(),
            MultiVecEnum::D2(mv) => mv.element_groups.iter(),
            MultiVecEnum::D3(mv) => mv.element_groups.iter(),
            MultiVecEnum::D4(mv) => mv.element_groups.iter(),
            MultiVecEnum::D5(mv) => mv.element_groups.iter(),
            MultiVecEnum::D6(mv) => mv.element_groups.iter(),
            MultiVecEnum::D7(mv) => mv.element_groups.iter(),
            MultiVecEnum::D8(mv) => mv.element_groups.iter(),
            MultiVecEnum::D9(mv) => mv.element_groups.iter(),
            MultiVecEnum::D10(mv) => mv.element_groups.iter(),
            MultiVecEnum::D11(mv) => mv.element_groups.iter(),
            MultiVecEnum::D12(mv) => mv.element_groups.iter(),
            MultiVecEnum::D13(mv) => mv.element_groups.iter(),
            MultiVecEnum::D14(mv) => mv.element_groups.iter(),
            MultiVecEnum::D15(mv) => mv.element_groups.iter(),
            MultiVecEnum::D16(mv) => mv.element_groups.iter(),
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
    let dipole = multi_vec!(D=5; Dipole => e41, e42, e43, e23, e31, e12, e15, e25, e35, e45);
    println!("{dipole}");

    let circle_again = multi_vec!(D=5; Circle => (e423, e431, e412, e321), (e415, e425, e435), (e235, e315, e125));
    println!("{circle_again}");

    let circle_again = multi_vec!(D=5; Circle => [e423, e431, e412, e321], [e415, e425, e435], [e235, e315, e125]);
    println!("{circle_again}");

    let dipole_again = multi_vec!(D=5; Dipole as e41, e42, e43 | e23, e31, e12 | e15, e25, e35, e45);
    println!("{dipole_again}");


    let mvs = multi_vecs!(D=5;
        Scalar      => [scalar];
        AntiScalar  => [e12345];
        DualNum     => [scalar, e12345];
    );
    println!("{mvs:?}");
    let mvs = multi_vecs!(D=5;
        FlatPoint   => (e15, e25, e35, e45);
        Line        => (e415, e425, e435), (e235, e315, e125);
        Plane       => (e4235, e4315, e4125, e3215);
    );
    println!("{mvs:?}");
    let mvs = multi_vecs!(D=5;
        RoundPoint  => e1, e2, e3, e4, e5;
        Dipole      => e41, e42, e43, e23, e31, e12, e15, e25, e35, e45;
        Circle      => e423, e431, e412, e321, e415, e425, e435, e235, e315, e125;
        Sphere      => e1234, e4235, e4315, e4125, e3215;
    );
    println!("{mvs:?}");
    let mvs = multi_vecs!(D=5;
        Motor       as e415, e425, e435, e12345 | e235, e315, e125;
        Flector     as e15, e25, e35, e45 | e4235, e4315, e4125, e3215;
    );
    println!("{mvs:?}");
}


pub struct FallbackWasUsed(AtomSetOnce<Box<()>>);
impl FallbackWasUsed {
    pub fn new() -> Self {
        FallbackWasUsed(AtomSetOnce::empty())
    }

    pub fn has_been_used(&self) -> bool {
        !self.0.is_none(Ordering::AcqRel)
    }

    pub fn mark_used(&self) {
        self.0.set_if_none(Box::new(()), Ordering::AcqRel);
    }
}


pub struct DeclareMultiVecs {
    ga: Arc<GeometricAlgebra>,
    anti_scalar_sig: BasisSignature,
    declared: BTreeMap<MultiVecSignatureEnum, Arc<MultiVecEnum>>,
}

impl DeclareMultiVecs {
    pub fn declare<const D: u8>(&mut self, multi_vec: MultiVec<D>) where
        MultiVec<D>: ConsolidateEnum<Output=MultiVecEnum>,
        MultiVecSignature<D>: ConsolidateEnum<Output=MultiVecSignatureEnum>,
        [(); mono_grade_elements(D)]: Sized,
        [(); mono_grade_groups(D)]: Sized,
        [(); num_elements(D)]: Sized {

        let anti_scalar = self.ga.anti_scalar();
        let gr = anti_scalar.grade();
        if D as u32 != gr {
            panic!("Cannot create a MultiVec of D={D} using GeometricAlgebra of dimension {gr}");
        }

        for el in multi_vec.elements() {
            if !self.anti_scalar_sig.contains(el.signature()) {
                let anti_scalar = self.ga.anti_scalar();
                panic!("Element does not fit in anti_scalar {anti_scalar}: {el} in {multi_vec}");
            }
        }

        self.declared.insert(
            multi_vec.signature.consolidate_enum(),
            Arc::new(multi_vec.consolidate_enum())
        );
    }

    pub fn new(ga: Arc<GeometricAlgebra>) -> Self {
        let anti_scalar = ga.anti_scalar();
        DeclareMultiVecs {
            ga,
            anti_scalar_sig: anti_scalar.signature(),
            declared: BTreeMap::new(),
        }
    }
}


pub struct MultiVecRepository {
    declarations: BTreeMap<MultiVecSignatureEnum, Arc<MultiVecEnum>>,
    fallback: BTreeMap<MultiVecSignatureEnum, (FallbackWasUsed, Arc<MultiVecEnum>)>,
    wanted: Mutex<BTreeMap<
        MultiVecSignatureEnum,
        Vec<Arc<RawTraitImplementation>>>>,
    strongly_wanted: Mutex<BTreeMap<
        MultiVecSignatureEnum,
        Vec<Arc<RawTraitImplementation>>>>
}


impl MultiVecRepository {

    pub fn default<const D: u8>(ga: Arc<GeometricAlgebra>) -> Arc<Self> where
        MultiVec<D>: ConsolidateEnum<Output=MultiVecEnum>,
        [(); mono_grade_elements(D)]: Sized,
        [(); mono_grade_groups(D)]: Sized,
        [(); num_elements(D)]: Sized {
        let dec = DeclareMultiVecs::new(ga);
        Self::new(dec)
    }

    pub fn new<const D: u8>(declarations: DeclareMultiVecs) -> Arc<Self> where
        MultiVec<D>: ConsolidateEnum<Output=MultiVecEnum>,
        [(); mono_grade_elements(D)]: Sized,
        [(); mono_grade_groups(D)]: Sized,
        [(); num_elements(D)]: Sized {

        let ga = declarations.ga.clone();
        let anti_scalar = ga.anti_scalar();
        let gr = anti_scalar.grade();
        if D != gr as u8 {
            panic!("Please create your MultiVectorRepository ({D}) using the same dimensionality \
            as your GeometricAlgebra ({gr})")
        }

        let mut mvr = MultiVecRepository {
            declarations: declarations.declared,
            fallback: BTreeMap::new(),
            wanted: Mutex::new(BTreeMap::new()),
            strongly_wanted: Mutex::new(BTreeMap::new()),
        };

        // Generate fallback types.
        let all_elements: Vec<_> = ga.all_elements().map(|el| ga.name_and_sign_out(el)).collect();

        use crate::algebra2::basis::elements::*;
        mvr.fallback_multi_vec(MultiVec::<D>::new("Scalar", [scalar]).consolidate_enum());
        mvr.fallback_multi_vec(MultiVec::<D>::new("AntiScalar", [anti_scalar]).consolidate_enum());
        mvr.fallback_multi_vec(MultiVec::<D>::new("DualNum", [scalar, anti_scalar]).consolidate_enum());
        mvr.fallback_multi_vec(MultiVec::<D>::new("MultiVector", all_elements.clone()).consolidate_enum());

        // 1..D skips scalar and anti_scalar
        for gr in 1..D {
            let els: Vec<_> = all_elements.clone().filter(|el| el.grade() == gr as u32).collect();
            if els.is_empty() {
                // This shouldn't happen because we checked D against anti_scalar.grade() already
                // (why bother checking at all then? Well let's not go feeding junk data to
                //  MultiVec::new for fun)
                unreachable!("There are no BasisElements of grade {gr} for our GA of D={D}")
            }
            let mv = match gr {
                // 0 is Scalar, defined above
                1 => MultiVec::<D>::new("Vector", els),
                2 => MultiVec::<D>::new("BiVector", els),
                3 => MultiVec::<D>::new("TriVector", els),
                4 => MultiVec::<D>::new("QuadVector", els),
                5 => MultiVec::<D>::new("VectorGr5", els),
                6 => MultiVec::<D>::new("VectorGr6", els),
                7 => MultiVec::<D>::new("VectorGr7", els),
                8 => MultiVec::<D>::new("VectorGr8", els),
                9 => MultiVec::<D>::new("VectorGr9", els),
                10 => MultiVec::<D>::new("VectorGr10", els),
                11 => MultiVec::<D>::new("VectorGr11", els),
                12 => MultiVec::<D>::new("VectorGr12", els),
                13 => MultiVec::<D>::new("VectorGr13", els),
                14 => MultiVec::<D>::new("VectorGr14", els),
                15 => MultiVec::<D>::new("VectorGr15", els),
                // 16 would be AntiScalar, defined above

                // This shouldn't really be possible because of the constraint
                // MultiVec<D>: ConsolidateEnum<Output=MultiVecEnum>
                _ => unreachable!("MultiVecs of D<0 or D>16 are not supported"),
            };
            mvr.fallback_multi_vec(mv.consolidate_enum());
        }

        Arc::new(mvr)
    }

    fn fallback_multi_vec(&mut self, multi_vec: MultiVecEnum) {
        self.fallback.insert(
            multi_vec.signature,
            (FallbackWasUsed::new(), Arc::new(multi_vec)));
    }

    pub fn get_at_least(self: Arc<Self>, signature: MultiVecSignature<D>) -> MultiVec<D> {
        todo!()
    }

    pub fn get_exact(self: Arc<Self>, signature: MultiVecSignature<D>) -> Option<MultiVec<D>> {
        todo!()
    }
}






//
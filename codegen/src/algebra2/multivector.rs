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
    (box $impl_type:ident $variant_prefix:ident => $( $variants:literal ),+ $(,)?) => {
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
    ($impl_type:ident $variant_prefix:ident => $( $variants:literal ),+ $(,)?) => {
        paste! {
            #[derive(Clone, PartialEq, Eq, Hash, Debug)]
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

consolidate_enum!(box MultiVec D => 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

impl MultiVecEnum {
    pub fn adapt_eq<const D: u8>(&self, other: &MultiVec<D>) -> bool where
        MultiVec<D>: ConsolidateEnum<Output=MultiVecEnum>,
        [(); mono_grade_groups(D)]: Sized,
        [(); num_elements(D)]: Sized {

        self == &other.clone().consolidate_enum()
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


pub struct DeclareMultiVecs<const D: u8> where
    [(); mono_grade_groups(D)]: Sized,
    [(); num_elements(D)]: Sized {

    ga: Arc<GeometricAlgebra>,
    anti_scalar_sig: BasisSignature,

    declared: BTreeMap<MultiVectorSignature<D>, MultiVec<D>>,
    fallback: BTreeMap<MultiVectorSignature<D>, (FallbackWasUsed, MultiVec<D>)>,

    wanted: Mutex<BTreeMap<
        MultiVectorSignature<D>,
        Vec<Arc<RawTraitImplementation>>>>,
    strongly_wanted: Mutex<BTreeMap<
        MultiVectorSignature<D>,
        Vec<Arc<RawTraitImplementation>>>>
}

impl<const D: u8> DeclareMultiVecs<D> where
    [(); mono_grade_elements(D)]: Sized,
    [(); mono_grade_groups(D)]: Sized,
    [(); num_elements(D)]: Sized {

    pub fn get_at_least(self: Arc<Self>, signature: MultiVectorSignature<D>) -> MultiVec<D> {
        todo!()
    }

    pub fn get_exact(self: Arc<Self>, signature: MultiVectorSignature<D>) -> Option<MultiVec<D>> {
        todo!()
    }

    pub fn declare(&mut self, multi_vec: MultiVec<D>) {
        self.ga.internalize_element_names(&multi_vec);

        // Is it really okay to mutate our fallbacks like this?
        // Yes, because right now we have a &mut MultiVecRepositoryD<D>, where you can
        // only put MultiVecs in, but not pull any out. You can't actually pull out any
        // MultiVecs out (and thereby create a dependency on the directions of BasisElements in
        // fallback MultiVecs) until you turn the repository immutable by wrapping it into an
        // Arc<MultiVecRepositoryD<D>>

        for (_, (_, mv)) in self.fallback.iter_mut() {
            for group in mv.element_groups.iter_mut() {
                for el in group.iter_mut() {
                    *el = self.ga.name_and_sign_out(*el);
                }
            }
        }
        self.declared.insert(multi_vec.signature, multi_vec);
    }
    fn fallback_multi_vec(&mut self, multi_vec: MultiVec<D>) {
        self.fallback.insert(multi_vec.signature, (FallbackWasUsed::new(), multi_vec));
    }

    pub fn new(ga: Arc<GeometricAlgebra>) -> Self {
        let anti_scalar = ga.anti_scalar();
        let gr = anti_scalar.grade();
        if D as u32 != gr {
            panic!("Cannot create a MultiVec of D={D} using GeometricAlgebra of dimension {gr}");
        }
        let mut mvr = DeclareMultiVecs {
            ga: ga.clone(),
            anti_scalar_sig: anti_scalar.signature(),
            declared: BTreeMap::new(),
            fallback: BTreeMap::new(),
            wanted: Mutex::new(BTreeMap::new()),
            strongly_wanted: Mutex::new(BTreeMap::new()),
        };

        // Generate fallback types.
        use crate::algebra2::basis::elements::*;
        mvr.fallback_multi_vec(MultiVec::<D>::new("Scalar", [scalar]));
        mvr.fallback_multi_vec(MultiVec::<D>::new("AntiScalar", [anti_scalar]));
        mvr.fallback_multi_vec(MultiVec::<D>::new("DualNum", [scalar, anti_scalar]));
        mvr.fallback_multi_vec(MultiVec::<D>::new("MultiVector", ga.all_elements()));
        // 1..D skips scalar and anti_scalar
        for gr in 1..D {
            let els: Vec<_> = ga.all_elements().filter(|el| el.grade() == gr as u32).collect();
            if els.is_empty() {
                panic!("There are no BasisElements of grade {gr} for our GA of D={D}")
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
                _ => panic!("MultiVecs of D<0 or D>16 are not supported"),
            };
            mvr.fallback_multi_vec(mv);
        }

        mvr
    }
}

pub enum MultiVecRepository {
    D1(DeclareMultiVecs<1>),
    D2(DeclareMultiVecs<2>),
    D3(DeclareMultiVecs<3>),
    D4(DeclareMultiVecs<4>),
    D5(DeclareMultiVecs<5>),
    D6(DeclareMultiVecs<6>),
    D7(DeclareMultiVecs<7>),
    D8(DeclareMultiVecs<8>),
    D9(DeclareMultiVecs<9>),
    D10(DeclareMultiVecs<10>),
    D11(DeclareMultiVecs<11>),
    D12(DeclareMultiVecs<12>),
    D13(DeclareMultiVecs<13>),
    D14(DeclareMultiVecs<14>),
    D15(DeclareMultiVecs<15>),
    D16(DeclareMultiVecs<16>),
}


// TODO maybe only allow adding declarations to the repo with const generic, and
//  only allow pulling out MultiVecs from this one without const generic
impl MultiVecRepository {
    pub fn declare_multi_vec<const D: u8>(&mut self, multi_vec: MultiVec<D>) where
        [(); mono_grade_groups(D)]: Sized,
        [(); num_elements(D)]: Sized {

        match (D, self) {
            (1, D1(mvr)) => {

            }
            (2, D1(mvr)) => {

            }
            (3, D1(mvr)) => {

            }
            (4, D1(mvr)) => {

            }
            (5, D1(mvr)) => {

            }
            (6, D1(mvr)) => {

            }
            (7, D1(mvr)) => {

            }
            (8, D1(mvr)) => {

            }
            (9, D1(mvr)) => {

            }
            (10, D1(mvr)) => {

            }
            (11, D1(mvr)) => {

            }
            (12, D1(mvr)) => {

            }
            (13, D1(mvr)) => {

            }
            (14, D1(mvr)) => {

            }
            (15, D1(mvr)) => {

            }
            (16, D1(mvr)) => {

            }
            (_, _) => panic!("MultiVecs of D<0 or D>16 are not supported"),
        }
    }
}






//
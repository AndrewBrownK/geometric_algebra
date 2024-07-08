#![allow(non_upper_case_globals)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use std::sync::atomic::Ordering;

use atom::AtomSetOnce;
use const_panic::concat_panic;
use parking_lot::Mutex;

use crate::algebra2::basis::{BasisElement, BasisSignature};
use crate::algebra2::basis::elements::*;
use crate::algebra2::basis::grades::Grades;
use crate::algebra2::GeometricAlgebra;
use crate::ast2::datatype::MultiVector;
use crate::ast2::traits::RawTraitImplementation;
use crate::utility::ConstVec;

// We COULD use { qty_groups(AntiScalar) } everywhere to specify the size of
// ConstVec<BasisElement, N>. And this would make the arrays only as small as necessary. However,
// then we have to infect everything with the constraint:
// where [(); { qty_groups(AntiScalar) }]: Sized
// I wouldn't mind handling that constraint to some extents.... except the infection goes too far.
// It's one thing to add the constraint to MultiVec methods, it is another thing to infect the
// TraitImplBuilder API. So instead we are going to hard code the qty so that we don't need
// a size constraint.
// Assuming in all cases the average elements per group is at least 3 (and for margin error +1).
// 16 dimensions: 2^16 = 65536 -> (n / 3) + 1 = 21846
// 12 dimensions: 2^12 = 4096 -> (n / 3) + 1 = 1366
//  8 dimensions: 2^8  = 256 -> (n / 3) + 1 = 86

#[cfg(feature = "very-large-basis-elements")]
pub const QTY_GROUPS: usize = 21846;
#[cfg(all(feature = "large-basis-elements", not(feature = "very-large-basis-elements")))]
pub const QTY_GROUPS: usize = 1366;
#[cfg(not(feature = "large-basis-elements"))]
pub const QTY_GROUPS: usize = 86;



#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum BasisElementGroup {
    G1(BasisElement),
    G2(BasisElement, BasisElement),
    G3(BasisElement, BasisElement, BasisElement),
    G4(BasisElement, BasisElement, BasisElement, BasisElement),
}
#[derive(Copy, Clone)]
pub struct BasisElementGroupIter(usize, BasisElement, [Option<BasisElement>; 3]);
impl IntoIterator for BasisElementGroup {
    type Item = BasisElement;
    type IntoIter = BasisElementGroupIter;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            BasisElementGroup::G1(a) => BasisElementGroupIter(0, a, [None; 3]),
            BasisElementGroup::G2(a, b) => BasisElementGroupIter(0, a, [Some(b), None, None]),
            BasisElementGroup::G3(a, b, c) => BasisElementGroupIter(0, a, [Some(b), Some(c), None]),
            BasisElementGroup::G4(a, b, c, d) => BasisElementGroupIter(0, a, [Some(b), Some(c), Some(d)]),
        }
    }
}
impl Iterator for BasisElementGroupIter {
    type Item = BasisElement;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.0 {
            0 => Some(self.1),
            1 => self.2[0],
            2 => self.2[1],
            3 => self.2[2],
            _ => None
        };
        self.0 += 1;
        result
    }
}
impl BasisElementGroup {
    pub const fn can_push(&self) -> bool {
        match self {
            BasisElementGroup::G4(_, _, _, _) => false,
            _ => true,
        }
    }

    pub const fn push(&mut self, el: BasisElement) {
        *self = match self {
            BasisElementGroup::G1(a) => BasisElementGroup::G2(*a, el),
            BasisElementGroup::G2(a, b) => BasisElementGroup::G3(*a, *b, el),
            BasisElementGroup::G3(a, b, c) => BasisElementGroup::G4(*a, *b, *c, el),
            _ => panic!("Please check can_push() before using push()")
        }
    }

    pub const fn const_clone(&self) -> Self {
        match self {
            BasisElementGroup::G1(a) => BasisElementGroup::G1(*a),
            BasisElementGroup::G2(a, b) => BasisElementGroup::G2(*a, *b),
            BasisElementGroup::G3(a, b, c) => BasisElementGroup::G3(*a, *b, *c),
            BasisElementGroup::G4(a, b, c, d) => BasisElementGroup::G4(*a, *b, *c, *d),
        }
    }

    pub const fn from_vec(v: ConstVec<BasisElement, 4>) -> Self {
        match v.len() {
            1 => BasisElementGroup::G1(*v.get(0)),
            2 => BasisElementGroup::G2(*v.get(0), *v.get(1)),
            3 => BasisElementGroup::G3(*v.get(0), *v.get(1), *v.get(2)),
            4 => BasisElementGroup::G4(*v.get(0), *v.get(1), *v.get(2), *v.get(3)),
            0 => panic!("Cannot create empty BasisElementGroup"),
            _ => panic!("Unreachable: ConstVec has type level size")
        }
    }

    pub const fn into_vec(self) -> ConstVec<BasisElement, 4> {
        match self {
            BasisElementGroup::G1(a) => {
                let mut v = ConstVec::new();
                v.push(a);
                v
            }
            BasisElementGroup::G2(a, b) =>{
                let mut v = ConstVec::new();
                v.push(a);
                v.push(b);
                v
            }
            BasisElementGroup::G3(a, b, c) => {
                let mut v = ConstVec::new();
                v.push(a);
                v.push(b);
                v.push(c);
                v
            }
            BasisElementGroup::G4(a, b, c, d) => {
                let mut v = ConstVec::new();
                v.push(a);
                v.push(b);
                v.push(c);
                v.push(d);
                v
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MultiVec<const AntiScalar: BasisElement> {
    name: &'static str,
    grades: Grades,
    element_groups: ConstVec<BasisElementGroup, QTY_GROUPS>,
}

impl<const AntiScalar: BasisElement> Debug for MultiVec<AntiScalar> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.name;
        write!(f, "MultiVec {{ name: \"{n}\", element_groups: [")?;
        let mut i = 0;
        while i < self.element_groups.len() {
            let group = self.element_groups.get(i);
            let comma = if i == 0 { "" } else { ", " };
            write!(f, "{comma}[")?;
            let mut j = 0;
            let gr = group.clone().into_vec();
            while j < gr.len() {
                let el = gr.get(j);
                let comma = if j == 0 { "" } else { ", " };
                write!(f, "{comma}{el}")?;
                j += 1;
            }
            write!(f, "]")?;
            i += 1;
        }
        write!(f, "] }}")
    }
}
impl<const AntiScalar: BasisElement> Display for MultiVec<AntiScalar> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.name;
        write!(f, "{n}(")?;
        let mut i = 0;
        while i < self.element_groups.len() {
            let group = self.element_groups.get(i);
            let comma = if i == 0 { "" } else { ", " };
            write!(f, "{comma}(")?;
            let mut j = 0;
            let gr = group.clone().into_vec();
            while j < gr.len() {
                let comma = if j == 0 { "" } else { ", " };
                let el = gr.get(j);
                write!(f, "{comma}{el}")?;
                j += 1;
            }
            write!(f, ")")?;
            i += 1;
        }
        write!(f, ")")
    }
}



impl<const AntiScalar: BasisElement> MultiVec<AntiScalar> {

    pub fn fmt_for_macro(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.name;
        write!(f, "{n} as ")?;
        let mut i = 0;
        while i < self.element_groups.len() {
            let group = self.element_groups.get(i).into_vec();
            let group_separator = if i == 0 { "" } else { " | " };
            write!(f, "{group_separator}")?;
            let mut j = 0;
            while j < group.len() {
                let el = group.get(j);
                let comma = if j == 0 { "" } else { ", " };
                write!(f, "{comma}{el}")?;
                j += 1;
            }
            i += 1;
        }
        write!(f, ";")
    }

    pub fn elements(&self) -> Vec<BasisElement> {
        let mut v = vec![];
        let mut i = 0;
        while i < self.element_groups.len() {
            let group = self.element_groups.get(i).clone().into_vec();
            let mut j = 0;
            while j < group.len() {
                v.push(*group.get(j));
                j += 1;
            }
            i += 1;
        }
        v
    }

    pub fn groups(&self) -> &ConstVec<BasisElementGroup, QTY_GROUPS> {
        &self.element_groups
    }

    pub fn new<E: IntoIterator<Item=BasisElement>>(name: &'static str, elements: E) -> Self {
        let mut elements = elements.into_iter().collect::<Vec<_>>();
        elements.sort();
        let mut active_grade = elements.get(0).map(|it| it.grade()).unwrap_or(0);
        let mut grouped = ConstVec::<BasisElementGroup, QTY_GROUPS>::new();

        let mut group_as_vec = ConstVec::<BasisElement, 4>::new();

        let mut i = 0;
        while i < elements.len() {
            let element = elements[i];
            let element_grade = element.grade();
            if element_grade == active_grade {
                // Same grade, same group
                group_as_vec.push(element);
            } else {
                // New grade, new group.
                // If you want extra-compact grouping, then use new_by_groups with manually
                // specified groups instead. Or I guess we could add more heuristics here
                // at some point, but not feeling rushed for it.
                grouped.push(BasisElementGroup::from_vec(group_as_vec));
                group_as_vec = ConstVec::<BasisElement, 4>::new();
                active_grade = element_grade;

                group_as_vec.push(element);
            }

            if group_as_vec.len() >= 4 {
                grouped.push(BasisElementGroup::from_vec(group_as_vec));
                group_as_vec = ConstVec::<BasisElement, 4>::new();
            }
            i += 1;
        }
        if group_as_vec.len() > 0 {
            grouped.push(BasisElementGroup::from_vec(group_as_vec));
        }
        Self::new_by_groups(name, grouped)
    }
}



impl <const AntiScalar: BasisElement> MultiVec<AntiScalar> {
    pub const fn new_by_groups(name: &'static str, element_groups: ConstVec<BasisElementGroup, QTY_GROUPS>) -> Self {
        if ((AntiScalar.grade() / 3) + 1) as usize > QTY_GROUPS {
            panic!("If you want to create an 9-12 dimensional GA, then enable the \
            \"large-basis-elements\" feature. If you want to create a 13-16 dimensional GA, then \
            enable the \"very-large-basis-elements\" feature.")
        }

        let mut used_signatures = ConstVec::<BasisElement, QTY_GROUPS>::new();
        let mut grades = Grades::none;
        let mut i = 0;
        while i < element_groups.len() {
            let group = element_groups.get(i);
            let group_vec = group.const_clone().into_vec();
            let mut j = 0;
            while j < group_vec.len() {
                let el = group_vec.get(j);
                let el_sig = el.signature();
                if !AntiScalar.signature().contains(el_sig) {
                    concat_panic!("MultiVector belonging to AntiScalar(", AntiScalar, ") \
                        is defined to include ", el, " which does not fit. ");
                }
                let mut k = 0;
                while k < used_signatures.len() {
                    let u = used_signatures.get(k);
                    let u_sig = u.signature();
                    let u_sig_bits = u_sig.bits();
                    let el_sig_bits = el_sig.bits();
                    if u_sig_bits == el_sig_bits {
                        concat_panic!("MultiVec named ", name, " already has ", el, ". Do not \
                            define MultiVectors using redundant or duplicate BasisSignatures. \
                            Don't forget that reordered or sign flipped BasisElements can share \
                            the same BasisSignature. ")
                    }
                    k += 1;
                }
                used_signatures.push(*el);
                grades = grades.const_bitor(Grades::from_sig(el_sig));
                j += 1;
            }
            i += 1;
        }
        MultiVec { name, grades, element_groups, }
    }
}



#[macro_export]
macro_rules! multi_vec {
    // grouped using tuples
    ($mv_name:ident<$anti_scalar:ident> => $( ($($basis_element:ident),+ $(,)?)),+ $(,)?) => {
        {
            use $crate::algebra2::basis::elements::*;
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra2::multivector::BasisElementGroup,
                { $crate::algebra2::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra2::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra2::multivector::MultiVec::<{$anti_scalar}>::new_by_groups(name, groups)
        }
    };
    // grouped using arrays
    ($mv_name:ident<$anti_scalar:ident> => $( [$($basis_element:ident),+ $(,)?]),+ $(,)?) => {
        {
            use $crate::algebra2::basis::elements::*;
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra2::multivector::BasisElementGroup,
                { $crate::algebra2::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra2::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra2::multivector::MultiVec::<{$anti_scalar}>::new_by_groups(name, groups)
        }
    };
    // ungrouped list of BasisElement
    ($mv_name:ident<$anti_scalar:ident> => $( $basis_element:ident ),+ $(,)?) => {
        {
            // Allocations are not allowed in static/const, but can't be bothered to make a
            // compatible version when specifying groups is fine instead.
            use $crate::algebra2::basis::elements::*;
            let name: &'static str = stringify!($mv_name);
            let elements: std::vec::Vec<$crate::algebra2::basis::BasisElement> = vec![
                $($basis_element),+,
            ];
            $crate::algebra2::multivector::MultiVec::<{$anti_scalar}>::new(name, elements)
        }
    };
    // Elegant and sparse
    ($mv_name:ident<$anti_scalar:ident> as $( $($basis_element:ident),+ $(,)?)|+ ) => {
        {
            use $crate::algebra2::basis::elements::*;
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra2::multivector::BasisElementGroup,
                { $crate::algebra2::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra2::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra2::multivector::MultiVec::<$anti_scalar>::new_by_groups(name, groups)
        }
    };
}

#[macro_export]
macro_rules! multi_vecs {
    // Grouped using tuples
    ($anti_scalar:ident; $( $mv_name:ident => $( ($($basis_element:ident),+ $(,)?)),+ $(,)? );+ $(;)?) => {
        $(
        pub static $mv_name: $crate::algebra2::multivector::MultiVec<{$anti_scalar}> = {
            use $crate::algebra2::basis::elements::*;
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra2::multivector::BasisElementGroup,
                { $crate::algebra2::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra2::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra2::multivector::MultiVec::<{$anti_scalar}>::new_by_groups(name, groups)
        };
        )+
        pub fn register_multi_vecs(ga: std::sync::Arc<$crate::algebra2::GeometricAlgebra<{$anti_scalar}>>) -> $crate::algebra2::multivector::DeclareMultiVecs<{$anti_scalar}> {
            $crate::algebra2::multivector::DeclareMultiVecs::declare(ga, [
                $(&$mv_name,)+
            ])
        }
    };
    // Grouped using arrays
    ($anti_scalar:ident; $( $mv_name:ident => $( [$($basis_element:ident),+ $(,)?]),+ $(,)? );+ $(;)?) => {
        use $crate::algebra2::basis::elements::*;
        $(
        pub static $mv_name: $crate::algebra2::multivector::MultiVec<{$anti_scalar}> = {
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra2::multivector::BasisElementGroup,
                { $crate::algebra2::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra2::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra2::multivector::MultiVec::<{$anti_scalar}>::new_by_groups(name, groups)
        };
        )+
        pub fn register_multi_vecs(ga: std::sync::Arc<$crate::algebra2::GeometricAlgebra<{$anti_scalar}>>) -> $crate::algebra2::multivector::DeclareMultiVecs<{$anti_scalar}> {
            $crate::algebra2::multivector::DeclareMultiVecs::declare(ga, [
                $(&$mv_name, )+
            ])
        }
    };
    // Elegant and sparse
    ($anti_scalar:ident; $( $mv_name:ident as $( $($basis_element:ident),+ $(,)?)|+ );+ $(;)?) => {
        use $crate::algebra2::basis::elements::*;
        $(
        pub static $mv_name: $crate::algebra2::multivector::MultiVec<{$anti_scalar}> = {
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra2::multivector::BasisElementGroup,
                { $crate::algebra2::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra2::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra2::multivector::MultiVec::<$anti_scalar>::new_by_groups(name, groups)
        };
        )+
        pub fn register_multi_vecs(ga: std::sync::Arc<$crate::algebra2::GeometricAlgebra<{$anti_scalar}>>) -> $crate::algebra2::multivector::DeclareMultiVecs<{$anti_scalar}> {
            $crate::algebra2::multivector::DeclareMultiVecs::declare(ga, [
                $(&$mv_name, )+
            ])
        }
    };
}



#[const_trait]
pub trait TupleToGroup {
    fn tuple_to_group(self) -> BasisElementGroup;
}
impl const TupleToGroup for (BasisElement,) {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G1(self.0)
    }
}
impl const TupleToGroup for (BasisElement, BasisElement) {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G2(self.0, self.1)
    }
}
impl const TupleToGroup for (BasisElement, BasisElement, BasisElement) {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G3(self.0, self.1, self.2)
    }
}
impl const TupleToGroup for (BasisElement, BasisElement, BasisElement, BasisElement) {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G4(self.0, self.1, self.2, self.3)
    }
}
impl const TupleToGroup for [BasisElement; 4] {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G4(self[0], self[1], self[2], self[3])
    }
}
impl const TupleToGroup for [BasisElement; 3] {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G3(self[0], self[1], self[2])
    }
}
impl const TupleToGroup for [BasisElement; 2] {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G2(self[0], self[1])
    }
}
impl const TupleToGroup for [BasisElement; 1] {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G1(self[0])
    }
}


static Circle: MultiVec<{e12345}> = MultiVec::<e12345>::new_by_groups("Circle", {
    let mut cv = ConstVec::new();
    cv.push((e423, e431, e412, e321).tuple_to_group());
    cv.push((e415, e425, e435).tuple_to_group());
    cv.push((e235, e315, e125).tuple_to_group());
    cv
});

// Allocations are not allowed in static/const, but can't be bothered to make a compatible version
// when specifying groups is fine instead.
// static Dipole: MultiVec<{e12345}> = multi_vec!(Dipole<e12345> => e41, e42, e43, e23, e31, e12, e15, e25, e35, e45);
static Circle2: MultiVec<{e12345}> = multi_vec!(Circle<e12345> => (e423, e431, e412, e321), (e415, e425, e435), (e235, e315, e125));
static Circle3: MultiVec<{e12345}> = multi_vec!(Circle<e12345> => [e423, e431, e412, e321], [e415, e425, e435], [e235, e315, e125]);

static Dipole2: MultiVec<{e12345}> = multi_vec!(Dipole<e12345> as e41, e42, e43 | e23, e31, e12 | e15, e25, e35, e45);

#[test]
fn test_construction() {
    println!("{Circle:?}");
    println!("{Circle}");

    let circle_again = &Circle2;
    println!("{circle_again}");

    let circle_again = &Circle3;
    println!("{circle_again}");

    let dipole_again = &Dipole2;
    println!("{dipole_again}");
}


pub struct FallbackWasUsed(AtomSetOnce<Box<()>>);
impl FallbackWasUsed {
    pub fn new() -> Self {
        FallbackWasUsed(AtomSetOnce::empty())
    }

    pub fn has_been_used(&self) -> bool {
        !self.0.is_none(Ordering::AcqRel)
    }

    /// Returns true if you won the race
    pub fn mark_used(&self) -> bool {
        self.0.set_if_none(Box::new(()), Ordering::AcqRel).is_none()
    }
}


pub struct DeclareMultiVecs<const AntiScalar: BasisElement> {
    ga: Arc<GeometricAlgebra<AntiScalar>>,
    anti_scalar_sig: BasisSignature,
    declared: Vec<&'static MultiVec<AntiScalar>>,
}

impl<const AntiScalar: BasisElement> DeclareMultiVecs<AntiScalar> {
    pub fn declare<const N: usize>(
        ga: Arc<GeometricAlgebra<AntiScalar>>,
        multi_vecs: [&'static MultiVec<AntiScalar>; N],
    ) -> Self {
        // TODO disallow duplicated multivector names

        let mut nb = ga.named_bases.write();
        let mut declared = vec![];
        for multi_vec in multi_vecs {
            for el in multi_vec.elements() {
                if !AntiScalar.signature().contains(el.signature()) {
                    panic!("Element does not fit in anti_scalar {AntiScalar}: {el} in {multi_vec}");
                }
                match nb.accept_name(el) {
                    Ok(_) => {}
                    Err(err) => panic!("Could not accept BasisElement {el}: {err}"),
                }
            }
            declared.push(multi_vec);
        }
        drop(nb);
        DeclareMultiVecs {
            ga,
            anti_scalar_sig: AntiScalar.signature(),
            declared,
        }
    }

    pub fn new(ga: Arc<GeometricAlgebra<AntiScalar>>) -> Self {
        let anti_scalar = ga.anti_scalar();
        DeclareMultiVecs {
            ga,
            anti_scalar_sig: anti_scalar.signature(),
            declared: vec![],
        }
    }

    pub fn finished(self) -> Arc<MultiVecRepository<AntiScalar>> {
        MultiVecRepository::new(self)
    }

    // TODO some methods to dynamically generate some MultiVecs e.g. OnOrigin or AtInfinity variants
}


pub struct MultiVecRepository<const AntiScalar: BasisElement> {
    declarations: DeclareMultiVecs<AntiScalar>,
    uniform_grade_groups: BTreeMap<BasisSignature, Vec<&'static BasisElementGroup>>,
    mixed_grade_groups: BTreeMap<(BasisSignature, Grades), Vec<&'static BasisElementGroup>>,

    fallback: Vec<(FallbackWasUsed, &'static MultiVec<AntiScalar>)>,

    // TODO I might need a MultiVecSignature type thing here instead of named MultiVec
    wanted: Mutex<Vec<(&'static MultiVec<AntiScalar>, Vec<Arc<RawTraitImplementation>>)>>,
    strongly_wanted: Mutex<Vec<(&'static MultiVec<AntiScalar>, Vec<Arc<RawTraitImplementation>>)>>,
}


impl<const AntiScalar: BasisElement> MultiVecRepository<AntiScalar> {
    pub fn default(ga: Arc<GeometricAlgebra<AntiScalar>>) -> Arc<Self> {
        Self::new(DeclareMultiVecs::new(ga))
    }

    pub fn new(declarations: DeclareMultiVecs<AntiScalar>) -> Arc<Self> {
        let ga = declarations.ga.clone();
        let mut mvr = MultiVecRepository {
            declarations,
            uniform_grade_groups: Default::default(),
            mixed_grade_groups: Default::default(),
            fallback: vec![],
            wanted: Default::default(),
            strongly_wanted: Default::default(),
        };

        // Generate fallback types.
        let all_elements: Vec<_> = ga.all_elements().map(|el| ga.name_and_sign_out(el)).collect();

        use crate::algebra2::basis::elements::*;
        mvr.fallback(MultiVec::<AntiScalar>::new("Scalar", [scalar]));
        mvr.fallback(MultiVec::<AntiScalar>::new("AntiScalar", [AntiScalar]));
        mvr.fallback(MultiVec::<AntiScalar>::new("DualNum", [scalar, AntiScalar]));
        mvr.fallback(MultiVec::<AntiScalar>::new("MultiVector", all_elements.clone()));

        // 1..AntiScalar.grade() skips scalar and anti_scalar (since we already added them)
        for gr in 1..AntiScalar.grade() {
            let els: Vec<_> = all_elements.clone().into_iter().filter(|el| el.grade() == gr).collect();
            let mv = match gr {
                // 0 is Scalar, defined above
                1 => MultiVec::<AntiScalar>::new("Vector", els),
                2 => MultiVec::<AntiScalar>::new("BiVector", els),
                3 => MultiVec::<AntiScalar>::new("TriVector", els),
                4 => MultiVec::<AntiScalar>::new("QuadVector", els),
                5 => MultiVec::<AntiScalar>::new("VectorGr5", els),
                6 => MultiVec::<AntiScalar>::new("VectorGr6", els),
                7 => MultiVec::<AntiScalar>::new("VectorGr7", els),
                8 => MultiVec::<AntiScalar>::new("VectorGr8", els),
                9 => MultiVec::<AntiScalar>::new("VectorGr9", els),
                10 => MultiVec::<AntiScalar>::new("VectorGr10", els),
                11 => MultiVec::<AntiScalar>::new("VectorGr11", els),
                12 => MultiVec::<AntiScalar>::new("VectorGr12", els),
                13 => MultiVec::<AntiScalar>::new("VectorGr13", els),
                14 => MultiVec::<AntiScalar>::new("VectorGr14", els),
                15 => MultiVec::<AntiScalar>::new("VectorGr15", els),
                // 16 would be AntiScalar, defined above

                // This isn't possible because max grade of AntiScalar is 16
                _ => unreachable!("MultiVecs of D<0 or D>16 are not supported"),
            };
            mvr.fallback(mv);
        }

        Arc::new(mvr)
    }

    fn fallback(&mut self, multi_vec: MultiVec<AntiScalar>) {
        self.fallback.push((
            FallbackWasUsed::new(),
            Box::leak(Box::new(multi_vec))
        ));
    }

    pub fn scalar(&self) -> Arc<MultiVec<AntiScalar>> {
        todo!()
    }

    pub fn anti_scalar(&self) -> Arc<MultiVec<AntiScalar>> {
        todo!()
    }

    pub fn full_multi_vector(&self) -> Arc<MultiVec<AntiScalar>> {
        todo!()
    }

    pub fn get_at_least(self: Arc<Self>, signature: BTreeSet<BasisSignature>) -> Arc<MultiVec<AntiScalar>> {
        todo!()
    }

    pub fn get_exact(self: Arc<Self>, signature: BTreeSet<BasisSignature>) -> Option<Arc<MultiVec<AntiScalar>>> {
        todo!()
    }
}






//
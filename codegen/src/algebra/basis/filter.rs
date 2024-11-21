use std::collections::BTreeSet;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitXor, Neg, Not};

use crate::algebra::basis::{BasisElement, BasisSignature};

pub trait SigFilter: Copy {
    fn filter_sig(&self, sig: BasisSignature) -> bool;
    fn all_match(self) -> SignatureSetFilter<Self> {
        SignatureSetFilter(self, AnyOrAll::All)
    }
    fn any_match(self) -> SignatureSetFilter<Self> {
        SignatureSetFilter(self, AnyOrAll::Any)
    }
}
pub trait SigSetFilter: Copy {
    fn filter_sig_set(&self, sigs: &BTreeSet<BasisSignature>) -> bool;
}

#[derive(Clone, Copy)]
pub struct SignatureFilter {
    require: Option<BasisSignature>,
    forbid: Option<BasisSignature>,
}
impl SigFilter for SignatureFilter {
    fn filter_sig(&self, sig: BasisSignature) -> bool {
        match (self.require, self.forbid) {
            (Some(r), Some(f)) => sig.contains(r) && !sig.contains(f),
            (Some(r), None) => sig.contains(r),
            (None, Some(f)) => !sig.contains(f),
            (None, None) => true,
        }
    }
}
pub fn allow_all_signatures() -> SignatureSetFilter<SignatureFilter> {
    SignatureSetFilter(SignatureFilter { require: None, forbid: None }, AnyOrAll::Any)
}
pub fn signatures_containing(signature_in_element: BasisElement) -> SignatureFilter {
    SignatureFilter {
        require: Some(signature_in_element.signature),
        forbid: None,
    }
}
pub fn signatures_not_containing(signature_in_element: BasisElement) -> SignatureFilter {
    SignatureFilter {
        require: None,
        forbid: Some(signature_in_element.signature),
    }
}
impl Not for SignatureFilter {
    type Output = SignatureFilter;

    fn not(self) -> Self::Output {
        let mut s = self.clone();
        s.forbid = self.require;
        s.require = self.forbid;
        s
    }
}

#[derive(Clone, Copy)]
pub struct FilterNot<A>(A);
impl<A: SigFilter> SigFilter for FilterNot<A> {
    fn filter_sig(&self, sig: BasisSignature) -> bool {
        !self.0.filter_sig(sig)
    }
}
impl<A: SigFilter, B: SigFilter> Not for FilterOr<A, B> {
    type Output = FilterNot<FilterOr<A, B>>;

    fn not(self) -> Self::Output {
        FilterNot(self)
    }
}
impl<A: SigFilter, B: SigFilter> Not for FilterAnd<A, B> {
    type Output = FilterNot<FilterAnd<A, B>>;

    fn not(self) -> Self::Output {
        FilterNot(self)
    }
}
impl<A: SigFilter, B: SigFilter> Not for FilterXor<A, B> {
    type Output = FilterNot<FilterXor<A, B>>;

    fn not(self) -> Self::Output {
        FilterNot(self)
    }
}
impl<A: SigFilter> Not for FilterNot<A> {
    type Output = A;

    fn not(self) -> Self::Output {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct FilterOr<A, B>(A, B);
impl<A: SigFilter, B: SigFilter> SigFilter for FilterOr<A, B> {
    fn filter_sig(&self, sig: BasisSignature) -> bool {
        self.0.filter_sig(sig) || self.1.filter_sig(sig)
    }
}
impl<A: SigFilter, B: SigFilter, C: SigFilter> BitOr<C> for FilterOr<A, B> {
    type Output = FilterOr<FilterOr<A, B>, C>;

    fn bitor(self, rhs: C) -> Self::Output {
        FilterOr(self, rhs)
    }
}
impl<A: SigFilter, B: SigFilter, C: SigFilter> BitOr<C> for FilterAnd<A, B> {
    type Output = FilterOr<FilterAnd<A, B>, C>;

    fn bitor(self, rhs: C) -> Self::Output {
        FilterOr(self, rhs)
    }
}
impl<A: SigFilter, B: SigFilter, C: SigFilter> BitOr<C> for FilterXor<A, B> {
    type Output = FilterOr<FilterXor<A, B>, C>;

    fn bitor(self, rhs: C) -> Self::Output {
        FilterOr(self, rhs)
    }
}
impl<A: SigFilter, C: SigFilter> BitOr<C> for FilterNot<A> {
    type Output = FilterOr<FilterNot<A>, C>;

    fn bitor(self, rhs: C) -> Self::Output {
        FilterOr(self, rhs)
    }
}
impl<C: SigFilter> BitOr<C> for SignatureFilter {
    type Output = FilterOr<SignatureFilter, C>;

    fn bitor(self, rhs: C) -> Self::Output {
        FilterOr(self, rhs)
    }
}

#[derive(Clone, Copy)]
pub struct FilterXor<A, B>(A, B);
impl<A: SigFilter, B: SigFilter> SigFilter for FilterXor<A, B> {
    fn filter_sig(&self, sig: BasisSignature) -> bool {
        self.0.filter_sig(sig) ^ self.1.filter_sig(sig)
    }
}
impl<A: SigFilter, B: SigFilter, C: SigFilter> BitXor<C> for FilterOr<A, B> {
    type Output = FilterXor<FilterOr<A, B>, C>;

    fn bitxor(self, rhs: C) -> Self::Output {
        FilterXor(self, rhs)
    }
}
impl<A: SigFilter, B: SigFilter, C: SigFilter> BitXor<C> for FilterAnd<A, B> {
    type Output = FilterXor<FilterAnd<A, B>, C>;

    fn bitxor(self, rhs: C) -> Self::Output {
        FilterXor(self, rhs)
    }
}
impl<A: SigFilter, B: SigFilter, C: SigFilter> BitXor<C> for FilterXor<A, B> {
    type Output = FilterXor<FilterXor<A, B>, C>;

    fn bitxor(self, rhs: C) -> Self::Output {
        FilterXor(self, rhs)
    }
}
impl<A: SigFilter, C: SigFilter> BitXor<C> for FilterNot<A> {
    type Output = FilterXor<FilterNot<A>, C>;

    fn bitxor(self, rhs: C) -> Self::Output {
        FilterXor(self, rhs)
    }
}
impl<C: SigFilter> BitXor<C> for SignatureFilter {
    type Output = FilterXor<SignatureFilter, C>;

    fn bitxor(self, rhs: C) -> Self::Output {
        FilterXor(self, rhs)
    }
}

#[derive(Clone, Copy)]
pub struct FilterAnd<A, B>(A, B);
impl<A: SigFilter, B: SigFilter> SigFilter for FilterAnd<A, B> {
    fn filter_sig(&self, sig: BasisSignature) -> bool {
        self.0.filter_sig(sig) && self.1.filter_sig(sig)
    }
}
impl<A: SigFilter, B: SigFilter, C: SigFilter> BitAnd<C> for FilterOr<A, B> {
    type Output = FilterAnd<FilterOr<A, B>, C>;

    fn bitand(self, rhs: C) -> Self::Output {
        FilterAnd(self, rhs)
    }
}
impl<A: SigFilter, B: SigFilter, C: SigFilter> BitAnd<C> for FilterAnd<A, B> {
    type Output = FilterAnd<FilterAnd<A, B>, C>;

    fn bitand(self, rhs: C) -> Self::Output {
        FilterAnd(self, rhs)
    }
}
impl<A: SigFilter, B: SigFilter, C: SigFilter> BitAnd<C> for FilterXor<A, B> {
    type Output = FilterAnd<FilterXor<A, B>, C>;

    fn bitand(self, rhs: C) -> Self::Output {
        FilterAnd(self, rhs)
    }
}
impl<A: SigFilter, C: SigFilter> BitAnd<C> for FilterNot<A> {
    type Output = FilterAnd<FilterNot<A>, C>;

    fn bitand(self, rhs: C) -> Self::Output {
        FilterAnd(self, rhs)
    }
}
impl<C: SigFilter> BitAnd<C> for SignatureFilter {
    type Output = FilterAnd<SignatureFilter, C>;

    fn bitand(self, rhs: C) -> Self::Output {
        FilterAnd(self, rhs)
    }
}

#[derive(Clone, Copy)]
pub enum AnyOrAll {
    Any,
    All,
}
#[derive(Clone, Copy)]
pub struct SignatureSetFilter<F>(F, AnyOrAll);
impl<F: SigFilter> SigSetFilter for SignatureSetFilter<F> {
    fn filter_sig_set(&self, sigs: &BTreeSet<BasisSignature>) -> bool {
        match self.1 {
            AnyOrAll::Any => sigs.iter().any(|it| self.0.filter_sig(*it)),
            AnyOrAll::All => sigs.iter().all(|it| self.0.filter_sig(*it)),
        }
    }
}

#[derive(Clone, Copy)]
pub struct SetFilterNot<A>(A);
impl<A: SigSetFilter> SigSetFilter for SetFilterNot<A> {
    fn filter_sig_set(&self, sigs: &BTreeSet<BasisSignature>) -> bool {
        !self.0.filter_sig_set(sigs)
    }
}
impl<A: SigSetFilter, B: SigSetFilter> Not for SetFilterOr<A, B> {
    type Output = SetFilterNot<SetFilterOr<A, B>>;

    fn not(self) -> Self::Output {
        SetFilterNot(self)
    }
}
impl<A: SigSetFilter, B: SigSetFilter> Not for SetFilterAnd<A, B> {
    type Output = SetFilterNot<SetFilterAnd<A, B>>;

    fn not(self) -> Self::Output {
        SetFilterNot(self)
    }
}
impl<A: SigSetFilter, B: SigSetFilter> Not for SetFilterXor<A, B> {
    type Output = SetFilterNot<SetFilterXor<A, B>>;

    fn not(self) -> Self::Output {
        SetFilterNot(self)
    }
}
impl<A: SigSetFilter> Not for SetFilterNot<A> {
    type Output = A;

    fn not(self) -> Self::Output {
        self.0
    }
}
impl<F: SigFilter> Not for SignatureSetFilter<F> {
    type Output = SetFilterNot<SignatureSetFilter<F>>;

    fn not(self) -> Self::Output {
        SetFilterNot(self)
    }
}

#[derive(Clone, Copy)]
pub struct SetFilterOr<A, B>(A, B);
impl<A: SigSetFilter, B: SigSetFilter> SigSetFilter for SetFilterOr<A, B> {
    fn filter_sig_set(&self, sigs: &BTreeSet<BasisSignature>) -> bool {
        self.0.filter_sig_set(sigs) || self.1.filter_sig_set(sigs)
    }
}
impl<A: SigSetFilter, B: SigSetFilter, C: SigSetFilter> BitOr<C> for SetFilterOr<A, B> {
    type Output = SetFilterOr<SetFilterOr<A, B>, C>;

    fn bitor(self, rhs: C) -> Self::Output {
        SetFilterOr(self, rhs)
    }
}
impl<A: SigSetFilter, B: SigSetFilter, C: SigSetFilter> BitOr<C> for SetFilterAnd<A, B> {
    type Output = SetFilterOr<SetFilterAnd<A, B>, C>;

    fn bitor(self, rhs: C) -> Self::Output {
        SetFilterOr(self, rhs)
    }
}
impl<A: SigSetFilter, B: SigSetFilter, C: SigSetFilter> BitOr<C> for SetFilterXor<A, B> {
    type Output = SetFilterOr<SetFilterXor<A, B>, C>;

    fn bitor(self, rhs: C) -> Self::Output {
        SetFilterOr(self, rhs)
    }
}
impl<A: SigSetFilter, C: SigSetFilter> BitOr<C> for SetFilterNot<A> {
    type Output = SetFilterOr<SetFilterNot<A>, C>;

    fn bitor(self, rhs: C) -> Self::Output {
        SetFilterOr(self, rhs)
    }
}
impl<F: SigFilter, C: SigSetFilter> BitOr<C> for SignatureSetFilter<F> {
    type Output = SetFilterOr<SignatureSetFilter<F>, C>;

    fn bitor(self, rhs: C) -> Self::Output {
        SetFilterOr(self, rhs)
    }
}

#[derive(Clone, Copy)]
pub struct SetFilterXor<A, B>(A, B);
impl<A: SigSetFilter, B: SigSetFilter> SigSetFilter for SetFilterXor<A, B> {
    fn filter_sig_set(&self, sigs: &BTreeSet<BasisSignature>) -> bool {
        self.0.filter_sig_set(sigs) ^ self.1.filter_sig_set(sigs)
    }
}
impl<A: SigSetFilter, B: SigSetFilter, C: SigSetFilter> BitXor<C> for SetFilterOr<A, B> {
    type Output = SetFilterXor<SetFilterOr<A, B>, C>;

    fn bitxor(self, rhs: C) -> Self::Output {
        SetFilterXor(self, rhs)
    }
}
impl<A: SigSetFilter, B: SigSetFilter, C: SigSetFilter> BitXor<C> for SetFilterAnd<A, B> {
    type Output = SetFilterXor<SetFilterAnd<A, B>, C>;

    fn bitxor(self, rhs: C) -> Self::Output {
        SetFilterXor(self, rhs)
    }
}
impl<A: SigSetFilter, B: SigSetFilter, C: SigSetFilter> BitXor<C> for SetFilterXor<A, B> {
    type Output = SetFilterXor<SetFilterXor<A, B>, C>;

    fn bitxor(self, rhs: C) -> Self::Output {
        SetFilterXor(self, rhs)
    }
}
impl<A: SigSetFilter, C: SigSetFilter> BitXor<C> for SetFilterNot<A> {
    type Output = SetFilterXor<SetFilterNot<A>, C>;

    fn bitxor(self, rhs: C) -> Self::Output {
        SetFilterXor(self, rhs)
    }
}
impl<F: SigFilter, C: SigSetFilter> BitXor<C> for SignatureSetFilter<F> {
    type Output = SetFilterXor<SignatureSetFilter<F>, C>;

    fn bitxor(self, rhs: C) -> Self::Output {
        SetFilterXor(self, rhs)
    }
}

#[derive(Clone, Copy)]
pub struct SetFilterAnd<A, B>(A, B);
impl<A: SigSetFilter, B: SigSetFilter> SigSetFilter for SetFilterAnd<A, B> {
    fn filter_sig_set(&self, sigs: &BTreeSet<BasisSignature>) -> bool {
        self.0.filter_sig_set(sigs) && self.1.filter_sig_set(sigs)
    }
}
impl<A: SigSetFilter, B: SigSetFilter, C: SigSetFilter> BitAnd<C> for SetFilterOr<A, B> {
    type Output = SetFilterAnd<SetFilterOr<A, B>, C>;

    fn bitand(self, rhs: C) -> Self::Output {
        SetFilterAnd(self, rhs)
    }
}
impl<A: SigSetFilter, B: SigSetFilter, C: SigSetFilter> BitAnd<C> for SetFilterAnd<A, B> {
    type Output = SetFilterAnd<SetFilterAnd<A, B>, C>;

    fn bitand(self, rhs: C) -> Self::Output {
        SetFilterAnd(self, rhs)
    }
}
impl<A: SigSetFilter, B: SigSetFilter, C: SigSetFilter> BitAnd<C> for SetFilterXor<A, B> {
    type Output = SetFilterAnd<SetFilterXor<A, B>, C>;

    fn bitand(self, rhs: C) -> Self::Output {
        SetFilterAnd(self, rhs)
    }
}
impl<A: SigSetFilter, C: SigSetFilter> BitAnd<C> for SetFilterNot<A> {
    type Output = SetFilterAnd<SetFilterNot<A>, C>;

    fn bitand(self, rhs: C) -> Self::Output {
        SetFilterAnd(self, rhs)
    }
}
impl<F: SigFilter, C: SigSetFilter> BitAnd<C> for SignatureSetFilter<F> {
    type Output = SetFilterAnd<SignatureSetFilter<F>, C>;

    fn bitand(self, rhs: C) -> Self::Output {
        SetFilterAnd(self, rhs)
    }
}

//

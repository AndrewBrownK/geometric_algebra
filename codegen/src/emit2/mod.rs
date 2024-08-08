use std::collections::{BTreeSet, HashSet};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use anyhow::bail;

use crate::algebra2::basis::BasisElement;
use crate::algebra2::multivector::MultiVec;
use crate::ast2::traits::{BinaryOps, RawTraitDefinition, RawTraitImplementation, TraitKey};
use crate::utility::CollectResults;

pub mod rust;


fn sort_trait_impls(
    trait_implementations: &mut Vec<Arc<RawTraitImplementation>>,
    mut already_ordered: HashSet<TraitKey>,
) -> anyhow::Result<()> {
    // Ok now lets properly order the implementations that
    // are actually declared here.
    let mut needs_ordering: Vec<_> = trait_implementations.clone();
    let mut ordered_implementations = vec![];
    while !needs_ordering.is_empty() {
        let size_before = needs_ordering.len();
        let mut already_disqualified_this_pass = HashSet::new();
        needs_ordering.retain(|it| {
            let k = it.definition.names.trait_key;
            if already_ordered.contains(&k) {
                ordered_implementations.push(it.clone());
                return false
            }
            if already_disqualified_this_pass.contains(&k) {
                return true;
            }
            let deps = it.definition.dependencies.lock();
            if deps.iter().all(|dep| already_ordered.contains(dep)) {
                already_ordered.insert(k);
                ordered_implementations.push(it.clone());
                return false
            }
            already_disqualified_this_pass.insert(k);
            return true
        });
        let size_after = needs_ordering.len();
        if size_before == size_after {
            bail!("There is a missing dependency of a trait implementation. It needs to be \
                included/declared in this file, or else imported to this file.")
        }
    }
    *trait_implementations = ordered_implementations;
    Ok(())
}


pub trait AstEmitter: Copy + Send + Sync + 'static {
    fn file_extension() -> &'static str;
    fn declare_multi_vector<W: Write, const AntiScalar: BasisElement>(
        &self,
        w: &mut W,
        multi_vec: &'static MultiVec<AntiScalar>,
        docs: Option<String>,
    ) -> anyhow::Result<()>;
    fn declare_trait_def<W: Write>(
        &self,
        w: &mut W,
        def: Arc<RawTraitDefinition>,
    ) -> anyhow::Result<()>;
    fn declare_trait_impl<W: Write>(
        &self,
        w: &mut W,
        impls: Arc<RawTraitImplementation>,
        already_granted_infix: &mut BTreeSet<&'static str>
    ) -> anyhow::Result<()>;
    fn emit_comment<W: Write, S: Into<String>>(&self, w: &mut W, is_documentation: bool, s: S) -> anyhow::Result<()>;

    fn format_file<P: AsRef<Path>>(&self, p: P) -> anyhow::Result<()>;
}













//
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::algebra2::basis::BasisElement;
use crate::algebra2::basis::grades::Grades;
use crate::algebra2::multivector::{MultiVec, MultiVecRepository};
use crate::ast2::datatype::MultiVector;
use crate::ast2::traits::{RawTraitDefinition, RawTraitImplementation, TraitImplRegistry, TraitKey};

#[derive(Copy, Clone)]
pub enum ExtensiveFile {
    OverwriteAllInOneFile,
    CustomizableStubWithIncludes,
}
#[derive(Copy, Clone)]
pub enum DataTypesVsTraits {
    Adjacent,
    SeparateFolders,
    OneGinormousFile,
}
#[derive(Copy, Clone)]
pub enum DataTypesBelong {
    AllTogether,
    FilePerGrade,
    FilePerType,
    FilePerGradeThenPerType,
}
#[derive(Copy, Clone)]
pub enum TraitDefsBelong {
    AllTogether,
    FilePerArity,
    FilePerDef,
    FilePerArityThenPerDef,
}
#[derive(Copy, Clone)]
pub enum TraitImplsBelong {
    WithTraitDef,
    WithOwnerType,
}
#[derive(Clone)]
pub struct FileOrganizing {
    pub algebra_name: &'static str,
    pub overall_split: DataTypesVsTraits,
    pub data_types: (DataTypesBelong, ExtensiveFile),
    pub trait_defs: (TraitDefsBelong, ExtensiveFile),
    pub trait_impls: TraitImplsBelong,
    pub override_data_types: HashMap<MultiVector, (DataTypesBelong, ExtensiveFile)>,
    pub override_trait_defs: HashMap<TraitKey,    (TraitDefsBelong, ExtensiveFile)>,
}
impl FileOrganizing {
    pub fn recommended_for_rust(algebra_name: &'static str) -> Self {
        Self {
            algebra_name,
            overall_split: DataTypesVsTraits::Adjacent,
            data_types: (DataTypesBelong::FilePerType, ExtensiveFile::CustomizableStubWithIncludes),
            trait_defs: (TraitDefsBelong::FilePerArityThenPerDef, ExtensiveFile::OverwriteAllInOneFile),
            trait_impls: TraitImplsBelong::WithTraitDef,
            override_data_types: Default::default(),
            override_trait_defs: Default::default(),
        }
    }

    pub fn recommended_for_shaders(algebra_name: &'static str) -> Self {
        Self {
            algebra_name,
            overall_split: DataTypesVsTraits::OneGinormousFile,
            data_types: (DataTypesBelong::AllTogether, ExtensiveFile::OverwriteAllInOneFile),
            trait_defs: (TraitDefsBelong::AllTogether, ExtensiveFile::OverwriteAllInOneFile),
            trait_impls: TraitImplsBelong::WithTraitDef,
            override_data_types: Default::default(),
            override_trait_defs: Default::default(),
        }
    }

    pub async fn go_do_it<P: AsRef<Path>, const AntiScalar: BasisElement>(
        self,
        root: P,
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: TraitImplRegistry,
    ) {
        match self.overall_split {
            DataTypesVsTraits::Adjacent => {

            }
            DataTypesVsTraits::SeparateFolders => {}
            DataTypesVsTraits::OneGinormousFile => {}
        }
    }
}

fn folder_of_grades(gr: Grades) -> &'static str {
    let bits = gr.into_bits();
    // Grade 0 takes 1 bit of grades
    // So grade 0 = 0x1
    // Grade 1 = 0x2
    // and NO GRADES that is to say NOT EVEN GRADE 0 is represented as 0x0
    match bits {
        1 => "scalar",
        2 => "vector",
        4 => "bivector",
        8 => "trivector",
        16 => "quadvector",
        32 => "vector_gr5",
        64 => "vector_gr6",
        128 => "vector_gr7",
        256 => "vector_gr8",
        512 => "vector_gr9",
        1024 => "vector_gr10",
        2048 => "vector_gr11",
        4096 => "vector_gr12",
        8192 => "vector_gr13",
        16384 => "vector_gr14",
        32768 => "vector_gr15",
        65536 => "vector_gr16",
        _ => "mixed_grade"
    }
}


impl IdentifierQualifier for FileOrganizing {
    fn qualifying_path_of_data_type<const AntiScalar: BasisElement>(&self, data_type: &'static MultiVec<AntiScalar>) -> PathBuf {
        let mut path = match self.overall_split {
            DataTypesVsTraits::Adjacent => PathBuf::new(),
            DataTypesVsTraits::SeparateFolders => Path::new("data").to_path_buf(),
            DataTypesVsTraits::OneGinormousFile => PathBuf::new(),
        };
        let mv = MultiVector::from(data_type);
        let (belong, _) = match self.override_data_types.get(&mv) {
            None => self.data_types,
            Some(stuff) => *stuff,
        };
        return match belong {
            DataTypesBelong::AllTogether => path,
            DataTypesBelong::FilePerGrade => {
                let mut gr = Grades::none;
                for el in data_type.elements() {
                    gr |= el.grades();
                }
                path.join(Path::new(folder_of_grades(gr)))
            }
            DataTypesBelong::FilePerType => {
                // Hi if you are reading this line of code because you defined
                // a MultiVec with a non-UpperCamelCase name and the error said
                // something about TraitKeys, but you debugged your way here,
                // sorry for the inconvenience. I'll reorganize camel/snake
                // conversions eventually.
                let n = TraitKey::new(data_type.name).as_lower_snake();
                path.join(Path::new(&n))
            }
            DataTypesBelong::FilePerGradeThenPerType => {
                let mut gr = Grades::none;
                for el in data_type.elements() {
                    gr |= el.grades();
                }
                let n = TraitKey::new(data_type.name).as_lower_snake();
                path.join(Path::new(folder_of_grades(gr))).join(Path::new(&n))
            }
        }
    }

    fn qualifying_path_of_trait_def(&self, trait_def: Arc<RawTraitDefinition>) -> PathBuf {
        let mut path = match self.overall_split {
            DataTypesVsTraits::Adjacent => PathBuf::new(),
            DataTypesVsTraits::SeparateFolders => Path::new("traits").to_path_buf(),
            DataTypesVsTraits::OneGinormousFile => PathBuf::new(),
        };
        let k = trait_def.names.trait_key;
        let (belong, _) = match self.override_trait_defs.get(&k) {
            None => self.trait_defs,
            Some(stuff) => *stuff,
        };
        match belong {
            TraitDefsBelong::AllTogether => path,
            TraitDefsBelong::FilePerArity => {
                match trait_def.arity {
                    0 => path.join(Path::new("arity0")),
                    1 => path.join(Path::new("arity1")),
                    2 => path.join(Path::new("arity2")),
                    _ => panic!("High arity traits are not supported yet")
                }
            }
            TraitDefsBelong::FilePerDef => {
                let n = k.as_lower_snake();
                path.join(Path::new(&n))
            }
            TraitDefsBelong::FilePerArityThenPerDef => {
                path = match trait_def.arity {
                    0 => path.join(Path::new("arity0")),
                    1 => path.join(Path::new("arity1")),
                    2 => path.join(Path::new("arity2")),
                    _ => panic!("High arity traits are not supported yet")
                };
                let n = k.as_lower_snake();
                path.join(Path::new(&n))
            }
        }
    }
}


pub trait IdentifierQualifier {
    fn qualifying_path_of_data_type<const AntiScalar: BasisElement>(&self, data_type: &'static MultiVec<AntiScalar>) -> PathBuf;
    fn qualifying_path_of_trait_def(&self, trait_def: Arc<RawTraitDefinition>) -> PathBuf;
}

pub trait AstEmitter {
    fn emit_multi_vector<W: std::io::Write, Q: IdentifierQualifier, const AntiScalar: BasisElement>(
        w: W,
        q: &Q,
        multi_vec: &'static MultiVec<AntiScalar>,
    );
    fn emit_trait_def<W: std::io::Write, Q: IdentifierQualifier>(
        w: W,
        q: &Q,
        defs: Arc<RawTraitDefinition>,
    );
    fn emit_trait_impl<W: std::io::Write, Q: IdentifierQualifier>(
        w: W,
        q: &Q,
        impls: Arc<RawTraitImplementation>,
    );
}










//
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::bail;
use tokio::task::JoinSet;

use crate::algebra2::basis::BasisElement;
use crate::algebra2::basis::grades::Grades;
use crate::algebra2::multivector::{MultiVec, MultiVecRepository};
use crate::ast2::datatype::{ExpressionType, MultiVector};
use crate::ast2::traits::{RawTraitDefinition, RawTraitImplementation, TraitImplRegistry, TraitKey, TraitParam, TraitTypeConsensus};
use crate::utility::CollectResults;

pub mod rust;

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
    pub override_data_types: Arc<HashMap<MultiVector, (DataTypesBelong, ExtensiveFile)>>,
    pub override_trait_defs: Arc<HashMap<TraitKey,    (TraitDefsBelong, ExtensiveFile)>>,
    pub override_trait_impls: Arc<HashMap<TraitKey,   TraitImplsBelong>>,
}
impl FileOrganizing {
    pub fn recommended_for_rust(algebra_name: &'static str) -> Self {
        Self {
            algebra_name,
            overall_split: DataTypesVsTraits::Adjacent,
            data_types: (DataTypesBelong::FilePerGradeThenPerType, ExtensiveFile::OverwriteAllInOneFile),
            trait_defs: (TraitDefsBelong::FilePerArityThenPerDef, ExtensiveFile::OverwriteAllInOneFile),
            trait_impls: TraitImplsBelong::WithTraitDef,
            override_data_types: Default::default(),
            override_trait_defs: Default::default(),
            override_trait_impls: Arc::new(Default::default()),
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
            override_trait_impls: Arc::new(Default::default()),
        }
    }

    pub async fn go_do_it<P: AsRef<Path> + AsRef<OsStr>, E: AstEmitter, const AntiScalar: BasisElement>(
        self,
        e: E,
        root: P,
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: Arc<TraitImplRegistry>,
    ) -> anyhow::Result<()> {
        let defs = impls.get_defs().await;
        let impls = impls.get_impls().await;
        let mvs = multi_vecs.declarations();

        let root: &Path = root.as_ref();
        fs::create_dir_all(root)?;

        let mut separate_folders = false;
        let (dt_folder, ts_folder) = match self.overall_split {
            DataTypesVsTraits::Adjacent => {
                (root.clone().to_path_buf(), root.clone().to_path_buf())
            },
            DataTypesVsTraits::SeparateFolders => {
                separate_folders = true;
                let dt = root.join(Path::new("data"));
                let ts = root.join(Path::new("traits"));
                (dt, ts)
            }
            DataTypesVsTraits::OneGinormousFile => {
                let mut join_set: JoinSet<anyhow::Result<()>> = JoinSet::new();
                self.write_file_smart(
                    e, &mut join_set, root, self.algebra_name, false, false,
                    mvs, defs, impls,
                ).await?;
                return join_set.collect_results().await
            }
        };

        let mut mv_guide = HashMap::new();
        let mut mv_files: HashMap<
            (PathBuf, String),
            (Vec<&'static MultiVec<AntiScalar>>, Vec<Arc<RawTraitImplementation>>, bool)
        > = HashMap::new();
        for multi_vec in mvs {
            let mv = MultiVector::from(multi_vec);
            let (belong, extensive) = match self.override_data_types.get(&mv) {
                None => self.data_types,
                Some(stuff) => *stuff,
            };
            let mut is_per_type = false;
            let (folder, file) = match belong {
                // Naming a file "mod" is a little biased towards rust, but we'll roll with it for now.
                DataTypesBelong::AllTogether => if separate_folders { (dt_folder.clone(), "mod".to_string()) } else { (root.to_path_buf(), "data".to_string()) },
                DataTypesBelong::FilePerGrade => (dt_folder.clone(), folder_of_grades(multi_vec.grades).to_string()),
                DataTypesBelong::FilePerType => {
                    is_per_type = true;
                    (dt_folder.clone(), TraitKey::new(multi_vec.name).as_lower_snake())
                },
                DataTypesBelong::FilePerGradeThenPerType => {
                    is_per_type = true;
                    (dt_folder.join(Path::new(folder_of_grades(multi_vec.grades))),
                        TraitKey::new(multi_vec.name).as_lower_snake())
                },
            };
            mv_guide.insert(mv, (folder.clone(), file.clone()));
            mv_files.entry((folder, file))
                .and_modify(|(mvs, impls, _)| mvs.push(multi_vec))
                .or_insert((vec![multi_vec], vec![], is_per_type));
        }

        let mut td_guide = HashMap::new();
        let mut td_files: HashMap<
            (PathBuf, String),
            (Vec<Arc<RawTraitDefinition>>, Vec<Arc<RawTraitImplementation>>, bool)
        > = HashMap::new();
        for td in defs {
            let k = td.names.trait_key;
            let (belong, _) = match self.override_trait_defs.get(&k) {
                None => self.trait_defs,
                Some(stuff) => *stuff,
            };
            let arity = match td.arity {
                0 => "arity_0",
                1 => "arity_1",
                2 => "arity_2",
                _ => panic!("High arity traits are not supported yet")
            };

            let mut is_per_trait = false;
            let (folder, file) = match belong {
                // Naming a file "mod" is a little biased towards rust, but we'll roll with it for now.
                TraitDefsBelong::AllTogether => if separate_folders { (ts_folder.clone(), "mod".to_string()) } else { (root.to_path_buf(), "traits".to_string()) },
                TraitDefsBelong::FilePerArity => (ts_folder.clone(), arity.to_string()),
                TraitDefsBelong::FilePerDef => {
                    is_per_trait = true;
                    (ts_folder.clone(), k.as_lower_snake())
                }
                TraitDefsBelong::FilePerArityThenPerDef => {
                    is_per_trait = true;
                    (ts_folder.join(Path::new(arity)), k.as_lower_snake())
                }
            };
            td_guide.insert(k, (folder.clone(), file.clone()));
            let td2 = td.clone();
            td_files.entry((folder, file))
                .and_modify(move |(tds, impls, _)| tds.push(td))
                .or_insert((vec![td2], vec![], is_per_trait));
        }

        for i in impls {
            let k = i.definition.names.trait_key;
            let belong = match self.override_trait_impls.get(&k) {
                None => self.trait_impls,
                Some(stuff) => *stuff,
            };
            match (&i.owner, belong) {
                (TraitParam::Fixed(ExpressionType::Class(mv)), TraitImplsBelong::WithOwnerType) => {
                    // Belongs with owner type
                    let mv_k = match mv_guide.get(&mv) {
                        None => bail!("Owning type should have file arranged already. 1"),
                        Some(k) => k,
                    };
                    let mut stuff = match mv_files.get_mut(mv_k) {
                        None => bail!("Owning type should have file arranged already. 2"),
                        Some(k) => k,
                    };
                    stuff.1.push(i);
                },
                _ => {
                    // Belongs with trait def
                    let td_k = match td_guide.get(&k) {
                        None => bail!("Trait Def should have file arranged already. 1"),
                        Some(k) => k,
                    };
                    let mut stuff = match td_files.get_mut(td_k) {
                        None => bail!("Trait Def should have file arranged already. 2"),
                        Some(k) => k
                    };
                    stuff.1.push(i);
                }
            };
        }

        let mut join_set: JoinSet<anyhow::Result<()>> = JoinSet::new();
        for ((p, n), (mvs, tis, is_per_type)) in mv_files {
            let slf = self.clone();
            join_set.spawn(async move {
                let mut join_set: JoinSet<anyhow::Result<()>> = JoinSet::new();
                slf.write_file_smart(
                    e, &mut join_set, p, n, is_per_type, false,
                    mvs, vec![], tis,
                ).await?;
                join_set.collect_results().await
            });
        }
        for ((p, n), (tds, tis, is_per_trait)) in td_files {
            let slf = self.clone();
            join_set.spawn(async move {
                let mut join_set: JoinSet<anyhow::Result<()>> = JoinSet::new();
                slf.write_file_smart::<PathBuf, String, E, AntiScalar>(
                    e, &mut join_set, p, n, false, is_per_trait,
                    vec![], tds, tis,
                ).await?;
                join_set.collect_results().await
            });
        }
        join_set.collect_results().await
    }

    async fn write_file_smart<
        P: AsRef<Path> + AsRef<OsStr>, S: Into<String>, E: AstEmitter, const AntiScalar: BasisElement
    >(
        &self,
        e: E,
        join_set: &mut JoinSet<anyhow::Result<()>>,
        folder_owning_file: P,
        file_name_no_extension: S,
        is_per_type: bool,
        is_per_trait: bool,
        types: Vec<&'static MultiVec<AntiScalar>>,
        trait_declarations: Vec<Arc<RawTraitDefinition>>,
        trait_implementations: Vec<Arc<RawTraitImplementation>>,
    ) -> anyhow::Result<()> {

        fs::create_dir_all(&folder_owning_file)?;

        let mut use_customizable_stub = false;
        for multi_vec in types.iter() {
            let mv = MultiVector::from(*multi_vec);
            let (_, extensive_file) = match self.override_data_types.get(&mv) {
                None => self.data_types,
                Some(stuff) => *stuff,
            };
            if let ExtensiveFile::CustomizableStubWithIncludes = extensive_file {
                use_customizable_stub = true;
                break;
            }
        }
        if !use_customizable_stub {
            for td in trait_declarations.iter() {
                let k = td.names.trait_key;
                let (_, extensive_file) = match self.override_trait_defs.get(&k) {
                    None => self.trait_defs,
                    Some(stuff) => *stuff,
                };
                if let ExtensiveFile::CustomizableStubWithIncludes = extensive_file {
                    use_customizable_stub = true;
                    break;
                }
            }
        }

        let mut trait_deps = vec![];
        let mut mv_deps = vec![];
        let mut trait_deps_set = HashSet::new();
        let mut mv_deps_set = HashSet::new();
        if e.supports_imports() {
            for td in trait_declarations.iter() {
                let out = td.output.read();
                match *out {
                    TraitTypeConsensus::AllAgree(ExpressionType::Class(mv)) => {
                        let mv: Option<&'static MultiVec<AntiScalar>> = mv.into();
                        let mv = match mv {
                            None => bail!("Must use correct AntiScalar"),
                            Some(mv) => mv,
                        };
                        if mv_deps_set.insert(mv.name) {
                            mv_deps.push(mv);
                        }
                    }
                    _ => {}
                }
            }
            for ti in trait_implementations.iter() {
                for m in ti.multivector_dependencies.iter() {
                    let mv: Option<&'static MultiVec<AntiScalar>> = (*m).into();
                    let mv = match mv {
                        None => bail!("Must use correct AntiScalar"),
                        Some(mv) => mv,
                    };
                    if mv_deps_set.insert(mv.name) {
                        mv_deps.push(mv);
                    }
                }
                for (_, d) in ti.traits10_dependencies.iter() {
                    if trait_deps_set.insert(d.definition.names.trait_key) {
                        trait_deps.push(d.definition.clone());
                    }
                }
                for (_, d) in ti.traits11_dependencies.iter() {
                    if trait_deps_set.insert(d.definition.names.trait_key) {
                        trait_deps.push(d.definition.clone());
                    }
                }
                for (_, d) in ti.traits21_dependencies.iter() {
                    if trait_deps_set.insert(d.definition.names.trait_key) {
                        trait_deps.push(d.definition.clone());
                    }
                }
                for (_, d) in ti.traits22_dependencies.iter() {
                    if trait_deps_set.insert(d.definition.names.trait_key) {
                        trait_deps.push(d.definition.clone());
                    }
                }
            }
        }

        let file_name_no_extension = file_name_no_extension.into();
        let file_name = PathBuf::from(&folder_owning_file)
            .join(Path::new(file_name_no_extension.as_str()))
            .with_extension(E::file_extension());
        if !use_customizable_stub || !e.supports_includes() {
            let slf = self.clone();
            join_set.spawn(async move {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(file_name)?;
                e.emit_comment(
                    &mut file,
                    "AUTO-GENERATED - DO NOT MODIFY BY HAND\n\
                    Changes to this file may be clobbered by code generation at any time.",
                    false
                )?;
                slf.write_file_dumb::<&mut File, E, AntiScalar>(
                    e, &mut file, mv_deps, trait_deps, types, trait_declarations, trait_implementations
                ).await
            });
            return Ok(())
        }
        let maybe_stub_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_name);
        let mut stub_file = match maybe_stub_file {
            Ok(f) => Some(f),
            Err(e) if e.kind() == AlreadyExists => None,
            Err(e) => Err(e)?
        };

        if let Some(stub_file) = &mut stub_file {
            e.emit_comment(
                stub_file,
                "This file may `include` other files in its contents.\n\
                    This file will not be clobbered by code generation, \n\
                    and so can be manually customized or documented by hand.\n\
                    Any `included` files seen across this file WILL be \n\
                    clobbered by code generation.\n\
                    If you accidentally break this file, or the content \n\
                    it relies upon changes, you can regenerate this file \n\
                    from scratch by deleting it first.",
                false
            )?;
            self.write_file_dumb::<&mut File, E, AntiScalar>(
                e, stub_file, mv_deps, trait_deps, vec![], vec![], vec![]
            ).await?;
        }
        for mv in types {
            if let Some(stub_file) = &mut stub_file {
                e.emit_comment(stub_file, "TODO custom documentation", true)?;
            }
            // TODO allow included files to have different root than stub files
            let included_file_name = if is_per_type {
                let mut n = file_name_no_extension.clone();
                n.push_str("_datatype");
                PathBuf::from(&folder_owning_file)
                    .join(Path::new(n.as_str()))
                    .with_extension(E::file_extension())
            } else {
                let mut n = file_name_no_extension.clone();
                n.push_str("_");
                n.push_str(TraitKey::new(mv.name).as_lower_snake().as_str());
                PathBuf::from(&folder_owning_file)
                    .join(Path::new(file_name_no_extension.as_str()))
                    .with_extension(E::file_extension())
            };
            if let Some(stub_file) = &mut stub_file {
                e.include_file(stub_file, &included_file_name)?;
            }
            let slf = self.clone();
            join_set.spawn(async move {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(included_file_name)?;
                slf.write_file_dumb::<&mut File, E, AntiScalar>(
                    e, &mut file, vec![], vec![], vec![mv], vec![], vec![]
                ).await
            });
        }
        for def in trait_declarations {
            if let Some(stub_file) = &mut stub_file {
                e.emit_comment(stub_file, "TODO custom documentation", true)?;
            }
            let included_file_name = if is_per_trait {
                let mut n = file_name_no_extension.clone();
                n.push_str("_def");
                PathBuf::from(&folder_owning_file)
                    .join(Path::new(n.as_str()))
                    .with_extension(E::file_extension())
            } else {
                let mut n = file_name_no_extension.clone();
                n.push_str("_");
                n.push_str(def.names.trait_key.as_lower_snake().as_str());
                PathBuf::from(&folder_owning_file)
                    .join(Path::new(file_name_no_extension.as_str()))
                    .with_extension(E::file_extension())
            };
            if let Some(stub_file) = &mut stub_file {
                e.include_file(stub_file, &included_file_name)?;
            }
            let slf = self.clone();
            join_set.spawn(async move {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(included_file_name)?;
                slf.write_file_dumb::<&mut File, E, AntiScalar>(
                    e, &mut file, vec![], vec![], vec![], vec![def], vec![]
                ).await
            });
        }
        if !trait_implementations.is_empty() {
            let included_file_name = {
                let mut n = file_name_no_extension.clone();
                n.push_str("_impls");
                PathBuf::from(&folder_owning_file)
                    .join(Path::new(file_name_no_extension.as_str()))
                    .with_extension(E::file_extension())
            };
            if let Some(stub_file) = &mut stub_file {
                e.include_file(stub_file, &included_file_name)?;
            }
            let slf = self.clone();
            join_set.spawn(async move {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(included_file_name)?;
                slf.write_file_dumb::<&mut File, E, AntiScalar>(
                    e, &mut file, vec![], vec![], vec![], vec![], trait_implementations
                ).await
            });
        }
        Ok(())
    }

    async fn write_file_dumb<W: Write, E: AstEmitter, const AntiScalar: BasisElement>(
        &self,
        e: E,
        mut file: W,
        type_dependencies: Vec<&'static MultiVec<AntiScalar>>,
        trait_dependencies: Vec<Arc<RawTraitDefinition>>,
        types: Vec<&'static MultiVec<AntiScalar>>,
        trait_declarations: Vec<Arc<RawTraitDefinition>>,
        trait_implementations: Vec<Arc<RawTraitImplementation>>,
    ) -> anyhow::Result<()> {
        // Trait implementations have to be ordered such that dependent
        // items are not declared before their dependencies (for shaders at least).
        // So start by indicating that imports are considered declared.
        let mut already_ordered = HashSet::new();

        // Alright now actually start emitting stuff.
        for dep in type_dependencies {
            e.import_multi_vector(&mut file, self, dep)?;
        }
        for dep in trait_dependencies {
            already_ordered.insert(dep.names.trait_key);
            e.import_trait_def(&mut file, self, dep)?;
        }
        for multi_vec in types {
            e.emit_multi_vector(&mut file, self, multi_vec)?;
        }
        for td in trait_declarations {
            e.emit_trait_def(&mut file, self, td)?;
        }

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

        for ti in ordered_implementations {
            e.emit_trait_impl(&mut file, self, ti)?;
        }
        Ok(())
    }
}

fn folder_of_grades(gr: Grades) -> &'static str {
    let bits = gr.into_bits();
    // Grade 0 takes 1 bit of grades
    // So grade 0 = 0x1
    // Grade 1 = 0x2
    // and NO GRADES that is to say NOT EVEN GRADE 0 is represented as 0x0
    // match bits {
    //     1 => "scalar",
    //     2 => "vector",
    //     4 => "bivector",
    //     8 => "trivector",
    //     16 => "quadvector",
    //     32 => "vector_gr5",
    //     64 => "vector_gr6",
    //     128 => "vector_gr7",
    //     256 => "vector_gr8",
    //     512 => "vector_gr9",
    //     1024 => "vector_gr10",
    //     2048 => "vector_gr11",
    //     4096 => "vector_gr12",
    //     8192 => "vector_gr13",
    //     16384 => "vector_gr14",
    //     32768 => "vector_gr15",
    //     65536 => "vector_gr16",
    //     _ => "mixed_grade"
    // }
    match bits {
        1 => "vector_00",
        2 => "vector_01",
        4 => "vector_02",
        8 => "vector_03",
        16 => "vector_04",
        32 => "vector_05",
        64 => "vector_06",
        128 => "vector_07",
        256 => "vector_08",
        512 => "vector_09",
        1024 => "vector_10",
        2048 => "vector_11",
        4096 => "vector_12",
        8192 => "vector_13",
        16384 => "vector_14",
        32768 => "vector_15",
        65536 => "vector_16",
        _ => "vector_mixed"
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
                path.join(Path::new(folder_of_grades(data_type.grades)))
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
                let n = TraitKey::new(data_type.name).as_lower_snake();
                path.join(Path::new(folder_of_grades(data_type.grades))).join(Path::new(&n))
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
                    0 => path.join(Path::new("arity_0")),
                    1 => path.join(Path::new("arity_1")),
                    2 => path.join(Path::new("arity_2")),
                    _ => panic!("High arity traits are not supported yet")
                }
            }
            TraitDefsBelong::FilePerDef => {
                let n = k.as_lower_snake();
                path.join(Path::new(&n))
            }
            TraitDefsBelong::FilePerArityThenPerDef => {
                path = match trait_def.arity {
                    0 => path.join(Path::new("arity_0")),
                    1 => path.join(Path::new("arity_1")),
                    2 => path.join(Path::new("arity_2")),
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

pub trait AstEmitter: Copy + Send + Sync + 'static {
    fn file_extension() -> &'static str;
    fn supports_includes(&self) -> bool { false }
    fn include_file<W: Write, P: AsRef<Path>>(&self, w: &mut W, p: P) -> anyhow::Result<()> { bail!("Includes are not supported") }
    fn supports_imports(&self) -> bool { false }
    fn import_multi_vector<W: Write, Q: IdentifierQualifier, const AntiScalar: BasisElement>(
        &self,
        w: &mut W,
        q: &Q,
        multi_vec: &'static MultiVec<AntiScalar>,
    ) -> anyhow::Result<()> { bail!("Imports not supported") }
    fn import_trait_def<W: Write, Q: IdentifierQualifier>(
        &self,
        w: &mut W,
        q: &Q,
        def: Arc<RawTraitDefinition>,
    ) -> anyhow::Result<()> { bail!("Imports not supported") }
    fn emit_multi_vector<W: Write, Q: IdentifierQualifier, const AntiScalar: BasisElement>(
        &self,
        w: &mut W,
        q: &Q,
        multi_vec: &'static MultiVec<AntiScalar>,
    ) -> anyhow::Result<()>;
    fn emit_trait_def<W: Write, Q: IdentifierQualifier>(
        &self,
        w: &mut W,
        q: &Q,
        defs: Arc<RawTraitDefinition>,
    ) -> anyhow::Result<()>;
    fn emit_trait_impl<W: Write, Q: IdentifierQualifier>(
        &self,
        w: &mut W,
        q: &Q,
        impls: Arc<RawTraitImplementation>,
    ) -> anyhow::Result<()>;
    fn emit_comment<W: Write, S: Into<String>>(&self, w: &mut W, s: S, is_documentation: bool) -> anyhow::Result<()>;
}













//
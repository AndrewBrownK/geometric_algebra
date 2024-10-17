use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use indicatif::ProgressFinish;
use crate::algebra2::basis::BasisElement;
use crate::algebra2::multivector::{MultiVec, MultiVecRepository};
use crate::ast2::datatype::MultiVector;
use crate::ast2::traits::{progress_style, RawTraitImplementation, TraitImplRegistry, TraitKey};

// Generate WGSL shader source code
#[derive(Copy, Clone)]
pub struct Wgsl;


impl Wgsl {
    pub fn new() -> Self {
        Wgsl
    }

    pub fn write_shader_file<P: AsRef<Path>, const AntiScalar: BasisElement>(
        self,
        src_folder: P,
        algebra_name: &str,
        version_major: usize,
        version_minor: usize,
        version_patch: usize,
        version_pre: &str,
        description: &str,
        repository: &str,
        authors: &[&str],
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: Arc<TraitImplRegistry>,
    ) {
        // So... are we really going to put all the contents into a single source file?
        // It probably sounds crazy at first glance, but the answer is yes.
        // The reason it sounds crazy can be considered from several steps back though, not just
        // for wgsl and shaders, but the rust code generation too.
        //
        // So, the real question is, why is it okay to have several megabyte source files? Or in the
        // case of putting EVERYTHING into one source file like right here, perhaps even gigabytes?
        // Won't things break at some point? Shouldn't there be a limit?
        //
        // The answer is yes. The limit at which things break is unknown at the time of writing.
        // But if you don't put a limit on the number of dimensions of your algebra, the number of
        // multivector types, and the number of traits you want to define, then you're going to have
        // explosive quantities of code. If you don't hold yourself back, then maybe some
        // compiler or file system or VCS will choke at some point. Our purpose here isn't to tell
        // you what that limit should be.... our purpose here is to generate the code, and it's up
        // to you to define your algebra with sane limits that won't inconvenience you or break
        // your build pipeline.
        //
        // Q: So is there a good motivation for having so much code to begin with?
        // A: Yes, as when you 'facet' multivector types down to narrower types, you can get
        //    much more precise geometric information on the type of object you are working with,
        //    and also can avoid unnecessary floating point math that is supposed to cancel to zero.
        //    So what you get is a quadratic quantity of arity 2 trait implementations with respect
        //    to the quantity of multivector types. The quadratic quantity might seem crazy for a
        //    source file, but after it is compiled, you can be sure it is reasonably efficient
        //    floating point math, and every single unused trait implementation will simply not
        //    be included in your binary.
        //
        // Q: HOWEVER, DOESN'T THIS RELY ON THE ASSUMPTION THAT THE CODE IS COMPILED???
        // A: Yes, absolutely. Ideally, compiled ahead of time by a developer, not just-in-time on
        //    some client's machine. It would be utterly insane to send megabytes of javascript
        //    source over the network, or ask a python consumer to compile gigabytes from scratch
        //    every time. So what about shaders? USUALLY and HISTORICALLY shaders are compiled on
        //    the client machine (if I understand correctly, I'm still pretty new to shaders).
        //    However, these days we have some fancy new-ish technology. That is, we can use naga
        //    to compile shaders down to SPIRV. And we can use naga_oil to prune unused content
        //    before doing so. Therefore, by putting the entire algebra into a single source file,
        //    it makes it easy to compile an application's shader with a single dependency (and let
        //    naga_oil sort out the details), rather than wrangling a bajillion files and trying to
        //    plug them into naga_oil manually and hopefully not forget something or mess something
        //    up. In any case, as long as naga_oil doesn't choke, then gigabytes of "shader library
        //    source" can be stripped down to only the parts you use in your "shader application
        //    source", and the final SPIRV artifact should be very reasonably sized to ship to end
        //    users.
        //
        // Q: So what is the limit at which naga_oil will break?
        //    (Not to mention VCS, filesystems, IDEs)
        // A: I don't know, but some daring (courageous? foolhardy?) soul creating algebras here is
        //    likely to be the one who finds out. Or if we never find that limit? Then even better,
        //    and kudos to the team working on naga_oil (and naga too, of course).

        // Alright, enough excuses for this insanity, let's get started

        let src_folder = src_folder.as_ref().to_path_buf();
        let folder_integrations = src_folder.join(Path::new("integrations"));
        fs::create_dir_all(&folder_integrations)?;

        let file_name = format!("{algebra_name}.wgsl");
        let file_path = folder_integrations.join(Path::new(file_name.as_str()));

        let e: anyhow::Result<()> = try {
            let mut file = fs::OpenOptions::new().truncate(true).write(true).open(&file_path)?;
            let mut pre = version_pre.to_string();
            if !pre.is_empty() {
                pre = format!("-{pre}");
            }
            writeln!(&mut file, r#"
// {file_name}
// v{version_major}.{version_minor}.{version_patch}{pre}
// {description}
// authors = ["Andrew Brown <Andrew.Brown.UNL@gmail.com>", "{additional_authors}]
// License: MIWT
//
"#)?;
            let mut mvs = multi_vecs.declarations();
            mvs.sort_by(|a, b| a.name.cmp(&b.name));
            let mvs = mvs;

            let multi_progress = Arc::new(indicatif::MultiProgress::new());
            let qty_mvs = mvs.len() as u64;
            let data_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_mvs).with_finish(ProgressFinish::AndLeave)));
            data_pb.set_style(progress_style());
            data_pb.set_message("WGSL - Data Definitions");

            let qty_impls = impls.len() as u64;
            let impls_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_impls)));
            impls_pb.set_style(progress_style());
            impls_pb.set_message("WGSL - Trait Implementations");

            for multi_vec in mvs.iter() {
                self.declare_multi_vector(&mut file, *multi_vec)?;
                data_pb.inc(1);
            }

            for i in impls {
                self.declare_trait_impl(&mut file, i)?;
                impls_pb.inc(1);
            }

            Ok(())
        };
        if let Err(e) = e {
            panic!("WGSL Errors: {e:?}");
        }
    }

    fn declare_multi_vector<W: Write, const AntiScalar: BasisElement>(
        &self, w: &mut W, multi_vec: &'static MultiVec<AntiScalar>
    ) -> anyhow::Result<()> {
        // TODO
        Ok(())
    }

    fn declare_trait_impl<W: Write>(
        &self, w: &mut W, impls: Arc<RawTraitImplementation>
    ) -> anyhow::Result<()> {
        // TODO
        Ok(())
    }
}

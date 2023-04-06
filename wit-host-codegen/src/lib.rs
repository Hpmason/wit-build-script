use std::{path::{Path, PathBuf}, fs::{self, File}, io::{self, BufWriter, Write}};

use wasmtime_wit_bindgen::Opts;
use wit_parser::Resolve;

#[derive(Default, Clone)]
pub struct GenOptions {
    opts: Opts,
}

impl GenOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rustfmt(&mut self) -> &mut Self {
        self.opts.rustfmt = true;
        self
    }

    pub fn with_async(&mut self) -> &mut Self {
        self.opts.async_ = true;
        self
    }

    pub fn with_tracing(&mut self) -> &mut Self {
        self.opts.tracing = true;
        self
    }

    pub fn build<P: AsRef<Path>>(&mut self, wit_path: P) -> Generator {
        Generator::new(self.clone(), wit_path)
    }
}

pub struct Generator {
    options: GenOptions,
    wit_path: PathBuf,
}

impl Generator {
    pub fn new<P: AsRef<Path>>(options: GenOptions, wit_path: P) -> Self {
        Self { 
            options,
            wit_path: wit_path.as_ref().to_path_buf(),
        }
    }

    pub fn generate<S: AsRef<str>, P: AsRef<Path>>(&self, world: S, output_path: P) -> io::Result<()> {
        let output_path = output_path.as_ref();

        if let Some(parent_folder) = output_path.parent() {
            if !parent_folder.exists() {
                fs::create_dir(parent_folder)?;
            }
        }

        let mut resolve = Resolve::default();
        let (pkg, _wit_files) = resolve.push_dir(&self.wit_path).unwrap();
        let world = resolve.select_world(pkg, Some(world.as_ref())).unwrap();
        let content = self.options.opts.generate(&resolve, world);

        let mut f = BufWriter::new(File::create(output_path)?);
        writeln!(f, "// DO NOT EDIT!!!")?;
        writeln!(f, "// This file automatically generated by {} version {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))?;
        f.write_all(content.as_bytes())?;
        Ok(())
    }
}
use std::{
    collections::HashMap,
    fs::File,
    io::Result,
    io::{Cursor, Error, Write},
    path::{Path, PathBuf},
    process::{Child, Command},
};

use grass_ir::GrassIR;
use serde::Deserialize;
use sha::{sha256::Sha256, utils::DigestExt};
use tempfile::TempDir;

use crate::{
    cache::CacheState,
    dependency::{Dependency, DependencySource},
};

use crate::return_true;

#[derive(Deserialize)]
pub enum BuildFlavor {
    Debug,
    Release,
    ReleaseWithDebugInfo,
}

impl Default for BuildFlavor {
    fn default() -> Self {
        Self::Release
    }
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct JobDefinition {
    ir: Vec<GrassIR>,

    // ################### Runtime confiruation ################
    working_dir: PathBuf,
    #[serde(default)]
    cmdline_args: Vec<String>,
    #[serde(default)]
    env_vars: HashMap<String, String>,
    #[serde(default)]
    const_bag_types: Vec<String>,

    // ############# Runtime Configuration ######################
    #[serde(default = "default_runtime")]
    runtime_package: String,
    #[serde(default = "default_macro")]
    macro_package: String,

    runtime_source: DependencySource,
    macro_source: DependencySource,
   
    #[serde(default = "latest_grass_runtime")]
    runtime_version: String,
    #[serde(default = "latest_grass_runtime")]
    macro_version: String,
    
    #[serde(default)]
    macro_features: Vec<String>,
    #[serde(default)]
    runtime_features: Vec<String>,

    // ################### Package Information ####################
    #[serde(default)]
    deps: Vec<Dependency>,
    #[serde(default = "default_version")]
    version: String,
    #[serde(default)]
    build_flavor: BuildFlavor,
    #[serde(default)]
    temp_dir: Option<PathBuf>,

    // ################### Cache Control #########################
    #[serde(default = "return_true")]
    use_cache: bool,
    #[serde(default = "return_true")]
    update_cache: bool,
    #[serde(default = "default_cache_root")]
    cache_root: PathBuf,

    // ################ Runtime values ####################
    #[serde(default)]
    tool_chain_path: Option<PathBuf>,
    #[serde(skip)]
    compilation_dir: Option<TempDir>,
    #[serde(skip)]
    ir_hash: String,
    #[serde(skip)]
    artifact_path: Option<PathBuf>,
}

fn default_version() -> String {
    "0.0.1".to_string()
}

fn default_runtime() -> String {
    "grass-runtime".to_string()
}

fn default_macro() -> String {
    "grass-macro".to_string()
}

fn latest_grass_runtime() -> String {
    "*".to_string()
}

fn default_cache_root() -> PathBuf {
    let mut ret: PathBuf = std::env::var("HOME")
        .unwrap_or_else(|_| "/".to_string())
        .into();
    ret.push(".grass-ql");
    ret.push("cache");
    ret
}

impl JobDefinition {
    pub fn get_stderr_log(&mut self) -> Result<File> {
        File::open(self.get_compilation_dir()?.join("stderr.log"))
    }
    pub fn cargo(&mut self, args: &[&str], capture_stdout: bool) -> Result<Child> {
        let mut cmd = Command::new("cargo");

        if let Some(tool_chain_path) = self.tool_chain_path.as_ref() {
            cmd.env("PATH", tool_chain_path);
        }

        cmd.args(args);

        let comp_dir = self.get_compilation_dir()?;
        cmd.current_dir(comp_dir);

        let err_log = File::create(comp_dir.join("stderr.log"))?;
        cmd.stderr(err_log);

        if capture_stdout {
            let out_log = File::create(comp_dir.join("stdout.txt"))?;
            cmd.stdout(out_log);
        }

        cmd.spawn()
    }
    fn populate_ir_hash(&mut self) -> Result<()> {
        let mut buffer = Vec::new();
        {
            let mut buffer_writer = Cursor::new(&mut buffer);
            serde_json::to_writer(&mut buffer_writer, &self.ir)?;

            let mut deps = self.deps.clone();
            deps.sort_by_key(|x| x.name.clone());

            serde_json::to_writer(&mut buffer_writer, &deps)?;

            serde_json::to_writer(&mut buffer_writer, &self.macro_package)?;
            serde_json::to_writer(&mut buffer_writer, &self.macro_version)?;
            serde_json::to_writer(&mut buffer_writer, &self.macro_source)?;
            serde_json::to_writer(&mut buffer_writer, &self.macro_features)?;

            serde_json::to_writer(&mut buffer_writer, &self.runtime_package)?;
            serde_json::to_writer(&mut buffer_writer, &self.runtime_version)?;
            serde_json::to_writer(&mut buffer_writer, &self.runtime_source)?;
            serde_json::to_writer(&mut buffer_writer, &self.runtime_features)?;
        }
        let input_str = String::from_utf8(buffer).unwrap();
        log::debug!("Job hasher input: {}", input_str);
        let mut hasher = Sha256::default();

        hasher.write_all(input_str.as_bytes())?;

        hasher.flush()?;

        let ir_hash = hasher.to_hex();

        self.ir_hash = ir_hash;
        Ok(())
    }
    fn get_artifact_hash(&mut self) -> Result<&str> {
        if self.ir_hash.len() == 0 {
            self.populate_ir_hash()?;
        }
        Ok(self.ir_hash.as_str())
    }
    fn get_artifact_name(&mut self) -> Result<String> {
        Ok(format!("grass-artifact-{}", self.get_artifact_hash()?))
    }
    fn get_artifact_path(&mut self) -> Result<PathBuf> {
        let mut ret = self.get_compilation_dir()?.to_path_buf();
        ret.push("target");
        match self.build_flavor {
            BuildFlavor::Debug => ret.push("debug"),
            BuildFlavor::Release | BuildFlavor::ReleaseWithDebugInfo => ret.push("release"),
        }
        ret.push(self.get_artifact_name()?);
        Ok(ret)
    }
    fn write_manifest_file(&mut self, root: &Path) -> Result<()> {
        let manifest_path = root.join("Cargo.toml");
        let mut manifest_file = File::create(&manifest_path)?;
        writeln!(&mut manifest_file, "[package]")?;
        writeln!(
            &mut manifest_file,
            "name = \"{}\"",
            self.get_artifact_name()?
        )?;
        writeln!(&mut manifest_file, "version = \"{}\"", self.version)?;
        writeln!(&mut manifest_file, "edition = \"2021\"")?;
        writeln!(&mut manifest_file, "[dependencies]")?;
        for dep in self.deps.iter().chain([
            &Dependency::create_grass_dep(
                &self.runtime_package,
                &self.runtime_source,
                &self.runtime_version,
                &self.runtime_features,
            ),
            &Dependency::create_grass_dep(
                &self.macro_package,
                &self.macro_source,
                &self.macro_version,
                &self.macro_features,
            ),
        ]) {
            dep.write_dependency_line(&mut manifest_file)?;
        }
        match self.build_flavor {
            BuildFlavor::Release => {
                writeln!(&mut manifest_file, "[profile.release]")?;
                writeln!(&mut manifest_file, "strip = true")?;
            }
            BuildFlavor::ReleaseWithDebugInfo => {
                writeln!(&mut manifest_file, "[profile.release]")?;
                writeln!(&mut manifest_file, "debug = true")?;
            }
            _ => (),
        }
        Ok(())
    }
    fn write_source_code(&self, root: &Path) -> Result<()> {
        let source_dir = root.join("src");
        std::fs::create_dir(&source_dir)?;

        let source_path = source_dir.join("main.rs");
        let mut source_file = File::create(source_path)?;

        writeln!(&mut source_file, "#[allow(unused_imports)]")?;
        writeln!(
            &mut source_file,
            "use grass_runtime::const_bag::{{ConstBagRef, ConstBagType}};"
        )?;

        for (id, t) in self.const_bag_types.iter().enumerate() {
            let ty = match t.as_str() {
                "str" => "String",
                "i64" => "i64",
                "f64" => "f64",
                _ => {
                    panic!("Unsupported const bag type: {}", t);
                }
            };
            writeln!(
                &mut source_file,
                "static __CONST_BAG_VALUE_{id} : ConstBagRef<{ty}> = ConstBagRef::<{ty}>::new({id});",
                id = id,
                ty = ty,
            )?;
        }

        for (id, ir) in self.ir.iter().enumerate() {
            let ir_path = source_dir.as_path().join(format!("grass_ir_{}.json", id));
            let ir_file = File::create(&ir_path)?;
            serde_json::to_writer(ir_file, ir)?;

            writeln!(
                &mut source_file,
                "fn grass_query_{id}(cmd_args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {{",
                id = id
            )?;
            writeln!(
                &mut source_file,
                "    grass_macro::import_grass_ir_from_file!(\"{ir_file}\");",
                ir_file = ir_path.as_os_str().to_string_lossy()
            )?;
            writeln!(&mut source_file, "    Ok(())")?;
            writeln!(&mut source_file, "}}")?;
        }

        writeln!(
            &mut source_file,
            "fn main() -> Result<(), Box<dyn std::error::Error>> {{"
        )?;

        writeln!(
            &mut source_file,
            "    let owned_cmd_args: Vec<_> = std::env::args().collect();"
        )?;

        writeln!(
            &mut source_file,
            "    let cmd_args: Vec<_> = owned_cmd_args.iter().map(|a| a.as_str()).collect();"
        )?;

        for id in 0..self.ir.len() {
            writeln!(
                &mut source_file,
                "    grass_query_{id}(&cmd_args)?;",
                id = id
            )?;
        }
        writeln!(&mut source_file, "    Ok(())")?;
        writeln!(&mut source_file, "}}")?;

        Ok(())
    }
    pub fn get_compilation_dir(&mut self) -> Result<&Path> {
        if self.compilation_dir.is_some() {
            Ok(self.compilation_dir.as_ref().unwrap().as_ref())
        } else {
            let mut root_dir = tempfile::Builder::new();

            root_dir.prefix("grass-workspace-");
            root_dir.rand_bytes(5);
            let compilation_dir = if let Some(temp_dir) = &self.temp_dir {
                root_dir.tempdir_in(temp_dir.as_path())?
            } else {
                root_dir.tempdir()?
            };

            self.write_manifest_file(compilation_dir.path())?;
            self.write_source_code(compilation_dir.path())?;

            self.compilation_dir = Some(compilation_dir);
            Ok(self.compilation_dir.as_ref().unwrap().path())
        }
    }
    pub fn build_artifact(&mut self) -> Result<&Path> {
        log::info!("Building artifact {}", self.get_artifact_name()?);
        let mut child = match self.build_flavor {
            BuildFlavor::Debug => self.cargo(&["build"], true),
            BuildFlavor::Release | BuildFlavor::ReleaseWithDebugInfo => {
                self.cargo(&["build", "--release"], true)
            }
        }?;
        let status = child.wait()?;
        if status.success() {
            self.artifact_path = Some(self.get_artifact_path()?);
            return self.get_artifact();
        }

        Err(Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Cargo exited with error code {}",
                status.code().unwrap_or(0)
            ),
        ))
    }
    pub fn get_artifact(&mut self) -> Result<&Path> {
        if self.artifact_path.is_some() {
            return Ok(self.artifact_path.as_ref().unwrap());
        }

        let mut cache = if self.use_cache || self.update_cache {
            Some(CacheState::load_cache(&self.cache_root)?)
        } else {
            None
        };

        if self.use_cache {
            log::info!(
                "Checking binary cache for binary {}",
                self.get_artifact_hash()?
            );
            let mut cached_artifact = PathBuf::new();
            if cache
                .as_mut()
                .unwrap()
                .query_cache_entry(self.get_artifact_hash()?, &mut cached_artifact)?
            {
                log::info!(
                    "Found cached binary at {}",
                    cached_artifact.to_str().unwrap()
                );
                self.artifact_path = Some(cached_artifact);
                return self.get_artifact();
            }
        }

        if self.update_cache {
            let hash = self.get_artifact_hash()?.to_string();
            let mut cached_path = PathBuf::new();
            cache.as_mut().unwrap().update_cache(
                &hash,
                |buf| {
                    let path = self.build_artifact()?;
                    *buf = path.to_path_buf();
                    Ok(())
                },
                &mut cached_path,
            )?;
            self.artifact_path = Some(cached_path);
            return self.get_artifact();
        }
        self.build_artifact()
    }

    pub fn execute_artifact(&mut self) -> Result<Child> {
        let working_dir = self.working_dir.clone();
        let environment = self.env_vars.clone();
        let cmdline_args = self.cmdline_args.clone();
        let artifact_path = self.get_artifact()?;
        log::info!(
            "Launching artifact {}",
            artifact_path.as_os_str().to_string_lossy()
        );
        log::info!("Working directory: {:?}", working_dir);
        log::info!("Environment vars: {:?}", environment);
        log::info!("Command line arguments: {:?}", cmdline_args);
        Ok(Command::new(artifact_path)
            .current_dir(&self.working_dir)
            .envs(&self.env_vars)
            .args(&self.cmdline_args)
            .spawn()?)
    }
    pub fn print_expanded_code(&mut self) -> Result<()> {
        self.cargo(&["expand"], false)?.wait()?;
        Ok(())
    }
}
